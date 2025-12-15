//! Icons view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the icons section content
pub fn spawn_icons_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Material Icons",
                "Google Material Symbols with variable font support"
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
                    let icons = [
                        MaterialIcon::check(),
                        MaterialIcon::home(),
                        MaterialIcon::settings(),
                        MaterialIcon::favorite(),
                        MaterialIcon::search(),
                    ];

                    for icon in icons {
                        row.spawn((
                            icon,
                            IconStyle::default().with_size(24.0).with_color(theme.on_surface),
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(theme.surface_container),
                            BorderRadius::all(Val::Px(8.0)),
                        ));
                    }
                });

            spawn_code_block(section, theme,
r#"// Using Material Symbols icons
use bevy_material_ui::icons::{ICON_CHECK, icon_by_name};

// By constant
commands.spawn((
    Text::new(ICON_CHECK),
    TextFont { font: icon_font.0.clone(), font_size: 24.0, ..default() },
));

// By name lookup
if let Some(codepoint) = icon_by_name("home") {
    // Use codepoint...
}"#);
        });
}
