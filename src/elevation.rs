//! Elevation system for Material Design 3
//!
//! Elevation creates visual separation between surfaces using shadows and tonal color.
//! Reference: <https://m3.material.io/styles/elevation/overview>
//!
//! This module now leverages Bevy 0.17's native `BoxShadow` component for rendering
//! Material Design elevation shadows.

use bevy::prelude::*;
use bevy::ui::{BoxShadow, ShadowStyle, Val};

/// Elevation levels in Material Design 3
#[derive(Debug, Clone, Copy, PartialEq, Default, Component)]
pub enum Elevation {
    /// Level 0: No elevation (0dp)
    #[default]
    Level0,
    /// Level 1: Low elevation (1dp) - Cards, buttons
    Level1,
    /// Level 2: Medium-low elevation (3dp) - FABs, snackbars
    Level2,
    /// Level 3: Medium elevation (6dp) - Navigation drawer, bottom sheet
    Level3,
    /// Level 4: Medium-high elevation (8dp) - Menus, dialogs
    Level4,
    /// Level 5: High elevation (12dp) - Modal side sheets
    Level5,
}

impl Elevation {
    /// Get the elevation value in dp
    pub fn dp(&self) -> f32 {
        match self {
            Elevation::Level0 => 0.0,
            Elevation::Level1 => 1.0,
            Elevation::Level2 => 3.0,
            Elevation::Level3 => 6.0,
            Elevation::Level4 => 8.0,
            Elevation::Level5 => 12.0,
        }
    }

    /// Get the shadow opacity for this elevation level
    pub fn shadow_opacity(&self) -> f32 {
        match self {
            Elevation::Level0 => 0.0,
            Elevation::Level1 => 0.05,
            Elevation::Level2 => 0.08,
            Elevation::Level3 => 0.11,
            Elevation::Level4 => 0.12,
            Elevation::Level5 => 0.14,
        }
    }

    /// Get the tonal overlay opacity for this elevation level (for dark themes)
    pub fn tonal_overlay_opacity(&self) -> f32 {
        match self {
            Elevation::Level0 => 0.0,
            Elevation::Level1 => 0.05,
            Elevation::Level2 => 0.08,
            Elevation::Level3 => 0.11,
            Elevation::Level4 => 0.12,
            Elevation::Level5 => 0.14,
        }
    }

    /// Get the shadow blur radius for this elevation level
    pub fn shadow_blur(&self) -> f32 {
        self.dp() * 2.0
    }

    /// Get the shadow y-offset for this elevation level
    pub fn shadow_offset_y(&self) -> f32 {
        self.dp() * 0.5
    }

    /// Move to the next higher elevation level
    pub fn raise(&self) -> Self {
        match self {
            Elevation::Level0 => Elevation::Level1,
            Elevation::Level1 => Elevation::Level2,
            Elevation::Level2 => Elevation::Level3,
            Elevation::Level3 => Elevation::Level4,
            Elevation::Level4 => Elevation::Level5,
            Elevation::Level5 => Elevation::Level5,
        }
    }

    /// Move to the next lower elevation level
    pub fn lower(&self) -> Self {
        match self {
            Elevation::Level0 => Elevation::Level0,
            Elevation::Level1 => Elevation::Level0,
            Elevation::Level2 => Elevation::Level1,
            Elevation::Level3 => Elevation::Level2,
            Elevation::Level4 => Elevation::Level3,
            Elevation::Level5 => Elevation::Level4,
        }
    }

    /// Convert this elevation to a Bevy `BoxShadow` component
    ///
    /// This leverages Bevy 0.17's native shadow rendering for better performance.
    pub fn to_box_shadow(&self) -> BoxShadow {
        if *self == Elevation::Level0 {
            return BoxShadow::default();
        }

        BoxShadow::new(
            Color::srgba(0.0, 0.0, 0.0, self.shadow_opacity()),
            Val::Px(0.0),
            Val::Px(self.shadow_offset_y()),
            Val::Px(0.0),
            Val::Px(self.shadow_blur()),
        )
    }

    /// Create a `ShadowStyle` for this elevation level
    ///
    /// Useful when you need more control over the shadow styling.
    pub fn to_shadow_style(&self) -> ShadowStyle {
        ShadowStyle {
            color: Color::srgba(0.0, 0.0, 0.0, self.shadow_opacity()),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(self.shadow_offset_y()),
            spread_radius: Val::Px(0.0),
            blur_radius: Val::Px(self.shadow_blur()),
        }
    }
}

/// Shadow styling based on elevation
///
/// **Note**: Consider using `Elevation::to_box_shadow()` for native Bevy shadow rendering,
/// which offers better performance. This struct is retained for backwards compatibility.
#[derive(Debug, Clone)]
pub struct ElevationShadow {
    /// Shadow color
    pub color: Color,
    /// Horizontal offset
    pub offset_x: f32,
    /// Vertical offset  
    pub offset_y: f32,
    /// Blur radius
    pub blur: f32,
    /// Spread radius
    pub spread: f32,
}

impl ElevationShadow {
    /// Create shadow styling for an elevation level
    pub fn from_elevation(elevation: Elevation) -> Self {
        let base_color = Color::srgba(0.0, 0.0, 0.0, elevation.shadow_opacity());

        Self {
            color: base_color,
            offset_x: 0.0,
            offset_y: elevation.shadow_offset_y(),
            blur: elevation.shadow_blur(),
            spread: 0.0,
        }
    }
}
