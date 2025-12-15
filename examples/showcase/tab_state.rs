//! Tab state cache to persist selections across UI rebuilds

use bevy::prelude::Resource;
use std::collections::HashMap;

use super::showcase::ComponentSection;

/// Stores tab selections across UI rebuilds
#[derive(Resource, Default)]
pub struct TabStateCache {
    /// Maps section name to selected tab index
    pub selections: HashMap<ComponentSection, usize>,
}
