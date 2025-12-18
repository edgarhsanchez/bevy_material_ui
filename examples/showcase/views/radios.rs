//! Radio buttons view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning radio buttons.
//! The radio component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::radio::SpawnRadioChild;

use crate::showcase::common::*;

/// Spawn the radio buttons section content
pub fn spawn_radios_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Radio Buttons",
                "Single selection within a group - only one can be selected",
            );

            section
                .spawn((
                    RadioGroup::new("example_group"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        margin: UiRect::vertical(Val::Px(8.0)),
                        ..default()
                    },
                ))
                .with_children(|col| {
                    // Simple, clean API - just pass theme, selected state, group name, and label
                    col.spawn_radio(theme, true, "example_group", "Choice A");
                    col.spawn_radio(theme, false, "example_group", "Choice B");
                    col.spawn_radio(theme, false, "example_group", "Choice C");
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create radios in a group - simple and clean!
commands.spawn((
    RadioGroup::new("my_group"),
    Node { flex_direction: FlexDirection::Column, ..default() },
)).with_children(|group| {
    // Each radio is spawned with just a few parameters
    group.spawn_radio(&theme, true, "my_group", "Option 1");
    group.spawn_radio(&theme, false, "my_group", "Option 2");
    group.spawn_radio(&theme, false, "my_group", "Option 3");
    
    // Or use the builder for more control
    group.spawn_radio_with(
        &theme,
        RadioBuilder::new()
            .selected(false)
            .disabled(true)
            .group("my_group"),
        "Disabled Option"
    );
});"#,
            );
        });
}
