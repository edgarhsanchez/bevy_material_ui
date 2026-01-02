//! Slider Demo
//!
//! Demonstrates Material Design 3 sliders with different configurations.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::slider::spawn_slider_control;

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

    // Opt-in tracing: `MUI_SLIDER_TRACE=1` (and set `RUST_LOG=bevy_material_ui::slider=info`).
    if std::env::var("MUI_SLIDER_TRACE").is_ok() {
        commands.insert_resource(SliderTraceSettings {
            enabled: true,
            ..default()
        });
    }

    let mut sliders: Vec<(&'static str, Entity)> = Vec::new();

    commands
        .spawn((
            ScrollContainerBuilder::new().vertical().build(),
            ScrollPosition::default(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Both axes must be Scroll for Bevy's scroll system.
                // The ScrollContainer direction controls which direction actually scrolls.
                overflow: Overflow::scroll(),
                ..default()
            },
        ))
        .insert_test_id("slider_demo/root", &telemetry)
        .with_children(|scroller| {
            scroller
                .spawn(Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(32.0),
                    padding: UiRect::all(Val::Px(48.0)),
                    ..default()
                })
                .with_children(|root| {
                    root.spawn(Node {
                        width: Val::Px(520.0),
                        max_width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        column_gap: Val::Px(48.0),
                        ..default()
                    })
                    .with_children(|row| {
                        // Horizontal examples
                        row.spawn(Node {
                            width: Val::Px(420.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(24.0),
                            ..default()
                        })
                        .with_children(|col| {
                            col.spawn_slider(
                                &theme,
                                0.0,
                                100.0,
                                40.0,
                                Some("Horizontal (Continuous)"),
                            );
                            col.spawn_discrete_slider(
                                &theme,
                                0.0,
                                100.0,
                                60.0,
                                20.0,
                                Some("Horizontal (Discrete)"),
                            );
                        });

                        // Vertical example
                        row.spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(12.0),
                            ..default()
                        })
                        .with_children(|col| {
                            col.spawn((
                                Text::new("Vertical"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));

                            col.spawn(Node {
                                width: Val::Px(48.0),
                                height: Val::Px(240.0),
                                ..default()
                            })
                            .with_children(|slot| {
                                let slider =
                                    MaterialSlider::new(0.0, 100.0).with_value(40.0).vertical();
                                let slider_entity = spawn_slider_control(slot, &theme, slider);
                                sliders.push(("vertical", slider_entity));
                            });
                        });
                    });
                });
        });

    for (kind, entity) in sliders {
        commands
            .entity(entity)
            .insert_test_id(format!("slider_demo/slider/{kind}"), &telemetry);
    }

    // Alternatively, enable trace logging directly:
    // commands.insert_resource(SliderTraceSettings { enabled: true, ..default() });
}
