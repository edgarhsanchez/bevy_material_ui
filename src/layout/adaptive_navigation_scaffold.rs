use bevy::prelude::*;

use crate::{adaptive::WindowSizeClass, theme::MaterialTheme};

use super::{
    spawn_bottom_navigation_scaffold, spawn_navigation_rail_scaffold, spawn_permanent_drawer_scaffold,
    BottomNavigationScaffold, NavigationRailScaffold, PermanentDrawerScaffold, ScaffoldEntities,
};

/// Material 3 **navigation suite scaffold**.
///
/// Selects an appropriate navigation pattern based on Material window size classes.
#[derive(Debug, Clone)]
pub struct AdaptiveNavigationScaffold {
    pub bottom: BottomNavigationScaffold,
    pub rail: NavigationRailScaffold,
    pub drawer: PermanentDrawerScaffold,
}

impl Default for AdaptiveNavigationScaffold {
    fn default() -> Self {
        Self {
            bottom: BottomNavigationScaffold::default(),
            rail: NavigationRailScaffold::default(),
            drawer: PermanentDrawerScaffold::default(),
        }
    }
}

/// Spawn an adaptive navigation scaffold.
///
/// Mapping:
/// - `Large` / `ExtraLarge` => permanent (expanded) drawer
/// - `Medium` / `Expanded`  => navigation rail
/// - `Compact`              => bottom navigation
pub fn spawn_adaptive_navigation_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    size_class: &WindowSizeClass,
    config: &AdaptiveNavigationScaffold,
    nav_children: impl FnOnce(&mut ChildSpawnerCommands),
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    if size_class.use_expanded_drawer() {
        spawn_permanent_drawer_scaffold(parent, theme, &config.drawer, nav_children, content_children)
    } else if size_class.use_nav_rail() {
        spawn_navigation_rail_scaffold(parent, theme, &config.rail, nav_children, content_children)
    } else {
        // Bottom navigation places the nav slot after the content.
        spawn_bottom_navigation_scaffold(parent, theme, &config.bottom, content_children, nav_children)
    }
}

/// Material 3 naming alias for [`AdaptiveNavigationScaffold`].
pub type NavigationSuiteScaffold = AdaptiveNavigationScaffold;

/// Material 3 naming alias for [`spawn_adaptive_navigation_scaffold`].
pub fn spawn_navigation_suite_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    size_class: &WindowSizeClass,
    config: &NavigationSuiteScaffold,
    nav_children: impl FnOnce(&mut ChildSpawnerCommands),
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    spawn_adaptive_navigation_scaffold(
        parent,
        theme,
        size_class,
        config,
        nav_children,
        content_children,
    )
}
