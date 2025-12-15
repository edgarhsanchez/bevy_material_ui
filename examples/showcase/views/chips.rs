//! Chips view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the chips section content
pub fn spawn_chips_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, _icon_font: Handle<Font>) {
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
                "Chips",
                "Compact elements for filters, selections, and actions"
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_chip(row, theme, "Filter", false);
                    spawn_chip(row, theme, "Selected", true);
                    spawn_chip(row, theme, "Tag", false);
                    spawn_chip(row, theme, "Action", false);
                });

            spawn_code_block(section, theme,
r#"// Create an assist chip
let chip = MaterialChip::assist("Label");

// Create a filter chip (toggleable)
let chip = MaterialChip::filter("Category")
    .selected(true);

// Create an input chip (with close button)
let chip = MaterialChip::input("User Input");"#);
        });
}

fn spawn_chip(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, label: &str, selected: bool) {
    let chip_for_color = MaterialChip::filter(label).with_selected(selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            Interaction::None,
            ChipBuilder::filter(label).selected(selected).build(theme),
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
