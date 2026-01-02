//! FAB Demo
//!
//! Demonstrates regular/small/extended FAB variants.

use bevy::prelude::*;
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
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("fab_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn_fab(&theme, "add", FabSize::Regular);

            root.spawn_small_fab(&theme, "edit");

            root.spawn_extended_fab(&theme, "add", "Create");
        });
}
