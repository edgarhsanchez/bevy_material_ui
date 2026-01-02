//! Icons view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the icons section content
pub fn spawn_icons_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Handle<Font>,
) {
    let _ = icon_font;

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
                "showcase.section.icons.title",
                "Material Icons",
                "showcase.section.icons.description",
                "Embedded Material icons",
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
                    for icon_name in ["check", "home", "settings", "favorite", "search"] {
                        row.spawn((
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme.surface_container),
                            BorderRadius::all(Val::Px(8.0)),
                        ))
                        .with_children(|cell| {
                            if let Some(icon) = MaterialIcon::from_name(icon_name) {
                                cell.spawn(icon.with_size(24.0).with_color(theme.on_surface));
                            }
                        });
                    }
                });

            spawn_code_block(
                section,
                theme,
                r#"// Using embedded icons
use bevy_material_ui::icons::MaterialIcon;

if let Some(icon) = MaterialIcon::from_name("home") {
    commands.spawn(icon);
}"#,
            );
        });
}
