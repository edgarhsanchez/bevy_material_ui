//! List Demo
//!
//! Demonstrates Material Design 3 lists with various item configurations.

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
        .insert_test_id("list_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(420.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
            ))
            .insert_test_id("list_demo/panel", &telemetry)
            .with_children(|panel| {
                panel
                    .spawn((
                        ListBuilder::new().max_height(360.0).build_scrollable(),
                        BackgroundColor(theme.surface),
                    ))
                    .insert_test_id("list_demo/list", &telemetry)
                    .with_children(|list| {
                        for i in 1..=20 {
                            let builder = if i % 3 == 0 {
                                ListItemBuilder::new(format!("Item {i}"))
                                    .two_line()
                                    .supporting_text("Supporting text")
                            } else {
                                ListItemBuilder::new(format!("Item {i}")).one_line()
                            };

                            list.spawn_list_item_with(&theme, builder);

                            if i % 5 == 0 && i != 20 {
                                list.spawn_list_divider(&theme, false);
                            }
                        }
                    });
            });
        });
}
