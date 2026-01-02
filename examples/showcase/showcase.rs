#[path = "common.rs"]
pub mod common;

#[path = "i18n_helpers.rs"]
pub mod i18n_helpers;

#[path = "navigation.rs"]
pub mod navigation;

#[path = "views/mod.rs"]
pub mod views;

#[path = "tab_state.rs"]
pub mod tab_state;

use bevy::asset::{AssetPlugin, RenderAssetUsages};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::{keyboard::KeyCode, ButtonInput};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::ui::{ComputedNode, OverflowAxis, ScrollPosition, UiGlobalTransform, UiSystems};
use bevy::window::{PresentMode, PrimaryWindow};
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::InputType;
use bevy_material_ui::theme::ThemeMode;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use common::*;
use navigation::*;
use views::*;

pub use common::ComponentSection;
pub use tab_state::TabStateCache;

use bevy_material_ui::list::MaterialListItem;

#[derive(Component)]
struct SpinningDice;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct SidebarNavScroll;

#[derive(Component)]
struct MainContentScroll;

#[derive(Component)]
struct DetailSurface;

#[derive(Component)]
struct FpsOverlay;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct SettingsDialog;

#[derive(Component)]
struct SettingsVsyncSwitch;

#[derive(Component)]
struct SettingsDialogOkButton;

#[derive(Resource, Default)]
struct PresentModeSettings {
    auto_no_vsync: bool,
}

#[derive(Resource)]
struct ShowcaseI18nAssets {
    #[allow(dead_code)]
    handles: Vec<Handle<MaterialTranslations>>,
}

/// Global font handle for international text support.
///
/// **Required Font**: Noto Sans (or another font with comprehensive Unicode coverage)
///
/// Download from: https://fonts.google.com/noto/specimen/Noto+Sans
/// Place the font file at: `assets/fonts/NotoSans-Regular.ttf`
///
/// This font supports:
/// - Latin (English, Spanish, French, German)
/// - CJK (Chinese, Japanese, Korean)
/// - Hebrew, Arabic, Cyrillic, Greek
/// - And many more scripts
///
/// **Fallback**: If the font file is not found, Bevy's default font will be used,
/// but international characters (Chinese, Japanese, Hebrew, etc.) will not render correctly.
#[derive(Resource, Clone)]
pub struct ShowcaseFont {
    pub latin: Handle<Font>,
    pub cjk: Handle<Font>,
    pub hebrew: Handle<Font>,
}

#[derive(Resource)]
struct SettingsUiEntities {
    dialog: Entity,
    vsync_switch: Entity,
}

#[derive(Resource, Default)]
struct ThemeRebuildGate {
    initialized: bool,
}

#[derive(Resource)]
struct ListDemoOptions {
    mode: ListSelectionMode,
}

impl Default for ListDemoOptions {
    fn default() -> Self {
        Self {
            mode: ListSelectionMode::Single,
        }
    }
}

#[derive(Resource, Default)]
struct DialogDemoOptions {
    position: DialogPosition,
}

pub fn run() {
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: asset_root.to_string_lossy().to_string(),
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(MaterialUiPlugin)
        .init_resource::<ShowcaseThemeSelection>()
        // Default seed theme (Material You purple)
        .insert_resource(MaterialTheme::from_seed(
            Color::srgb_u8(0x67, 0x50, 0xA4),
            ThemeMode::Dark,
        ))
        .init_resource::<SelectedSection>()
        .init_resource::<ComponentTelemetry>()
        .init_resource::<SnackbarDemoOptions>()
        .init_resource::<TooltipDemoOptions>()
        .init_resource::<ListDemoOptions>()
        .init_resource::<DialogDemoOptions>()
        .init_resource::<PresentModeSettings>()
        .add_systems(Startup, load_showcase_i18n_assets_system)
        .init_resource::<TranslationsDemoState>()
        .init_resource::<TranslationsDemoAssets>()
        .init_resource::<TabStateCache>()
        .init_resource::<ThemeRebuildGate>()
        .add_systems(Startup, (setup_3d_scene, setup_ui, setup_telemetry))
        .add_systems(
            Update,
            (
                toggle_language_system,
                rotate_dice,
                handle_nav_clicks,
                update_nav_highlights,
                update_detail_content,
                progress_demo_animate_system,
                demo_click_log_system,
                snackbar_demo_options_system,
                snackbar_demo_trigger_system,
                snackbar_demo_style_system,
                snackbar_demo_action_log_system,
                apply_international_font_system,
                update_font_on_language_change_system,
            ),
        )
        .add_systems(
            Update,
            (
                tooltip_demo_options_system,
                tooltip_demo_apply_system,
                tooltip_demo_style_system,
                menu_demo_system,
                date_picker_demo_system,
                time_picker_demo_system,
                fps_overlay_system,
                settings_button_click_system,
                settings_vsync_toggle_system,
                settings_dialog_ok_close_system,
            ),
        )
        .add_systems(
            Update,
            (
                sidebar_scroll_telemetry_system,
                main_scroll_telemetry_system,
            ),
        )
        .add_systems(Update, email_validation_system)
        .add_systems(
            Update,
            (
                translations_rescan_files_system,
                translations_populate_select_options_system,
                translations_validate_new_filename_system,
                translations_select_change_system,
                translations_create_file_system,
                translations_save_file_system,
            ),
        )
        .add_systems(
            Update,
            (
                dialog_demo_position_options_system,
                dialog_demo_position_style_system,
                dialog_demo_apply_position_system,
                dialog_demo_open_close_system,
                list_demo_mode_options_system,
                list_demo_mode_style_system,
                list_demo_apply_selection_mode_system,
                theme_mode_option_system,
                theme_seed_option_system,
                rebuild_ui_on_theme_change_system,
            ),
        )
        .add_systems(
            Update,
            (
                ensure_automation_test_ids_clickables_system,
                ensure_automation_test_ids_inputs_system,
                ensure_automation_test_ids_overlays_system,
                telemetry_from_component_events_system,
                telemetry_list_selection_state_system,
                telemetry_snapshot_system,
                write_telemetry,
            ),
        )
        // Keep debug probing after UI layout so ComputedNode sizes are meaningful.
        .add_systems(
            PostUpdate,
            debug_lists_visibility_system.after(UiSystems::Layout),
        )
        .run();
}

// ============================================================================
// Translations (i18n) showcase workflow
// ============================================================================

const TRANSLATIONS_SELECT_LABEL_KEY: &str = "showcase.translations.language_file";
const TRANSLATION_KEY_EMAIL_LABEL: &str = "showcase.text_fields.email.label";
const TRANSLATION_KEY_EMAIL_PLACEHOLDER: &str = "showcase.text_fields.email.placeholder";
const TRANSLATION_KEY_EMAIL_SUPPORTING: &str = "showcase.text_fields.email.supporting";

#[derive(Debug, Clone)]
struct TranslationFileEntry {
    asset_path: String,
    file_name: String,
    language_tag: String,
}

#[derive(Resource, Default)]
struct TranslationsDemoState {
    entries: Vec<TranslationFileEntry>,
    selected_asset_path: Option<String>,
    needs_rescan: bool,
}

#[derive(Resource, Default)]
struct TranslationsDemoAssets {
    handles_by_path: HashMap<String, Handle<MaterialTranslations>>,
}

fn translations_assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("i18n")
}

fn parse_translation_file_language(path: &std::path::Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    json.get("language")?.as_str().map(|s| s.to_string())
}

fn parse_translation_file_strings(path: &std::path::Path) -> Option<HashMap<String, String>> {
    let bytes = fs::read(path).ok()?;
    let json: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    let strings = json.get("strings")?.as_object()?;
    let mut out = HashMap::with_capacity(strings.len());
    for (k, v) in strings {
        if let Some(s) = v.as_str() {
            out.insert(k.clone(), s.to_string());
        }
    }
    Some(out)
}

fn translations_rescan_files_system(
    mut state: ResMut<TranslationsDemoState>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }

    // Only scan on a slow cadence unless we just created/saved a file.
    let should_scan = state.needs_rescan || timer.tick(time.delta()).just_finished();
    if !should_scan {
        return;
    }
    state.needs_rescan = false;

    let dir = translations_assets_dir();
    let Ok(read_dir) = fs::read_dir(&dir) else {
        state.entries.clear();
        return;
    };

    let mut entries = Vec::new();
    for item in read_dir.flatten() {
        let path = item.path();
        if path.extension().and_then(|e| e.to_str()) != Some("mui_lang") {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        let Some(language_tag) = parse_translation_file_language(&path) else {
            continue;
        };

        // Asset paths are relative to the asset root.
        let asset_path = format!("i18n/{file_name}");
        entries.push(TranslationFileEntry {
            asset_path,
            file_name,
            language_tag,
        });
    }

    entries.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    state.entries = entries;
}

fn translations_populate_select_options_system(
    state: Res<TranslationsDemoState>,
    mut assets: ResMut<TranslationsDemoAssets>,
    asset_server: Res<AssetServer>,
    mut selects: Query<(
        &mut MaterialSelect,
        &bevy_material_ui::select::SelectLocalization,
    )>,
) {
    // Keep handles alive so the i18n ingest system sees newly created files.
    for entry in state.entries.iter() {
        if assets.handles_by_path.contains_key(&entry.asset_path) {
            continue;
        }
        let handle = asset_server.load::<MaterialTranslations>(entry.asset_path.clone());
        assets
            .handles_by_path
            .insert(entry.asset_path.clone(), handle);
    }

    let Some((mut select, _loc)) = selects
        .iter_mut()
        .find(|(_, loc)| loc.label_key.as_deref() == Some(TRANSLATIONS_SELECT_LABEL_KEY))
    else {
        return;
    };

    let mut options = Vec::with_capacity(state.entries.len());
    for entry in state.entries.iter() {
        options.push(SelectOption::new(entry.file_name.clone()).value(entry.asset_path.clone()));
    }

    select.options = options;

    // Keep selection stable.
    if let Some(selected) = state.selected_asset_path.as_deref() {
        if let Some(idx) = select
            .options
            .iter()
            .position(|o| o.value.as_deref() == Some(selected))
        {
            select.selected_index = Some(idx);
        }
    }
}

fn is_snake_case_file_stem(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    for c in chars {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return false;
        }
    }
    true
}

fn translations_validate_new_filename_system(
    state: Res<TranslationsDemoState>,
    mut name_fields: Query<&mut MaterialTextField, With<views::TranslationsNewFileNameField>>,
    mut create_buttons: Query<
        &mut MaterialButton,
        (
            With<views::TranslationsCreateFileButton>,
            Without<views::TranslationsSaveFileButton>,
        ),
    >,
    mut save_buttons: Query<
        &mut MaterialButton,
        (
            With<views::TranslationsSaveFileButton>,
            Without<views::TranslationsCreateFileButton>,
        ),
    >,
) {
    let Some(mut field) = name_fields.iter_mut().next() else {
        return;
    };
    let Some(mut button) = create_buttons.iter_mut().next() else {
        return;
    };
    let mut save_button = save_buttons.iter_mut().next();

    let raw = field.value.trim();
    if raw.is_empty() {
        field.error = false;
        field.error_text = None;
        button.disabled = true;

        // Allow saving the currently selected file when the new filename input is empty.
        if let Some(save_button) = save_button.as_mut() {
            save_button.disabled = state.selected_asset_path.is_none();
        }
        return;
    }

    let Some(stem) = raw.strip_suffix(".mui_lang") else {
        field.error = true;
        field.error_text = Some("Translation files must end with .mui_lang.".to_string());
        button.disabled = true;

        if let Some(save_button) = save_button.as_mut() {
            save_button.disabled = true;
        }
        return;
    };

    let valid = is_snake_case_file_stem(stem);
    if !valid {
        field.error = true;
        field.error_text = Some("Snake case must be used for translation files.".to_string());
        button.disabled = true;

        if let Some(save_button) = save_button.as_mut() {
            save_button.disabled = true;
        }
        return;
    }

    let file_name = raw;
    let target = translations_assets_dir().join(file_name);
    if target.exists() {
        field.error = true;
        field.error_text = Some("Snake case must be used for translation files.".to_string());
        button.disabled = true;

        if let Some(save_button) = save_button.as_mut() {
            save_button.disabled = true;
        }
        return;
    }

    field.error = false;
    field.error_text = None;
    button.disabled = false;

    if let Some(save_button) = save_button.as_mut() {
        save_button.disabled = state.selected_asset_path.is_none();
    }
}

fn translations_select_change_system(
    mut change_events: MessageReader<SelectChangeEvent>,
    selects: Query<(
        &MaterialSelect,
        &bevy_material_ui::select::SelectLocalization,
    )>,
    mut state: ResMut<TranslationsDemoState>,
    mut language: ResMut<MaterialLanguage>,
    mut editor_fields: ParamSet<(
        Query<&mut MaterialTextField, With<views::TranslationKeyFieldLabel>>,
        Query<&mut MaterialTextField, With<views::TranslationKeyFieldPlaceholder>>,
        Query<&mut MaterialTextField, With<views::TranslationKeyFieldSupporting>>,
    )>,
) {
    for ev in change_events.read() {
        let Ok((_select, loc)) = selects.get(ev.entity) else {
            continue;
        };
        if loc.label_key.as_deref() != Some(TRANSLATIONS_SELECT_LABEL_KEY) {
            continue;
        }

        let Some(asset_path) = ev.option.value.clone() else {
            continue;
        };

        state.selected_asset_path = Some(asset_path.clone());

        // Update language tag to match the file’s declared language.
        if let Some(entry) = state.entries.iter().find(|e| e.asset_path == asset_path) {
            language.tag = entry.language_tag.clone();
        }

        // Hydrate editor fields from disk so edits map 1:1 to file content.
        let disk_path = translations_assets_dir().join(
            asset_path
                .strip_prefix("i18n/")
                .unwrap_or(asset_path.as_str()),
        );
        if let Some(strings) = parse_translation_file_strings(&disk_path) {
            if let Some(mut f) = editor_fields.p0().iter_mut().next() {
                if !f.focused {
                    f.value = strings
                        .get(TRANSLATION_KEY_EMAIL_LABEL)
                        .cloned()
                        .unwrap_or_default();
                    f.has_content = !f.value.is_empty();
                }
            }
            if let Some(mut f) = editor_fields.p1().iter_mut().next() {
                if !f.focused {
                    f.value = strings
                        .get(TRANSLATION_KEY_EMAIL_PLACEHOLDER)
                        .cloned()
                        .unwrap_or_default();
                    f.has_content = !f.value.is_empty();
                }
            }
            if let Some(mut f) = editor_fields.p2().iter_mut().next() {
                if !f.focused {
                    f.value = strings
                        .get(TRANSLATION_KEY_EMAIL_SUPPORTING)
                        .cloned()
                        .unwrap_or_default();
                    f.has_content = !f.value.is_empty();
                }
            }
        }
    }
}

fn translations_create_file_system(
    mut click_events: MessageReader<ButtonClickEvent>,
    create_buttons: Query<(), With<views::TranslationsCreateFileButton>>,
    name_fields: Query<&MaterialTextField, With<views::TranslationsNewFileNameField>>,
    mut editor_fields: ParamSet<(
        Query<&MaterialTextField, With<views::TranslationKeyFieldLabel>>,
        Query<&MaterialTextField, With<views::TranslationKeyFieldPlaceholder>>,
        Query<&MaterialTextField, With<views::TranslationKeyFieldSupporting>>,
    )>,
    mut state: ResMut<TranslationsDemoState>,
    mut language: ResMut<MaterialLanguage>,
    mut i18n: Option<ResMut<MaterialI18n>>,
) {
    for ev in click_events.read() {
        if create_buttons.get(ev.entity).is_err() {
            continue;
        }

        let Some(name_field) = name_fields.iter().next() else {
            continue;
        };
        let file_name = name_field.value.trim();
        let Some(stem) = file_name.strip_suffix(".mui_lang") else {
            continue;
        };
        if !is_snake_case_file_stem(stem) {
            continue;
        }
        let dir = translations_assets_dir();
        let path = dir.join(&file_name);
        if path.exists() {
            continue;
        }

        let Some(label_value) = ({ editor_fields.p0().iter().next().map(|f| f.value.clone()) })
        else {
            continue;
        };
        let Some(placeholder_value) =
            ({ editor_fields.p1().iter().next().map(|f| f.value.clone()) })
        else {
            continue;
        };
        let Some(supporting_value) =
            ({ editor_fields.p2().iter().next().map(|f| f.value.clone()) })
        else {
            continue;
        };

        let mut strings_map = HashMap::new();
        strings_map.insert(TRANSLATION_KEY_EMAIL_LABEL.to_string(), label_value);
        strings_map.insert(
            TRANSLATION_KEY_EMAIL_PLACEHOLDER.to_string(),
            placeholder_value,
        );
        strings_map.insert(
            TRANSLATION_KEY_EMAIL_SUPPORTING.to_string(),
            supporting_value,
        );

        if let Some(i18n) = i18n.as_deref_mut() {
            // Immediately apply without relying on file watching.
            i18n.insert_bundle(stem.to_string(), strings_map.clone());
        }

        let mut strings = serde_json::Map::new();
        for (k, v) in strings_map.iter() {
            strings.insert(k.clone(), serde_json::Value::String(v.clone()));
        }

        let json = serde_json::json!({
            "language": stem,
            "strings": strings,
        });

        let _ = fs::create_dir_all(&dir);
        if fs::write(&path, serde_json::to_vec_pretty(&json).unwrap_or_default()).is_ok() {
            state.needs_rescan = true;
            state.selected_asset_path = Some(format!("i18n/{file_name}"));
            language.tag = stem.to_string();
        }
    }
}

fn translations_save_file_system(
    mut click_events: MessageReader<ButtonClickEvent>,
    save_buttons: Query<(), With<views::TranslationsSaveFileButton>>,
    mut editor_fields: ParamSet<(
        Query<&MaterialTextField, With<views::TranslationKeyFieldLabel>>,
        Query<&MaterialTextField, With<views::TranslationKeyFieldPlaceholder>>,
        Query<&MaterialTextField, With<views::TranslationKeyFieldSupporting>>,
    )>,
    mut state: ResMut<TranslationsDemoState>,
    mut i18n: Option<ResMut<MaterialI18n>>,
) {
    for ev in click_events.read() {
        if save_buttons.get(ev.entity).is_err() {
            continue;
        }

        let Some(asset_path) = state.selected_asset_path.clone() else {
            continue;
        };

        let disk_path = translations_assets_dir().join(
            asset_path
                .strip_prefix("i18n/")
                .unwrap_or(asset_path.as_str()),
        );

        let mut strings = parse_translation_file_strings(&disk_path).unwrap_or_default();

        let Some(label_value) = ({ editor_fields.p0().iter().next().map(|f| f.value.clone()) })
        else {
            continue;
        };
        let Some(placeholder_value) =
            ({ editor_fields.p1().iter().next().map(|f| f.value.clone()) })
        else {
            continue;
        };
        let Some(supporting_value) =
            ({ editor_fields.p2().iter().next().map(|f| f.value.clone()) })
        else {
            continue;
        };

        strings.insert(TRANSLATION_KEY_EMAIL_LABEL.to_string(), label_value);
        strings.insert(
            TRANSLATION_KEY_EMAIL_PLACEHOLDER.to_string(),
            placeholder_value,
        );
        strings.insert(
            TRANSLATION_KEY_EMAIL_SUPPORTING.to_string(),
            supporting_value,
        );

        let language_tag = parse_translation_file_language(&disk_path)
            .or_else(|| {
                disk_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "en-US".to_string());

        let json = serde_json::json!({
            "language": language_tag,
            "strings": strings,
        });

        if fs::write(
            &disk_path,
            serde_json::to_vec_pretty(&json).unwrap_or_default(),
        )
        .is_ok()
        {
            if let Some(i18n) = i18n.as_deref_mut() {
                // Immediately apply without relying on file watching.
                let Some(strings) = parse_translation_file_strings(&disk_path) else {
                    state.needs_rescan = true;
                    continue;
                };
                let language_tag = parse_translation_file_language(&disk_path)
                    .unwrap_or_else(|| "en-US".to_string());
                i18n.insert_bundle(language_tag, strings);
            }
            state.needs_rescan = true;
        }
    }
}

fn load_showcase_i18n_assets_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = vec![
        asset_server.load::<MaterialTranslations>("i18n/en-US.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/es-ES.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/fr-FR.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/de-DE.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/ja-JP.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/zh-CN.mui_lang"),
        asset_server.load::<MaterialTranslations>("i18n/he-IL.mui_lang"),
    ];

    commands.insert_resource(ShowcaseI18nAssets { handles });

    // Load all international fonts.
    info!("📝 Loading international fonts:");
    info!("   - NotoSans-Regular.ttf (Latin: English, Spanish, French, German)");
    info!("   - NotoSansSC-Regular.ttf (CJK: Chinese, Japanese)");
    info!("   - NotoSerifHebrew-Regular.ttf (Hebrew)");

    commands.insert_resource(ShowcaseFont {
        latin: asset_server.load::<Font>("fonts/NotoSans-Regular.ttf"),
        cjk: asset_server.load::<Font>("fonts/NotoSansSC-Regular.ttf"),
        hebrew: asset_server.load::<Font>("fonts/NotoSerifHebrew-Regular.ttf"),
    });
}

fn toggle_language_system(keys: Res<ButtonInput<KeyCode>>, mut language: ResMut<MaterialLanguage>) {
    if keys.just_pressed(KeyCode::KeyL) {
        language.tag = if language.tag == "es-ES" {
            "en-US".to_string()
        } else {
            "es-ES".to_string()
        };

        info!("MaterialLanguage.tag set to '{}'", language.tag);
    }
}

/// Apply international font to text nodes marked with `NeedsInternationalFont`.
///
/// This system runs for each marked text node, waiting until the font has loaded
/// before applying it. Only removes the marker once the font is successfully applied.
fn apply_international_font_system(
    mut commands: Commands,
    font_resource: Option<Res<ShowcaseFont>>,
    fonts: Res<Assets<Font>>,
    language: Res<MaterialLanguage>,
    mut query: Query<(Entity, &mut TextFont), With<common::NeedsInternationalFont>>,
    mut logged: Local<bool>,
) {
    let Some(font_resource) = font_resource else {
        return;
    };

    // Check if all fonts are loaded
    let latin_loaded = fonts.get(&font_resource.latin).is_some();
    let cjk_loaded = fonts.get(&font_resource.cjk).is_some();
    let hebrew_loaded = fonts.get(&font_resource.hebrew).is_some();

    if !latin_loaded || !cjk_loaded || !hebrew_loaded {
        // Fonts not fully loaded yet - keep markers and try again next frame
        return;
    }

    // Log once when all fonts are successfully loaded
    if !*logged {
        let count = query.iter().count();
        info!(
            "✅ All international fonts loaded! Applying to {} text elements",
            count
        );
        *logged = true;
    }

    // Select font based on current language
    let font_handle = match language.tag.as_str() {
        "zh-CN" | "ja-JP" => &font_resource.cjk,
        "he-IL" => &font_resource.hebrew,
        _ => &font_resource.latin, // en-US, es-ES, fr-FR, de-DE
    };

    // Apply appropriate font to all marked entities
    for (entity, mut text_font) in query.iter_mut() {
        text_font.font = font_handle.clone();

        // Remove marker to prevent reprocessing
        commands
            .entity(entity)
            .remove::<common::NeedsInternationalFont>();
    }
}

/// Update fonts when language changes.
///
/// This system runs whenever the language changes and updates all localized text
/// to use the appropriate font for that language.
fn update_font_on_language_change_system(
    font_resource: Option<Res<ShowcaseFont>>,
    fonts: Res<Assets<Font>>,
    language: Res<MaterialLanguage>,
    mut query: Query<&mut TextFont, With<LocalizedText>>,
) {
    // Only run when language actually changes
    if !language.is_changed() {
        return;
    }

    let Some(font_resource) = font_resource else {
        return;
    };

    // Check if all fonts are loaded
    let latin_loaded = fonts.get(&font_resource.latin).is_some();
    let cjk_loaded = fonts.get(&font_resource.cjk).is_some();
    let hebrew_loaded = fonts.get(&font_resource.hebrew).is_some();

    if !latin_loaded || !cjk_loaded || !hebrew_loaded {
        return;
    }

    // Select font based on current language
    let font_handle = match language.tag.as_str() {
        "zh-CN" | "ja-JP" => {
            info!("🔤 Switching to CJK font for {}", language.tag);
            &font_resource.cjk
        }
        "he-IL" => {
            info!("🔤 Switching to Hebrew font");
            &font_resource.hebrew
        }
        _ => {
            info!("🔤 Switching to Latin font for {}", language.tag);
            &font_resource.latin
        }
    };

    // Update all localized text to use the appropriate font
    for mut text_font in query.iter_mut() {
        text_font.font = font_handle.clone();
    }
}

fn fps_overlay_system(
    diagnostics: Res<DiagnosticsStore>,
    theme: Res<MaterialTheme>,
    i18n: Res<MaterialI18n>,
    language: Res<MaterialLanguage>,
    mut fps: Query<(&mut Text, &mut TextColor), With<FpsText>>,
) {
    let Some((mut text, mut color)) = fps.iter_mut().next() else {
        return;
    };

    // Keep it legible across theme changes.
    color.0 = theme.on_surface;

    let fps_value = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed());

    let prefix = i18n
        .translate(&language.tag, "showcase.fps.prefix")
        .unwrap_or("FPS:");

    let label = match fps_value {
        Some(v) if v.is_finite() => format!("{prefix} {v:>5.1}"),
        _ => format!("{prefix}  --.-"),
    };

    *text = Text::new(label);
}

#[derive(Debug)]
struct InsertTestIdIfExists {
    entity: Entity,
    test_id: TestId,
}

impl Command for InsertTestIdIfExists {
    fn apply(self, world: &mut World) {
        if let Ok(mut entity) = world.get_entity_mut(self.entity) {
            // Only insert if still missing; entity may have been rebuilt.
            if entity.get::<TestId>().is_none() {
                entity.insert(self.test_id);
            }
        }
    }
}

fn ensure_automation_test_ids_clickables_system(
    selected: Res<SelectedSection>,
    telemetry: Res<ComponentTelemetry>,
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &UiGlobalTransform), (With<MaterialButton>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialChip>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialFab>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialBadge>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialLinearProgress>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialCircularProgress>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialCard>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialDivider>, Without<TestId>)>,
    )>,
    icons: Query<(Entity, &UiGlobalTransform), (With<MaterialIcon>, Without<TestId>)>,
    icon_buttons: Query<(Entity, &UiGlobalTransform), (With<MaterialIconButton>, Without<TestId>)>,
) {
    if !telemetry.enabled {
        return;
    }

    match selected.current {
        ComponentSection::Buttons => {
            let mut items: Vec<(Entity, f32)> = queries
                .p0()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("button_{}", i)),
                });
            }
        }
        ComponentSection::AppBar => {
            let mut icons: Vec<(Entity, f32)> = icon_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            icons.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in icons.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("app_bar_icon_{}", i)),
                });
            }

            let mut fab_items: Vec<(Entity, f32)> = queries
                .p2()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            fab_items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in fab_items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("app_bar_fab_{}", i)),
                });
            }
        }
        ComponentSection::Chips => {
            let mut items: Vec<(Entity, f32)> = queries
                .p1()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("chip_{}", i)),
                });
            }
        }
        ComponentSection::Fab => {
            let mut items: Vec<(Entity, f32)> = queries
                .p2()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("fab_{}", i)),
                });
            }
        }
        ComponentSection::Badges => {
            let mut items: Vec<(Entity, f32)> = queries
                .p3()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("badge_{}", i)),
                });
            }
        }
        ComponentSection::Progress => {
            let mut linear: Vec<(Entity, f32)> = queries
                .p4()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            linear.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in linear.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("progress_linear_{}", i)),
                });
            }

            let mut circular: Vec<(Entity, f32)> = queries
                .p5()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            circular.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in circular.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("progress_circular_{}", i)),
                });
            }
        }
        ComponentSection::Cards => {
            let mut items: Vec<(Entity, f32)> = queries
                .p6()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("card_{}", i)),
                });
            }
        }
        ComponentSection::Dividers => {
            let mut items: Vec<(Entity, f32)> = queries
                .p7()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("divider_{}", i)),
                });
            }
        }
        ComponentSection::Icons => {
            let mut items: Vec<(Entity, f32)> =
                icons.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("icon_{}", i)),
                });
            }
        }
        ComponentSection::IconButtons => {
            let mut items: Vec<(Entity, f32)> = icon_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("icon_button_{}", i)),
                });
            }
        }
        _ => {}
    }
}

fn ensure_automation_test_ids_inputs_system(
    selected: Res<SelectedSection>,
    telemetry: Res<ComponentTelemetry>,
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &UiGlobalTransform), (With<MaterialCheckbox>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialSwitch>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialRadio>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialSlider>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<SliderTrack>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<SliderHandle>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialTextField>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialSelect>, Without<TestId>)>,
    )>,
    select_options: Query<
        (Entity, &UiGlobalTransform),
        (
            With<bevy_material_ui::select::SelectOptionItem>,
            Without<TestId>,
        ),
    >,
) {
    if !telemetry.enabled {
        return;
    }

    match selected.current {
        ComponentSection::Checkboxes => {
            let mut items: Vec<(Entity, f32)> = queries
                .p0()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("checkbox_{}", i)),
                });
            }
        }
        ComponentSection::Switches => {
            let mut items: Vec<(Entity, f32)> = queries
                .p1()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("switch_{}", i)),
                });
            }
        }
        ComponentSection::RadioButtons => {
            let mut items: Vec<(Entity, f32)> = queries
                .p2()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("radio_{}", i)),
                });
            }
        }
        ComponentSection::Sliders => {
            // Slider root entities (for mapping slider_0_value, etc.)
            let mut items: Vec<(Entity, f32)> = queries
                .p3()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("slider_{}", i)),
                });
            }

            // Slider tracks (used by some tests for direct clicking)
            let mut tracks: Vec<(Entity, f32)> = queries
                .p4()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            tracks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in tracks.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("slider_track_{}", i)),
                });
            }

            // Slider thumbs (used as drag start points)
            let mut thumbs: Vec<(Entity, f32)> = queries
                .p5()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            thumbs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in thumbs.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("slider_thumb_{}", i)),
                });
            }
        }
        ComponentSection::TextFields => {
            let mut items: Vec<(Entity, f32)> = queries
                .p6()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("text_field_{}", i)),
                });
            }
        }
        ComponentSection::Select => {
            let mut roots: Vec<(Entity, f32)> = queries
                .p7()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            roots.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in roots.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("select_{}", i)),
                });
            }

            // Options are spawned when a select is opened; assign IDs when present.
            let mut opts: Vec<(Entity, f32)> = select_options
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opts.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("select_option_{}", i)),
                });
            }
        }
        _ => {}
    }
}

fn ensure_automation_test_ids_overlays_system(
    selected: Res<SelectedSection>,
    telemetry: Res<ComponentTelemetry>,
    mut commands: Commands,
    mut overlays_primary: ParamSet<(
        Query<(Entity, &UiGlobalTransform), (With<ShowDialogButton>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<DialogContainer>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<DialogCloseButton>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<DialogConfirmButton>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<DatePickerOpenButton>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialDatePicker>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<TimePickerOpenButton>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MaterialTimePicker>, Without<TestId>)>,
    )>,
    mut overlays_menu: ParamSet<(
        Query<(Entity, &UiGlobalTransform), (With<MenuTrigger>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MenuDropdown>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<MenuItemMarker>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<SnackbarTrigger>, Without<TestId>)>,
        Query<(Entity, &UiGlobalTransform), (With<TooltipDemoButton>, Without<TestId>)>,
    )>,
) {
    if !telemetry.enabled {
        return;
    }

    match selected.current {
        ComponentSection::Dialogs => {
            let mut opens: Vec<(Entity, f32)> = overlays_primary
                .p0()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opens.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_open_{}", i)),
                });
            }

            let mut containers: Vec<(Entity, f32)> = overlays_primary
                .p1()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            containers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in containers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_container_{}", i)),
                });
            }

            let mut closes: Vec<(Entity, f32)> = overlays_primary
                .p2()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            closes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in closes.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_close_{}", i)),
                });
            }

            let mut confirms: Vec<(Entity, f32)> = overlays_primary
                .p3()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            confirms.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in confirms.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_confirm_{}", i)),
                });
            }
        }
        ComponentSection::DatePicker => {
            let mut opens: Vec<(Entity, f32)> = overlays_primary
                .p4()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opens.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("date_picker_open_{}", i)),
                });
            }

            let mut pickers: Vec<(Entity, f32)> = overlays_primary
                .p5()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            pickers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in pickers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("date_picker_{}", i)),
                });
            }
        }
        ComponentSection::TimePicker => {
            let mut opens: Vec<(Entity, f32)> = overlays_primary
                .p6()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opens.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("time_picker_open_{}", i)),
                });
            }

            let mut pickers: Vec<(Entity, f32)> = overlays_primary
                .p7()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            pickers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in pickers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("time_picker_{}", i)),
                });
            }
        }
        ComponentSection::Menus => {
            let mut triggers: Vec<(Entity, f32)> = overlays_menu
                .p0()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            triggers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in triggers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("menu_trigger_{}", i)),
                });
            }

            let mut dropdowns: Vec<(Entity, f32)> = overlays_menu
                .p1()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            dropdowns.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in dropdowns.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("menu_dropdown_{}", i)),
                });
            }

            let mut items: Vec<(Entity, f32)> = overlays_menu
                .p2()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("menu_item_{}", i)),
                });
            }
        }
        ComponentSection::Snackbar => {
            let mut items: Vec<(Entity, f32)> = overlays_menu
                .p3()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("snackbar_trigger_{}", i)),
                });
            }
        }
        ComponentSection::Tooltips => {
            let mut items: Vec<(Entity, f32)> = overlays_menu
                .p4()
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("tooltip_demo_{}", i)),
                });
            }
        }
        _ => {}
    }
}

fn telemetry_from_component_events_system(
    mut checkbox_events: MessageReader<CheckboxChangeEvent>,
    mut switch_events: MessageReader<SwitchChangeEvent>,
    mut radio_events: MessageReader<RadioChangeEvent>,
    mut slider_events: MessageReader<SliderChangeEvent>,
    mut tab_events: MessageReader<TabChangeEvent>,
    slider_ids: Query<&TestId, With<MaterialSlider>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    for ev in checkbox_events.read() {
        telemetry.log_event(&format!("Checkbox changed: {:?}", ev.entity));
    }
    for ev in switch_events.read() {
        telemetry.log_event(&format!(
            "Switch changed: {:?} -> {}",
            ev.entity, ev.selected
        ));
    }
    for ev in radio_events.read() {
        telemetry.log_event(&format!("Radio changed: {:?}", ev.entity));
    }
    for ev in tab_events.read() {
        telemetry
            .states
            .insert("tab_selected".to_string(), ev.index.to_string());
        telemetry.log_event(&format!("Tab changed: {}", ev.index));
    }
    for ev in slider_events.read() {
        if let Ok(test_id) = slider_ids.get(ev.entity) {
            if let Some(idx) = test_id.id().strip_prefix("slider_") {
                telemetry
                    .states
                    .insert(format!("slider_{}_value", idx), format!("{:.2}", ev.value));
            }
        }
        telemetry.log_event(&format!(
            "Slider changed: {:?} -> {:.2}",
            ev.entity, ev.value
        ));
    }
}

fn telemetry_list_selection_state_system(
    selected: Res<SelectedSection>,
    items_changed: Query<(), (With<SelectableListItem>, Changed<MaterialListItem>)>,
    all_items: Query<(&TestId, &MaterialListItem), With<SelectableListItem>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    if selected.current != ComponentSection::Lists {
        return;
    }

    // Only recompute when something changed OR if the key is missing (first entry).
    let needs_update =
        !items_changed.is_empty() || !telemetry.states.contains_key("list_selected_items");
    if !needs_update {
        return;
    }

    let mut selected_ids: Vec<String> = Vec::new();
    for (test_id, item) in all_items.iter() {
        if item.selected {
            selected_ids.push(test_id.id().to_string());
        }
    }
    selected_ids.sort();

    let list_json = serde_json::to_string(&selected_ids).unwrap_or_else(|_| "[]".to_string());
    telemetry
        .states
        .insert("list_selected_items".to_string(), list_json);
    telemetry.states.insert(
        "list_selected_count".to_string(),
        selected_ids.len().to_string(),
    );
}

fn telemetry_snapshot_system(
    time: Res<Time>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    selected: Res<SelectedSection>,
    tabs: Query<&MaterialTabs>,
    nodes: Query<(&TestId, &ComputedNode, &UiGlobalTransform)>,
    mut telemetry: ResMut<ComponentTelemetry>,
    mut timer: Local<Timer>,
) {
    if !telemetry.enabled {
        return;
    }

    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    // Always keep selected section up-to-date in case it was set elsewhere.
    telemetry.states.insert(
        "selected_section".to_string(),
        selected.current.telemetry_name().to_string(),
    );

    if selected.current == ComponentSection::Tabs {
        if let Some(tabs) = tabs.iter().next() {
            telemetry
                .states
                .insert("tab_selected".to_string(), tabs.selected.to_string());
        }
    }

    if let Some(window) = windows.iter().next() {
        telemetry.states.insert(
            "window_width".to_string(),
            window.resolution.physical_width().to_string(),
        );
        telemetry.states.insert(
            "window_height".to_string(),
            window.resolution.physical_height().to_string(),
        );
    }

    telemetry.elements.clear();

    for (test_id, computed_node, transform) in nodes.iter() {
        let size = computed_node.size();
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }

        // Bevy UI uses physical pixels for `UiGlobalTransform`/`ComputedNode`.
        // Coordinates are in the window's client area.
        let center = transform.translation;
        let x = center.x - size.x / 2.0;
        let y = center.y - size.y / 2.0;

        telemetry.elements.insert(
            test_id.id().to_string(),
            ElementBounds {
                test_id: test_id.id().to_string(),
                x,
                y,
                width: size.x,
                height: size.y,
                parent: None,
            },
        );
    }

    let elements_with_bounds = telemetry.elements.len();
    telemetry.states.insert(
        "elements_with_bounds".to_string(),
        elements_with_bounds.to_string(),
    );
}

fn debug_lists_visibility_system(
    selected: Res<SelectedSection>,
    lists: Query<(Entity, Option<&Children>, Option<&ComputedNode>), With<ListDemoRoot>>,
    scroll_contents: Query<
        (Entity, Option<&Children>, Option<&ComputedNode>),
        With<bevy_material_ui::scroll::ScrollContent>,
    >,
    items: Query<Entity, With<bevy_material_ui::list::MaterialListItem>>,
    icons: Query<(), With<MaterialIcon>>,
    texts: Query<(), With<Text>>,
    test_ids: Query<(Entity, &TestId, Option<&Children>, Option<&ComputedNode>)>,
    children_q: Query<&Children>,
    parents: Query<&ChildOf>,
    scroll_positions: Query<&ScrollPosition>,
    mut did_log: Local<bool>,
    mut attempts: Local<u32>,
) {
    if !cfg!(debug_assertions) {
        return;
    }

    if selected.current != ComponentSection::Lists {
        *did_log = false;
        *attempts = 0;
        return;
    }
    if *did_log {
        return;
    }

    // Try to find the list root. Prefer the marker, but fall back to TestId for robustness.
    let mut list_entity = None;
    let mut list_children = None;
    let mut list_computed = None;
    if let Some((e, c, comp)) = lists.iter().next() {
        list_entity = Some(e);
        list_children = c;
        list_computed = comp;
    } else {
        for (e, id, c, comp) in test_ids.iter() {
            if id.id() == "list_scroll_area" {
                list_entity = Some(e);
                list_children = c;
                list_computed = comp;
                break;
            }
        }
    }

    let Some(list_entity) = list_entity else {
        *attempts += 1;
        if *attempts == 1 || (*attempts % 30 == 0) {
            bevy::log::warn!(
                "[lists debug] List not found yet (attempt {}). UI may not be spawned this frame.",
                *attempts
            );
        }
        return;
    };

    let list_child_count = list_children.map(|c| c.len()).unwrap_or(0);
    let list_size = list_computed.map(|c| c.size());
    let list_scroll = scroll_positions.get(list_entity).ok().map(|p| **p);
    bevy::log::info!(
        "[lists debug] ListDemoRoot={:?} children={} size={:?} scroll={:?}",
        list_entity,
        list_child_count,
        list_size,
        list_scroll
    );

    // Find a ScrollContent descendant (not necessarily direct child).
    let mut content_entity = None;
    let mut stack: Vec<Entity> = vec![list_entity];
    for _ in 0..64 {
        let Some(node) = stack.pop() else { break };
        if scroll_contents.get(node).is_ok() {
            content_entity = Some(node);
            break;
        }
        if let Ok(children) = children_q.get(node) {
            for child in children.iter() {
                stack.push(child);
            }
        }
    }

    if let Some(content) = content_entity {
        let (content_e, content_children, content_computed) =
            scroll_contents.get(content).ok().unwrap();
        let content_child_count = content_children.map(|c| c.len()).unwrap_or(0);
        let content_size = content_computed.map(|c| c.size());
        let content_scroll = scroll_positions.get(content_e).ok().map(|p| **p);
        bevy::log::info!(
            "[lists debug] ScrollContent={:?} children={} size={:?} scroll={:?}",
            content_e,
            content_child_count,
            content_size,
            content_scroll
        );
    } else {
        bevy::log::warn!("[lists debug] No ScrollContent child found under ListDemoRoot (yet)");
    }

    // Count list items that are under this list (walk up parents to see if list is an ancestor).
    let mut item_count = 0usize;
    for entity in items.iter() {
        let mut current = Some(entity);
        for _ in 0..64 {
            let Some(e) = current else { break };
            if e == list_entity {
                item_count += 1;
                break;
            }
            current = parents.get(e).ok().map(|p| p.0);
        }
    }
    bevy::log::info!(
        "[lists debug] MaterialListItem descendants under list: {}",
        item_count
    );

    // Inspect a representative list item so we know whether it has visible content.
    // (If item nodes have non-zero size and contain Text, rendering should be happening.)
    if let Some((item_e, _id, _c, item_comp)) = test_ids
        .iter()
        .find(|(_e, id, _c, _comp)| id.id() == "list_item_0")
    {
        let item_size = item_comp.map(|c| c.size());
        let item_scroll = scroll_positions.get(item_e).ok().map(|p| **p);

        // Count descendants for quick sanity.
        let mut text_count = 0usize;
        let mut icon_count = 0usize;
        let mut stack: Vec<Entity> = vec![item_e];
        for _ in 0..128 {
            let Some(node) = stack.pop() else { break };
            if texts.get(node).is_ok() {
                text_count += 1;
            }
            if icons.get(node).is_ok() {
                icon_count += 1;
            }
            if let Ok(children) = children_q.get(node) {
                for child in children.iter() {
                    stack.push(child);
                }
            }
        }

        bevy::log::info!(
            "[lists debug] list_item_0={:?} size={:?} scroll={:?} text_desc={} icon_desc={}",
            item_e,
            item_size,
            item_scroll,
            text_count,
            icon_count
        );
    }

    *did_log = true;
}

fn sidebar_scroll_telemetry_system(
    sidebar: Query<&ScrollPosition, With<SidebarNavScroll>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    let Some(pos) = sidebar.iter().next() else {
        return;
    };

    telemetry
        .states
        .insert("sidebar_scroll_y".to_string(), (**pos).y.to_string());
    telemetry
        .states
        .insert("sidebar_scroll_x".to_string(), (**pos).x.to_string());
}

fn main_scroll_telemetry_system(
    main: Query<&ScrollPosition, With<MainContentScroll>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    let Some(pos) = main.iter().next() else {
        return;
    };

    telemetry
        .states
        .insert("main_scroll_y".to_string(), (**pos).y.to_string());
    telemetry
        .states
        .insert("main_scroll_x".to_string(), (**pos).x.to_string());
}

fn progress_demo_animate_system(
    time: Res<Time>,
    mut bars: Query<(&mut MaterialLinearProgress, &mut ShowcaseProgressOscillator)>,
    mut labels: Query<&mut Text>,
) {
    for (mut progress, mut osc) in bars.iter_mut() {
        if progress.mode != ProgressMode::Determinate {
            continue;
        }

        let mut value = progress.progress + osc.direction * osc.speed * time.delta_secs();
        if value >= 1.0 {
            value = 1.0;
            osc.direction = -1.0;
        } else if value <= 0.0 {
            value = 0.0;
            osc.direction = 1.0;
        }

        progress.progress = value;

        if let Ok(mut text) = labels.get_mut(osc.label) {
            *text = Text::new(format!("{:>3}%", (value * 100.0).round() as i32));
        }
    }
}

fn argb_to_seed_color(argb: u32) -> Color {
    let r = ((argb >> 16) & 0xFF) as u8;
    let g = ((argb >> 8) & 0xFF) as u8;
    let b = (argb & 0xFF) as u8;
    Color::srgb_u8(r, g, b)
}

fn demo_click_log_system(
    mut icon_clicks: MessageReader<IconButtonClickEvent>,
    mut fab_clicks: MessageReader<FabClickEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for ev in icon_clicks.read() {
        telemetry.log_event(&format!("IconButton clicked: {:?}", ev.entity));
    }
    for ev in fab_clicks.read() {
        telemetry.log_event(&format!("FAB clicked: {:?}", ev.entity));
    }
}

fn list_demo_mode_options_system(
    mut options: ResMut<ListDemoOptions>,
    mut mode_buttons: Query<(&ListSelectionModeOption, &Interaction), Changed<Interaction>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (opt, interaction) in mode_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if options.mode != opt.0 {
            options.mode = opt.0;
            telemetry.log_event("List: selection mode changed");
        }
    }
}

fn list_demo_mode_style_system(
    theme: Res<MaterialTheme>,
    options: Res<ListDemoOptions>,
    mut chips: Query<(&ListSelectionModeOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in chips.iter_mut() {
        chip.selected = opt.0 == options.mode;
    }
}

fn list_demo_apply_selection_mode_system(
    options: Res<ListDemoOptions>,
    lists_added: Query<(), Added<ListDemoRoot>>,
    mut lists: Query<(Entity, &mut bevy_material_ui::list::MaterialList), With<ListDemoRoot>>,
    children_query: Query<&Children>,
    mut items: Query<&mut bevy_material_ui::list::MaterialListItem>,
) {
    if !options.is_changed() && lists_added.is_empty() {
        return;
    }

    for (list_entity, mut list) in lists.iter_mut() {
        list.selection_mode = options.mode;

        // If switching to single-select, ensure at most one item is selected.
        if options.mode == bevy_material_ui::list::ListSelectionMode::Single {
            let mut kept_one = false;
            let mut stack: Vec<Entity> = vec![list_entity];
            while let Some(node) = stack.pop() {
                if let Ok(children) = children_query.get(node) {
                    for child in children.iter() {
                        if let Ok(mut item) = items.get_mut(child) {
                            if item.selected {
                                if kept_one {
                                    item.selected = false;
                                } else {
                                    kept_one = true;
                                }
                            }
                        }
                        stack.push(child);
                    }
                }
            }
        }
    }
}

fn setup_telemetry(mut telemetry: ResMut<ComponentTelemetry>) {
    telemetry.enabled = std::env::var("BEVY_TELEMETRY").is_ok();
    if telemetry.enabled {
        info!("📊 Telemetry enabled - writing to telemetry.json");
        telemetry.log_event("Showcase started");
    }
}

fn write_telemetry(telemetry: Res<ComponentTelemetry>) {
    if telemetry.is_changed() {
        telemetry.write_to_file();
    }
}

fn setup_ui(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    windows: Query<&Window, With<PrimaryWindow>>,
    selected: Res<SelectedSection>,
    tab_cache: Res<TabStateCache>,
    theme_selection: Res<ShowcaseThemeSelection>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
) {
    // UI camera (renders over the 3d scene)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));

    // Icons are embedded and rendered via `MaterialIcon` (no icon font needed).
    let icon_font = Handle::<Font>::default();

    // Global snackbar host overlay (required for ShowSnackbar events to display).
    commands.spawn(SnackbarHostBuilder::build());

    // Persist present mode choice and sync initial UI state from the actual window.
    let auto_no_vsync = windows
        .iter()
        .next()
        .map(|w| matches!(w.present_mode, PresentMode::AutoNoVsync))
        .unwrap_or(false);
    commands.insert_resource(PresentModeSettings { auto_no_vsync });

    // Bottom-right overlay (FPS + Settings).
    commands
        .spawn((
            FpsOverlay,
            GlobalZIndex(100),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                bottom: Val::Px(12.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
        ))
        .with_children(|overlay| {
            overlay.spawn((
                FpsText,
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            // Settings icon button.
            let icon_name = "settings";
            let button =
                MaterialIconButton::new(icon_name).with_variant(IconButtonVariant::Standard);
            let bg_color = button.background_color(&theme);
            let border_color = button.border_color(&theme);
            let icon_color = button.icon_color(&theme);

            overlay
                .spawn((
                    SettingsButton,
                    button,
                    Button,
                    Interaction::None,
                    RippleHost::new(),
                    Node {
                        width: Val::Px(ICON_BUTTON_SIZE),
                        height: Val::Px(ICON_BUTTON_SIZE),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(if border_color == Color::NONE {
                            0.0
                        } else {
                            1.0
                        })),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                    BorderColor::all(border_color),
                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                ))
                .with_children(|btn| {
                    if let Some(icon) = MaterialIcon::from_name(icon_name)
                        .or_else(|| MaterialIcon::from_name("tune"))
                        .or_else(|| MaterialIcon::from_name("settings_applications"))
                    {
                        btn.spawn(icon.with_size(ICON_SIZE).with_color(icon_color));
                    }
                });
        });

    // Settings dialog (initially closed) + linked scrim.
    let mut vsync_switch_entity: Option<Entity> = None;
    let dialog_entity = commands
        .spawn((
            SettingsDialog,
            DialogBuilder::new().title("").modal(true).build(&theme),
        ))
        .with_children(|dialog| {
            // Headline
            dialog.spawn((
                DialogHeadline,
                Text::new(""),
                LocalizedText::new("showcase.settings.title").with_default("Settings"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(theme.on_surface),
                Node {
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            ));

            // Content
            dialog
                .spawn((
                    DialogContent,
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        ..default()
                    },
                ))
                .with_children(|content| {
                    // Row: label + switch
                    content
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new(""),
                                LocalizedText::new("showcase.settings.vsync_mode")
                                    .with_default("AutoNoVsync"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));

                            // Spawn a switch track we can identify via SettingsVsyncSwitch.
                            let switch = MaterialSwitch::new().selected(auto_no_vsync);
                            let bg_color = switch.track_color(&theme);
                            let border_color = switch.track_outline_color(&theme);
                            let handle_color = switch.handle_color(&theme);
                            let handle_size = switch.handle_size();
                            let has_border = !switch.selected;
                            let justify = if switch.selected {
                                JustifyContent::FlexEnd
                            } else {
                                JustifyContent::FlexStart
                            };

                            let track_entity = row
                                .spawn((
                                    SettingsVsyncSwitch,
                                    switch,
                                    Button,
                                    Interaction::None,
                                    RippleHost::new(),
                                    Node {
                                        width: Val::Px(SWITCH_TRACK_WIDTH),
                                        height: Val::Px(SWITCH_TRACK_HEIGHT),
                                        justify_content: justify,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::horizontal(Val::Px(2.0)),
                                        border: UiRect::all(Val::Px(if has_border {
                                            2.0
                                        } else {
                                            0.0
                                        })),
                                        ..default()
                                    },
                                    BackgroundColor(bg_color),
                                    BorderColor::all(border_color),
                                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                ))
                                .with_children(|track| {
                                    track.spawn((
                                        SwitchHandle,
                                        Node {
                                            width: Val::Px(handle_size),
                                            height: Val::Px(handle_size),
                                            ..default()
                                        },
                                        BackgroundColor(handle_color),
                                        BorderRadius::all(Val::Px(handle_size / 2.0)),
                                    ));
                                })
                                .id();

                            vsync_switch_entity = Some(track_entity);
                        });
                });

            // Actions (OK button closes the dialog)
            let ok_label = "";
            let ok_button = MaterialButton::new(ok_label).with_variant(ButtonVariant::Filled);
            let ok_text_color = ok_button.text_color(&theme);

            dialog
                .spawn((
                    DialogActions,
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        margin: UiRect::top(Val::Px(16.0)),
                        ..default()
                    },
                ))
                .with_children(|actions| {
                    actions
                        .spawn((
                            SettingsDialogOkButton,
                            Interaction::None,
                            MaterialButtonBuilder::new(ok_label).filled().build(&theme),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(""),
                                LocalizedText::new("mui.common.ok").with_default("OK"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(ok_text_color),
                            ));
                        });
                });
        })
        .id();

    let scrim_entity = commands
        .spawn(create_dialog_scrim_for(&theme, dialog_entity, true))
        .id();
    commands.entity(scrim_entity).add_child(dialog_entity);

    if let Some(vsync_switch) = vsync_switch_entity {
        commands.insert_resource(SettingsUiEntities {
            dialog: dialog_entity,
            vsync_switch,
        });
    }

    spawn_ui_root(
        &mut commands,
        &theme,
        selected.current,
        icon_font,
        &tab_cache,
        theme_selection.seed_argb,
        &mut materials,
    );
}

fn settings_button_click_system(
    ui: Option<Res<SettingsUiEntities>>,
    settings: Res<PresentModeSettings>,
    mut click_events: MessageReader<IconButtonClickEvent>,
    buttons: Query<(), With<SettingsButton>>,
    mut dialogs: Query<&mut MaterialDialog, With<SettingsDialog>>,
    mut switches: Query<&mut MaterialSwitch, With<SettingsVsyncSwitch>>,
) {
    let Some(ui) = ui else { return };

    for event in click_events.read() {
        if !buttons.contains(event.entity) {
            continue;
        }

        if let Ok(mut dialog) = dialogs.get_mut(ui.dialog) {
            dialog.open = true;
        }

        // Sync toggle UI to actual current state on open.
        if let Ok(mut switch_) = switches.get_mut(ui.vsync_switch) {
            switch_.selected = settings.auto_no_vsync;
            switch_.animation_progress = if switch_.selected { 1.0 } else { 0.0 };
        }
    }
}

fn settings_vsync_toggle_system(
    mut change_events: MessageReader<SwitchChangeEvent>,
    switches: Query<(), With<SettingsVsyncSwitch>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<PresentModeSettings>,
) {
    for event in change_events.read() {
        if !switches.contains(event.entity) {
            continue;
        }

        settings.auto_no_vsync = event.selected;

        let Some(mut window) = windows.iter_mut().next() else {
            continue;
        };

        window.present_mode = if event.selected {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
    }
}

fn settings_dialog_ok_close_system(
    ui: Option<Res<SettingsUiEntities>>,
    mut dialogs: Query<&mut MaterialDialog, With<SettingsDialog>>,
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<SettingsDialogOkButton>)>,
) {
    let Some(ui) = ui else { return };
    let Ok(mut dialog) = dialogs.get_mut(ui.dialog) else {
        return;
    };

    let should_close = interactions.iter_mut().any(|i| *i == Interaction::Pressed);
    if should_close {
        dialog.open = false;
    }
}

fn spawn_ui_root(
    commands: &mut Commands,
    theme: &MaterialTheme,
    selected: ComponentSection,
    icon_font: Handle<Font>,
    tab_cache: &TabStateCache,
    seed_argb: u32,
    materials: &mut Assets<ShapeMorphMaterial>,
) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
        ))
        .with_children(|root| {
            let scaffold = PermanentDrawerScaffold {
                navigation_width_px: 240.0,
                navigation_padding_px: 12.0,
                content_padding_px: 0.0,
                ..default()
            };

            spawn_permanent_drawer_scaffold(
                root,
                theme,
                &scaffold,
                |sidebar| {
                    sidebar.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.app.title")
                            .with_default("Material UI Showcase"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        Node {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                    ));

                    // Scrollable navigation list (real MaterialList + ScrollContainer)
                    sidebar
                        .spawn(ListBuilder::new().build_scrollable())
                        .insert(SidebarNavScroll)
                        .insert(TestId::new("sidebar_scroll_container"))
                        .insert(Node {
                            flex_grow: 1.0,
                            width: Val::Percent(100.0),
                            // Important for scroll containers inside flex columns
                            min_height: Val::Px(0.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            overflow: Overflow::scroll(),
                            ..default()
                        })
                        .with_children(|nav| {
                            for section in ComponentSection::all() {
                                spawn_nav_item(nav, theme, *section, *section == selected);
                            }
                            // Scrollbars spawn automatically
                        });
                },
                |content| {
                    content
                        .spawn((
                            DetailContent,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(16.0)),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(theme.surface),
                        ))
                        .with_children(|detail| {
                            spawn_detail_scroller(
                                detail, theme, selected, icon_font, tab_cache, seed_argb, materials,
                            );
                        });
                },
            );
        });
}

fn spawn_detail_scroller(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    selected: ComponentSection,
    icon_font: Handle<Font>,
    tab_cache: &TabStateCache,
    seed_argb: u32,
    materials: &mut Assets<ShapeMorphMaterial>,
) {
    parent
        .spawn((
            MainContentScroll,
            TestId::new("main_scroll_container"),
            ScrollContainerBuilder::new().both().build(),
            ScrollPosition::default(),
            Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Important for scroll containers inside flex parents
                min_height: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow {
                    x: OverflowAxis::Scroll,
                    y: OverflowAxis::Scroll,
                },
                ..default()
            },
        ))
        .with_children(|scroller| {
            scroller
                .spawn((
                    DetailSurface,
                    Node {
                        // Allow the surface to grow wider than the viewport when needed.
                        // This enables horizontal overflow detection (and thus a horizontal
                        // scrollbar) when content has an intrinsic width larger than the window.
                        width: Val::Auto,
                        min_width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    BorderRadius::all(Val::Px(16.0)),
                ))
                .with_children(|surface| {
                    spawn_selected_section(
                        surface, theme, selected, icon_font, tab_cache, seed_argb, materials,
                    );
                });

            // Scrollbars spawn automatically via ScrollPlugin's ensure_scrollbars_system
        });
}

fn theme_mode_option_system(
    mut theme: ResMut<MaterialTheme>,
    selection: Res<ShowcaseThemeSelection>,
    mut options: Query<(&ThemeModeOption, &Interaction), Changed<Interaction>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (opt, interaction) in options.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if theme.mode != opt.0 {
            *theme = MaterialTheme::from_seed(argb_to_seed_color(selection.seed_argb), opt.0);
            telemetry.log_event("Theme: mode changed");
        }
    }
}

fn theme_seed_option_system(
    mut theme: ResMut<MaterialTheme>,
    mut selection: ResMut<ShowcaseThemeSelection>,
    mut options: Query<(&ThemeSeedOption, &Interaction), Changed<Interaction>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (opt, interaction) in options.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if selection.seed_argb != opt.0 {
            selection.seed_argb = opt.0;
            *theme = MaterialTheme::from_seed(argb_to_seed_color(selection.seed_argb), theme.mode);
            telemetry.log_event("Theme: seed changed");
        }
    }
}

fn is_valid_email(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return true;
    }
    if value.contains(char::is_whitespace) {
        return false;
    }
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    // Very small, demo-oriented check: require at least one dot in the domain.
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn email_validation_system(
    mut changes: MessageReader<TextFieldChangeEvent>,
    mut fields: Query<&mut MaterialTextField>,
) {
    for ev in changes.read() {
        let Ok(mut field) = fields.get_mut(ev.entity) else {
            continue;
        };

        if field.input_type != InputType::Email {
            continue;
        }

        let valid = is_valid_email(&ev.value);
        if valid {
            field.error = false;
            field.error_text = None;
        } else {
            field.error = true;
            field.error_text = Some("Invalid email address".to_string());
        }
    }
}

#[allow(clippy::type_complexity)]
fn menu_demo_system(
    mut triggers: Query<(&ChildOf, &Interaction), (With<MenuTrigger>, Changed<Interaction>)>,
    mut dropdowns: Query<(&ChildOf, &mut Visibility), With<MenuDropdown>>,
    mut items: Query<(&ChildOf, &Interaction, &MenuItemMarker), Changed<Interaction>>,
    triggers_all: Query<(Entity, &ChildOf), With<MenuTrigger>>,
    mut selected_text: Query<(&ChildOf, &mut Text), With<MenuSelectedText>>,
    parents: Query<&ChildOf>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    // Build lookup: container -> trigger entity
    let mut trigger_by_container: std::collections::HashMap<Entity, Entity> =
        std::collections::HashMap::new();
    for (trigger_entity, parent) in triggers_all.iter() {
        trigger_by_container.insert(parent.0, trigger_entity);
    }

    // Toggle dropdown on trigger press
    for (parent, interaction) in triggers.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let container = parent.0;
        for (drop_parent, mut vis) in dropdowns.iter_mut() {
            if drop_parent.0 == container {
                *vis = match *vis {
                    Visibility::Hidden => Visibility::Inherited,
                    _ => Visibility::Hidden,
                };
            }
        }
    }

    // Select item
    for (parent, interaction, label) in items.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Item parent is the dropdown; dropdown parent is the container.
        let dropdown_entity = parent.0;
        let Ok(container_parent) = parents.get(dropdown_entity) else {
            continue;
        };
        let container = container_parent.0;

        // Update selected text on trigger button
        if let Some(trigger_entity) = trigger_by_container.get(&container).copied() {
            for (text_parent, mut text) in selected_text.iter_mut() {
                if text_parent.0 == trigger_entity {
                    *text = Text::new(label.0.as_str());
                }
            }
        }

        // Close dropdown
        for (drop_parent, mut vis) in dropdowns.iter_mut() {
            if drop_parent.0 == container {
                *vis = Visibility::Hidden;
            }
        }

        telemetry.log_event(&format!("Menu: selected {}", label.0));
    }
}

#[allow(clippy::type_complexity)]
fn date_picker_demo_system(
    mut open_buttons: Query<(&Interaction, &DatePickerOpenButton), Changed<Interaction>>,
    mut pickers: ParamSet<(Query<&mut MaterialDatePicker>, Query<&MaterialDatePicker>)>,
    mut submit: MessageReader<DatePickerSubmitEvent>,
    mut cancel: MessageReader<DatePickerCancelEvent>,
    mut result_texts: Query<(&DatePickerResultDisplay, &mut Text)>,
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let prefix = i18n
        .translate(&language.tag, "showcase.common.result_prefix")
        .unwrap_or("Result:")
        .to_string();
    let none = i18n
        .translate(&language.tag, "showcase.common.none")
        .unwrap_or("None")
        .to_string();
    let canceled = i18n
        .translate(&language.tag, "showcase.common.canceled")
        .unwrap_or("Canceled")
        .to_string();
    let to_word = i18n
        .translate(&language.tag, "showcase.date_picker.to")
        .unwrap_or("to")
        .to_string();
    let selecting = i18n
        .translate(&language.tag, "showcase.date_picker.selecting")
        .unwrap_or("(selecting...)")
        .to_string();

    // Open picker when the demo button is pressed.
    for (interaction, open_button) in open_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.p0().get_mut(open_button.0) {
            picker.open = true;
        }
    }

    // Consume picker events (selection state is read directly below).
    for _ in submit.read() {}
    for _ in cancel.read() {}

    // Render the current selection for each result display.
    for (display, mut text) in result_texts.iter_mut() {
        let label = if let Ok(picker) = pickers.p1().get(display.0) {
            match picker.selection() {
                Some(DateSelection::Single(date)) => {
                    format!(
                        "{prefix} {}-{:02}-{:02}",
                        date.year, date.month as u8, date.day
                    )
                }
                Some(DateSelection::Range { start, end }) => {
                    if let Some(end) = end {
                        format!(
                            "{prefix} {}-{:02}-{:02} {to_word} {}-{:02}-{:02}",
                            start.year,
                            start.month as u8,
                            start.day,
                            end.year,
                            end.month as u8,
                            end.day
                        )
                    } else {
                        format!(
                            "{prefix} {}-{:02}-{:02} {selecting}",
                            start.year, start.month as u8, start.day
                        )
                    }
                }
                None => format!("{prefix} {none}"),
            }
        } else {
            format!("{prefix} {canceled}")
        };

        text.0 = label;
    }
}

fn time_picker_demo_system(
    mut open_buttons: Query<(&Interaction, &TimePickerOpenButton), Changed<Interaction>>,
    mut pickers: ParamSet<(Query<&mut MaterialTimePicker>, Query<&MaterialTimePicker>)>,
    mut submit: MessageReader<TimePickerSubmitEvent>,
    mut cancel: MessageReader<TimePickerCancelEvent>,
    mut result_texts: Query<(&TimePickerResultDisplay, &mut Text)>,
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let prefix = i18n
        .translate(&language.tag, "showcase.common.result_prefix")
        .unwrap_or("Result:")
        .to_string();
    let canceled = i18n
        .translate(&language.tag, "showcase.common.canceled")
        .unwrap_or("Canceled")
        .to_string();

    // Open picker when the demo button is pressed.
    for (interaction, open_button) in open_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.p0().get_mut(open_button.0) {
            picker.open = true;
        }
    }

    // Consume picker events (selection state is read directly below).
    for _ in submit.read() {}
    for _ in cancel.read() {}

    // Render the current time for each result display.
    for (display, mut text) in result_texts.iter_mut() {
        let label = if let Ok(picker) = pickers.p1().get(display.0) {
            format!("{prefix} {:02}:{:02}", picker.hour, picker.minute)
        } else {
            format!("{prefix} {canceled}")
        };

        text.0 = label;
    }
}

fn rebuild_ui_on_theme_change_system(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
    tab_cache: Res<TabStateCache>,
    theme_selection: Res<ShowcaseThemeSelection>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    mut gate: ResMut<ThemeRebuildGate>,
    roots: Query<Entity, With<UiRoot>>,
    children_q: Query<&Children>,
) {
    // `MaterialTheme` is inserted during app startup, which marks it as changed.
    // Skip the first tick to avoid rebuilding UI immediately (and causing double-despawn warnings).
    if !gate.initialized {
        gate.initialized = true;
        return;
    }

    if !theme.is_changed() {
        return;
    }

    for root in roots.iter() {
        clear_children_recursive(&mut commands, &children_q, root);
        commands.entity(root).despawn();
    }

    spawn_ui_root(
        &mut commands,
        &theme,
        selected.current,
        Handle::<Font>::default(),
        &tab_cache,
        theme_selection.seed_argb,
        &mut materials,
    );
}

fn snackbar_demo_options_system(
    mut options: ResMut<SnackbarDemoOptions>,
    mut duration_buttons: Query<(&SnackbarDurationOption, &Interaction), Changed<Interaction>>,
    mut action_toggle: Query<&Interaction, (Changed<Interaction>, With<SnackbarActionToggle>)>,
) {
    for (opt, interaction) in duration_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.duration = opt.0;
        }
    }

    for interaction in action_toggle.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.has_action = !options.has_action;
        }
    }
}

fn snackbar_demo_trigger_system(
    options: Res<SnackbarDemoOptions>,
    mut triggers: Query<&Interaction, (Changed<Interaction>, With<SnackbarTrigger>)>,
    mut show: MessageWriter<ShowSnackbar>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for interaction in triggers.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let mut evt = if options.has_action {
            ShowSnackbar::with_action("Item deleted", "UNDO")
        } else {
            ShowSnackbar::message("Item deleted")
        };

        evt.duration = Some(options.duration);
        show.write(evt);
        telemetry.log_event("Snackbar: show");
    }
}

fn snackbar_demo_style_system(
    theme: Res<MaterialTheme>,
    options: Res<SnackbarDemoOptions>,
    mut duration_chips: Query<
        (&SnackbarDurationOption, &mut MaterialChip),
        Without<SnackbarActionToggle>,
    >,
    mut action_toggle_chip: Query<
        &mut MaterialChip,
        (With<SnackbarActionToggle>, Without<SnackbarDurationOption>),
    >,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in duration_chips.iter_mut() {
        chip.selected = (opt.0 - options.duration).abs() < 0.01;
    }

    for mut chip in action_toggle_chip.iter_mut() {
        chip.selected = options.has_action;
    }
}

fn snackbar_demo_action_log_system(
    mut actions: MessageReader<SnackbarActionEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for ev in actions.read() {
        telemetry.log_event(&format!("Snackbar action: {}", ev.action));
    }
}

fn tooltip_demo_options_system(
    mut options: ResMut<TooltipDemoOptions>,
    mut position_buttons: Query<(&TooltipPositionOption, &Interaction), Changed<Interaction>>,
    mut delay_buttons: Query<(&TooltipDelayOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in position_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.position = opt.0;
        }
    }

    for (opt, interaction) in delay_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.delay = opt.0;
        }
    }
}

fn tooltip_demo_apply_system(
    options: Res<TooltipDemoOptions>,
    mut triggers: Query<&mut TooltipTrigger, With<TooltipDemoButton>>,
    mut tooltips: Query<&mut Tooltip>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !options.is_changed() {
        return;
    }

    for mut trigger in triggers.iter_mut() {
        trigger.position = options.position;
        trigger.delay = options.delay;

        // If a tooltip is currently visible, update its position immediately.
        if let Some(tooltip_entity) = trigger.tooltip_entity {
            if let Ok(mut tooltip) = tooltips.get_mut(tooltip_entity) {
                tooltip.position = options.position;
            }
        }
    }

    telemetry.log_event("Tooltip: options changed");
}

fn tooltip_demo_style_system(
    theme: Res<MaterialTheme>,
    options: Res<TooltipDemoOptions>,
    mut position_buttons: Query<
        (
            Entity,
            &TooltipPositionOption,
            &mut MaterialButton,
            &Children,
        ),
        Without<TooltipDelayOption>,
    >,
    mut delay_buttons: Query<
        (Entity, &TooltipDelayOption, &mut MaterialButton, &Children),
        Without<TooltipPositionOption>,
    >,
    mut label_colors: Query<&mut TextColor, With<ButtonLabel>>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (_entity, opt, mut button, children) in position_buttons.iter_mut() {
        let selected = opt.0 == options.position;
        button.variant = if selected {
            ButtonVariant::FilledTonal
        } else {
            ButtonVariant::Outlined
        };

        let text_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = label_colors.get_mut(child) {
                *color = TextColor(text_color);
            }
        }
    }

    for (_entity, opt, mut button, children) in delay_buttons.iter_mut() {
        let selected = (opt.0 - options.delay).abs() < 0.01;
        button.variant = if selected {
            ButtonVariant::FilledTonal
        } else {
            ButtonVariant::Outlined
        };

        let text_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = label_colors.get_mut(child) {
                *color = TextColor(text_color);
            }
        }
    }
}

fn dialog_demo_position_options_system(
    mut options: ResMut<DialogDemoOptions>,
    mut position_buttons: Query<(&DialogPositionOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in position_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.position = opt.0;
        }
    }
}

fn dialog_demo_position_style_system(
    theme: Res<MaterialTheme>,
    options: Res<DialogDemoOptions>,
    mut position_chips: Query<(&DialogPositionOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in position_chips.iter_mut() {
        chip.selected = opt.0 == options.position;
    }
}

fn dialog_demo_apply_position_system(
    options: Res<DialogDemoOptions>,
    dialogs_added: Query<(), Added<DialogContainer>>,
    mut dialogs: Query<&mut Node, With<DialogContainer>>,
) {
    if !options.is_changed() && dialogs_added.is_empty() {
        return;
    }

    for mut node in dialogs.iter_mut() {
        match options.position {
            DialogPosition::CenterParent => {
                node.position_type = PositionType::Relative;
                node.left = Val::Auto;
                node.top = Val::Auto;
                node.right = Val::Auto;
                node.bottom = Val::Auto;
                node.align_self = AlignSelf::Center;
                node.margin = UiRect::vertical(Val::Px(8.0));
            }
            DialogPosition::BelowTrigger => {
                node.position_type = PositionType::Relative;
                node.left = Val::Auto;
                node.top = Val::Auto;
                node.right = Val::Auto;
                node.bottom = Val::Auto;
                node.align_self = AlignSelf::Start;
                node.margin = UiRect::top(Val::Px(12.0));
            }
            DialogPosition::CenterWindow => {
                // Approximate center by anchoring the dialog's top-left near center.
                // (UI centering with translation isn't directly available here.)
                node.position_type = PositionType::Absolute;
                node.left = Val::Percent(50.0);
                node.top = Val::Percent(50.0);
                node.right = Val::Auto;
                node.bottom = Val::Auto;
                node.align_self = AlignSelf::Auto;
                // Dialog width is fixed at 280px in the view; offset half width to better center.
                node.margin = UiRect {
                    left: Val::Px(-140.0),
                    top: Val::Px(-100.0),
                    ..default()
                };
            }
        }
    }
}

fn dialog_demo_open_close_system(
    mut show_buttons: Query<&Interaction, (Changed<Interaction>, With<ShowDialogButton>)>,
    mut close_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogCloseButton>)>,
    mut confirm_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogConfirmButton>)>,
    mut dialogs: Query<(&mut MaterialDialog, Option<&mut Visibility>), With<DialogContainer>>,
    mut result_text: Query<&mut Text, With<DialogResultDisplay>>,
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let prefix = i18n
        .translate(&language.tag, "showcase.common.result_prefix")
        .unwrap_or("Result:")
        .to_string();
    let cancelled = i18n
        .translate(&language.tag, "showcase.dialogs.result.cancelled")
        .unwrap_or("Cancelled")
        .to_string();
    let confirmed = i18n
        .translate(&language.tag, "showcase.dialogs.result.confirmed")
        .unwrap_or("Confirmed")
        .to_string();

    let mut open = false;
    let mut close_reason: Option<String> = None;

    for interaction in show_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            open = true;
        }
    }

    for interaction in close_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_reason = Some(cancelled.clone());
        }
    }

    for interaction in confirm_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_reason = Some(confirmed.clone());
        }
    }

    if open {
        for (mut dialog, maybe_vis) in dialogs.iter_mut() {
            dialog.open = true;
            if let Some(mut vis) = maybe_vis {
                *vis = Visibility::Visible;
            }
        }
    }

    if let Some(reason) = close_reason {
        for (mut dialog, maybe_vis) in dialogs.iter_mut() {
            dialog.open = false;
            if let Some(mut vis) = maybe_vis {
                *vis = Visibility::Visible;
            }
        }
        for mut text in result_text.iter_mut() {
            text.0 = format!("{prefix} {reason}");
        }
    }
}

fn update_detail_content(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
    tab_cache: Res<TabStateCache>,
    theme_selection: Res<ShowcaseThemeSelection>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    detail: Query<Entity, With<DetailContent>>,
    children_q: Query<&Children>,
) {
    if !selected.is_changed() {
        return;
    }

    let Some(detail_entity) = detail.iter().next() else {
        return;
    };

    clear_children_recursive(&mut commands, &children_q, detail_entity);

    let section = selected.current;
    commands.entity(detail_entity).with_children(|detail| {
        spawn_detail_scroller(
            detail,
            &theme,
            section,
            Handle::<Font>::default(),
            &tab_cache,
            theme_selection.seed_argb,
            &mut materials,
        );
    });
}

fn spawn_selected_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
    icon_font: Handle<Font>,
    tab_cache: &TabStateCache,
    seed_argb: u32,
    materials: &mut Assets<ShapeMorphMaterial>,
) {
    match section {
        ComponentSection::Buttons => spawn_buttons_section(parent, theme),
        ComponentSection::Checkboxes => spawn_checkboxes_section(parent, theme, Some(icon_font)),
        ComponentSection::Switches => spawn_switches_section(parent, theme),
        ComponentSection::RadioButtons => spawn_radios_section(parent, theme),
        ComponentSection::Chips => spawn_chips_section(parent, theme, icon_font),
        ComponentSection::Fab => spawn_fab_section(parent, theme, icon_font),
        ComponentSection::Badges => spawn_badges_section(parent, theme, icon_font),
        ComponentSection::Progress => spawn_progress_section(parent, theme),
        ComponentSection::Cards => spawn_cards_section(parent, theme),
        ComponentSection::Dividers => spawn_dividers_section(parent, theme),
        ComponentSection::Lists => spawn_list_section(parent, theme, icon_font),
        ComponentSection::Icons => spawn_icons_section(parent, theme, icon_font),
        ComponentSection::IconButtons => spawn_icon_buttons_section(parent, theme, icon_font),
        ComponentSection::Sliders => spawn_sliders_section(parent, theme),
        ComponentSection::TextFields => spawn_text_fields_section(parent, theme),
        ComponentSection::Dialogs => spawn_dialogs_section(parent, theme),
        ComponentSection::DatePicker => spawn_date_picker_section(parent, theme),
        ComponentSection::TimePicker => spawn_time_picker_section(parent, theme),
        ComponentSection::Menus => spawn_menus_section(parent, theme, icon_font),
        ComponentSection::Tabs => spawn_tabs_section(parent, theme, tab_cache),
        ComponentSection::Select => spawn_select_section(parent, theme, icon_font),
        ComponentSection::Snackbar => spawn_snackbar_section(parent, theme, icon_font),
        ComponentSection::Tooltips => spawn_tooltip_section(parent, theme, icon_font),
        ComponentSection::AppBar => spawn_app_bar_section(parent, theme, icon_font),
        ComponentSection::Toolbar => spawn_toolbar_section(parent, theme, icon_font),
        ComponentSection::Layouts => spawn_layouts_section(parent, theme, icon_font),
        ComponentSection::LoadingIndicator => {
            spawn_loading_indicator_section(parent, theme, materials)
        }
        ComponentSection::Search => spawn_search_section(parent, theme),
        ComponentSection::ThemeColors => spawn_theme_section(parent, theme, seed_argb),
        ComponentSection::Translations => spawn_translations_section(parent, theme),
    }
}

fn clear_children_recursive(
    commands: &mut Commands,
    children_q: &Query<&Children>,
    entity: Entity,
) {
    let Ok(children) = children_q.get(entity) else {
        return;
    };

    for child in children.iter() {
        clear_children_recursive(commands, children_q, child);
        commands.entity(child).despawn();
    }
}

fn rotate_dice(time: Res<Time>, mut dice: Query<&mut Transform, With<SpinningDice>>) {
    for mut transform in dice.iter_mut() {
        transform.rotate_y(time.delta_secs() * 0.8);
        transform.rotate_x(time.delta_secs() * 0.4);
    }
}

fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.08)),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 2500.0,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mesh = meshes.add(create_d10_mesh());
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.22, 0.28),
        metallic: 0.2,
        perceptual_roughness: 0.35,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpinningDice,
    ));
}

fn create_d10_mesh() -> Mesh {
    use std::f32::consts::PI;

    // A D10 is a pentagonal trapezohedron.
    let n: usize = 5;
    let top_height: f32 = 1.2;
    let bottom_height: f32 = -1.2;
    let mid_top: f32 = 0.35;
    let mid_bottom: f32 = -0.35;
    let top_radius: f32 = 0.9;
    let bottom_radius: f32 = 0.9;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let top_point = [0.0, top_height, 0.0];
    let bottom_point = [0.0, bottom_height, 0.0];

    let mut upper_ring: Vec<[f32; 3]> = Vec::with_capacity(n);
    for i in 0..n {
        let angle = (i as f32) * 2.0 * PI / (n as f32);
        upper_ring.push([top_radius * angle.cos(), mid_top, top_radius * angle.sin()]);
    }

    let mut lower_ring: Vec<[f32; 3]> = Vec::with_capacity(n);
    for i in 0..n {
        let angle = ((i as f32) + 0.5) * 2.0 * PI / (n as f32);
        lower_ring.push([
            bottom_radius * angle.cos(),
            mid_bottom,
            bottom_radius * angle.sin(),
        ]);
    }

    for i in 0..n {
        let next_i = (i + 1) % n;
        let prev_i = (i + n - 1) % n;

        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            top_point,
            upper_ring[i],
            lower_ring[i],
        );
        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            top_point,
            lower_ring[i],
            upper_ring[next_i],
        );

        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            bottom_point,
            lower_ring[i],
            upper_ring[i],
        );
        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            bottom_point,
            upper_ring[i],
            lower_ring[prev_i],
        );
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

fn add_triangle(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
) {
    let start = positions.len() as u32;
    positions.push(a);
    positions.push(b);
    positions.push(c);

    let ab = Vec3::from_array(b) - Vec3::from_array(a);
    let ac = Vec3::from_array(c) - Vec3::from_array(a);
    let n = ab.cross(ac).normalize_or_zero().to_array();

    normals.push(n);
    normals.push(n);
    normals.push(n);

    indices.push(start);
    indices.push(start + 1);
    indices.push(start + 2);
}
