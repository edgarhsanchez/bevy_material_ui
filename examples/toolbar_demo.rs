//! Toolbar Demo
//!
//! Demonstrates the Material toolbar with navigation and actions.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_MENU, ICON_SEARCH};
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("toolbar_demo/root", &telemetry)
        .with_children(|root| {
            // Spawn the toolbar using the standard helper.
            root.spawn_toolbar_with(
                &theme,
                ToolbarBuilder::new("Inventory")
                    .navigation_icon_name(ICON_MENU)
                    .action_name(ICON_SEARCH, "search"),
            );

            // Simple body
            root.spawn((
                Text::new("Toolbar (nav + action)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
        });
}
