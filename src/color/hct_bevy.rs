//! Bevy adapter for HCT (Hue, Chroma, Tone)
//!
//! `bevy_material_ui` exposes an `Hct` type that is convenient for Bevy users
//! (e.g. conversion to/from `bevy::prelude::Color`).
//!
//! All color science / gamut-mapping math is delegated to the published
//! `hct-cam16` crate.

use bevy::prelude::Color;

/// HCT color (Hue, Chroma, Tone), backed by the `hct-cam16` crate.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hct(hct_cam16::Hct);

impl Hct {
    /// Create HCT from hue, chroma, and tone.
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        Self(hct_cam16::Hct::new(hue, chroma, tone))
    }

    /// Create HCT from an ARGB integer (0xAARRGGBB).
    pub fn from_argb(argb: u32) -> Self {
        Self(hct_cam16::Hct::from_argb(argb))
    }

    /// Create HCT from sRGB components.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(hct_cam16::Hct::from_rgb(r, g, b))
    }

    /// Create HCT from a hex string (e.g., "#6750A4" or "6750A4").
    pub fn from_hex(hex: &str) -> Option<Self> {
        hct_cam16::Hct::from_hex(hex).map(Self)
    }

    /// Hue angle in degrees [0, 360).
    pub fn hue(&self) -> f64 {
        self.0.hue()
    }

    /// Chroma (colorfulness).
    pub fn chroma(&self) -> f64 {
        self.0.chroma()
    }

    /// Tone (lightness) [0, 100].
    pub fn tone(&self) -> f64 {
        self.0.tone()
    }

    /// ARGB integer (0xAARRGGBB).
    pub fn to_argb(&self) -> u32 {
        self.0.to_argb()
    }

    /// RGB tuple (r, g, b) each [0, 255].
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        self.0.to_rgb()
    }

    /// Hex string (e.g., "#6750A4").
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    /// Convert to Bevy Color (sRGB).
    pub fn to_bevy_color(&self) -> Color {
        let (r, g, b) = self.to_rgb();
        Color::srgb_u8(r, g, b)
    }

    /// Create from Bevy Color.
    pub fn from_bevy_color(color: Color) -> Self {
        let srgba = color.to_srgba();

        let r = float_to_u8(srgba.red);
        let g = float_to_u8(srgba.green);
        let b = float_to_u8(srgba.blue);

        Self::from_rgb(r, g, b)
    }

    /// Create a new HCT with a different hue.
    pub fn with_hue(&self, hue: f64) -> Self {
        Self(self.0.with_hue(hue))
    }

    /// Create a new HCT with a different chroma.
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self(self.0.with_chroma(chroma))
    }

    /// Create a new HCT with a different tone.
    pub fn with_tone(&self, tone: f64) -> Self {
        Self(self.0.with_tone(tone))
    }
}

fn float_to_u8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}
