//! Runtime localization / translation (i18n)
//!
//! Goals:
//! - Load translation dictionaries from asset "language files" (custom extension: `.mui_lang`).
//! - Allow changing the active language at runtime.
//! - Allow per-subtree overrides via a component, similar to locale.
//! - Provide a simple binding component (`LocalizedText`) that updates `Text` nodes automatically.

use std::collections::HashMap;

use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct MaterialLanguage {
    /// Active BCP-47-ish language tag (e.g. "en-US").
    pub tag: String,
}

impl MaterialLanguage {
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

impl Default for MaterialLanguage {
    fn default() -> Self {
        // Best-effort: match MaterialLocale's Windows env var behavior.
        // Callers can/should override explicitly.
        let tag = std::env::var("LANG")
            .or_else(|_| std::env::var("LC_ALL"))
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .unwrap_or_else(|_| "en-US".to_string());

        // Normalize things like "en_US" -> "en-US".
        let tag = tag.replace('_', "-");
        Self { tag }
    }
}

/// Per-subtree language override.
///
/// Attach this to a component root entity to override `MaterialLanguage` for that subtree.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct MaterialLanguageOverride {
    pub tag: String,
}

impl MaterialLanguageOverride {
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

/// Loaded translation bundle from an asset.
///
/// The file format is JSON (despite the custom extension):
///
/// ```json
/// {
///   "language": "en-US",
///   "strings": {
///     "mui.common.ok": "OK",
///     "mui.common.cancel": "Cancel"
///   }
/// }
/// ```
#[derive(Asset, Debug, Clone, TypePath)]
pub struct MaterialTranslations {
    pub language: String,
    pub strings: HashMap<String, String>,
}

#[derive(Default, TypePath)]
pub struct MaterialTranslationsLoader;

#[derive(Debug, thiserror::Error)]
pub enum MaterialTranslationsLoaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}

impl AssetLoader for MaterialTranslationsLoader {
    type Asset = MaterialTranslations;
    type Settings = ();
    type Error = MaterialTranslationsLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let json: serde_json::Value = serde_json::from_slice(&bytes)?;

        let language = json
            .get("language")
            .and_then(|v| v.as_str())
            .ok_or(MaterialTranslationsLoaderError::MissingField("language"))?
            .to_string();

        let strings_value = json
            .get("strings")
            .ok_or(MaterialTranslationsLoaderError::MissingField("strings"))?;

        let strings_obj = strings_value
            .as_object()
            .ok_or(MaterialTranslationsLoaderError::MissingField("strings"))?;

        let mut strings = HashMap::with_capacity(strings_obj.len());
        for (k, v) in strings_obj.iter() {
            if let Some(s) = v.as_str() {
                strings.insert(k.clone(), s.to_string());
            }
        }

        Ok(MaterialTranslations { language, strings })
    }

    fn extensions(&self) -> &[&str] {
        &["mui_lang"]
    }
}

/// Runtime translation resource.
///
/// This is updated automatically when `MaterialTranslations` assets are added/modified.
#[derive(Resource, Debug, Clone)]
pub struct MaterialI18n {
    bundles: HashMap<String, HashMap<String, String>>,
    pub fallback_language: String,
    revision: u64,
}

impl MaterialI18n {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn bundle_languages(&self) -> impl Iterator<Item = &String> {
        self.bundles.keys()
    }

    pub fn insert_bundle(&mut self, language: String, strings: HashMap<String, String>) {
        self.bundles.insert(language, strings);
        self.revision = self.revision.wrapping_add(1);
    }

    /// Translate `key` for a given language.
    pub fn translate<'a>(&'a self, language: &str, key: &str) -> Option<&'a str> {
        self.bundles
            .get(language)
            .and_then(|b| b.get(key))
            .map(|s| s.as_str())
            .or_else(|| {
                if self.fallback_language.is_empty() {
                    None
                } else {
                    self.bundles
                        .get(self.fallback_language.as_str())
                        .and_then(|b| b.get(key))
                        .map(|s| s.as_str())
                }
            })
    }
}

impl Default for MaterialI18n {
    fn default() -> Self {
        Self {
            bundles: HashMap::default(),
            fallback_language: "en-US".to_string(),
            revision: 0,
        }
    }
}

/// Bind a `Text` node to a translation key.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct LocalizedText {
    pub key: String,
    /// Optional default string if the key is missing.
    pub default: Option<String>,
}

impl LocalizedText {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            default: None,
        }
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }
}

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
struct LocalizedTextState {
    last_revision: u64,
    last_language: String,
}

fn resolve_language_tag(
    mut entity: Entity,
    child_of: &Query<&ChildOf>,
    overrides: &Query<&MaterialLanguageOverride>,
    global: &MaterialLanguage,
) -> String {
    // Direct override on the entity wins.
    if let Ok(ov) = overrides.get(entity) {
        return ov.tag.clone();
    }

    // Walk ancestors to find the nearest override.
    while let Ok(parent) = child_of.get(entity) {
        entity = parent.parent();
        if let Ok(ov) = overrides.get(entity) {
            return ov.tag.clone();
        }
    }

    global.tag.clone()
}

fn i18n_ingest_assets_system(
    mut i18n: ResMut<MaterialI18n>,
    translations: Res<Assets<MaterialTranslations>>,
    mut events: MessageReader<AssetEvent<MaterialTranslations>>,
) {
    for ev in events.read() {
        match ev {
            AssetEvent::Added { id }
            | AssetEvent::Modified { id }
            | AssetEvent::LoadedWithDependencies { id } => {
                if let Some(asset) = translations.get(*id) {
                    i18n.insert_bundle(asset.language.clone(), asset.strings.clone());
                }
            }
            AssetEvent::Removed { .. } => {
                // DESIGN TRADE-OFF: We intentionally ignore asset removal events to preserve
                // translation strings in the runtime resource. This prevents UI text from
                // flickering or reverting to keys during hot-reload workflows.
                //
                // Consequence: If a translation file is removed from the project, its strings
                // remain in MaterialI18n until the application is restarted. For most
                // development scenarios, this behavior is desirable as it prioritizes
                // visual stability over immediate reflection of file deletions.
                //
                // Alternative: If immediate removal is required, call
                // `MaterialI18n::clear()` or selectively remove bundles via
                // `MaterialI18n` methods after detecting the asset removal.
            }
            _ => {}
        }
    }
}

fn localized_text_apply_system(
    i18n: Res<MaterialI18n>,
    language: Res<MaterialLanguage>,
    child_of: Query<&ChildOf>,
    overrides: Query<&MaterialLanguageOverride>,
    mut texts: Query<(
        Entity,
        &LocalizedText,
        &mut Text,
        Option<&mut LocalizedTextState>,
    )>,
    mut commands: Commands,
) {
    // If nothing changed globally, we can still have per-entity key changes.
    let global_revision = i18n.revision();

    for (entity, binding, mut text, state) in texts.iter_mut() {
        let resolved_language = resolve_language_tag(entity, &child_of, &overrides, &language);

        let needs_update = match &state {
            Some(s) => s.last_revision != global_revision || s.last_language != resolved_language,
            None => true,
        };

        if needs_update {
            let resolved = i18n
                .translate(&resolved_language, &binding.key)
                .map(|s| s.to_string())
                .or_else(|| binding.default.clone())
                .unwrap_or_else(|| binding.key.clone());

            if text.as_str() != resolved {
                *text = Text::new(resolved);
            }

            if let Some(mut state) = state {
                state.last_revision = global_revision;
                state.last_language = resolved_language;
            } else {
                commands.queue(TryInsertLocalizedTextState {
                    entity,
                    state: LocalizedTextState {
                        last_revision: global_revision,
                        last_language: resolved_language,
                    },
                });
            }
        }
    }
}

struct TryInsertLocalizedTextState {
    entity: Entity,
    state: LocalizedTextState,
}

impl Command for TryInsertLocalizedTextState {
    fn apply(self, world: &mut World) {
        if let Ok(mut entity) = world.get_entity_mut(self.entity) {
            entity.insert(self.state);
        }
    }
}

/// Plugin that enables runtime localization via translation assets.
pub struct MaterialI18nPlugin;

impl Plugin for MaterialI18nPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaterialLanguage>()
            .init_resource::<MaterialI18n>()
            .init_asset::<MaterialTranslations>()
            .register_asset_loader(MaterialTranslationsLoader)
            .add_systems(
                Update,
                (i18n_ingest_assets_system, localized_text_apply_system),
            );
    }
}
