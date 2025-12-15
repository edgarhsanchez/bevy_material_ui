use bevy::prelude::*;

use crate::theme::MaterialTheme;

use super::{ScaffoldEntities, ScaffoldTestIds};

/// Configuration for a permanent-drawer scaffold (left navigation + content).
#[derive(Debug, Clone)]
pub struct PermanentDrawerScaffold {
    pub navigation_width_px: f32,
    pub root_padding_px: f32,
    pub root_gap_px: f32,
    pub navigation_padding_px: f32,
    pub content_padding_px: f32,
    pub test_ids: ScaffoldTestIds,
}

impl Default for PermanentDrawerScaffold {
    fn default() -> Self {
        Self {
            navigation_width_px: 240.0,
            root_padding_px: 0.0,
            root_gap_px: 0.0,
            navigation_padding_px: 12.0,
            content_padding_px: 16.0,
            test_ids: ScaffoldTestIds::default(),
        }
    }
}

/// Spawn a permanent-drawer scaffold.
///
/// This is the simplest Material layout used by the showcase: a left navigation
/// area and a right content area.
///
/// The layout is intentionally minimal and does not impose app bars or bottom
/// bars; those can be built inside the `content` slot.
pub fn spawn_permanent_drawer_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &PermanentDrawerScaffold,
    nav_children: impl FnOnce(&mut ChildSpawnerCommands),
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    let mut navigation = Entity::PLACEHOLDER;
    let mut content = Entity::PLACEHOLDER;

    let root = parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(config.root_padding_px)),
                column_gap: Val::Px(config.root_gap_px),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
            config.test_ids.root.clone(),
        ))
        .with_children(|root| {
            navigation = root
                .spawn((
                    Node {
                        width: Val::Px(config.navigation_width_px),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(config.navigation_padding_px)),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    config.test_ids.navigation.clone(),
                ))
                .with_children(nav_children)
                .id();

            content = root
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(config.content_padding_px)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                    config.test_ids.content.clone(),
                ))
                .with_children(content_children)
                .id();
        })
        .id();

    ScaffoldEntities {
        root,
        navigation,
        content,
    }
}
