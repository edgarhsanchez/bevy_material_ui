//! Progress indicators view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the progress section content
pub fn spawn_progress_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.progress.title",
                "Progress Indicators",
                "showcase.section.progress.description",
                "Visual feedback for loading and progress states",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Animated determinate progress (oscillates up/down)
                    spawn_animated_linear_progress(col, theme, 0.15, 0.35);
                    spawn_animated_linear_progress(col, theme, 0.75, 0.55);

                    // Indeterminate example
                    col.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new(""),
                            LocalizedText::new("showcase.progress.indeterminate")
                                .with_default("Indeterminate"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                            Node {
                                width: Val::Px(90.0),
                                ..default()
                            },
                            NeedsInternationalFont,
                        ));

                        row.spawn(
                            LinearProgressBuilder::new()
                                .indeterminate()
                                .width(Val::Px(200.0))
                                .height_px(8.0)
                                .build(theme),
                        );
                    });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Linear progress (determinate)
let progress = LinearProgress::new(0.5); // 50%

// Indeterminate progress
let progress = LinearProgress::indeterminate();

// Circular progress
let progress = CircularProgress::new(0.75);"#,
            );
        });
}

/// Marker for progress bars animated by the showcase.
#[derive(Component, Clone, Copy)]
pub struct ShowcaseProgressOscillator {
    pub speed: f32,
    pub direction: f32,
    pub label: Entity,
}

fn spawn_animated_linear_progress(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    initial: f32,
    speed: f32,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            let label_entity = row
                .spawn((
                    Text::new(format!(
                        "{:>3}%",
                        (initial.clamp(0.0, 1.0) * 100.0).round() as i32
                    )),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                    Node {
                        width: Val::Px(48.0),
                        ..default()
                    },
                ))
                .id();

            row.spawn((
                ShowcaseProgressOscillator {
                    speed,
                    direction: 1.0,
                    label: label_entity,
                },
                LinearProgressBuilder::new()
                    .progress(initial)
                    .width(Val::Px(200.0))
                    .height_px(8.0)
                    .build(theme),
            ));
        });
}
