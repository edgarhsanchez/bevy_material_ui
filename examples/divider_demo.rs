//! Divider Demo
//!
//! Demonstrates horizontal, inset, and vertical dividers.

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
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("divider_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((Node {
                width: Val::Px(420.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..default()
            },))
                .with_children(|col| {
                    col.spawn((
                        Text::new("Above"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    col.spawn_horizontal_divider(&theme);

                    col.spawn((
                        Text::new("Between"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    col.spawn_inset_divider(&theme);

                    col.spawn((
                        Text::new("Below"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));
                });

            root.spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                align_items: AlignItems::Center,
                ..default()
            },))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Left"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    row.spawn_vertical_divider(&theme);

                    row.spawn((
                        Text::new("Right"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));
                });
        });
}
