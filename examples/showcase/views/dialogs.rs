//! Dialogs view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::dialog::create_dialog_scrim_for;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the dialogs section content
pub fn spawn_dialogs_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.dialogs.title",
                "Dialogs",
                "showcase.section.dialogs.description",
                "Modal windows with positioning options",
            );

            // Position options
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("Dialog Position:"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    col.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    })
                    .with_children(|row| {
                        spawn_dialog_position_option(
                            row,
                            theme,
                            "Center Window",
                            DialogPosition::CenterWindow,
                            true,
                        );
                        spawn_dialog_position_option(
                            row,
                            theme,
                            "Center Parent",
                            DialogPosition::CenterParent,
                            false,
                        );
                        spawn_dialog_position_option(
                            row,
                            theme,
                            "Below Trigger",
                            DialogPosition::BelowTrigger,
                            false,
                        );
                    });
                });

            // Modal options
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("Dialog Modality:"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    col.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    })
                    .with_children(|row| {
                        spawn_dialog_modal_option(row, theme, "Modal (blocks clicks)", true, true);
                        spawn_dialog_modal_option(row, theme, "Click-through", false, false);
                    });
                });

            // Show Dialog button and result display
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Show Dialog button
                    let show_label = "Show Dialog";
                    let show_button =
                        MaterialButton::new(show_label).with_variant(ButtonVariant::Filled);
                    let show_text_color = show_button.text_color(theme);

                    row.spawn((
                        ShowDialogButton,
                        Interaction::None,
                        MaterialButtonBuilder::new(show_label).filled().build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(show_label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(show_text_color),
                        ));
                    });

                    // Result display
                    row.spawn((
                        DialogResultDisplay,
                        Text::new("Result: None"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                });

            // Dialog + scrim (dialog hidden by default via MaterialDialog.open = false)
            let dialog_entity = section
                .spawn((
                    DialogContainer,
                    GlobalZIndex(1001),
                    DialogBuilder::new()
                        .title("Confirm Action")
                        .modal(true)
                        .build(theme),
                ))
                .with_children(|dialog| {
                    // Content
                    dialog.spawn((
                        Text::new(
                            "Are you sure you want to proceed? This action cannot be undone.",
                        ),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        Node {
                            margin: UiRect::bottom(Val::Px(16.0)),
                            ..default()
                        },
                    ));

                    // Actions
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::End,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|actions| {
                            // Cancel button
                            let cancel_label = "Cancel";
                            actions
                                .spawn((
                                    DialogCloseButton,
                                    Interaction::None,
                                    MaterialButtonBuilder::new(cancel_label).text().build(theme),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        ButtonLabel,
                                        Text::new(cancel_label),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(theme.primary),
                                    ));
                                });

                            // Confirm button
                            let confirm_label = "Confirm";
                            let confirm_button = MaterialButton::new(confirm_label)
                                .with_variant(ButtonVariant::Filled);
                            let confirm_text_color = confirm_button.text_color(theme);

                            actions
                                .spawn((
                                    DialogConfirmButton,
                                    Interaction::None,
                                    MaterialButtonBuilder::new(confirm_label)
                                        .filled()
                                        .build(theme),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        ButtonLabel,
                                        Text::new(confirm_label),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(confirm_text_color),
                                    ));
                                });
                        });
                })
                .id();

            // Scrim follows dialog open state and modal option
            section.spawn(create_dialog_scrim_for(theme, dialog_entity, true));

            spawn_code_block(
                section,
                theme,
                r#"// Create a modal dialog (blocks clicks behind it)
let dialog = MaterialDialog::new()
    .title("Delete Item?")
    .modal(true)
    .open(true);

// Or allow click-through (non-modal scrim)
let dialog = MaterialDialog::new()
    .title("Info")
    .modal(false)
    .open(true);"#,
            );
        });
}

fn spawn_dialog_position_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    position: DialogPosition,
    is_selected: bool,
) {
    let chip_for_color = MaterialChip::filter(label).with_selected(is_selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            DialogPositionOption(position),
            Interaction::None,
            ChipBuilder::filter(label)
                .selected(is_selected)
                .build(theme),
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

fn spawn_dialog_modal_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    modal: bool,
    is_selected: bool,
) {
    let chip_for_color = MaterialChip::filter(label).with_selected(is_selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            DialogModalOption(modal),
            Interaction::None,
            ChipBuilder::filter(label)
                .selected(is_selected)
                .build(theme),
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
