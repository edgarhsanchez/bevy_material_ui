//! Radio Demo
//!
//! Demonstrates Material Design 3 radio buttons and group exclusivity.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Resource, Default)]
struct RadioDemoRows(Vec<(Entity, String)>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .insert_resource(RadioDemoRows::default())
        .add_systems(Startup, setup)
        .add_systems(Update, attach_radio_child_test_ids)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    telemetry: Res<TelemetryConfig>,
    mut rows: ResMut<RadioDemoRows>,
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
        .insert_test_id("radio_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Radios"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
        });

    let column = commands
        .spawn(Node {
            width: Val::Px(360.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .insert_test_id("radio_demo/column", &telemetry)
        .id();

    // Two radios in the same group (exclusive)
    for (key, selected, label) in [
        ("a", true, "Group 1: Option A"),
        ("b", false, "Group 1: Option B"),
    ] {
        let row = commands.spawn_radio(&theme, selected, "group_1", label);

        if telemetry.enabled {
            commands
                .entity(row)
                .insert(TestId::new(format!("radio_demo/row/group_1/{key}")));
        }

        commands.entity(column).add_child(row);
        rows.0
            .push((row, format!("radio_demo/radio/group_1/{key}")));
    }

    // A separate group
    for (key, selected, label) in [
        ("x", false, "Group 2: Option X"),
        ("y", true, "Group 2: Option Y"),
    ] {
        let row = commands.spawn_radio(&theme, selected, "group_2", label);

        if telemetry.enabled {
            commands
                .entity(row)
                .insert(TestId::new(format!("radio_demo/row/group_2/{key}")));
        }

        commands.entity(column).add_child(row);
        rows.0
            .push((row, format!("radio_demo/radio/group_2/{key}")));
    }
}

fn attach_radio_child_test_ids(
    mut commands: Commands,
    telemetry: Res<TelemetryConfig>,
    rows: Res<RadioDemoRows>,
    children_query: Query<&Children>,
    is_radio: Query<(), With<MaterialRadio>>,
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
            if is_radio.get(child).is_ok() {
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
