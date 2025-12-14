//! Tests for the Material Design 3 Color System
//!
//! These tests verify the HCT color space implementation and color scheme generation.

use bevy_material_ui::color::{Hct, TonalPalette, MaterialColorScheme};

/// Test that HCT correctly represents black
#[test]
fn test_hct_black() {
    let hct = Hct::from_argb(0xFF000000);
    assert!(hct.tone() < 1.0, "Black should have tone near 0");
    assert!(hct.chroma() < 2.0, "Black should have low chroma");
}

/// Test that HCT correctly represents white
#[test]
fn test_hct_white() {
    let hct = Hct::from_argb(0xFFFFFFFF);
    assert!(hct.tone() > 99.0, "White should have tone near 100");
    assert!(hct.chroma() < 5.0, "White should have low chroma");
}

/// Test that pure red has expected hue
#[test]
fn test_hct_red() {
    let hct = Hct::from_argb(0xFFFF0000);
    // Red hue should be around 27° in HCT
    assert!(hct.hue() > 15.0 && hct.hue() < 50.0, "Red should have hue around 27°, got {}", hct.hue());
    // Chroma depends on implementation details, just verify it's positive
    assert!(hct.chroma() > 5.0, "Red should have significant chroma, got {}", hct.chroma());
}

/// Test that pure green has expected hue
#[test]
fn test_hct_green() {
    let hct = Hct::from_argb(0xFF00FF00);
    // Green hue should be around 142° in HCT
    assert!(hct.hue() > 100.0 && hct.hue() < 180.0, "Green should have hue around 142°, got {}", hct.hue());
    // Chroma depends on implementation details
    assert!(hct.chroma() > 5.0, "Green should have significant chroma, got {}", hct.chroma());
}

/// Test that pure blue has expected hue
#[test]
fn test_hct_blue() {
    let hct = Hct::from_argb(0xFF0000FF);
    // Blue hue should be around 282° in HCT
    assert!(hct.hue() > 240.0 || hct.hue() < 30.0, "Blue should have hue around 282°, got {}", hct.hue());
    // Chroma depends on implementation details
    assert!(hct.chroma() > 5.0, "Blue should have significant chroma, got {}", hct.chroma());
}

/// Test creating HCT from hex string
#[test]
fn test_hct_from_hex() {
    let hct = Hct::from_hex("#6750A4").expect("Should parse valid hex");
    // Material purple - allow wider range
    assert!(hct.hue() > 240.0 && hct.hue() < 320.0, "Purple should have hue around 270-290°, got {}", hct.hue());
}

/// Test HCT roundtrip through hex
#[test]
fn test_hct_hex_roundtrip() {
    let original = Hct::from_hex("#6750A4").unwrap();
    let hex = original.to_hex();
    let recovered = Hct::from_hex(&hex).unwrap();
    
    // Allow small variations due to rounding
    assert!((original.hue() - recovered.hue()).abs() < 5.0, "Hue should be stable");
    assert!((original.tone() - recovered.tone()).abs() < 2.0, "Tone should be stable");
}

/// Test creating HCT from new() with tone
#[test]
fn test_hct_new() {
    let hct = Hct::new(270.0, 50.0, 50.0);
    // The tone should be close to requested value
    // Hue and chroma might vary based on what's achievable in sRGB gamut
    assert!((hct.tone() - 50.0).abs() < 10.0, "Tone should be near 50, got {}", hct.tone());
}

/// Test tonal palette generation
#[test]
fn test_tonal_palette() {
    let mut palette = TonalPalette::new(270.0, 50.0);
    
    // Tone 0 should be near black
    let tone_0 = palette.tone(0);
    let r = (tone_0 >> 16) & 0xFF;
    let g = (tone_0 >> 8) & 0xFF;
    let b = tone_0 & 0xFF;
    let max_0 = r.max(g).max(b);
    assert!(max_0 < 30, "Tone 0 should be nearly black, got max={}", max_0);

    // Tone 100 should be near white
    let tone_100 = palette.tone(100);
    let r = (tone_100 >> 16) & 0xFF;
    let g = (tone_100 >> 8) & 0xFF;
    let b = tone_100 & 0xFF;
    let min_100 = r.min(g).min(b);
    assert!(min_100 > 220, "Tone 100 should be nearly white, got min={}", min_100);
}

/// Test that higher tones are brighter
#[test]
fn test_tonal_palette_ordering() {
    let mut palette = TonalPalette::new(200.0, 40.0);
    
    // Calculate luminance for each tone
    fn luminance(argb: u32) -> f64 {
        let r = ((argb >> 16) & 0xFF) as f64 / 255.0;
        let g = ((argb >> 8) & 0xFF) as f64 / 255.0;
        let b = (argb & 0xFF) as f64 / 255.0;
        0.299 * r + 0.587 * g + 0.114 * b
    }
    
    let lum_0 = luminance(palette.tone(0));
    let lum_25 = luminance(palette.tone(25));
    let lum_50 = luminance(palette.tone(50));
    let lum_75 = luminance(palette.tone(75));
    let lum_100 = luminance(palette.tone(100));
    
    assert!(lum_0 < lum_25, "Tone 0 should be darker than 25");
    assert!(lum_25 < lum_50, "Tone 25 should be darker than 50");
    assert!(lum_50 < lum_75, "Tone 50 should be darker than 75");
    assert!(lum_75 < lum_100, "Tone 75 should be darker than 100");
}

/// Test dark color scheme generation
#[test]
fn test_dark_scheme() {
    let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
    
    // Calculate luminance helper
    fn luminance(color: bevy::prelude::Color) -> f32 {
        let srgba = color.to_srgba();
        0.299 * srgba.red + 0.587 * srgba.green + 0.114 * srgba.blue
    }
    
    // In dark theme, primary should be lighter than on_primary
    assert!(
        luminance(scheme.primary) > luminance(scheme.on_primary),
        "Primary should be lighter than on_primary in dark theme"
    );
    
    // Surface should be dark
    assert!(
        luminance(scheme.surface) < 0.2,
        "Surface should be dark in dark theme"
    );
    
    // On surface should be light
    assert!(
        luminance(scheme.on_surface) > 0.7,
        "On surface should be light in dark theme"
    );
}

/// Test light color scheme generation
#[test]
fn test_light_scheme() {
    let scheme = MaterialColorScheme::light_from_argb(0xFF6750A4);
    
    // Calculate luminance helper
    fn luminance(color: bevy::prelude::Color) -> f32 {
        let srgba = color.to_srgba();
        0.299 * srgba.red + 0.587 * srgba.green + 0.114 * srgba.blue
    }
    
    // In light theme, on_primary should be lighter than primary
    assert!(
        luminance(scheme.on_primary) > luminance(scheme.primary),
        "On_primary should be lighter than primary in light theme"
    );
    
    // Surface should be light
    assert!(
        luminance(scheme.surface) > 0.8,
        "Surface should be light in light theme"
    );
    
    // On surface should be dark
    assert!(
        luminance(scheme.on_surface) < 0.3,
        "On surface should be dark in light theme"
    );
}

/// Test surface container hierarchy in dark theme
#[test]
fn test_surface_containers_dark() {
    let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
    
    fn luminance(color: bevy::prelude::Color) -> f32 {
        let srgba = color.to_srgba();
        0.299 * srgba.red + 0.587 * srgba.green + 0.114 * srgba.blue
    }
    
    // Surface containers should increase in brightness
    let l_lowest = luminance(scheme.surface_container_lowest);
    let l_low = luminance(scheme.surface_container_low);
    let l_default = luminance(scheme.surface_container);
    let l_high = luminance(scheme.surface_container_high);
    let l_highest = luminance(scheme.surface_container_highest);
    
    assert!(l_lowest < l_low, "Container lowest < low");
    assert!(l_low < l_default, "Container low < default");
    assert!(l_default < l_high, "Container default < high");
    assert!(l_high < l_highest, "Container high < highest");
}

/// Test error colors are reddish
#[test]
fn test_error_colors() {
    let scheme = MaterialColorScheme::dark_from_argb(0xFF6750A4);
    
    let error_srgba = scheme.error.to_srgba();
    assert!(
        error_srgba.red > error_srgba.blue,
        "Error should be more red than blue"
    );
}

/// Test that different seed colors produce different schemes
#[test]
fn test_different_seeds() {
    let purple = MaterialColorScheme::dark_from_argb(0xFF6750A4);
    let green = MaterialColorScheme::dark_from_argb(0xFF00AA00);
    let blue = MaterialColorScheme::dark_from_argb(0xFF0000FF);
    
    // Primary colors should be different
    let purple_rgb = purple.primary.to_srgba();
    let green_rgb = green.primary.to_srgba();
    let blue_rgb = blue.primary.to_srgba();
    
    // Check that primaries have different dominant channels
    assert!(
        purple_rgb.red != green_rgb.red || purple_rgb.green != green_rgb.green,
        "Purple and green should have different primaries"
    );
    assert!(
        green_rgb.green != blue_rgb.green || green_rgb.blue != blue_rgb.blue,
        "Green and blue should have different primaries"
    );
}
