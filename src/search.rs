//! Material Design 3 Search Bar and Search View
//!
//! The Search Bar represents a floating search field with affordances for search and navigation.
//! Reference: <https://m3.material.io/components/search/overview>

use bevy::prelude::*;

use crate::{
    icons::{IconStyle, MaterialIcon},
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for search components
pub struct SearchPlugin;

impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SearchBarClickEvent>()
            .add_message::<SearchQueryEvent>()
            .add_systems(Update, search_bar_interaction_system);
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

// ============================================================================
// Constants
// ============================================================================

pub const SEARCH_BAR_HEIGHT: f32 = 56.0;

// ============================================================================
// Builder
// ============================================================================

pub struct SearchBarBuilder {
    search_bar: MaterialSearchBar,
}

impl SearchBarBuilder {
    pub fn new(hint: impl Into<String>) -> Self {
        Self {
            search_bar: MaterialSearchBar::new(hint),
        }
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
        let _hint = self.search_bar.hint.clone();
        let text = self.search_bar.text.clone();
        let _has_text = !text.is_empty();

        (
            self.search_bar,
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

        self.spawn(builder.build(theme)).with_children(|bar| {
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
                if has_text {
                    container.spawn((
                        Text::new(&text),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));
                } else {
                    container.spawn((
                        Text::new(&hint),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                }
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
