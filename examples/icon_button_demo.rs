//! Icon Button Demo
//!
//! Demonstrates standard/filled/tonal/outlined icon buttons.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_FAVORITE, ICON_SEARCH, ICON_SETTINGS};
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
                column_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("icon_button_demo/root", &telemetry)
        .with_children(|root| {
            let buttons = [
                ("standard", IconButtonBuilder::new(ICON_SEARCH).standard()),
                ("filled", IconButtonBuilder::new(ICON_FAVORITE).filled()),
                (
                    "filled_tonal",
                    IconButtonBuilder::new(ICON_SETTINGS).filled_tonal(),
                ),
                ("outlined", IconButtonBuilder::new(ICON_SEARCH).outlined()),
            ];

            for (id, builder) in buttons {
                root.spawn(builder.build(&theme))
                    .insert_test_id(format!("icon_button_demo/button/{id}"), &telemetry);
            }
        });
}
