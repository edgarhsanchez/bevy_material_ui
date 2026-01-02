//! Badge Demo
//!
//! Demonstrates Material badges (dot, count, text).

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
                column_gap: Val::Px(32.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("badge_demo/root", &telemetry)
        .with_children(|root| {
            // Dot badge
            root.spawn((
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme.surface_container),
                BorderRadius::all(Val::Px(8.0)),
            ))
            .insert_test_id("badge_demo/container/dot", &telemetry)
            .with_children(|container| {
                container.spawn((
                    Text::new("🔔"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                container.spawn_small_badge(&theme);
            });

            // Count badge
            root.spawn((
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme.surface_container),
                BorderRadius::all(Val::Px(8.0)),
            ))
            .insert_test_id("badge_demo/container/count", &telemetry)
            .with_children(|container| {
                container.spawn((
                    Text::new("🔔"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                container.spawn_badge_count(&theme, 3);
            });

            // Text badge
            root.spawn((
                Node {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(theme.surface_container),
                BorderRadius::all(Val::Px(8.0)),
            ))
            .insert_test_id("badge_demo/container/text", &telemetry)
            .with_children(|container| {
                container.spawn((
                    Text::new("🔔"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                container.spawn_badge_text(&theme, "NEW");
            });
        });
}
