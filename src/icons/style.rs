//! Icon Style Configuration
//!
//! Material Design icons are embedded as bitmaps from the `google-material-design-icons-bin` crate.
//! Icons support different styles:
//! - Fill: outlined or filled variants
//! - Weight: stroke thickness (100-700)
//! - Grade: visual prominence adjustment
//! - Optical Size: optimized for different display sizes (20, 24, 40, 48)

use bevy::prelude::*;

/// Icon weight (stroke thickness)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconWeight {
    /// Weight 100 - Thinnest
    Thin,
    /// Weight 200
    ExtraLight,
    /// Weight 300
    Light,
    /// Weight 400 - Default
    #[default]
    Regular,
    /// Weight 500
    Medium,
    /// Weight 600
    SemiBold,
    /// Weight 700 - Thickest
    Bold,
}

impl IconWeight {
    /// Get the numeric weight value (100-700)
    pub fn value(&self) -> u16 {
        match self {
            IconWeight::Thin => 100,
            IconWeight::ExtraLight => 200,
            IconWeight::Light => 300,
            IconWeight::Regular => 400,
            IconWeight::Medium => 500,
            IconWeight::SemiBold => 600,
            IconWeight::Bold => 700,
        }
    }
}

/// Icon grade (fine adjustment of weight)
///
/// Grade is used for subtle emphasis changes without affecting icon size.
/// Negative grades are lighter, positive grades are heavier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconGrade {
    /// Grade -25 - Reduced emphasis
    Low,
    /// Grade 0 - Default emphasis
    #[default]
    Normal,
    /// Grade 200 - High emphasis
    High,
}

impl IconGrade {
    /// Get the numeric grade value (-25 to 200)
    pub fn value(&self) -> i16 {
        match self {
            IconGrade::Low => -25,
            IconGrade::Normal => 0,
            IconGrade::High => 200,
        }
    }
}

/// Optical size for icon rendering
///
/// Icons are optimized for specific sizes. Use the size closest to your
/// actual display size for the best appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconOpticalSize {
    /// 20dp - Small icons, more detail
    Small,
    /// 24dp - Default icon size
    #[default]
    Default,
    /// 40dp - Medium-large icons
    Large,
    /// 48dp - Large icons, simpler forms
    ExtraLarge,
}

impl IconOpticalSize {
    /// Get the numeric optical size value (20, 24, 40, 48)
    pub fn value(&self) -> u8 {
        match self {
            IconOpticalSize::Small => 20,
            IconOpticalSize::Default => 24,
            IconOpticalSize::Large => 40,
            IconOpticalSize::ExtraLarge => 48,
        }
    }

    /// Get the recommended size in pixels at standard DPI
    pub fn size_px(&self) -> f32 {
        self.value() as f32
    }
}

/// Complete icon style configuration
#[derive(Debug, Clone, Copy, PartialEq, Default, Component)]
pub struct IconStyle {
    /// Whether the icon is filled (true) or outlined (false)
    pub filled: bool,
    /// Icon weight (stroke thickness)
    pub weight: IconWeight,
    /// Icon grade (emphasis adjustment)
    pub grade: IconGrade,
    /// Optical size optimization
    pub optical_size: IconOpticalSize,
    /// Icon color (None = inherit from theme/parent)
    pub color: Option<Color>,
    /// Icon size override (None = use optical_size)
    pub size: Option<f32>,
}

impl IconStyle {
    /// Create a new outlined icon style (default)
    pub fn outlined() -> Self {
        Self::default()
    }

    /// Create a new filled icon style
    pub fn filled() -> Self {
        Self {
            filled: true,
            ..default()
        }
    }

    /// Set whether the icon is filled
    pub fn with_fill(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Set the icon weight
    pub fn with_weight(mut self, weight: IconWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Set the icon grade
    pub fn with_grade(mut self, grade: IconGrade) -> Self {
        self.grade = grade;
        self
    }

    /// Set the optical size
    pub fn with_optical_size(mut self, size: IconOpticalSize) -> Self {
        self.optical_size = size;
        self
    }

    /// Set the icon color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set a custom size in pixels
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Get the effective size in pixels
    pub fn effective_size(&self) -> f32 {
        self.size.unwrap_or_else(|| self.optical_size.size_px())
    }

    /// Get the fill value for font variation (0.0 or 1.0)
    pub fn fill_value(&self) -> f32 {
        if self.filled {
            1.0
        } else {
            0.0
        }
    }

    /// Create style for small icons (20dp)
    pub fn small() -> Self {
        Self {
            optical_size: IconOpticalSize::Small,
            ..default()
        }
    }

    /// Create style for large icons (40dp)
    pub fn large() -> Self {
        Self {
            optical_size: IconOpticalSize::Large,
            ..default()
        }
    }

    /// Create style for extra large icons (48dp)
    pub fn extra_large() -> Self {
        Self {
            optical_size: IconOpticalSize::ExtraLarge,
            ..default()
        }
    }

    /// Create a bold icon style
    pub fn bold() -> Self {
        Self {
            weight: IconWeight::Bold,
            ..default()
        }
    }

    /// Create a light icon style
    pub fn light() -> Self {
        Self {
            weight: IconWeight::Light,
            ..default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_weight_values() {
        assert_eq!(IconWeight::Thin.value(), 100);
        assert_eq!(IconWeight::Regular.value(), 400);
        assert_eq!(IconWeight::Bold.value(), 700);
    }

    #[test]
    fn test_icon_grade_values() {
        assert_eq!(IconGrade::Low.value(), -25);
        assert_eq!(IconGrade::Normal.value(), 0);
        assert_eq!(IconGrade::High.value(), 200);
    }

    #[test]
    fn test_icon_optical_size_values() {
        assert_eq!(IconOpticalSize::Small.value(), 20);
        assert_eq!(IconOpticalSize::Default.value(), 24);
        assert_eq!(IconOpticalSize::Large.value(), 40);
        assert_eq!(IconOpticalSize::ExtraLarge.value(), 48);
    }

    #[test]
    fn test_icon_style_defaults() {
        let style = IconStyle::default();
        assert!(!style.filled);
        assert_eq!(style.weight, IconWeight::Regular);
        assert_eq!(style.grade, IconGrade::Normal);
        assert_eq!(style.optical_size, IconOpticalSize::Default);
    }

    #[test]
    fn test_icon_style_builder() {
        let style = IconStyle::outlined()
            .with_fill(true)
            .with_weight(IconWeight::Bold)
            .with_size(32.0);

        assert!(style.filled);
        assert_eq!(style.weight, IconWeight::Bold);
        assert_eq!(style.effective_size(), 32.0);
    }

    #[test]
    fn test_icon_style_presets() {
        assert!(IconStyle::filled().filled);
        assert!(!IconStyle::outlined().filled);
        assert_eq!(IconStyle::small().optical_size, IconOpticalSize::Small);
        assert_eq!(IconStyle::large().optical_size, IconOpticalSize::Large);
        assert_eq!(IconStyle::bold().weight, IconWeight::Bold);
        assert_eq!(IconStyle::light().weight, IconWeight::Light);
    }
}
