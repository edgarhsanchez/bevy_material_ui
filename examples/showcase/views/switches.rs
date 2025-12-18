//! Switches view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning switches.
//! The switch component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::switch::SpawnSwitchChild;

use crate::showcase::common::*;

/// Spawn the switches section content
pub fn spawn_switches_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Switches",
                "Toggle on/off with sliding thumb animation",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Simple, clean API - just pass theme, selected state, and label
                    col.spawn_switch(theme, true, "Wi-Fi");
                    col.spawn_switch(theme, false, "Bluetooth");
                    col.spawn_switch(theme, false, "Dark Mode");
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create switches with the simple spawn API
parent.spawn_switch(&theme, false, "Notifications");
parent.spawn_switch(&theme, true, "Auto-update");

// Or use the builder for more control
parent.spawn_switch_with(
    &theme,
    SwitchBuilder::new()
        .selected(true)
        .with_icon()
        .disabled(false),
    "Advanced Mode"
);

// Listen for changes
fn handle_switch_changes(
    mut events: MessageReader<SwitchChangeEvent>,
) {
    for event in events.read() {
        info!("Switch {:?} -> {}", event.entity, event.selected);
    }
}"#,
            );
        });
}
