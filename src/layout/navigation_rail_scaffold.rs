use bevy::prelude::*;

use crate::theme::MaterialTheme;

use super::{ScaffoldEntities, ScaffoldTestIds};

/// Configuration for a navigation-rail scaffold (left rail + content).
#[derive(Debug, Clone)]
pub struct NavigationRailScaffold {
    pub rail_width_px: f32,
    pub root_padding_px: f32,
    pub root_gap_px: f32,
    pub rail_padding_px: f32,
    pub content_padding_px: f32,
    pub test_ids: ScaffoldTestIds,
}

impl Default for NavigationRailScaffold {
    fn default() -> Self {
        Self {
            rail_width_px: 80.0,
            root_padding_px: 0.0,
            root_gap_px: 0.0,
            rail_padding_px: 8.0,
            content_padding_px: 16.0,
            test_ids: ScaffoldTestIds::default(),
        }
    }
}

/// Spawn a navigation-rail scaffold.
///
/// This is similar to a permanent drawer scaffold but uses a narrower navigation
/// surface intended for icons.
pub fn spawn_navigation_rail_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &NavigationRailScaffold,
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
                        width: Val::Px(config.rail_width_px),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(config.rail_padding_px)),
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
