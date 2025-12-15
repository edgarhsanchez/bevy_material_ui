use bevy::prelude::*;

use crate::theme::MaterialTheme;

use super::{PaneEntities, PaneTestIds};

/// Configuration for a list-detail scaffold (two-pane canonical layout).
#[derive(Debug, Clone)]
pub struct ListDetailScaffold {
    pub primary_min_width_px: f32,
    pub secondary_min_width_px: f32,
    pub gutter_px: f32,
    pub padding_px: f32,
    pub test_ids: PaneTestIds,
}

impl Default for ListDetailScaffold {
    fn default() -> Self {
        Self {
            primary_min_width_px: 260.0,
            secondary_min_width_px: 340.0,
            gutter_px: 24.0,
            padding_px: 16.0,
            test_ids: PaneTestIds::default_two_pane(),
        }
    }
}

/// Spawn a list-detail scaffold with two panes.
pub fn spawn_list_detail_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &ListDetailScaffold,
    primary_children: impl FnOnce(&mut ChildSpawnerCommands),
    secondary_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> PaneEntities {
    let mut primary = Entity::PLACEHOLDER;
    let mut secondary = Entity::PLACEHOLDER;

    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(config.gutter_px),
                padding: UiRect::all(Val::Px(config.padding_px)),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
        ))
        .with_children(|root| {
            primary = root
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        min_width: Val::Px(config.primary_min_width_px),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    config.test_ids.primary.clone(),
                ))
                .with_children(primary_children)
                .id();

            secondary = root
                .spawn((
                    Node {
                        flex_grow: 2.0,
                        min_width: Val::Px(config.secondary_min_width_px),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                    config.test_ids.secondary.clone(),
                ))
                .with_children(secondary_children)
                .id();
        });

    PaneEntities {
        primary,
        secondary: Some(secondary),
        supporting: None,
    }
}
