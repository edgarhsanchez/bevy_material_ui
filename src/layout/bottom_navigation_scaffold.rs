use bevy::prelude::*;

use crate::theme::MaterialTheme;

use super::{ScaffoldEntities, ScaffoldTestIds};

/// Configuration for a Material 3 navigation bar scaffold (content + bottom bar).
///
/// This is historically called "bottom navigation", but in Material 3 the component
/// is the **navigation bar**.
#[derive(Debug, Clone)]
pub struct BottomNavigationScaffold {
    pub bottom_bar_height_px: f32,
    pub root_padding_px: f32,
    pub bottom_bar_padding_px: f32,
    pub content_padding_px: f32,
    pub test_ids: ScaffoldTestIds,
}

impl Default for BottomNavigationScaffold {
    fn default() -> Self {
        Self {
            bottom_bar_height_px: 80.0,
            root_padding_px: 0.0,
            bottom_bar_padding_px: 12.0,
            content_padding_px: 0.0,
            test_ids: ScaffoldTestIds::default(),
        }
    }
}

/// Spawn a bottom-navigation scaffold.
///
/// The `navigation` slot is placed at the bottom of the screen. The `content`
/// slot fills the remaining space.
pub fn spawn_bottom_navigation_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &BottomNavigationScaffold,
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
    nav_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    let mut navigation = Entity::PLACEHOLDER;
    let mut content = Entity::PLACEHOLDER;

    let root = parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(config.root_padding_px)),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
            config.test_ids.root.clone(),
        ))
        .with_children(|root| {
            content = root
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        // Important for flex + scroll containers; allows children to shrink
                        // instead of forcing the column to overflow and get clipped.
                        min_height: Val::Px(0.0),
                        padding: UiRect::all(Val::Px(config.content_padding_px)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                    config.test_ids.content.clone(),
                ))
                .with_children(content_children)
                .id();

            navigation = root
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(config.bottom_bar_height_px),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(config.bottom_bar_padding_px)),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    config.test_ids.navigation.clone(),
                ))
                .with_children(nav_children)
                .id();
        })
        .id();

    ScaffoldEntities {
        root,
        navigation,
        content,
    }
}

/// Material 3 naming alias for [`BottomNavigationScaffold`].
pub type NavigationBarScaffold = BottomNavigationScaffold;

/// Material 3 naming alias for [`spawn_bottom_navigation_scaffold`].
pub fn spawn_navigation_bar_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &NavigationBarScaffold,
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
    nav_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    spawn_bottom_navigation_scaffold(parent, theme, config, content_children, nav_children)
}
