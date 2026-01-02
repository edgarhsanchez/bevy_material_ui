//! Checkbox Demo
//!
//! Demonstrates Material Design 3 checkboxes.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Resource, Default)]
struct CheckboxDemoRows(Vec<(Entity, String)>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .insert_resource(CheckboxDemoRows::default())
        .add_systems(Startup, setup)
        .add_systems(Update, attach_checkbox_child_test_ids)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    mut rows: ResMut<CheckboxDemoRows>,
) {
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
        .insert_test_id("checkbox_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Checkboxes"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
        });

    // Spawn checkboxes as separate rows so we can attach stable IDs.
    let checkbox_defs = [
        ("unchecked", CheckboxState::Unchecked, "Unchecked"),
        ("checked", CheckboxState::Checked, "Checked"),
        (
            "indeterminate",
            CheckboxState::Indeterminate,
            "Indeterminate",
        ),
    ];

    // A simple column for the checkboxes.
    let column = commands
        .spawn(Node {
            width: Val::Px(360.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .insert_test_id("checkbox_demo/column", &telemetry)
        .id();

    for (key, state, label) in checkbox_defs {
        let row = commands.spawn_checkbox(&theme, state, label);

        if telemetry.enabled {
            commands
                .entity(row)
                .insert(TestId::new(format!("checkbox_demo/row/{key}")));
        }

        commands.entity(column).add_child(row);
        rows.0.push((row, format!("checkbox_demo/checkbox/{key}")));
    }
}

fn attach_checkbox_child_test_ids(
    mut commands: Commands,
    telemetry: Res<TelemetryConfig>,
    rows: Res<CheckboxDemoRows>,
    children_query: Query<&Children>,
    is_checkbox: Query<(), With<MaterialCheckbox>>,
    is_text: Query<(), With<Text>>,
) {
    if !telemetry.enabled {
        return;
    }

    for (row_entity, base) in rows.0.iter() {
        let Ok(children) = children_query.get(*row_entity) else {
            continue;
        };

        for child in children.iter() {
            if is_checkbox.get(child).is_ok() {
                commands
                    .entity(child)
                    .insert_test_id(format!("{base}"), &telemetry);
            } else if is_text.get(child).is_ok() {
                commands
                    .entity(child)
                    .insert_test_id(format!("{base}/label"), &telemetry);
            }
        }
    }
}
