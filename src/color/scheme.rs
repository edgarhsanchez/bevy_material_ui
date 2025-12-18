//! Material Design 3 Color Scheme Generation
//!
//! This module generates complete MD3 color schemes from a seed color.
//! It provides all 26 standard color roles plus add-on roles.
//!
//! # Color Roles
//!
//! ## Primary Colors
//! - `primary` - Main brand color for key components
//! - `on_primary` - Text/icons on primary color
//! - `primary_container` - Standout container color
//! - `on_primary_container` - Text/icons on primary container
//!
//! ## Secondary Colors  
//! - `secondary` - Accent color for less prominent components
//! - `on_secondary` - Text/icons on secondary color
//! - `secondary_container` - Container for secondary elements
//! - `on_secondary_container` - Text/icons on secondary container
//!
//! ## Tertiary Colors
//! - `tertiary` - Contrast accent for creative expression
//! - `on_tertiary` - Text/icons on tertiary color
//! - `tertiary_container` - Container for tertiary elements
//! - `on_tertiary_container` - Text/icons on tertiary container
//!
//! ## Error Colors
//! - `error` - Error state color
//! - `on_error` - Text/icons on error color
//! - `error_container` - Container for error states
//! - `on_error_container` - Text/icons on error container
//!
//! ## Surface Colors
//! - `surface` - Default background
//! - `on_surface` - Primary text color
//! - `on_surface_variant` - Secondary text color
//! - `surface_container_lowest/low/default/high/highest` - Container hierarchy
//!
//! ## Outline Colors
//! - `outline` - Prominent borders
//! - `outline_variant` - Subtle borders

use super::palette::CorePalette;
use bevy::prelude::Color;

/// A complete Material Design 3 color scheme
///
/// Contains all 26 standard color roles derived from a seed color.
#[derive(Debug, Clone)]
pub struct MaterialColorScheme {
    // Primary
    /// Primary brand color
    pub primary: Color,
    /// Content color on primary
    pub on_primary: Color,
    /// Primary container background
    pub primary_container: Color,
    /// Content color on primary container
    pub on_primary_container: Color,

    // Secondary
    /// Secondary brand color
    pub secondary: Color,
    /// Content color on secondary
    pub on_secondary: Color,
    /// Secondary container background
    pub secondary_container: Color,
    /// Content color on secondary container
    pub on_secondary_container: Color,

    // Tertiary
    /// Tertiary accent color
    pub tertiary: Color,
    /// Content color on tertiary
    pub on_tertiary: Color,
    /// Tertiary container background
    pub tertiary_container: Color,
    /// Content color on tertiary container
    pub on_tertiary_container: Color,

    // Error
    /// Error state color
    pub error: Color,
    /// Content color on error
    pub on_error: Color,
    /// Error container background
    pub error_container: Color,
    /// Content color on error container
    pub on_error_container: Color,

    // Surface
    /// Main surface background
    pub surface: Color,
    /// Bright surface for emphasis
    pub surface_bright: Color,
    /// Dim surface for de-emphasis
    pub surface_dim: Color,
    /// Primary content on surface
    pub on_surface: Color,
    /// Secondary content on surface
    pub on_surface_variant: Color,

    // Surface Containers
    /// Lowest emphasis container
    pub surface_container_lowest: Color,
    /// Low emphasis container
    pub surface_container_low: Color,
    /// Default container
    pub surface_container: Color,
    /// High emphasis container
    pub surface_container_high: Color,
    /// Highest emphasis container
    pub surface_container_highest: Color,

    // Outline
    /// Primary outline/border
    pub outline: Color,
    /// Subtle outline/border
    pub outline_variant: Color,

    // Inverse (for contrast)
    /// Inverse surface color
    pub inverse_surface: Color,
    /// Content on inverse surface
    pub inverse_on_surface: Color,
    /// Inverse primary color
    pub inverse_primary: Color,

    // Fixed Accent (constant across themes)
    /// Fixed primary color
    pub primary_fixed: Color,
    /// Dimmed fixed primary
    pub primary_fixed_dim: Color,
    /// Content on fixed primary
    pub on_primary_fixed: Color,
    /// Variant content on fixed primary
    pub on_primary_fixed_variant: Color,
    /// Fixed secondary color
    pub secondary_fixed: Color,
    /// Dimmed fixed secondary
    pub secondary_fixed_dim: Color,
    /// Content on fixed secondary
    pub on_secondary_fixed: Color,
    /// Variant content on fixed secondary
    pub on_secondary_fixed_variant: Color,
    /// Fixed tertiary color
    pub tertiary_fixed: Color,
    /// Dimmed fixed tertiary
    pub tertiary_fixed_dim: Color,
    /// Content on fixed tertiary
    pub on_tertiary_fixed: Color,
    /// Variant content on fixed tertiary
    pub on_tertiary_fixed_variant: Color,

    // Utility
    /// Scrim overlay color
    pub scrim: Color,
    /// Shadow color
    pub shadow: Color,
}

impl Default for MaterialColorScheme {
    fn default() -> Self {
        // Default Material You purple
        Self::dark_from_argb(0xFF6750A4)
    }
}

impl MaterialColorScheme {
    /// Generate a dark color scheme from a seed ARGB color
    pub fn dark_from_argb(seed: u32) -> Self {
        let mut palette = CorePalette::from_argb(seed);
        palette.cache_all();
        Self::dark_from_palette(&mut palette)
    }

    /// Generate a light color scheme from a seed ARGB color
    pub fn light_from_argb(seed: u32) -> Self {
        let mut palette = CorePalette::from_argb(seed);
        palette.cache_all();
        Self::light_from_palette(&mut palette)
    }

    /// Generate a dark color scheme from a Bevy Color seed
    pub fn dark_from_bevy_color(color: Color) -> Self {
        let mut palette = CorePalette::from_bevy_color(color);
        palette.cache_all();
        Self::dark_from_palette(&mut palette)
    }

    /// Generate a light color scheme from a Bevy Color seed
    pub fn light_from_bevy_color(color: Color) -> Self {
        let mut palette = CorePalette::from_bevy_color(color);
        palette.cache_all();
        Self::light_from_palette(&mut palette)
    }

    /// Generate a dark color scheme from a CorePalette
    pub fn dark_from_palette(p: &mut CorePalette) -> Self {
        Self {
            // Primary - dark theme uses lighter tones for foreground
            primary: argb_to_color(p.primary.tone(80)),
            on_primary: argb_to_color(p.primary.tone(20)),
            primary_container: argb_to_color(p.primary.tone(30)),
            on_primary_container: argb_to_color(p.primary.tone(90)),

            // Secondary
            secondary: argb_to_color(p.secondary.tone(80)),
            on_secondary: argb_to_color(p.secondary.tone(20)),
            secondary_container: argb_to_color(p.secondary.tone(30)),
            on_secondary_container: argb_to_color(p.secondary.tone(90)),

            // Tertiary
            tertiary: argb_to_color(p.tertiary.tone(80)),
            on_tertiary: argb_to_color(p.tertiary.tone(20)),
            tertiary_container: argb_to_color(p.tertiary.tone(30)),
            on_tertiary_container: argb_to_color(p.tertiary.tone(90)),

            // Error
            error: argb_to_color(p.error.tone(80)),
            on_error: argb_to_color(p.error.tone(20)),
            error_container: argb_to_color(p.error.tone(30)),
            on_error_container: argb_to_color(p.error.tone(90)),

            // Surface - dark theme
            surface: argb_to_color(p.neutral.tone(6)),
            surface_bright: argb_to_color(p.neutral.tone(24)),
            surface_dim: argb_to_color(p.neutral.tone(6)),
            on_surface: argb_to_color(p.neutral.tone(90)),
            on_surface_variant: argb_to_color(p.neutral_variant.tone(80)),

            // Surface Containers - dark theme
            surface_container_lowest: argb_to_color(p.neutral.tone(4)),
            surface_container_low: argb_to_color(p.neutral.tone(10)),
            surface_container: argb_to_color(p.neutral.tone(12)),
            surface_container_high: argb_to_color(p.neutral.tone(17)),
            surface_container_highest: argb_to_color(p.neutral.tone(22)),

            // Outline
            outline: argb_to_color(p.neutral_variant.tone(60)),
            outline_variant: argb_to_color(p.neutral_variant.tone(30)),

            // Inverse
            inverse_surface: argb_to_color(p.neutral.tone(90)),
            inverse_on_surface: argb_to_color(p.neutral.tone(20)),
            inverse_primary: argb_to_color(p.primary.tone(40)),

            // Fixed Accent (same in both themes)
            primary_fixed: argb_to_color(p.primary.tone(90)),
            primary_fixed_dim: argb_to_color(p.primary.tone(80)),
            on_primary_fixed: argb_to_color(p.primary.tone(10)),
            on_primary_fixed_variant: argb_to_color(p.primary.tone(30)),
            secondary_fixed: argb_to_color(p.secondary.tone(90)),
            secondary_fixed_dim: argb_to_color(p.secondary.tone(80)),
            on_secondary_fixed: argb_to_color(p.secondary.tone(10)),
            on_secondary_fixed_variant: argb_to_color(p.secondary.tone(30)),
            tertiary_fixed: argb_to_color(p.tertiary.tone(90)),
            tertiary_fixed_dim: argb_to_color(p.tertiary.tone(80)),
            on_tertiary_fixed: argb_to_color(p.tertiary.tone(10)),
            on_tertiary_fixed_variant: argb_to_color(p.tertiary.tone(30)),

            // Utility
            scrim: Color::BLACK,
            shadow: Color::BLACK,
        }
    }

    /// Generate a light color scheme from a CorePalette
    pub fn light_from_palette(p: &mut CorePalette) -> Self {
        Self {
            // Primary - light theme uses darker tones for foreground
            primary: argb_to_color(p.primary.tone(40)),
            on_primary: argb_to_color(p.primary.tone(100)),
            primary_container: argb_to_color(p.primary.tone(90)),
            on_primary_container: argb_to_color(p.primary.tone(10)),

            // Secondary
            secondary: argb_to_color(p.secondary.tone(40)),
            on_secondary: argb_to_color(p.secondary.tone(100)),
            secondary_container: argb_to_color(p.secondary.tone(90)),
            on_secondary_container: argb_to_color(p.secondary.tone(10)),

            // Tertiary
            tertiary: argb_to_color(p.tertiary.tone(40)),
            on_tertiary: argb_to_color(p.tertiary.tone(100)),
            tertiary_container: argb_to_color(p.tertiary.tone(90)),
            on_tertiary_container: argb_to_color(p.tertiary.tone(10)),

            // Error
            error: argb_to_color(p.error.tone(40)),
            on_error: argb_to_color(p.error.tone(100)),
            error_container: argb_to_color(p.error.tone(90)),
            on_error_container: argb_to_color(p.error.tone(10)),

            // Surface - light theme
            surface: argb_to_color(p.neutral.tone(98)),
            surface_bright: argb_to_color(p.neutral.tone(98)),
            surface_dim: argb_to_color(p.neutral.tone(87)),
            on_surface: argb_to_color(p.neutral.tone(10)),
            on_surface_variant: argb_to_color(p.neutral_variant.tone(30)),

            // Surface Containers - light theme
            surface_container_lowest: argb_to_color(p.neutral.tone(100)),
            surface_container_low: argb_to_color(p.neutral.tone(96)),
            surface_container: argb_to_color(p.neutral.tone(94)),
            surface_container_high: argb_to_color(p.neutral.tone(92)),
            surface_container_highest: argb_to_color(p.neutral.tone(90)),

            // Outline
            outline: argb_to_color(p.neutral_variant.tone(50)),
            outline_variant: argb_to_color(p.neutral_variant.tone(80)),

            // Inverse
            inverse_surface: argb_to_color(p.neutral.tone(20)),
            inverse_on_surface: argb_to_color(p.neutral.tone(95)),
            inverse_primary: argb_to_color(p.primary.tone(80)),

            // Fixed Accent (same in both themes)
            primary_fixed: argb_to_color(p.primary.tone(90)),
            primary_fixed_dim: argb_to_color(p.primary.tone(80)),
            on_primary_fixed: argb_to_color(p.primary.tone(10)),
            on_primary_fixed_variant: argb_to_color(p.primary.tone(30)),
            secondary_fixed: argb_to_color(p.secondary.tone(90)),
            secondary_fixed_dim: argb_to_color(p.secondary.tone(80)),
            on_secondary_fixed: argb_to_color(p.secondary.tone(10)),
            on_secondary_fixed_variant: argb_to_color(p.secondary.tone(30)),
            tertiary_fixed: argb_to_color(p.tertiary.tone(90)),
            tertiary_fixed_dim: argb_to_color(p.tertiary.tone(80)),
            on_tertiary_fixed: argb_to_color(p.tertiary.tone(10)),
            on_tertiary_fixed_variant: argb_to_color(p.tertiary.tone(30)),

            // Utility
            scrim: Color::BLACK,
            shadow: Color::BLACK,
        }
    }
}

/// Convert ARGB to Bevy Color
fn argb_to_color(argb: u32) -> Color {
    let r = ((argb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((argb >> 8) & 0xFF) as f32 / 255.0;
    let b = (argb & 0xFF) as f32 / 255.0;
    Color::srgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_scheme_generation() {
        let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);

        // Primary should be lighter in dark theme
        let primary_srgba = scheme.primary.to_srgba();
        let on_primary_srgba = scheme.on_primary.to_srgba();

        // Primary should be brighter than on_primary in dark theme
        let primary_lum =
            0.299 * primary_srgba.red + 0.587 * primary_srgba.green + 0.114 * primary_srgba.blue;
        let on_primary_lum = 0.299 * on_primary_srgba.red
            + 0.587 * on_primary_srgba.green
            + 0.114 * on_primary_srgba.blue;
        assert!(
            primary_lum > on_primary_lum,
            "Primary should be lighter than on_primary in dark theme"
        );
    }

    #[test]
    fn test_light_scheme_generation() {
        let scheme = MaterialColorScheme::light_from_argb(0xFF6750A4);

        // Primary should be darker in light theme
        let primary_srgba = scheme.primary.to_srgba();
        let on_primary_srgba = scheme.on_primary.to_srgba();

        // on_primary should be brighter than primary in light theme
        let primary_lum =
            0.299 * primary_srgba.red + 0.587 * primary_srgba.green + 0.114 * primary_srgba.blue;
        let on_primary_lum = 0.299 * on_primary_srgba.red
            + 0.587 * on_primary_srgba.green
            + 0.114 * on_primary_srgba.blue;
        assert!(
            on_primary_lum > primary_lum,
            "on_primary should be lighter than primary in light theme"
        );
    }

    #[test]
    fn test_surface_hierarchy_dark() {
        let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);

        // Surface containers should increase in brightness
        fn luminance(c: Color) -> f32 {
            let srgba = c.to_srgba();
            0.299 * srgba.red + 0.587 * srgba.green + 0.114 * srgba.blue
        }

        assert!(
            luminance(scheme.surface_container_lowest) < luminance(scheme.surface_container_low)
        );
        assert!(luminance(scheme.surface_container_low) < luminance(scheme.surface_container));
        assert!(luminance(scheme.surface_container) < luminance(scheme.surface_container_high));
        assert!(
            luminance(scheme.surface_container_high) < luminance(scheme.surface_container_highest)
        );
    }

    #[test]
    fn test_error_colors() {
        let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);

        // Error should be reddish
        let error_srgba = scheme.error.to_srgba();
        assert!(
            error_srgba.red > error_srgba.blue,
            "Error should be more red than blue"
        );
    }

    #[test]
    fn test_from_bevy_color() {
        let seed = Color::srgb(0.4, 0.31, 0.64);
        let scheme = MaterialColorScheme::dark_from_bevy_color(seed);

        // Should generate valid colors
        let primary_srgba = scheme.primary.to_srgba();
        assert!(primary_srgba.red >= 0.0 && primary_srgba.red <= 1.0);
        assert!(primary_srgba.green >= 0.0 && primary_srgba.green <= 1.0);
        assert!(primary_srgba.blue >= 0.0 && primary_srgba.blue <= 1.0);
    }
}
