//! Button Demo
//!
//! Demonstrates Material Design 3 buttons: elevated, filled, filled tonal,
//! outlined, and text buttons.

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
        .insert_test_id("button_demo/root", &telemetry)
        .with_children(|root| {
            let buttons = [
                (
                    "Elevated",
                    ButtonVariant::Elevated,
                    MaterialButtonBuilder::new("Elevated").elevated(),
                ),
                (
                    "Filled",
                    ButtonVariant::Filled,
                    MaterialButtonBuilder::new("Filled").filled(),
                ),
                (
                    "Filled Tonal",
                    ButtonVariant::FilledTonal,
                    MaterialButtonBuilder::new("Filled Tonal").filled_tonal(),
                ),
                (
                    "Outlined",
                    ButtonVariant::Outlined,
                    MaterialButtonBuilder::new("Outlined").outlined(),
                ),
                (
                    "Text",
                    ButtonVariant::Text,
                    MaterialButtonBuilder::new("Text").text(),
                ),
            ];

            for (id, variant, builder) in buttons {
                let button = MaterialButton::new(id).with_variant(variant);
                let text_color = button.text_color(&theme);

                root.spawn(builder.build(&theme))
                    .insert_test_id(format!("button_demo/button/{id}"), &telemetry)
                    .with_children(|b| {
                        b.spawn((
                            ButtonLabel,
                            Text::new(id),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(text_color),
                        ));
                    });
            }
        });
}
