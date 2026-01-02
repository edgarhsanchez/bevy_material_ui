//! Tooltips view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the tooltip section content
pub fn spawn_tooltip_section(
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
                "showcase.section.tooltips.title",
                "Tooltips",
                "showcase.section.tooltips.description",
                "Contextual information on hover - Configure options below",
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
                    // Position options
                    options
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Position:"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));

                            // Position buttons (exclusive)
                            for (label, pos) in [
                                ("Top", TooltipPosition::Top),
                                ("Bottom", TooltipPosition::Bottom),
                                ("Left", TooltipPosition::Left),
                                ("Right", TooltipPosition::Right),
                            ] {
                                let selected = pos == TooltipPosition::Bottom;
                                let button = MaterialButton::new(label).with_variant(if selected {
                                    ButtonVariant::FilledTonal
                                } else {
                                    ButtonVariant::Outlined
                                });
                                let label_color = button.text_color(theme);

                                row.spawn((
                                    TooltipPositionOption(pos),
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

                    // Delay options
                    options
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|row| {
                            row.spawn((
                                Text::new("Delay:"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));

                            for (label, delay) in
                                [("0.15s", 0.15_f32), ("0.5s", 0.5_f32), ("1.0s", 1.0_f32)]
                            {
                                let selected = (delay - 0.5).abs() < 0.01;
                                let button = MaterialButton::new(label).with_variant(if selected {
                                    ButtonVariant::FilledTonal
                                } else {
                                    ButtonVariant::Outlined
                                });
                                let label_color = button.text_color(theme);

                                row.spawn((
                                    TooltipDelayOption(delay),
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
                });

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(32.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|row| {
                    // Interactive tooltip demo button
                    let demo_label = "Hover Me";
                    let demo_button =
                        MaterialButton::new(demo_label).with_variant(ButtonVariant::Filled);
                    let demo_text_color = demo_button.text_color(theme);

                    row.spawn((
                        TooltipDemoButton,
                        TooltipTrigger::new("Hover to see tooltip!").bottom(),
                        Interaction::None,
                        MaterialButtonBuilder::new(demo_label).filled().build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(demo_label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(demo_text_color),
                        ));
                    });

                    row.spawn((
                        Text::new("← Hover to test tooltip with selected options"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                });

            spawn_code_block(
                section,
                theme,
                r#"// Add tooltip to an element
commands.spawn((
    Button,
    TooltipTrigger::new("Add to favorites")
        .with_position(TooltipPosition::Bottom)
        .with_delay(0.5),  // 500ms delay
));

// Rich tooltip with title
let trigger = TooltipTrigger::rich("Title", "Description text")
    .with_position(TooltipPosition::Right);"#,
            );
        });
}
