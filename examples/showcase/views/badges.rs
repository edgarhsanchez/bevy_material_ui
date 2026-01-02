//! Badges view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::icons::ICON_NOTIFICATIONS;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the badges section content
pub fn spawn_badges_section(
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
                "showcase.section.badges.title",
                "Badges",
                "showcase.section.badges.description",
                "Notification indicators for counts and status",
            );
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(32.0),
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Dot badge
                    spawn_badge_example(row, theme, None);
                    // Small count
                    spawn_badge_example(row, theme, Some("3"));
                    // Large count
                    spawn_badge_example(row, theme, Some("99+"));
                });

            spawn_code_block(
                section,
                theme,
                r#"// Dot badge (no text)
let badge = MaterialBadge::dot();

// Count badge
let badge = MaterialBadge::count(5);

// Count badge with max
let badge = MaterialBadge::count(150).max(99); // Shows "99+""#,
            );
        });
}

fn spawn_badge_example(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    count: Option<&str>,
) {
    parent
        .spawn((
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
        .with_children(|container| {
            // Notification icon
            if let Some(icon) = MaterialIcon::from_name(ICON_NOTIFICATIONS) {
                container.spawn(icon.with_size(24.0).with_color(theme.on_surface));
            }

            // Badge (real MaterialBadge component)
            match count {
                None => {
                    container.spawn_small_badge(theme);
                }
                Some("99+") => {
                    container.spawn_badge_with(theme, BadgeBuilder::count(150).max(99));
                }
                Some(c) => {
                    let parsed = c.parse::<u32>().unwrap_or(0);
                    container.spawn_badge_count(theme, parsed);
                }
            }
        });
}
