//! Material Design 3 Divider component
//!
//! Dividers are thin lines that group content in lists and layouts.
//! Reference: <https://m3.material.io/components/divider/overview>

use bevy::prelude::*;

use crate::theme::MaterialTheme;

/// Divider variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DividerVariant {
    /// Full-width divider (default)
    #[default]
    FullWidth,
    /// Inset divider (has left margin)
    Inset,
    /// Middle inset divider (has margins on both sides)
    MiddleInset,
}

/// Material divider component
#[derive(Component)]
pub struct MaterialDivider {
    /// Divider variant
    pub variant: DividerVariant,
    /// Whether the divider is vertical
    pub vertical: bool,
}

impl MaterialDivider {
    /// Create a new horizontal divider
    pub fn new() -> Self {
        Self {
            variant: DividerVariant::default(),
            vertical: false,
        }
    }

    /// Create a vertical divider
    pub fn vertical() -> Self {
        Self {
            variant: DividerVariant::default(),
            vertical: true,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: DividerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make inset
    pub fn inset(self) -> Self {
        self.with_variant(DividerVariant::Inset)
    }

    /// Make middle inset
    pub fn middle_inset(self) -> Self {
        self.with_variant(DividerVariant::MiddleInset)
    }

    /// Get the color
    pub fn color(&self, theme: &MaterialTheme) -> Color {
        theme.outline_variant
    }
}

impl Default for MaterialDivider {
    fn default() -> Self {
        Self::new()
    }
}

/// Divider thickness
pub const DIVIDER_THICKNESS: f32 = 1.0;
/// Inset margin
pub const DIVIDER_INSET: f32 = 16.0;

/// Builder for dividers
pub struct DividerBuilder {
    divider: MaterialDivider,
}

impl DividerBuilder {
    /// Create a new horizontal divider builder
    pub fn new() -> Self {
        Self {
            divider: MaterialDivider::new(),
        }
    }

    /// Create a vertical divider builder
    pub fn vertical() -> Self {
        Self {
            divider: MaterialDivider::vertical(),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: DividerVariant) -> Self {
        self.divider.variant = variant;
        self
    }

    /// Make full-width
    pub fn full_width(self) -> Self {
        self.variant(DividerVariant::FullWidth)
    }

    /// Make inset
    pub fn inset(self) -> Self {
        self.variant(DividerVariant::Inset)
    }

    /// Make middle inset
    pub fn middle_inset(self) -> Self {
        self.variant(DividerVariant::MiddleInset)
    }

    /// Build the divider bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let color = self.divider.color(theme);
        let is_vertical = self.divider.vertical;
        let variant = self.divider.variant;

        let margin = match variant {
            DividerVariant::FullWidth => UiRect::ZERO,
            DividerVariant::Inset => {
                if is_vertical {
                    UiRect::top(Val::Px(DIVIDER_INSET))
                } else {
                    UiRect::left(Val::Px(DIVIDER_INSET))
                }
            }
            DividerVariant::MiddleInset => {
                if is_vertical {
                    UiRect::vertical(Val::Px(DIVIDER_INSET))
                } else {
                    UiRect::horizontal(Val::Px(DIVIDER_INSET))
                }
            }
        };

        (
            self.divider,
            Node {
                width: if is_vertical {
                    Val::Px(DIVIDER_THICKNESS)
                } else {
                    Val::Percent(100.0)
                },
                height: if is_vertical {
                    Val::Percent(100.0)
                } else {
                    Val::Px(DIVIDER_THICKNESS)
                },
                margin,
                ..default()
            },
            BackgroundColor(color),
        )
    }
}

impl Default for DividerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a simple horizontal divider
pub fn horizontal_divider(theme: &MaterialTheme) -> impl Bundle {
    DividerBuilder::new().build(theme)
}

/// Helper function to create a simple vertical divider
pub fn vertical_divider(theme: &MaterialTheme) -> impl Bundle {
    DividerBuilder::vertical().build(theme)
}

/// Helper function to create an inset divider
pub fn inset_divider(theme: &MaterialTheme) -> impl Bundle {
    DividerBuilder::new().inset().build(theme)
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material dividers as children
pub trait SpawnDividerChild {
    /// Spawn a horizontal divider
    fn spawn_horizontal_divider(&mut self, theme: &MaterialTheme);

    /// Spawn a vertical divider
    fn spawn_vertical_divider(&mut self, theme: &MaterialTheme);

    /// Spawn an inset divider
    fn spawn_inset_divider(&mut self, theme: &MaterialTheme);

    /// Spawn a divider with full builder control
    fn spawn_divider_with(&mut self, theme: &MaterialTheme, builder: DividerBuilder);
}

impl SpawnDividerChild for ChildSpawnerCommands<'_> {
    fn spawn_horizontal_divider(&mut self, theme: &MaterialTheme) {
        self.spawn(DividerBuilder::new().build(theme));
    }

    fn spawn_vertical_divider(&mut self, theme: &MaterialTheme) {
        self.spawn(DividerBuilder::vertical().build(theme));
    }

    fn spawn_inset_divider(&mut self, theme: &MaterialTheme) {
        self.spawn(DividerBuilder::new().inset().build(theme));
    }

    fn spawn_divider_with(&mut self, theme: &MaterialTheme, builder: DividerBuilder) {
        self.spawn(builder.build(theme));
    }
}
