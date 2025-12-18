use bevy::prelude::*;

/// Minimal coordinator helper for layouts with a top app bar. It offsets content by the app bar
/// height and optionally allows collapsing via a provided factor.
#[derive(Debug, Clone, Copy)]
pub struct AppBarOffsetConfig {
    pub height_px: f32,
    pub collapsed_height_px: f32,
    pub collapse_factor: f32, // 0.0 = expanded, 1.0 = fully collapsed
}

impl Default for AppBarOffsetConfig {
    fn default() -> Self {
        Self {
            height_px: 64.0,
            collapsed_height_px: 48.0,
            collapse_factor: 0.0,
        }
    }
}

/// Apply a top inset to a content root to account for a top app bar.
pub fn apply_app_bar_inset(content: &mut EntityCommands, config: AppBarOffsetConfig) {
    let inset = lerp(
        config.height_px,
        config.collapsed_height_px,
        config.collapse_factor,
    );
    content.insert(Node {
        margin: UiRect::top(Val::Px(inset)),
        ..default()
    });
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
