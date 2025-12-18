//! Checkboxes view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning checkboxes.
//! The checkbox component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::checkbox::SpawnCheckboxChild;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the checkboxes section content
pub fn spawn_checkboxes_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Option<Handle<Font>>,
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
                "Checkboxes",
                "Toggle selection with visual checkmark feedback",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Simple, clean API - just pass theme, state, and label
                    col.spawn_checkbox(theme, CheckboxState::Checked, "Option 1");
                    col.spawn_checkbox(theme, CheckboxState::Unchecked, "Option 2");
                    col.spawn_checkbox(theme, CheckboxState::Unchecked, "Option 3");
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create checkboxes with the simple spawn API
parent.spawn_checkbox(&theme, CheckboxState::Unchecked, "Accept terms");
parent.spawn_checkbox(&theme, CheckboxState::Checked, "Remember me");

// Or use the builder for more control
parent.spawn_checkbox_with(
    &theme,
    MaterialCheckbox::new()
        .with_state(CheckboxState::Indeterminate)
        .disabled(true),
    "Partial selection"
);

// Listen for changes
fn handle_checkbox_changes(
    mut events: MessageReader<CheckboxChangeEvent>,
) {
    for event in events.read() {
        info!("Checkbox {:?} -> {:?}", event.entity, event.state);
    }
}"#,
            );
        });
}
