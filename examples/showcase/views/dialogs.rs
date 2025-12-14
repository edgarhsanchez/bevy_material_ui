//! Dialogs view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};

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
                "Dialogs",
                "Modal windows with positioning options"
            );

            // Position options
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Dialog Position:"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                col.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                }).with_children(|row| {
                    spawn_dialog_position_option(row, theme, "Center Window", DialogPosition::CenterWindow, true);
                    spawn_dialog_position_option(row, theme, "Center Parent", DialogPosition::CenterParent, false);
                    spawn_dialog_position_option(row, theme, "Below Trigger", DialogPosition::BelowTrigger, false);
                });
            });

            // Show Dialog button and result display
            section.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|row| {
                // Show Dialog button
                let show_label = "Show Dialog";
                let show_button = MaterialButton::new(show_label).with_variant(ButtonVariant::Filled);
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
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(show_text_color),
                    ));
                });
                
                // Result display
                row.spawn((
                    DialogResultDisplay,
                    Text::new("Result: None"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface_variant),
                ));
            });

            // Dialog container (hidden by default)
            section.spawn((
                DialogContainer,
                Visibility::Hidden,
                GlobalZIndex(1000),
                Node {
                    width: Val::Px(280.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(24.0)),
                    row_gap: Val::Px(16.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container_high),
                BorderRadius::all(Val::Px(28.0)),
                BoxShadow::from(ShadowStyle {
                    color: Color::BLACK.with_alpha(0.3),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(8.0),
                    spread_radius: Val::Px(0.0),
                    blur_radius: Val::Px(24.0),
                }),
            )).with_children(|dialog| {
                // Title
                dialog.spawn((
                    Text::new("Confirm Action"),
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                // Content
                dialog.spawn((
                    Text::new("Are you sure you want to proceed? This action cannot be undone."),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface_variant),
                ));
                
                // Actions
                dialog.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    column_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|actions| {
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
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(theme.primary),
                            ));
                        });
                    
                    // Confirm button
                    let confirm_label = "Confirm";
                    let confirm_button = MaterialButton::new(confirm_label).with_variant(ButtonVariant::Filled);
                    let confirm_text_color = confirm_button.text_color(theme);

                    actions
                        .spawn((
                            DialogConfirmButton,
                            Interaction::None,
                            MaterialButtonBuilder::new(confirm_label).filled().build(theme),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(confirm_label),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(confirm_text_color),
                            ));
                        });
                });
            });

            spawn_code_block(section, theme,
r#"// Create a dialog with positioning
let dialog = MaterialDialog::new()
    .title("Delete Item?")
    .position(DialogPosition::CenterWindow)  // or CenterParent, BelowTrigger
    .open(true);

// Position can be set relative to:
// - CenterWindow: Centered in the application window
// - CenterParent: Centered within parent container
// - BelowTrigger: Positioned below the trigger button"#);
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
            ChipBuilder::filter(label).selected(is_selected).build(theme),
        ))
        .with_children(|chip| {
            chip.spawn((
                ChipLabel,
                Text::new(label),
                TextFont { font_size: 12.0, ..default() },
                TextColor(label_color),
            ));
        });
}
