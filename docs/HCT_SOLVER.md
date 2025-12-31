# HCT Solver

The CAM16/HCT solver implementation for `bevy_material_ui` lives in the standalone crate `hct-cam16`.

- Crates.io: https://crates.io/crates/hct-cam16
- Docs.rs: https://docs.rs/hct-cam16
- Repo: https://github.com/edgarhsanchez/hct_cam16

`bevy_material_ui::color::Hct` is a thin Bevy-friendly adapter (Bevy `Color` helpers) around the `hct_cam16` math.
The sRGB gamut is narrowest in the green region. At tone 50, pure green can only achieve about chroma 14, not 60. The old solver would:
1. Generate incorrect colors (too dark or wrong hue)
2. Fail to find colors near gamut boundaries
3. Produce inconsistent results across the hue range

### Solution: Accurate Gamut Mapping
The new solver:
1. Correctly finds maximum chroma for each hue/tone combination
2. Gracefully clamps requested chroma to achievable values
3. Maintains hue and tone accuracy while maximizing chroma

## Viewing Conditions

The solver uses standard sRGB viewing conditions:
- **White Point**: D65 (6504K daylight)
- **Adapting Luminance**: 11.72 cd/m² (~200 lux, typical office)
- **Background**: 20% gray
- **Surround**: Average (1.0)
- **Adaptation**: Full (D = 1.0)

## Mathematical Details

### CAM16 Chromatic Adaptation
```rust
fn chromatic_adaptation(component: f64) -> f64 {
    let af = component.abs().powf(0.42);
    component.signum() * 400.0 * af / (af + 27.13)
}
```

### Y ↔ L* Conversion
Uses CIE L*a*b* formulas with Y in [0, 100] range:
```rust
// L* → Y
fn y_from_lstar(lstar: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft = (lstar + 16.0) / 116.0;
    let ft3 = ft * ft * ft;
    
    100.0 * if ft3 > e { ft3 } else { (116.0 * ft - 16.0) / kappa }
}
```

### Gamut Boundary Check
```rust
fn is_bounded(x: f64) -> bool {
    0.0 <= x && x <= 100.0  // Linear RGB in [0, 100] range
}
```

## Testing

Run the color tests to verify correct operation:

```bash
cargo test color --lib

# Test specifically with green colors
cargo run --example test_green_colors
cargo run --example debug_hctsolver
```

Example output for greens:
```
Material Green (H:140, C:60, T:50):
  Result: Hue=139.34, Chroma=13.99, Tone=49.89
  ARGB: #FF278900

Green Tonal Palette (H:140, C:60):
  Tone  10: #FF042100 (actual chroma: 5.55)
  Tone  20: #FF0B3900 (actual chroma: 7.89)
  Tone  30: #FF145200 (actual chroma: 10.02)
  Tone  40: #FF1D6D00 (actual chroma: 12.08)
  Tone  50: #FF278900 (actual chroma: 13.99)
  Tone  60: #FF31A700 (actual chroma: 15.88)
  Tone  70: #FF3BC500 (actual chroma: 17.62)
  Tone  80: #FF4EE319 (actual chroma: 18.67)
```

## References

- [Material Design 3 - The Science of Color & Design](https://m3.material.io/blog/science-of-color-design)
- Reference implementation: `HctSolver.java` in the Material Components source tree.
- [CAM16 Color Appearance Model](https://doi.org/10.1002/col.22131)
- [CIE L*a*b* Color Space](https://en.wikipedia.org/wiki/CIELAB_color_space)

## License

This implementation is part of bevy_material_ui, licensed under the same terms as the parent project.

The algorithm is based on Material Design 3 reference implementation which is licensed under Apache 2.0.
