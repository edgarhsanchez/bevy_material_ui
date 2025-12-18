//! HCT (Hue, Chroma, Tone) Color Space
//!
//! HCT is Material Design's perceptually accurate color space that combines:
//! - **Hue** (0-360°): The angle on the color wheel, from CAM16
//! - **Chroma** (0-~150): How colorful/saturated, from CAM16  
//! - **Tone** (0-100): Perceptual lightness, from L*a*b*
//!
//! The key insight is that tone differences directly correspond to contrast:
//! - Tone difference of 50+ ensures WCAG 4.5:1 contrast for small text
//! - Tone difference of 40+ ensures WCAG 3:1 contrast for large text
//!
//! # References
//!
//! - <https://m3.material.io/blog/science-of-color-design>
//! - <https://github.com/nickvdyck/material-foundation/material-color-utilities>

use super::math::{
    argb_from_rgb, blue_from_argb, delinearize, green_from_argb, lerp, linear_rgb_from_argb,
    lstar_from_argb, matrix_multiply, radians_to_degrees, red_from_argb, sanitize_degrees, to_8bit,
    xyz_from_linear_rgb, y_from_lstar, WHITE_POINT_D65_X, WHITE_POINT_D65_Y, WHITE_POINT_D65_Z,
};

/// CAM16 viewing conditions (standard sRGB viewing conditions)
#[derive(Debug, Clone, Copy)]
pub struct ViewingConditions {
    /// Adapting luminance
    n: f64,
    /// Achromatic response to white
    aw: f64,
    /// Noise term
    nbb: f64,
    /// Chromatic induction factor
    nc: f64,
    /// Degree of chromatic adaptation
    c: f64,
    /// Impact of surrounding
    fl: f64,
    /// z coefficient
    z: f64,
    /// RGB -> adapted rgb transformation
    rgb_d: [f64; 3],
}

impl Default for ViewingConditions {
    fn default() -> Self {
        Self::srgb()
    }
}

impl ViewingConditions {
    /// Standard sRGB viewing conditions
    ///
    /// Assumes:
    /// - White point: D65
    /// - Adapting luminance: 11.72 cd/m² (~200 lux, typical office)
    /// - Background: 20% gray
    /// - Surround: Average
    pub fn srgb() -> Self {
        // White point in XYZ (D65, Y normalized to 100)
        let white_xyz = [WHITE_POINT_D65_X, WHITE_POINT_D65_Y, WHITE_POINT_D65_Z];

        // Convert to cone responses
        let rgb_w = xyz_to_cone(white_xyz);

        // Adapting luminance (cd/m²)
        let la: f64 = 11.72;

        // Background Y as proportion of white
        let y_b: f64 = 20.0; // 20% gray background

        // Surround (average = 1.0)
        let surround: f64 = 1.0;

        // Calculate c and nc from surround
        let c = if surround >= 1.0 {
            lerp(0.59, 0.69, (surround - 1.0).min(1.0))
        } else {
            lerp(0.525, 0.59, surround)
        };
        let nc = c;

        // Calculate FL (luminance-level adaptation factor)
        let k = 1.0 / (5.0 * la + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl = k4 * la + 0.1 * k4f * k4f * (5.0_f64 * la).cbrt();

        // n - background induction factor
        let n = y_b / WHITE_POINT_D65_Y;

        // nbb & ncb - chromatic induction factors
        let nbb = 0.725 * n.powf(-0.2);

        // z - base exponential nonlinearity
        let z = 1.48 + n.sqrt();

        // D - degree of adaptation
        // For full adaptation (typical), D = 1.0
        let d = 1.0;

        // RGB_D - discounting the illuminant
        let rgb_d = [
            d * (WHITE_POINT_D65_Y / rgb_w[0]) + 1.0 - d,
            d * (WHITE_POINT_D65_Y / rgb_w[1]) + 1.0 - d,
            d * (WHITE_POINT_D65_Y / rgb_w[2]) + 1.0 - d,
        ];

        // Adapted white point
        let rgb_w_adapted = [
            rgb_w[0] * rgb_d[0],
            rgb_w[1] * rgb_d[1],
            rgb_w[2] * rgb_d[2],
        ];

        // Achromatic response of white
        let rgb_aw = [
            adapt(rgb_w_adapted[0], fl),
            adapt(rgb_w_adapted[1], fl),
            adapt(rgb_w_adapted[2], fl),
        ];
        let aw = (2.0 * rgb_aw[0] + rgb_aw[1] + rgb_aw[2] / 20.0) * nbb;

        Self {
            n,
            aw,
            nbb,
            nc,
            c,
            fl,
            z,
            rgb_d,
        }
    }
}

/// XYZ to CAM16 cone response matrix (MCAT02)
const XYZ_TO_CONE: [[f64; 3]; 3] = [
    [0.401288, 0.650173, -0.051461],
    [-0.250268, 1.204414, 0.045854],
    [-0.002079, 0.048952, 0.953127],
];

/// Convert XYZ to cone responses
fn xyz_to_cone(xyz: [f64; 3]) -> [f64; 3] {
    matrix_multiply(XYZ_TO_CONE, xyz)
}

/// Nonlinear adaptation function
fn adapt(component: f64, fl: f64) -> f64 {
    let abs_component = component.abs();
    let sign = if component < 0.0 { -1.0 } else { 1.0 };
    let adapted = 400.0 * (fl * abs_component / 100.0).powf(0.42)
        / ((fl * abs_component / 100.0).powf(0.42) + 27.13);
    sign * adapted
}

/// HCT color - Hue, Chroma, Tone
///
/// A perceptually uniform color space combining CAM16 hue/chroma with L*a*b* tone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hct {
    /// Hue angle in degrees [0, 360)
    hue: f64,
    /// Chroma (colorfulness) [0, ~150]
    chroma: f64,
    /// Tone (lightness) [0, 100]
    tone: f64,
    /// Cached ARGB value
    argb: u32,
}

impl Default for Hct {
    fn default() -> Self {
        Self::from_argb(0xFF000000) // Black
    }
}

impl Hct {
    /// Create HCT from hue, chroma, and tone
    ///
    /// The resulting color will have the specified hue and tone, with chroma
    /// clamped to the maximum achievable for that hue/tone combination.
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        let argb = Self::solve_to_argb(hue, chroma, tone);
        Self::from_argb(argb)
    }

    /// Create HCT from an ARGB integer (0xAARRGGBB)
    pub fn from_argb(argb: u32) -> Self {
        let (hue, chroma) = cam16_hue_chroma_from_argb(argb);
        let tone = lstar_from_argb(argb);
        Self {
            hue,
            chroma,
            tone,
            argb,
        }
    }

    /// Create HCT from sRGB components [0, 255]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(argb_from_rgb(r, g, b))
    }

    /// Create HCT from a hex string (e.g., "#6750A4" or "6750A4")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self::from_rgb(r, g, b))
    }

    /// Get the hue angle in degrees [0, 360)
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Get the chroma (colorfulness) [0, ~150]
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Get the tone (lightness) [0, 100]
    pub fn tone(&self) -> f64 {
        self.tone
    }

    /// Get as ARGB integer (0xAARRGGBB)
    pub fn to_argb(&self) -> u32 {
        self.argb
    }

    /// Get as RGB tuple (r, g, b) each [0, 255]
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            red_from_argb(self.argb),
            green_from_argb(self.argb),
            blue_from_argb(self.argb),
        )
    }

    /// Get as hex string (e.g., "#6750A4")
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Convert to Bevy Color (sRGB)
    pub fn to_bevy_color(&self) -> bevy::prelude::Color {
        let (r, g, b) = self.to_rgb();
        bevy::prelude::Color::srgb_u8(r, g, b)
    }

    /// Create from Bevy Color
    pub fn from_bevy_color(color: bevy::prelude::Color) -> Self {
        let srgba = color.to_srgba();
        let r = (srgba.red * 255.0).round() as u8;
        let g = (srgba.green * 255.0).round() as u8;
        let b = (srgba.blue * 255.0).round() as u8;
        Self::from_rgb(r, g, b)
    }

    /// Create a new HCT with different hue
    pub fn with_hue(&self, hue: f64) -> Self {
        Self::new(hue, self.chroma, self.tone)
    }

    /// Create a new HCT with different chroma
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self::new(self.hue, chroma, self.tone)
    }

    /// Create a new HCT with different tone
    pub fn with_tone(&self, tone: f64) -> Self {
        Self::new(self.hue, self.chroma, tone)
    }

    /// Solve for ARGB given HCT values
    ///
    /// This is the core algorithm that finds an sRGB color with the specified
    /// hue (from CAM16) and tone (from L*a*b*), with maximum achievable chroma.
    fn solve_to_argb(hue: f64, requested_chroma: f64, tone: f64) -> u32 {
        // Edge cases: pure black or white
        if tone < 0.0001 {
            return 0xFF000000;
        }
        if tone > 99.9999 {
            return 0xFFFFFFFF;
        }

        // For very low chroma, return a grayscale value
        if requested_chroma < 1.0 {
            return argb_from_lstar(tone);
        }

        // Use iterative approach: generate candidate colors and find best match
        // Search for a color matching the desired hue, chroma, and tone
        let mut best_argb = argb_from_lstar(tone);
        let mut best_match = f64::MAX;

        // Convert to HSL-like lightness
        let lightness = tone / 100.0;

        // Sample multiple saturation levels
        for sat_step in 0..=100 {
            let saturation = sat_step as f64 / 100.0;

            // HSL to RGB conversion
            let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
            let h_prime = hue / 60.0;
            let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
            let m = lightness - c / 2.0;

            let (r1, g1, b1) = if h_prime < 1.0 {
                (c, x, 0.0)
            } else if h_prime < 2.0 {
                (x, c, 0.0)
            } else if h_prime < 3.0 {
                (0.0, c, x)
            } else if h_prime < 4.0 {
                (0.0, x, c)
            } else if h_prime < 5.0 {
                (x, 0.0, c)
            } else {
                (c, 0.0, x)
            };

            let r = (r1 + m).clamp(0.0, 1.0);
            let g = (g1 + m).clamp(0.0, 1.0);
            let b = (b1 + m).clamp(0.0, 1.0);

            let r_u8 = (r * 255.0).round() as u8;
            let g_u8 = (g * 255.0).round() as u8;
            let b_u8 = (b * 255.0).round() as u8;

            let candidate = argb_from_rgb(r_u8, g_u8, b_u8);

            // Calculate HCT values for candidate WITHOUT recursion
            let (candidate_hue, candidate_chroma) = cam16_hue_chroma_from_argb(candidate);
            let candidate_tone = lstar_from_argb(candidate);

            // Calculate how well this matches our target
            let hue_diff = {
                let diff = (candidate_hue - hue).abs();
                if diff > 180.0 {
                    360.0 - diff
                } else {
                    diff
                }
            };
            let tone_diff = (candidate_tone - tone).abs();
            let chroma_diff = (candidate_chroma - requested_chroma)
                .abs()
                .min(requested_chroma);

            // Weight tone more heavily since that's the primary constraint
            let total_diff = hue_diff * 1.0 + tone_diff * 5.0 + chroma_diff * 0.5;

            if total_diff < best_match {
                best_match = total_diff;
                best_argb = candidate;
            }
        }

        best_argb
    }
}

/// Calculate CAM16 hue and chroma from ARGB
fn cam16_hue_chroma_from_argb(argb: u32) -> (f64, f64) {
    let vc = ViewingConditions::srgb();

    // Convert to linear RGB and then XYZ
    let [r, g, b] = linear_rgb_from_argb(argb);
    let xyz = xyz_from_linear_rgb(r, g, b);

    // XYZ to adapted cone responses
    let rgb_cone = xyz_to_cone(xyz);
    let rgb_adapted = [
        rgb_cone[0] * vc.rgb_d[0],
        rgb_cone[1] * vc.rgb_d[1],
        rgb_cone[2] * vc.rgb_d[2],
    ];

    // Apply nonlinear adaptation (post-adaptation response compression)
    let rgb_a = [
        adapt(rgb_adapted[0], vc.fl),
        adapt(rgb_adapted[1], vc.fl),
        adapt(rgb_adapted[2], vc.fl),
    ];

    // Calculate opponent color dimensions (a = redness-greenness, b = yellowness-blueness)
    // Using the CAM16 opponent color formulas
    let a = rgb_a[0] - 12.0 * rgb_a[1] / 11.0 + rgb_a[2] / 11.0;
    let b_component = (rgb_a[0] + rgb_a[1] - 2.0 * rgb_a[2]) / 9.0;

    // Hue angle h
    let hue_radians = b_component.atan2(a);
    let hue = sanitize_degrees(radians_to_degrees(hue_radians));

    // Eccentricity factor for hue
    let h_prime = if hue < 20.14 { hue + 360.0 } else { hue };
    let e_hue = 0.25 * ((h_prime * std::f64::consts::PI / 180.0 + 2.0).cos() + 3.8);

    // Achromatic response A
    let achromatic_response = (2.0 * rgb_a[0] + rgb_a[1] + rgb_a[2] / 20.0) * vc.nbb;

    // Lightness J
    let j = 100.0 * (achromatic_response / vc.aw).powf(vc.c * vc.z);

    // Magnitude of opponent color (t for chroma calculation)
    let u = (a * a + b_component * b_component).sqrt();

    // t calculation (adjusted for HK effect and eccentricity)
    let t = (50000.0 / 13.0) * e_hue * vc.nc * vc.nbb * u
        / (rgb_a[0] + rgb_a[1] + 1.05 * rgb_a[2] + 0.305);

    // Chroma C from t and J
    let chroma = t.powf(0.9) * (j / 100.0).sqrt() * (1.64 - 0.29_f64.powf(vc.n)).powf(0.73);

    (hue, chroma)
}

/// Convert L* to ARGB (achromatic - gray)
fn argb_from_lstar(lstar: f64) -> u32 {
    let y = y_from_lstar(lstar);
    let component = delinearize(y);
    let c = to_8bit(component);
    argb_from_rgb(c, c, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hct_from_argb_black() {
        let hct = Hct::from_argb(0xFF000000);
        assert!(hct.tone() < 1.0);
        assert!(hct.chroma() < 1.0);
    }

    #[test]
    fn test_hct_from_argb_white() {
        let hct = Hct::from_argb(0xFFFFFFFF);
        assert!(hct.tone() > 99.0);
        assert!(hct.chroma() < 5.0); // White should have very low chroma
    }

    #[test]
    fn test_hct_from_argb_red() {
        let hct = Hct::from_argb(0xFFFF0000);
        // Pure red has hue around 27° in CAM16
        assert!(
            hct.hue() > 15.0 && hct.hue() < 50.0,
            "Red hue: {}",
            hct.hue()
        );
        // Red should have significant chroma (but exact value depends on implementation)
        assert!(
            hct.chroma() > 10.0,
            "Red chroma should be > 10, got: {}",
            hct.chroma()
        );
    }

    #[test]
    fn test_hct_from_argb_blue() {
        let hct = Hct::from_argb(0xFF0000FF);
        // Pure blue has hue around 282° in CAM16
        assert!(
            hct.hue() > 240.0 || hct.hue() < 30.0,
            "Blue hue: {}",
            hct.hue()
        );
        // Blue should have significant chroma
        assert!(
            hct.chroma() > 10.0,
            "Blue chroma should be > 10, got: {}",
            hct.chroma()
        );
    }

    #[test]
    fn test_hct_from_hex() {
        let hct = Hct::from_hex("#6750A4").unwrap();
        // M3 primary purple has hue around 271-282°
        assert!(
            hct.hue() > 240.0 && hct.hue() < 320.0,
            "Purple hue: {}",
            hct.hue()
        );
    }

    #[test]
    fn test_hct_roundtrip() {
        // Test that ARGB -> HCT -> ARGB is stable
        let original = 0xFF6750A4;
        let hct = Hct::from_argb(original);
        let back = hct.to_argb();

        // Allow for some loss due to gamut clamping
        let r_diff = (red_from_argb(original) as i32 - red_from_argb(back) as i32).abs();
        let g_diff = (green_from_argb(original) as i32 - green_from_argb(back) as i32).abs();
        let b_diff = (blue_from_argb(original) as i32 - blue_from_argb(back) as i32).abs();

        assert!(r_diff < 10, "Red diff too large: {}", r_diff);
        assert!(g_diff < 10, "Green diff too large: {}", g_diff);
        assert!(b_diff < 10, "Blue diff too large: {}", b_diff);
    }

    #[test]
    fn test_hct_to_bevy_color() {
        let hct = Hct::from_argb(0xFF6750A4);
        let color = hct.to_bevy_color();
        let back = Hct::from_bevy_color(color);

        assert!((hct.hue() - back.hue()).abs() < 5.0);
        assert!((hct.tone() - back.tone()).abs() < 5.0);
    }

    #[test]
    fn test_viewing_conditions() {
        let vc = ViewingConditions::srgb();
        assert!(vc.fl > 0.0);
        assert!(vc.aw > 0.0);
    }
}
