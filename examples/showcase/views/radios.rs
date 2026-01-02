//! Radio buttons view for the showcase application.
//!
//! This demonstrates the clean, simple API for spawning radio buttons.
//! The radio component handles all internal structure - users just provide
//! configuration options.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::i18n_helpers::spawn_radio_i18n;

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
                "showcase.section.radio_buttons.title",
                "Radio Buttons",
                "showcase.section.radio_buttons.description",
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
                    spawn_radio_i18n(
                        col,
                        theme,
                        true,
                        "example_group",
                        "showcase.radios.choice_a",
                        "Choice A",
                    );
                    spawn_radio_i18n(
                        col,
                        theme,
                        false,
                        "example_group",
                        "showcase.radios.choice_b",
                        "Choice B",
                    );
                    spawn_radio_i18n(
                        col,
                        theme,
                        false,
                        "example_group",
                        "showcase.radios.choice_c",
                        "Choice C",
                    );
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
