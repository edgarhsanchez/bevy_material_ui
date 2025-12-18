//! Window Size Classes for Adaptive Layouts
//!
//! Implements Material Design 3 window size classes for responsive UI.
//! Reference: <https://m3.material.io/foundations/layout/applying-layout/window-size-classes>

use bevy::prelude::*;

/// Window size class based on width
///
/// These breakpoints help create responsive layouts that work across devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WindowWidthClass {
    /// Compact: width < 600dp (phones in portrait)
    #[default]
    Compact,
    /// Medium: 600dp ≤ width < 840dp (tablets in portrait, foldables)
    Medium,
    /// Expanded: 840dp ≤ width < 1200dp (tablets in landscape, small laptops)
    Expanded,
    /// Large: 1200dp ≤ width < 1600dp (large tablets, laptops)
    Large,
    /// ExtraLarge: width ≥ 1600dp (desktops, wide monitors)
    ExtraLarge,
}

impl WindowWidthClass {
    /// Get the window width class from a pixel width
    pub fn from_width(width: f32) -> Self {
        match width {
            w if w < 600.0 => Self::Compact,
            w if w < 840.0 => Self::Medium,
            w if w < 1200.0 => Self::Expanded,
            w if w < 1600.0 => Self::Large,
            _ => Self::ExtraLarge,
        }
    }

    /// Returns true if the width class supports two-pane layouts
    pub fn supports_two_panes(&self) -> bool {
        matches!(self, Self::Expanded | Self::Large | Self::ExtraLarge)
    }

    /// Returns true if the width class supports navigation rail (vs bottom nav)
    pub fn supports_nav_rail(&self) -> bool {
        !matches!(self, Self::Compact)
    }

    /// Returns true if the width class supports expanded navigation drawer
    pub fn supports_nav_drawer(&self) -> bool {
        matches!(self, Self::Large | Self::ExtraLarge)
    }
}

/// Window size class based on height
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WindowHeightClass {
    /// Compact: height < 480dp (phones in landscape)
    Compact,
    /// Medium: 480dp ≤ height < 900dp (tablets in landscape, phones in portrait)
    #[default]
    Medium,
    /// Expanded: height ≥ 900dp (tablets in portrait)
    Expanded,
}

impl WindowHeightClass {
    /// Get the window height class from a pixel height
    pub fn from_height(height: f32) -> Self {
        match height {
            h if h < 480.0 => Self::Compact,
            h if h < 900.0 => Self::Medium,
            _ => Self::Expanded,
        }
    }

    /// Returns true if vertical space is constrained
    pub fn is_height_constrained(&self) -> bool {
        matches!(self, Self::Compact)
    }
}

/// Resource tracking the current window size class
#[derive(Resource, Debug, Clone, Default)]
pub struct WindowSizeClass {
    /// Width class
    pub width: WindowWidthClass,
    /// Height class  
    pub height: WindowHeightClass,
    /// Raw window dimensions
    pub width_px: f32,
    pub height_px: f32,
}

impl WindowSizeClass {
    /// Create from window dimensions
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: WindowWidthClass::from_width(width),
            height: WindowHeightClass::from_height(height),
            width_px: width,
            height_px: height,
        }
    }

    /// Returns true if the layout should use a list-detail pattern
    pub fn use_list_detail(&self) -> bool {
        self.width.supports_two_panes()
    }

    /// Returns true if navigation should use a rail (vs bottom bar)
    pub fn use_nav_rail(&self) -> bool {
        self.width.supports_nav_rail()
    }

    /// Returns true if navigation drawer should be expanded
    pub fn use_expanded_drawer(&self) -> bool {
        self.width.supports_nav_drawer()
    }

    /// Get the recommended number of content columns
    pub fn content_columns(&self) -> u32 {
        match self.width {
            WindowWidthClass::Compact => 1,
            WindowWidthClass::Medium => 2,
            WindowWidthClass::Expanded => 3,
            WindowWidthClass::Large => 4,
            WindowWidthClass::ExtraLarge => 6,
        }
    }

    /// Get recommended margin size based on window class
    pub fn margin(&self) -> f32 {
        match self.width {
            WindowWidthClass::Compact => 16.0,
            WindowWidthClass::Medium => 24.0,
            _ => 24.0,
        }
    }

    /// Get recommended gutter (gap) size between items
    pub fn gutter(&self) -> f32 {
        match self.width {
            WindowWidthClass::Compact => 16.0,
            _ => 24.0,
        }
    }
}

/// Plugin for window size class tracking
pub struct WindowSizeClassPlugin;

impl Plugin for WindowSizeClassPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WindowSizeClass>()
            // Initialize once as early as possible so the first UI layout uses real
            // window dimensions instead of the resource default (0x0).
            .add_systems(Startup, update_window_size_class)
            .add_systems(Update, update_window_size_class);
    }
}

/// System that updates the window size class resource when window resizes
fn update_window_size_class(windows: Query<&Window>, mut size_class: ResMut<WindowSizeClass>) {
    let Ok(window) = windows.single() else {
        return;
    };

    let new_class = WindowSizeClass::new(window.width(), window.height());

    // Update whenever either the *class* or the *pixel dimensions* change.
    // The default resource value matches a common Compact/Medium pairing, so
    // relying only on class changes can leave `width_px`/`height_px` stuck at 0.
    if size_class.width != new_class.width
        || size_class.height != new_class.height
        || (size_class.width_px - new_class.width_px).abs() > f32::EPSILON
        || (size_class.height_px - new_class.height_px).abs() > f32::EPSILON
    {
        *size_class = new_class;
    }
}

/// Event fired when window size class changes
#[derive(Event, bevy::prelude::Message, Clone, Debug)]
pub struct WindowSizeClassChanged {
    pub old_width: WindowWidthClass,
    pub new_width: WindowWidthClass,
    pub old_height: WindowHeightClass,
    pub new_height: WindowHeightClass,
}
