//! Tonal Palette for Material Design 3
//!
//! A tonal palette is a set of colors at different tones (lightness levels)
//! with the same hue and chroma. This enables creating harmonious color schemes
//! where colors at different tones automatically have good contrast.
//!
//! The standard tones used in MD3 are:
//! 0, 4, 6, 10, 12, 17, 20, 22, 24, 30, 40, 50, 60, 70, 80, 87, 90, 92, 94, 95, 96, 98, 99, 100

use super::Hct;
use bevy::prelude::Color;
use std::collections::HashMap;

/// Standard tones used in Material Design 3
pub const STANDARD_TONES: &[u8] = &[
    0, 4, 6, 10, 12, 17, 20, 22, 24, 30, 40, 50, 60, 70, 80, 87, 90, 92, 94, 95, 96, 98, 99, 100,
];

/// A tonal palette generates colors at different tones (lightness levels)
/// while maintaining the same hue and chroma.
#[derive(Debug, Clone)]
pub struct TonalPalette {
    /// Base hue angle (0-360)
    hue: f64,
    /// Base chroma value
    chroma: f64,
    /// Cache of generated colors
    cache: HashMap<u8, u32>,
}

impl TonalPalette {
    /// Create a new tonal palette from hue and chroma
    pub fn new(hue: f64, chroma: f64) -> Self {
        Self {
            hue,
            chroma,
            cache: HashMap::new(),
        }
    }

    /// Create a tonal palette from an HCT color
    pub fn from_hct(hct: &Hct) -> Self {
        Self::new(hct.hue(), hct.chroma())
    }

    /// Create a tonal palette from an ARGB color
    pub fn from_argb(argb: u32) -> Self {
        Self::from_hct(&Hct::from_argb(argb))
    }

    /// Create a tonal palette from a Bevy Color
    pub fn from_bevy_color(color: Color) -> Self {
        Self::from_hct(&Hct::from_bevy_color(color))
    }

    /// Get the hue of this palette
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Get the chroma of this palette
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Get a color at the specified tone (0-100)
    ///
    /// # Arguments
    /// * `tone` - Lightness level from 0 (black) to 100 (white)
    ///
    /// # Returns
    /// ARGB integer representing the color
    pub fn tone(&mut self, tone: u8) -> u32 {
        let tone = tone.min(100);

        if let Some(&cached) = self.cache.get(&tone) {
            return cached;
        }

        let hct = Hct::new(self.hue, self.chroma, tone as f64);
        let argb = hct.to_argb();
        self.cache.insert(tone, argb);
        argb
    }

    /// Get a Bevy Color at the specified tone
    pub fn tone_color(&mut self, tone: u8) -> Color {
        let argb = self.tone(tone);
        argb_to_bevy_color(argb)
    }

    /// Get an HCT color at the specified tone
    pub fn tone_hct(&self, tone: u8) -> Hct {
        Hct::new(self.hue, self.chroma, tone.min(100) as f64)
    }

    /// Pre-cache all standard tones
    pub fn cache_standard_tones(&mut self) {
        for &tone in STANDARD_TONES {
            self.tone(tone);
        }
    }
}

/// Core tonal palettes for a Material Design 3 color scheme
#[derive(Debug, Clone)]
pub struct CorePalette {
    /// Primary color palette (typically from the seed color)
    pub primary: TonalPalette,
    /// Secondary color palette (desaturated variant of primary)
    pub secondary: TonalPalette,
    /// Tertiary color palette (analogous hue)
    pub tertiary: TonalPalette,
    /// Neutral palette (for surfaces, very low chroma)
    pub neutral: TonalPalette,
    /// Neutral variant palette (slightly more chromatic neutral)
    pub neutral_variant: TonalPalette,
    /// Error palette (fixed red hue)
    pub error: TonalPalette,
}

impl CorePalette {
    /// Create a CorePalette from a seed ARGB color
    pub fn from_argb(seed: u32) -> Self {
        let hct = Hct::from_argb(seed);
        Self::from_hct(&hct)
    }

    /// Create a CorePalette from a seed HCT color
    pub fn from_hct(seed: &Hct) -> Self {
        let hue = seed.hue();
        let chroma = seed.chroma();

        Self {
            // Material Design 3 formula: primary uses max(48.0, seed_chroma).
            // 48.0 is MD3's minimum chroma specification for primary palettes to
            // maintain sufficient colorfulness, while higher seed chroma values
            // are preserved to keep highly chromatic seed colors vibrant.
            primary: TonalPalette::new(hue, chroma.max(48.0)),

            // Secondary uses the same hue with reduced chroma (16)
            secondary: TonalPalette::new(hue, 16.0),

            // Tertiary uses an analogous hue (60° rotation) with moderate chroma (24)
            tertiary: TonalPalette::new((hue + 60.0) % 360.0, 24.0),

            // Neutral has very low chroma (4) from the seed hue
            neutral: TonalPalette::new(hue, 4.0),

            // Neutral variant has slightly more chroma (8)
            neutral_variant: TonalPalette::new(hue, 8.0),

            // Error is always red
            error: TonalPalette::new(25.0, 84.0),
        }
    }

    /// Create a CorePalette from a Bevy Color seed
    pub fn from_bevy_color(color: Color) -> Self {
        Self::from_hct(&Hct::from_bevy_color(color))
    }

    /// Pre-cache all standard tones for all palettes
    pub fn cache_all(&mut self) {
        self.primary.cache_standard_tones();
        self.secondary.cache_standard_tones();
        self.tertiary.cache_standard_tones();
        self.neutral.cache_standard_tones();
        self.neutral_variant.cache_standard_tones();
        self.error.cache_standard_tones();
    }
}

/// Convert ARGB to Bevy Color
fn argb_to_bevy_color(argb: u32) -> Color {
    let r = ((argb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((argb >> 8) & 0xFF) as f32 / 255.0;
    let b = (argb & 0xFF) as f32 / 255.0;
    Color::srgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonal_palette_creation() {
        let palette = TonalPalette::new(270.0, 50.0);
        assert!((palette.hue() - 270.0).abs() < 0.001);
        assert!((palette.chroma() - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_tonal_palette_tones() {
        let mut palette = TonalPalette::new(270.0, 50.0);

        // Tone 0 should be near black
        let tone_0 = palette.tone(0);
        let r = (tone_0 >> 16) & 0xFF;
        let g = (tone_0 >> 8) & 0xFF;
        let b = tone_0 & 0xFF;
        assert!(r < 20 && g < 20 && b < 20, "Tone 0 should be near black");

        // Tone 100 should be near white
        let tone_100 = palette.tone(100);
        let r = (tone_100 >> 16) & 0xFF;
        let g = (tone_100 >> 8) & 0xFF;
        let b = tone_100 & 0xFF;
        assert!(
            r > 240 && g > 240 && b > 240,
            "Tone 100 should be near white"
        );
    }

    #[test]
    fn test_core_palette() {
        let palette = CorePalette::from_argb(0xFF6750A4);

        // Primary uses the MD3 target chroma.
        assert!((palette.primary.chroma() - 48.0).abs() < 0.001);

        // Secondary should have reduced chroma
        assert!((palette.secondary.chroma() - 16.0).abs() < 0.001);

        // Neutral should have very low chroma
        assert!(palette.neutral.chroma() <= 8.0);

        // Error should be red-ish hue
        assert!(palette.error.hue() < 50.0 || palette.error.hue() > 330.0);
    }

    #[test]
    fn test_palette_caching() {
        let mut palette = TonalPalette::new(200.0, 40.0);

        // First call should compute
        let first = palette.tone(50);

        // Second call should return cached value
        let second = palette.tone(50);

        assert_eq!(first, second);
    }

    #[test]
    fn test_bevy_color_conversion() {
        let mut palette = TonalPalette::new(120.0, 40.0);
        let color = palette.tone_color(50);

        // Should be a valid color
        let srgba = color.to_srgba();
        assert!(srgba.red >= 0.0 && srgba.red <= 1.0);
        assert!(srgba.green >= 0.0 && srgba.green <= 1.0);
        assert!(srgba.blue >= 0.0 && srgba.blue <= 1.0);
    }
}
