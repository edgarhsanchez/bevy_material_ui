//! Color System Benchmarks
//!
//! Measures the performance of HCT color space conversions,
//! palette generation, and color scheme creation.

use bevy_material_ui::color::{Hct, MaterialColorScheme, TonalPalette};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Benchmark HCT to sRGB conversion
fn bench_hct_to_srgb(c: &mut Criterion) {
    let mut group = c.benchmark_group("HCT Conversions");

    // Single conversion
    group.bench_function("hct_to_argb_single", |b| {
        b.iter(|| {
            let hct = Hct::new(black_box(210.0), black_box(50.0), black_box(50.0));
            black_box(hct.to_argb())
        })
    });

    // Batch conversions (simulating palette generation)
    for size in [10, 50, 100, 256].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("hct_to_argb_batch", size),
            size,
            |b, &size| {
                let hcts: Vec<_> = (0..size)
                    .map(|i| Hct::new((i as f64 * 3.6) % 360.0, 50.0, 50.0))
                    .collect();
                b.iter(|| {
                    for hct in &hcts {
                        black_box(hct.to_argb());
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark sRGB to HCT conversion
fn bench_srgb_to_hct(c: &mut Criterion) {
    let mut group = c.benchmark_group("sRGB to HCT");

    // Single conversion
    group.bench_function("argb_to_hct_single", |b| {
        b.iter(|| {
            let hct = Hct::from_argb(black_box(0xFF6750A4));
            black_box((hct.hue(), hct.chroma(), hct.tone()))
        })
    });

    // Various colors
    let colors = [
        0xFFFF0000u32, // Red
        0xFF00FF00,    // Green
        0xFF0000FF,    // Blue
        0xFF6750A4,    // MD3 Primary
        0xFF958DA5,    // MD3 Secondary
        0xFF625B71,    // MD3 Tertiary
    ];

    group.bench_function("argb_to_hct_batch", |b| {
        b.iter(|| {
            for &color in &colors {
                let hct = Hct::from_argb(black_box(color));
                black_box((hct.hue(), hct.chroma(), hct.tone()));
            }
        })
    });

    group.finish();
}

/// Benchmark tonal palette generation
fn bench_tonal_palette(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tonal Palette");

    // Create palette from hue and chroma
    group.bench_function("create_from_hue_chroma", |b| {
        b.iter(|| TonalPalette::new(black_box(210.0), black_box(36.0)))
    });

    // Create palette from argb
    group.bench_function("create_from_argb", |b| {
        b.iter(|| TonalPalette::from_argb(black_box(0xFF6750A4)))
    });

    // Get tones from palette
    group.bench_function("get_all_tones", |b| {
        let mut palette = TonalPalette::new(210.0, 36.0);
        let tones = [0u8, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];
        b.iter(|| {
            for &tone in &tones {
                black_box(palette.tone(tone));
            }
        })
    });

    // Tone caching performance
    group.bench_function("cached_tone_access", |b| {
        let mut palette = TonalPalette::new(210.0, 36.0);
        // Warm up cache
        for tone in 0..=100u8 {
            palette.tone(tone);
        }
        b.iter(|| {
            // Access cached values
            for tone in 0..=100u8 {
                black_box(palette.tone(tone));
            }
        })
    });

    group.finish();
}

/// Benchmark color scheme generation
fn bench_color_scheme(c: &mut Criterion) {
    let mut group = c.benchmark_group("Color Scheme");

    // Light scheme from argb
    group.bench_function("light_scheme_from_argb", |b| {
        b.iter(|| MaterialColorScheme::light_from_argb(black_box(0xFF6750A4)))
    });

    // Dark scheme from argb
    group.bench_function("dark_scheme_from_argb", |b| {
        b.iter(|| MaterialColorScheme::dark_from_argb(black_box(0xFF6750A4)))
    });

    // Multiple seed colors
    let seed_colors = [
        0xFFFF0000u32,
        0xFF00FF00,
        0xFF0000FF,
        0xFF6750A4,
        0xFFFFEB3B,
    ];

    group.bench_function("schemes_from_multiple_seeds", |b| {
        b.iter(|| {
            for &color in &seed_colors {
                black_box(MaterialColorScheme::light_from_argb(color));
            }
        })
    });

    group.finish();
}

/// Benchmark full theme generation (seed to scheme)
fn bench_theme_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Theme Generation");

    // Full theme from seed (light + dark)
    group.bench_function("full_theme_from_seed", |b| {
        b.iter(|| {
            let light = MaterialColorScheme::light_from_argb(black_box(0xFF6750A4));
            let dark = MaterialColorScheme::dark_from_argb(black_box(0xFF6750A4));
            black_box((light, dark))
        })
    });

    group.finish();
}

/// Benchmark HCT color manipulation
fn bench_hct_manipulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("HCT Manipulation");

    group.bench_function("create_and_modify", |b| {
        b.iter(|| {
            let hct = Hct::new(210.0, 50.0, 50.0);
            // Modify hue
            let new_hue = (hct.hue() + 30.0) % 360.0;
            let modified = Hct::new(new_hue, hct.chroma(), hct.tone());
            black_box(modified.to_argb())
        })
    });

    group.bench_function("tone_variations", |b| {
        let base_hct = Hct::from_argb(0xFF6750A4);
        let tones = [10, 20, 30, 40, 50, 60, 70, 80, 90];
        b.iter(|| {
            for &tone in &tones {
                let variant = Hct::new(base_hct.hue(), base_hct.chroma(), tone as f64);
                black_box(variant.to_argb());
            }
        })
    });

    group.finish();
}

/// Benchmark contrast calculations (for accessibility)
fn bench_contrast(c: &mut Criterion) {
    let mut group = c.benchmark_group("Contrast Calculations");

    group.bench_function("tone_difference", |b| {
        let hct1 = Hct::new(210.0, 50.0, 40.0);
        let hct2 = Hct::new(210.0, 50.0, 90.0);
        b.iter(|| {
            let diff = (hct1.tone() - hct2.tone()).abs();
            black_box(diff >= 50.0)
        })
    });

    // Find tone for contrast ratio
    group.bench_function("find_contrast_tone", |b| {
        let hct = Hct::new(210.0, 50.0, 50.0);
        b.iter(|| {
            // Binary search for tone with sufficient contrast
            let target_contrast = 4.5;
            let mut low_tone = 0.0;
            let mut high_tone = 100.0;
            for _ in 0..10 {
                let mid = (low_tone + high_tone) / 2.0;
                let contrast = (hct.tone() - mid).abs() / 100.0 * 21.0;
                if contrast < target_contrast {
                    low_tone = mid;
                } else {
                    high_tone = mid;
                }
            }
            black_box(high_tone)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_hct_to_srgb,
    bench_srgb_to_hct,
    bench_tonal_palette,
    bench_color_scheme,
    bench_theme_generation,
    bench_hct_manipulation,
    bench_contrast,
);

criterion_main!(benches);
