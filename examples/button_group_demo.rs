//! Button Group Demo
//!
//! Demonstrates Material Design 3 segmented buttons / toggle groups.

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
                row_gap: Val::Px(24.0),
                padding: UiRect::all(Val::Px(48.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Button Groups"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ))
            .insert_test_id("button_group_demo/title", &telemetry);

            // A typical segmented control: 0px spacing, single selection, selection required.
            root.spawn((
                MaterialButtonGroup::new()
                    .single_selection(true)
                    .selection_required(true)
                    .spacing(0.0),
                Node {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            ))
            .insert_test_id("button_group_demo/group/horizontal", &telemetry)
            .with_children(|group| {
                for (i, label) in ["One", "Two", "Three"].into_iter().enumerate() {
                    let selected = i == 0;
                    let builder = MaterialButtonBuilder::new(label)
                        .outlined()
                        .checkable(true)
                        .checked(selected)
                        .disabled(false);

                    group
                        .spawn(builder.build(&theme))
                        .insert_test_id(
                            format!("button_group_demo/group/horizontal/tab/{i}"),
                            &telemetry,
                        )
                        .with_children(|b| {
                            b.spawn((
                                ButtonLabel,
                                Text::new(label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                }
            });

            // Vertical group (same behavior).
            root.spawn((
                MaterialButtonGroup::new()
                    .vertical()
                    .single_selection(true)
                    .selection_required(false)
                    .spacing(0.0),
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            ))
            .insert_test_id("button_group_demo/group/vertical", &telemetry)
            .with_children(|group| {
                for (i, label) in ["A", "B", "C"].into_iter().enumerate() {
                    let builder = MaterialButtonBuilder::new(label)
                        .outlined()
                        .checkable(true)
                        .checked(i == 1);

                    group.spawn(builder.build(&theme)).with_children(|b| {
                        b.spawn((
                            ButtonLabel,
                            Text::new(label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ));
                    });
                }
            });
        });
}
