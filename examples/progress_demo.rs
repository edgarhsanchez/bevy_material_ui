//! Progress Demo
//!
//! Demonstrates Material Design 3 progress indicators.

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
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("progress_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(360.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
                BorderRadius::all(Val::Px(12.0)),
            ))
            .with_children(|col| {
                col.spawn((
                    Text::new("Progress"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                col.spawn((
                    Text::new("Linear (determinate)"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                col.spawn(
                    LinearProgressBuilder::new()
                        .progress(0.65)
                        .width(Val::Px(320.0))
                        .build(&theme),
                )
                .insert_test_id("progress_demo/linear/determinate", &telemetry);

                col.spawn((
                    Text::new("Linear (indeterminate)"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                col.spawn(
                    LinearProgressBuilder::new()
                        .indeterminate()
                        .width(Val::Px(320.0))
                        .build(&theme),
                )
                .insert_test_id("progress_demo/linear/indeterminate", &telemetry);

                col.spawn((
                    Text::new("Circular (determinate)"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                col.spawn(CircularProgressBuilder::new().progress(0.35).build(&theme))
                    .insert_test_id("progress_demo/circular/determinate", &telemetry);

                col.spawn((
                    Text::new("Circular (indeterminate)"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                col.spawn(CircularProgressBuilder::new().indeterminate().build(&theme))
                    .insert_test_id("progress_demo/circular/indeterminate", &telemetry);
            });
        });
}
