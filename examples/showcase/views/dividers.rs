//! Dividers view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the dividers section content
pub fn spawn_dividers_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.dividers.title",
                "Dividers",
                "showcase.section.dividers.description",
                "Visual separators between content sections",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.dividers.content_above")
                            .with_default("Content above divider"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        NeedsInternationalFont,
                    ));

                    // Full-width divider (real MaterialDivider)
                    col.spawn_horizontal_divider(theme);

                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.dividers.content_below")
                            .with_default("Content below divider"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        NeedsInternationalFont,
                    ));

                    // Inset divider (real MaterialDivider)
                    col.spawn_inset_divider(theme);

                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.dividers.after_inset")
                            .with_default("After inset divider"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        NeedsInternationalFont,
                    ));
                });

            spawn_code_block(
                section,
                theme,
                r#"// Full-width divider
commands.spawn((
    Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
    BackgroundColor(theme.outline_variant),
));

// Inset divider (with left margin)
commands.spawn((
    Node { 
        width: Val::Percent(100.0), 
        height: Val::Px(1.0),
        margin: UiRect::left(Val::Px(16.0)),
        ..default() 
    },
    BackgroundColor(theme.outline_variant),
));"#,
            );
        });
}
