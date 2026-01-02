//! Snackbar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::icons::ICON_CLOSE;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the snackbar section content
pub fn spawn_snackbar_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
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
                "showcase.section.snackbar.title",
                "Snackbars",
                "showcase.section.snackbar.description",
                "Brief messages about app processes - Configure options below",
            );

            // Options panel
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(12.0)),
                    row_gap: Val::Px(12.0),
                    ..default()
                })
                .with_children(|options| {
                    // Duration options
                    options
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Duration:"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));

                            for (label, duration) in
                                [("2s", 2.0_f32), ("4s", 4.0_f32), ("10s", 10.0_f32)]
                            {
                                let is_default = (duration - 4.0).abs() < 0.01;
                                let chip_for_color =
                                    MaterialChip::filter(label).with_selected(is_default);
                                let label_color = chip_for_color.label_color(theme);

                                row.spawn((
                                    SnackbarDurationOption(duration),
                                    Interaction::None,
                                    ChipBuilder::filter(label).selected(is_default).build(theme),
                                ))
                                .with_children(|chip| {
                                    chip.spawn((
                                        ChipLabel,
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

                    // Action toggle
                    options
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Show action:"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));

                            let toggle_label = "Toggle Action";
                            let chip_for_color =
                                MaterialChip::filter(toggle_label).with_selected(false);
                            let label_color = chip_for_color.label_color(theme);

                            row.spawn((
                                SnackbarActionToggle,
                                Interaction::None,
                                ChipBuilder::filter(toggle_label)
                                    .selected(false)
                                    .build(theme),
                            ))
                            .with_children(|chip| {
                                chip.spawn((
                                    ChipLabel,
                                    Text::new(toggle_label),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(label_color),
                                ));
                            });
                        });
                });

            // Trigger button
            section
                .spawn(Node {
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    let trigger_label = "Show Snackbar";
                    let trigger_button =
                        MaterialButton::new(trigger_label).with_variant(ButtonVariant::Filled);
                    let trigger_text_color = trigger_button.text_color(theme);

                    row.spawn((
                        SnackbarTrigger,
                        Interaction::None,
                        MaterialButtonBuilder::new(trigger_label)
                            .filled()
                            .build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(trigger_label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(trigger_text_color),
                        ));
                    });
                });

            // Snackbar preview (static example)
            section
                .spawn((
                    Node {
                        width: Val::Px(320.0),
                        height: Val::Px(48.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(theme.inverse_surface),
                    BorderRadius::all(Val::Px(4.0)),
                    BoxShadow::from(ShadowStyle {
                        color: Color::BLACK.with_alpha(0.2),
                        x_offset: Val::Px(0.0),
                        y_offset: Val::Px(2.0),
                        spread_radius: Val::Px(0.0),
                        blur_radius: Val::Px(4.0),
                    }),
                ))
                .with_children(|snackbar| {
                    snackbar.spawn((
                        Text::new("Item deleted"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.inverse_on_surface),
                        Node {
                            flex_grow: 1.0,
                            ..default()
                        },
                    ));

                    snackbar
                        .spawn((
                            Interaction::None,
                            MaterialButtonBuilder::new("UNDO").text().build(theme),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new("UNDO"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.inverse_primary),
                            ));
                        });

                    // Close button (X icon)
                    snackbar
                        .spawn((
                            Button,
                            Interaction::None,
                            Node {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderRadius::all(Val::Px(9999.0)),
                        ))
                        .with_children(|btn| {
                            if let Some(icon) = MaterialIcon::from_name(ICON_CLOSE) {
                                btn.spawn(
                                    icon.with_size(24.0).with_color(theme.inverse_on_surface),
                                );
                            }
                        });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Show a snackbar (via event)
commands.write_message(ShowSnackbar::message("File saved"));

// With action button
commands.write_message(
    ShowSnackbar::with_action("Item deleted", "UNDO")
        .duration(5.0)
);

// Handle action clicks
fn handle_snackbar(mut events: MessageReader<SnackbarActionEvent>) {
    for event in events.read() {
        if event.action == "UNDO" {
            // Handle undo
        }
    }
}"#,
            );
        });
}
