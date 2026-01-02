//! TextField Demo
//!
//! Demonstrates Material Design 3 text fields: filled and outlined variants.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::spawn_text_field_control;

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

    let mut fields: Vec<(&'static str, Entity)> = Vec::new();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(24.0),
                padding: UiRect::all(Val::Px(48.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("textfield_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Text fields"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            let filled = spawn_text_field_control(
                root,
                &theme,
                TextFieldBuilder::new()
                    .label("Name")
                    .placeholder("Enter a name")
                    .filled(),
            );
            fields.push(("filled", filled));

            let outlined = spawn_text_field_control(
                root,
                &theme,
                TextFieldBuilder::new()
                    .label("Email")
                    .placeholder("email@example.com")
                    .outlined(),
            );
            fields.push(("outlined", outlined));
        });

    for (kind, entity) in fields {
        commands
            .entity(entity)
            .insert_test_id(format!("textfield_demo/field/{kind}"), &telemetry);
    }
}
