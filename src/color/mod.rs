//! Material Design 3 Color System
//!
//! This module provides the HCT (Hue, Chroma, Tone) color space and utilities
//! for generating Material Design 3 color schemes from a seed color.
//!
//! # Overview
//!
//! The HCT color space combines:
//! - **Hue** and **Chroma** from CAM16 (color appearance model)
//! - **Tone** (lightness) from L*a*b* color space
//!
//! This combination enables:
//! - Perceptually accurate color manipulation
//! - Predictable contrast ratios via tone differences
//! - Dynamic color scheme generation from any seed color
//!
//! # Example
//!
//! ```rust,ignore
//! use bevy_material_ui::color::{Hct, TonalPalette, MaterialColorScheme};
//!
//! // Create an HCT color from a hex value
//! let seed = Hct::from_argb(0xFF6750A4);
//!
//! // Generate a tonal palette
//! let palette = TonalPalette::from_hct(&seed);
//!
//! // Get a specific tone
//! let tone_40 = palette.tone(40);
//!
//! // Generate a complete color scheme
//! let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
//! ```

mod hct_bevy;
mod palette;
mod scheme;

pub use hct_bevy::Hct;
pub use palette::TonalPalette;
pub use scheme::MaterialColorScheme;
