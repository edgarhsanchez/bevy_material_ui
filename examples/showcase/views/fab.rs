//! FAB (Floating Action Button) view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_extended_fab_i18n;

/// Spawn the FAB section content
pub fn spawn_fab_section(
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
                "showcase.section.fab.title",
                "Floating Action Buttons",
                "showcase.section.fab.description",
                "Primary actions with prominent visual treatment",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn_small_fab(theme, "add");
                    row.spawn_regular_fab(theme, "add");
                    row.spawn_large_fab(theme, "add");
                    spawn_extended_fab_i18n(row, theme, "add", "showcase.fab.create", "Create");
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create a FAB
let fab = MaterialFab::new()
    .icon("add")
    .size(FabSize::Regular);

// Extended FAB with label
let fab = MaterialFab::new()
    .icon("add")
    .label("Create")
    .extended(true);"#,
            );
        });
}
