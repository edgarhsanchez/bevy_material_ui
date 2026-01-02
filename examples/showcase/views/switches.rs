//! Switches view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning switches.
//! The switch component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_switch_i18n;

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
                "showcase.section.switches.title",
                "Switches",
                "showcase.section.switches.description",
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
                    spawn_switch_i18n(col, theme, true, "showcase.switches.wifi", "Wi-Fi");
                    spawn_switch_i18n(
                        col,
                        theme,
                        false,
                        "showcase.switches.bluetooth",
                        "Bluetooth",
                    );
                    spawn_switch_i18n(
                        col,
                        theme,
                        false,
                        "showcase.switches.dark_mode",
                        "Dark Mode",
                    );
                });

            spawn_code_block(
                section,
                theme,
                r#"// Without i18n - simple API
parent.spawn_switch(&theme, false, "Notifications");

// With i18n - use LocalizedText component
parent.spawn(Node::default()).with_children(|row| {
    row.spawn((
        MaterialSwitch::new().selected(true),
        Button,
        // ... other button components
    )).with_children(|switch| {
        // Add switch handle
        // ...
    });
    
    // Add localized label
    row.spawn((
        Text::new(""),
        LocalizedText::new("settings.wifi")
            .with_default("Wi-Fi"),
        TextFont { font_size: 14.0, ..default() },
        TextColor(theme.on_surface),
        NeedsInternationalFont,
    ));
});

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
