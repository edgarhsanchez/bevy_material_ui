//! Focus ring system for Material Design 3
//!
//! Focus rings provide visible keyboard focus indicators for accessibility.
//! Reference: <https://m3.material.io/foundations/interaction/states/state-layers>
//!
//! This module now leverages Bevy 0.17's native `Outline` component for rendering
//! focus rings, providing better performance and simpler implementation.

use bevy::prelude::*;
use bevy::ui::Outline;

/// Plugin for the focus ring system
pub struct FocusPlugin;

impl Plugin for FocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_focus_outline_system, update_focus_ring_system),
        );
    }
}

/// Component that enables focus ring on an entity
///
/// **New in Bevy 0.17**: This component now uses Bevy's native `Outline` component
/// for rendering focus rings when `use_native_outline` is enabled (default: true).
#[derive(Component, Default)]
pub struct Focusable {
    /// Whether the element is currently focused
    pub focused: bool,
    /// Whether focus came from keyboard navigation
    pub focus_visible: bool,
    /// Custom focus ring color
    pub ring_color: Option<Color>,
    /// Focus ring offset from the element
    pub ring_offset: f32,
    /// Focus ring width
    pub ring_width: f32,
    /// Whether to use Bevy's native Outline component (recommended)
    pub use_native_outline: bool,
}

impl Focusable {
    /// Create a new focusable component
    pub fn new() -> Self {
        Self {
            focused: false,
            focus_visible: false,
            ring_color: None,
            ring_offset: 2.0,
            ring_width: 3.0,
            use_native_outline: true,
        }
    }

    /// Create a focusable with legacy focus ring (child entity)
    pub fn legacy() -> Self {
        Self {
            use_native_outline: false,
            ..Self::new()
        }
    }

    /// Set the focus ring color
    pub fn with_color(mut self, color: Color) -> Self {
        self.ring_color = Some(color);
        self
    }

    /// Set the focus ring offset
    pub fn with_offset(mut self, offset: f32) -> Self {
        self.ring_offset = offset;
        self
    }

    /// Set the focus ring width
    pub fn with_width(mut self, width: f32) -> Self {
        self.ring_width = width;
        self
    }

    /// Convert to a Bevy `Outline` component
    ///
    /// This leverages Bevy 0.17's native outline rendering.
    pub fn to_outline(&self, default_color: Color) -> Outline {
        let color = if self.focus_visible {
            self.ring_color.unwrap_or(default_color)
        } else {
            Color::NONE
        };

        Outline::new(Val::Px(self.ring_width), Val::Px(self.ring_offset), color)
    }
}

/// Marker component for focus ring entities
#[derive(Component)]
pub struct FocusRing {
    /// The entity this focus ring belongs to
    pub target: Entity,
}

/// Event when an element gains focus
#[derive(Event, bevy::prelude::Message)]
pub struct FocusGained {
    /// The focused entity
    pub entity: Entity,
    /// Whether focus came from keyboard
    pub from_keyboard: bool,
}

/// Event when an element loses focus
#[derive(Event, bevy::prelude::Message)]
pub struct FocusLost {
    /// The entity that lost focus
    pub entity: Entity,
}

/// System to update focus using Bevy's native Outline component
///
/// This is the recommended approach for Bevy 0.17+ as it leverages
/// the engine's built-in outline rendering for better performance.
fn update_focus_outline_system(
    mut focusables: Query<(&Focusable, &mut Outline), Changed<Focusable>>,
) {
    for (focusable, mut outline) in focusables.iter_mut() {
        if focusable.use_native_outline {
            let default_color = Color::srgb(0.0, 0.47, 0.84); // MD3 primary default
            if focusable.focus_visible {
                outline.width = Val::Px(focusable.ring_width);
                outline.offset = Val::Px(focusable.ring_offset);
                outline.color = focusable.ring_color.unwrap_or(default_color);
            } else {
                outline.color = Color::NONE;
            }
        }
    }
}

/// System to update focus ring visibility (legacy approach using child entities)
fn update_focus_ring_system(
    focusables: Query<(&Focusable, &Children), Changed<Focusable>>,
    mut focus_rings: Query<&mut Node, With<FocusRing>>,
) {
    for (focusable, children) in focusables.iter() {
        // Skip if using native outline
        if focusable.use_native_outline {
            continue;
        }

        for child in children.iter() {
            if let Ok(mut node) = focus_rings.get_mut(child) {
                node.display = if focusable.focus_visible {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

/// Create a focus ring node bundle (legacy approach)
///
/// **Note**: Consider using `Focusable::to_outline()` with Bevy's native `Outline`
/// component for better performance. This function is retained for backwards compatibility.
pub fn create_focus_ring(target: Entity, color: Color, offset: f32, width: f32) -> impl Bundle {
    (
        FocusRing { target },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(-offset - width),
            top: Val::Px(-offset - width),
            right: Val::Px(-offset - width),
            bottom: Val::Px(-offset - width),
            border: UiRect::all(Val::Px(width)),
            display: Display::None,
            ..default()
        },
        BorderColor::all(color),
        BorderRadius::all(Val::Px(4.0 + offset)),
        BackgroundColor(Color::NONE),
    )
}

/// Create a native outline bundle for focus rings (recommended for Bevy 0.17+)
///
/// This uses Bevy's built-in `Outline` component which is more performant
/// than the legacy child entity approach.
pub fn create_native_focus_outline(color: Color, offset: f32, width: f32) -> Outline {
    Outline::new(Val::Px(width), Val::Px(offset), color)
}
