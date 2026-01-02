use bevy::prelude::*;

use crate::telemetry::TestId;

/// Entities for common scaffold layouts (navigation + content).
#[derive(Debug, Clone, Copy)]
pub struct ScaffoldEntities {
    pub root: Entity,
    pub navigation: Entity,
    pub content: Entity,
}

/// TestIds for common scaffold layouts (navigation + content).
#[derive(Debug, Clone)]
pub struct ScaffoldTestIds {
    pub root: TestId,
    pub navigation: TestId,
    pub content: TestId,
}

impl Default for ScaffoldTestIds {
    fn default() -> Self {
        Self {
            root: TestId::new("scaffold_root"),
            navigation: TestId::new("scaffold_navigation"),
            content: TestId::new("scaffold_content"),
        }
    }
}

/// Entities for multi-pane layouts.
#[derive(Debug, Clone, Copy)]
pub struct PaneEntities {
    pub primary: Entity,
    pub secondary: Option<Entity>,
    pub supporting: Option<Entity>,
}

/// TestIds for multi-pane layouts.
#[derive(Debug, Clone)]
pub struct PaneTestIds {
    pub primary: TestId,
    pub secondary: TestId,
    pub supporting: TestId,
}

impl PaneTestIds {
    pub fn default_two_pane() -> Self {
        Self {
            primary: TestId::new("pane_primary"),
            secondary: TestId::new("pane_secondary"),
            supporting: TestId::new("pane_supporting"),
        }
    }

    pub fn default_three_pane() -> Self {
        Self {
            primary: TestId::new("pane_primary"),
            secondary: TestId::new("pane_secondary"),
            supporting: TestId::new("pane_supporting"),
        }
    }
}
