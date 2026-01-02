//! Checkboxes view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning checkboxes.
//! The checkbox component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_checkbox_i18n;

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
                "showcase.section.checkboxes.title",
                "Checkboxes",
                "showcase.section.checkboxes.description",
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
                    spawn_checkbox_i18n(
                        col,
                        theme,
                        CheckboxState::Checked,
                        "showcase.checkboxes.option_1",
                        "Option 1",
                    );
                    spawn_checkbox_i18n(
                        col,
                        theme,
                        CheckboxState::Unchecked,
                        "showcase.checkboxes.option_2",
                        "Option 2",
                    );
                    spawn_checkbox_i18n(
                        col,
                        theme,
                        CheckboxState::Unchecked,
                        "showcase.checkboxes.option_3",
                        "Option 3",
                    );
                });

            spawn_code_block(
                section,
                theme,
                r#"// Without i18n - simple API
parent.spawn_checkbox(&theme, CheckboxState::Unchecked, "Accept terms");

// With i18n - use LocalizedText component
parent.spawn(Node::default()).with_children(|row| {
    row.spawn((
        MaterialCheckbox::new().with_state(CheckboxState::Checked),
        Button,
        // ... other button components
    )).with_children(|checkbox| {
        // Add checkbox internals (state layer, box, icon)
        // ...
    });
    
    // Add localized label
    row.spawn((
        Text::new(""),
        LocalizedText::new("settings.remember_me")
            .with_default("Remember me"),
        TextFont { font_size: 14.0, ..default() },
        TextColor(theme.on_surface),
        NeedsInternationalFont,
    ));
});

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
