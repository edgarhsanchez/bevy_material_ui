//! Chips view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_chip_i18n;

/// Spawn the chips section content
pub fn spawn_chips_section(
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
                "showcase.section.chips.title",
                "Chips",
                "showcase.section.chips.description",
                "Compact elements for filters, selections, and actions",
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
                    spawn_chip_i18n(row, theme, "showcase.chips.filter", "Filter", false);
                    spawn_chip_i18n(row, theme, "showcase.chips.selected", "Selected", true);
                    spawn_chip_i18n(row, theme, "showcase.chips.tag", "Tag", false);
                    spawn_chip_i18n(row, theme, "showcase.chips.action", "Action", false);
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create an assist chip
let chip = MaterialChip::assist("Label");

// Create a filter chip (toggleable)
let chip = MaterialChip::filter("Category")
    .selected(true);

// Create an input chip (with close button)
let chip = MaterialChip::input("User Input");"#,
            );
        });
}
