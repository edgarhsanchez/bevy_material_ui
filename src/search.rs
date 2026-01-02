//! Material Design 3 Search Bar and Search View
//!
//! The Search Bar represents a floating search field with affordances for search and navigation.
//! Reference: <https://m3.material.io/components/search/overview>

use bevy::prelude::*;

use crate::{
    i18n::{MaterialI18n, MaterialLanguage, MaterialLanguageOverride},
    icons::{IconStyle, MaterialIcon},
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for search components
pub struct SearchPlugin;

impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<SearchBarClickEvent>()
            .add_message::<SearchQueryEvent>()
            .add_systems(
                Update,
                (
                    search_bar_interaction_system,
                    search_bar_localization_system,
                    search_bar_display_text_system,
                )
                    .chain(),
            );
    }
}

// ============================================================================
// Events
// ============================================================================

/// Event fired when the search bar is clicked
#[derive(Event, bevy::prelude::Message, Clone)]
pub struct SearchBarClickEvent {
    pub search_bar: Entity,
}

/// Event fired when search query changes
#[derive(Event, bevy::prelude::Message, Clone)]
pub struct SearchQueryEvent {
    pub search_bar: Entity,
    pub query: String,
}

// ============================================================================
// Components
// ============================================================================

/// Search Bar component
#[derive(Component)]
pub struct MaterialSearchBar {
    /// Hint text shown when empty
    pub hint: String,
    /// Current search text
    pub text: String,
    /// Navigation icon (usually menu or back)
    pub navigation_icon: Option<MaterialIcon>,
    /// Whether to show the trailing action icon
    pub show_action: bool,
}

impl MaterialSearchBar {
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            hint: hint.into(),
            text: String::new(),
            navigation_icon: None,
            show_action: true,
        }
    }

    pub fn with_navigation(mut self, icon: MaterialIcon) -> Self {
        self.navigation_icon = Some(icon);
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

/// Marker for search bar navigation button
#[derive(Component)]
pub struct SearchBarNavigation;

/// Marker for search bar action button
#[derive(Component)]
pub struct SearchBarAction;

/// Marker for search bar text container
#[derive(Component)]
pub struct SearchBarTextContainer;

/// Optional localization keys for a search bar.
#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub struct SearchBarLocalization {
    pub hint_key: Option<String>,
    pub text_key: Option<String>,
}

/// Tracks the last i18n revision/language used to resolve a search bar.
#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
struct SearchBarLocalizationState {
    last_revision: u64,
    last_language: String,
}

/// Marker for the search bar's displayed text node.
#[derive(Component)]
pub struct SearchBarDisplayText;

/// Links a displayed text node to its owning search bar.
#[derive(Component)]
pub struct SearchBarDisplayTextFor(pub Entity);

// ============================================================================
// Constants
// ============================================================================

pub const SEARCH_BAR_HEIGHT: f32 = 56.0;

// ============================================================================
// Builder
// ============================================================================

pub struct SearchBarBuilder {
    search_bar: MaterialSearchBar,
    localization: SearchBarLocalization,
}

impl SearchBarBuilder {
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            search_bar: MaterialSearchBar::new(hint),
            localization: SearchBarLocalization::default(),
        }
    }

    /// Set the hint from an i18n key.
    pub fn hint_key(mut self, key: impl Into<String>) -> Self {
        self.search_bar.hint = String::new();
        self.localization.hint_key = Some(key.into());
        self
    }

    /// Set the initial text from an i18n key.
    pub fn text_key(mut self, key: impl Into<String>) -> Self {
        self.search_bar.text = String::new();
        self.localization.text_key = Some(key.into());
        self
    }

    pub fn with_navigation(mut self, icon: MaterialIcon) -> Self {
        self.search_bar.navigation_icon = Some(icon);
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.search_bar.text = text.into();
        self
    }

    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        (
            self.search_bar,
            self.localization,
            SearchBarLocalizationState::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(SEARCH_BAR_HEIGHT),
                padding: UiRect::horizontal(Val::Px(Spacing::EXTRA_SMALL)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::SMALL),
                ..default()
            },
            BackgroundColor(theme.surface_container_high),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
            RippleHost::new(),
            Button,
            Interaction::None,
        )
    }
}

// ============================================================================
// Spawn Trait
// ============================================================================

pub trait SpawnSearchBarChild {
    fn spawn_search_bar(&mut self, theme: &MaterialTheme, hint: impl Into<String>);

    fn spawn_search_bar_with(&mut self, theme: &MaterialTheme, builder: SearchBarBuilder);
}

impl SpawnSearchBarChild for ChildSpawnerCommands<'_> {
    fn spawn_search_bar(&mut self, theme: &MaterialTheme, hint: impl Into<String>) {
        self.spawn_search_bar_with(theme, SearchBarBuilder::new(hint));
    }

    fn spawn_search_bar_with(&mut self, theme: &MaterialTheme, builder: SearchBarBuilder) {
        let hint = builder.search_bar.hint.clone();
        let text = builder.search_bar.text.clone();
        let nav_icon = builder.search_bar.navigation_icon;
        let has_text = !text.is_empty();

        let mut cmds = self.spawn(builder.build(theme));
        let bar_entity = cmds.id();

        cmds.with_children(|bar| {
            // Navigation icon
            if let Some(icon) = nav_icon {
                bar.spawn((
                    SearchBarNavigation,
                    Button,
                    RippleHost::new(),
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        icon,
                        IconStyle::outlined()
                            .with_color(theme.on_surface)
                            .with_size(24.0),
                    ));
                });
            }

            // Text container
            bar.spawn((
                SearchBarTextContainer,
                Node {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|container| {
                let initial = if has_text { &text } else { &hint };
                let color = if has_text {
                    theme.on_surface
                } else {
                    theme.on_surface_variant
                };

                container.spawn((
                    SearchBarDisplayText,
                    SearchBarDisplayTextFor(bar_entity),
                    Text::new(initial),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(color),
                ));
            });

            // Trailing action (search icon)
            bar.spawn((
                SearchBarAction,
                Button,
                RippleHost::new(),
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(CornerRadius::FULL)),
            ))
            .with_children(|btn| {
                if let Some(icon) = MaterialIcon::from_name("search") {
                    btn.spawn((
                        icon,
                        IconStyle::outlined()
                            .with_color(theme.on_surface_variant)
                            .with_size(24.0),
                    ));
                }
            });
        });
    }
}

fn resolve_language_tag_for_entity(
    mut entity: Entity,
    child_of: &Query<&ChildOf>,
    overrides: &Query<&MaterialLanguageOverride>,
    global: &MaterialLanguage,
) -> String {
    if let Ok(ov) = overrides.get(entity) {
        return ov.tag.clone();
    }

    while let Ok(parent) = child_of.get(entity) {
        entity = parent.parent();
        if let Ok(ov) = overrides.get(entity) {
            return ov.tag.clone();
        }
    }

    global.tag.clone()
}

fn search_bar_localization_system(
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
    child_of: Query<&ChildOf>,
    overrides: Query<&MaterialLanguageOverride>,
    mut bars: Query<(
        Entity,
        &SearchBarLocalization,
        &mut MaterialSearchBar,
        &mut SearchBarLocalizationState,
    )>,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let global_revision = i18n.revision();

    for (entity, loc, mut bar, mut state) in bars.iter_mut() {
        if loc.hint_key.is_none() && loc.text_key.is_none() {
            continue;
        }

        let resolved_language =
            resolve_language_tag_for_entity(entity, &child_of, &overrides, &language);
        let needs_update =
            state.last_revision != global_revision || state.last_language != resolved_language;
        if !needs_update {
            continue;
        }

        if let Some(key) = loc.hint_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if bar.hint != next {
                    bar.hint = next;
                }
            }
        }

        if let Some(key) = loc.text_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if bar.text != next {
                    bar.text = next;
                }
            }
        }

        state.last_revision = global_revision;
        state.last_language = resolved_language;
    }
}

fn search_bar_display_text_system(
    theme: Option<Res<MaterialTheme>>,
    bars: Query<&MaterialSearchBar>,
    mut display_texts: Query<(&SearchBarDisplayTextFor, &mut Text, &mut TextColor)>,
) {
    let Some(theme) = theme else {
        return;
    };

    for (owner, mut text, mut color) in display_texts.iter_mut() {
        let Ok(bar) = bars.get(owner.0) else {
            continue;
        };

        let has_text = !bar.text.is_empty();
        let content = if has_text {
            bar.text.as_str()
        } else {
            bar.hint.as_str()
        };

        *text = Text::new(content);
        color.0 = if has_text {
            theme.on_surface
        } else {
            theme.on_surface_variant
        };
    }
}

// ============================================================================
// Systems
// ============================================================================

fn search_bar_interaction_system(
    search_bars: Query<(&Interaction, Entity), (Changed<Interaction>, With<MaterialSearchBar>)>,
    mut click_events: MessageWriter<SearchBarClickEvent>,
) {
    for (interaction, entity) in search_bars.iter() {
        if *interaction == Interaction::Pressed {
            click_events.write(SearchBarClickEvent { search_bar: entity });
        }
    }
}
