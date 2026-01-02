//! Material Design 3 Toolbar component
//!
//! Toolbars provide a compact top row for navigation, title, and actions.
//! This is a pragmatic MD3-style toolbar intended for desktop/game UIs.
//!
//! Reference: <https://m3.material.io/components/top-app-bar/overview>

use bevy::prelude::*;

use crate::{
    icon_button::IconButtonBuilder,
    icons::{IconStyle, MaterialIcon},
    theme::MaterialTheme,
    tokens::Spacing,
};

/// Plugin for the toolbar component.
pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<ToolbarNavigationEvent>()
            .add_message::<ToolbarActionEvent>()
            .add_systems(
                Update,
                (toolbar_interaction_system, toolbar_theme_refresh_system),
            );
    }
}

// ============================================================================
// Events
// ============================================================================

/// Event fired when the navigation icon is clicked.
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct ToolbarNavigationEvent {
    /// The toolbar entity.
    pub toolbar: Entity,
}

/// Event fired when an action button is clicked.
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct ToolbarActionEvent {
    /// The toolbar entity.
    pub toolbar: Entity,
    /// The action identifier.
    pub action: String,
}

// ============================================================================
// Components
// ============================================================================

/// Toolbar root component.
#[derive(Component, Clone)]
pub struct MaterialToolbar {
    /// Title text.
    pub title: String,
    /// Optional navigation icon.
    pub navigation_icon: Option<String>,
    /// Actions to show on the right side.
    pub actions: Vec<ToolbarAction>,
}

impl MaterialToolbar {
    /// Create a new toolbar.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            navigation_icon: None,
            actions: Vec::new(),
        }
    }

    /// Set the navigation icon by name.
    pub fn with_navigation_icon_name(mut self, icon_name: &str) -> Self {
        self.navigation_icon = Some(icon_name.to_string());
        self
    }

    /// Add an action.
    pub fn add_action(mut self, action: ToolbarAction) -> Self {
        self.actions.push(action);
        self
    }
}

impl Default for MaterialToolbar {
    fn default() -> Self {
        Self::new("")
    }
}

/// Action definition for toolbars.
#[derive(Debug, Clone)]
pub struct ToolbarAction {
    /// Icon to display.
    pub icon: String,
    /// Action identifier.
    pub id: String,
    /// Whether disabled.
    pub disabled: bool,
}

impl ToolbarAction {
    /// Create a new action.
    pub fn new(icon_name: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            icon: icon_name.into(),
            id: id.into(),
            disabled: false,
        }
    }

    /// Create a new action from an icon name.
    pub fn from_name(icon_name: &str, id: impl Into<String>) -> Self {
        Self::new(icon_name, id)
    }

    /// Set disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Component)]
struct ToolbarNavigation;

#[derive(Component)]
struct ToolbarActionButton {
    id: String,
}

#[derive(Component)]
struct ToolbarTitle;

// ============================================================================
// Constants
// ============================================================================

/// Standard toolbar height (matches MD3 small top app bar).
pub const TOOLBAR_HEIGHT: f32 = 64.0;

/// Icon size for toolbar buttons.
pub const TOOLBAR_ICON_SIZE: f32 = 24.0;

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating toolbars.
pub struct ToolbarBuilder {
    toolbar: MaterialToolbar,
}

impl ToolbarBuilder {
    /// Create a new toolbar builder.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            toolbar: MaterialToolbar::new(title),
        }
    }

    /// Set the navigation icon.
    pub fn navigation_icon_name(mut self, icon_name: &str) -> Self {
        self.toolbar.navigation_icon = Some(icon_name.to_string());
        self
    }

    /// Add an action.
    pub fn action_name(mut self, icon_name: &str, id: impl Into<String>) -> Self {
        self.toolbar.actions.push(ToolbarAction::new(icon_name, id));
        self
    }

    /// Build the toolbar root bundle.
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        (
            self.toolbar,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(TOOLBAR_HEIGHT),
                padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::MEDIUM),
                ..default()
            },
            BackgroundColor(theme.surface),
        )
    }
}

// ============================================================================
// Spawn Traits
// ============================================================================

/// Extension trait to spawn toolbars as children.
pub trait SpawnToolbarChild {
    /// Spawn a toolbar with the given builder.
    fn spawn_toolbar_with(&mut self, theme: &MaterialTheme, builder: ToolbarBuilder);

    /// Spawn a toolbar with a title.
    fn spawn_toolbar(&mut self, theme: &MaterialTheme, title: impl Into<String>);
}

impl SpawnToolbarChild for ChildSpawnerCommands<'_> {
    fn spawn_toolbar_with(&mut self, theme: &MaterialTheme, builder: ToolbarBuilder) {
        // Extract a copy of the logical config before we move it into the root bundle.
        let title = builder.toolbar.title.clone();
        let nav_icon = builder.toolbar.navigation_icon.clone();
        let actions = builder.toolbar.actions.clone();

        self.spawn(builder.build(theme)).with_children(|toolbar| {
            if let Some(icon_name) = nav_icon.as_deref() {
                // Navigation icon button.
                toolbar
                    .spawn((
                        ToolbarNavigation,
                        IconButtonBuilder::new(icon_name).standard().build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            MaterialIcon::from_name(icon_name)
                                .expect("embedded toolbar navigation icon not found"),
                            IconStyle::outlined()
                                .with_color(theme.on_surface_variant)
                                .with_size(TOOLBAR_ICON_SIZE),
                        ));
                    });
            }

            // Title
            toolbar.spawn((
                ToolbarTitle,
                Text::new(title),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(theme.on_surface),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // Actions
            if !actions.is_empty() {
                toolbar
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(Spacing::SMALL),
                        ..default()
                    })
                    .with_children(|row| {
                        for action in actions.iter() {
                            let mut button_entity = row.spawn((
                                ToolbarActionButton {
                                    id: action.id.clone(),
                                },
                                IconButtonBuilder::new(action.icon.as_str())
                                    .standard()
                                    .disabled(action.disabled)
                                    .build(theme),
                            ));

                            button_entity.with_children(|btn| {
                                btn.spawn((
                                    MaterialIcon::from_name(action.icon.as_str())
                                        .expect("embedded toolbar action icon not found"),
                                    IconStyle::outlined()
                                        .with_color(theme.on_surface_variant)
                                        .with_size(TOOLBAR_ICON_SIZE),
                                ));
                            });
                        }
                    });
            }
        });
    }

    fn spawn_toolbar(&mut self, theme: &MaterialTheme, title: impl Into<String>) {
        self.spawn_toolbar_with(theme, ToolbarBuilder::new(title));
    }
}

// ============================================================================
// Systems
// ============================================================================

fn toolbar_interaction_system(
    nav_buttons: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<ToolbarNavigation>)>,
    action_buttons: Query<(&Interaction, &ToolbarActionButton, &ChildOf), Changed<Interaction>>,
    toolbars: Query<Entity, With<MaterialToolbar>>,
    mut nav_events: MessageWriter<ToolbarNavigationEvent>,
    mut action_events: MessageWriter<ToolbarActionEvent>,
) {
    for (interaction, parent) in nav_buttons.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(toolbar) = toolbars.get(parent.parent()) {
                nav_events.write(ToolbarNavigationEvent { toolbar });
            }
        }
    }

    for (interaction, action, parent) in action_buttons.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(toolbar) = toolbars.get(parent.parent()) {
                action_events.write(ToolbarActionEvent {
                    toolbar,
                    action: action.id.clone(),
                });
            }
        }
    }
}

fn toolbar_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut toolbars: Query<(&MaterialToolbar, &mut BackgroundColor)>,
    mut titles: Query<&mut TextColor, With<ToolbarTitle>>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (_toolbar, mut bg) in toolbars.iter_mut() {
        *bg = BackgroundColor(theme.surface);
    }

    for mut color in titles.iter_mut() {
        color.0 = theme.on_surface;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icons::ICON_MENU;

    #[test]
    fn test_toolbar_creation() {
        let toolbar = MaterialToolbar::new("Title").with_navigation_icon_name(ICON_MENU);
        assert_eq!(toolbar.title, "Title");
        assert!(toolbar.navigation_icon.is_some());
    }

    #[test]
    fn test_toolbar_actions() {
        let toolbar =
            MaterialToolbar::new("Title").add_action(ToolbarAction::new(ICON_MENU, "menu"));
        assert_eq!(toolbar.actions.len(), 1);
        assert_eq!(toolbar.actions[0].id, "menu");
    }
}
