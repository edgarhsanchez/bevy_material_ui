//! Theme colors view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::spawn_text_field_control_with;
use bevy_material_ui::theme::ThemeMode;

use crate::showcase::common::*;

/// Spawn the theme colors section content
pub fn spawn_theme_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    seed_argb: u32,
) {
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
                "showcase.section.theme_colors.title",
                "Theme Colors",
                "showcase.section.theme_colors.description",
                "Material Design 3 color scheme with dynamic color support",
            );

            // Theme mode toggle
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Mode:"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    for (label, mode) in [("Dark", ThemeMode::Dark), ("Light", ThemeMode::Light)] {
                        let selected = theme.mode == mode;
                        let button = MaterialButton::new(label).with_variant(if selected {
                            ButtonVariant::FilledTonal
                        } else {
                            ButtonVariant::Outlined
                        });
                        let label_color = button.text_color(theme);

                        let test_id = format!("theme_mode_{}", label.to_lowercase());

                        row.spawn((
                            TestId::new(test_id),
                            ThemeModeOption(mode),
                            Interaction::None,
                            MaterialButtonBuilder::new(label)
                                .variant(if selected {
                                    ButtonVariant::FilledTonal
                                } else {
                                    ButtonVariant::Outlined
                                })
                                .build(theme),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(label),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(label_color),
                            ));
                        });
                    }
                });

            // Theme seed selection (simple preset seeds)
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Theme:"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    for (label, seed) in [
                        ("Purple", 0xFF6750A4),
                        // More saturated/recognizable seeds for dynamic color generation.
                        ("Teal", 0xFF00897B),
                        ("Green", 0xFF43A047),
                        ("Orange", 0xFFFB8C00),
                    ] {
                        let button =
                            MaterialButton::new(label).with_variant(ButtonVariant::Outlined);
                        let label_color = button.text_color(theme);

                        let test_id = format!("theme_seed_{}", label.to_lowercase());

                        row.spawn((
                            TestId::new(test_id),
                            ThemeSeedOption(seed),
                            Interaction::None,
                            MaterialButtonBuilder::new(label)
                                .variant(ButtonVariant::Outlined)
                                .build(theme),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(label),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(label_color),
                            ));
                        });
                    }
                });

            // Custom seed input (paste/type). Applies when the value parses as hex.
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Seed:"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    row.spawn((
                        ThemeSeedTextFieldSlot,
                        Node {
                            width: Val::Px(280.0),
                            ..default()
                        },
                    ))
                    .with_children(|slot| {
                        spawn_text_field_control_with(
                            slot,
                            theme,
                            TextFieldBuilder::new()
                                .label("Theme seed")
                                .value(argb_to_hex_rgb(seed_argb))
                                .placeholder("#RRGGBB")
                                .supporting_text("Paste/type a hex seed")
                                .outlined()
                                .width(Val::Percent(100.0)),
                            ThemeSeedTextField,
                        );
                    });
                });

            // Color groups
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(24.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|groups| {
                    // Primary colors
                    groups
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|group| {
                            group.spawn((
                                Text::new("Primary"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            group
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_color_swatch(
                                        row,
                                        theme.primary,
                                        "Primary",
                                        theme.on_primary,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_primary,
                                        "On Primary",
                                        theme.primary,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.primary_container,
                                        "Container",
                                        theme.on_primary_container,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_primary_container,
                                        "On Container",
                                        theme.primary_container,
                                    );
                                });
                        });

                    // Secondary colors
                    groups
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|group| {
                            group.spawn((
                                Text::new("Secondary"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            group
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_color_swatch(
                                        row,
                                        theme.secondary,
                                        "Secondary",
                                        theme.on_secondary,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_secondary,
                                        "On Secondary",
                                        theme.secondary,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.secondary_container,
                                        "Container",
                                        theme.on_secondary_container,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_secondary_container,
                                        "On Container",
                                        theme.secondary_container,
                                    );
                                });
                        });

                    // Tertiary colors
                    groups
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|group| {
                            group.spawn((
                                Text::new("Tertiary"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            group
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_color_swatch(
                                        row,
                                        theme.tertiary,
                                        "Tertiary",
                                        theme.on_tertiary,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_tertiary,
                                        "On Tertiary",
                                        theme.tertiary,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.tertiary_container,
                                        "Container",
                                        theme.on_tertiary_container,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_tertiary_container,
                                        "On Container",
                                        theme.tertiary_container,
                                    );
                                });
                        });

                    // Error colors
                    groups
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|group| {
                            group.spawn((
                                Text::new("Error"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            group
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_color_swatch(row, theme.error, "Error", theme.on_error);
                                    spawn_color_swatch(
                                        row,
                                        theme.on_error,
                                        "On Error",
                                        theme.error,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.error_container,
                                        "Container",
                                        theme.on_error_container,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.on_error_container,
                                        "On Container",
                                        theme.error_container,
                                    );
                                });
                        });

                    // Surface colors
                    groups
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|group| {
                            group.spawn((
                                Text::new("Surface"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            group
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(4.0),
                                    flex_wrap: FlexWrap::Wrap,
                                    row_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_color_swatch(
                                        row,
                                        theme.surface,
                                        "Surface",
                                        theme.on_surface,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.surface_container_lowest,
                                        "Lowest",
                                        theme.on_surface,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.surface_container_low,
                                        "Low",
                                        theme.on_surface,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.surface_container,
                                        "Container",
                                        theme.on_surface,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.surface_container_high,
                                        "High",
                                        theme.on_surface,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.surface_container_highest,
                                        "Highest",
                                        theme.on_surface,
                                    );
                                });
                        });

                    // Outline colors
                    groups
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|group| {
                            group.spawn((
                                Text::new("Outline"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            group
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|row| {
                                    spawn_color_swatch(
                                        row,
                                        theme.outline,
                                        "Outline",
                                        theme.surface,
                                    );
                                    spawn_color_swatch(
                                        row,
                                        theme.outline_variant,
                                        "Outline Variant",
                                        theme.surface,
                                    );
                                });
                        });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create a theme from a seed color
let seed_color = Color::srgb(0.4, 0.2, 0.8); // Purple seed
let theme = MaterialTheme::from_source_color(seed_color, is_dark);

// Access theme colors
let primary = theme.primary;
let on_primary = theme.on_primary;
let surface = theme.surface;

// Dynamic color tokens
BackgroundColor(theme.primary_container)
BorderColor(theme.outline_variant)"#,
            );
        });
}

fn argb_to_hex_rgb(argb: u32) -> String {
    let r = ((argb >> 16) & 0xFF) as u8;
    let g = ((argb >> 8) & 0xFF) as u8;
    let b = (argb & 0xFF) as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Spawn a color swatch with label
fn spawn_color_swatch(
    parent: &mut ChildSpawnerCommands,
    color: Color,
    label: &str,
    text_color: Color,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(80.0),
                height: Val::Px(64.0),
                padding: UiRect::all(Val::Px(8.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(color),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|swatch| {
            swatch.spawn((
                Text::new(label),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}
