//! Select Demo
//!
//! Demonstrates Material Design 3 select (dropdown) controls.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct SelectDemoRoot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, attach_select_test_ids)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            SelectDemoRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("select_demo/root", &telemetry)
        .with_children(|root| {
            let options = vec![
                SelectOption::new("Apple").value("apple").icon("nutrition"),
                SelectOption::new("Banana")
                    .value("banana")
                    .icon("emoji_food_beverage"),
                SelectOption::new("Cherry (disabled)")
                    .value("cherry")
                    .disabled(),
            ];

            root.spawn((
                Text::new("Select"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|col| {
                col.spawn_select_with(
                    &theme,
                    SelectBuilder::new(options.clone())
                        .label("Fruit (Filled)")
                        .filled()
                        .selected(0)
                        .width(Val::Px(320.0)),
                );

                col.spawn_select_with(
                    &theme,
                    SelectBuilder::new(options.clone())
                        .label("Fruit (Outlined)")
                        .outlined()
                        .selected(1)
                        .width(Val::Px(320.0)),
                );

                col.spawn_select_with(
                    &theme,
                    SelectBuilder::new(options)
                        .label("Disabled")
                        .filled()
                        .disabled(true)
                        .width(Val::Px(320.0)),
                );
            });
        });
}

fn attach_select_test_ids(
    mut commands: Commands,
    telemetry: Res<TelemetryConfig>,
    selects: Query<(Entity, &MaterialSelect), Added<MaterialSelect>>,
) {
    if !telemetry.enabled {
        return;
    }

    for (entity, select) in selects.iter() {
        let Some(label) = select.label.as_deref() else {
            continue;
        };

        let id = match label {
            "Fruit (Filled)" => "select_demo/select/fruit_filled",
            "Fruit (Outlined)" => "select_demo/select/fruit_outlined",
            "Disabled" => "select_demo/select/disabled",
            _ => continue,
        };

        commands.entity(entity).insert(TestId::new(id));
    }
}
