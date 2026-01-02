//! Card Demo
//!
//! Demonstrates Material Design 3 cards: elevated, filled, and outlined variants.

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
                padding: UiRect::all(Val::Px(48.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("card_demo/root", &telemetry)
        .with_children(|root| {
            let cards = [
                ("elevated", CardBuilder::new().elevated()),
                ("filled", CardBuilder::new().filled()),
                ("outlined", CardBuilder::new().outlined()),
            ];

            for (id, builder) in cards {
                root.spawn(
                    builder
                        .width(Val::Px(220.0))
                        .height(Val::Px(140.0))
                        .build(&theme),
                )
                .insert_test_id(format!("card_demo/card/{id}"), &telemetry)
                .with_children(|card| {
                    card.spawn((
                        Text::new(format!("{id} card")),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));
                });
            }
        });
}
