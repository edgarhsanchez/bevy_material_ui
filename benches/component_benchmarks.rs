//! Component Benchmarks
//!
//! Measures the performance of UI component creation and configuration.
//! These benchmarks focus on the data structure operations without the ECS.

use bevy_material_ui::{
    button::{ButtonVariant, MaterialButton},
    card::MaterialCard,
    checkbox::{CheckboxState, MaterialCheckbox},
    chip::{ChipVariant, MaterialChip},
    divider::MaterialDivider,
    elevation::Elevation,
    fab::{FabSize, MaterialFab},
    icon_button::{IconButtonVariant, MaterialIconButton},
    list::MaterialListItem,
    loading_indicator::MaterialLoadingIndicator,
    progress::{MaterialCircularProgress, MaterialLinearProgress},
    radio::MaterialRadio,
    search::MaterialSearchBar,
    slider::MaterialSlider,
    switch::MaterialSwitch,
    tokens::{CornerRadius, Duration, Easing, Spacing},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Benchmark button component creation
fn bench_button_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Button Component");

    group.bench_function("create_filled", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Filled),
            )
        })
    });

    group.bench_function("create_outlined", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Outlined),
            )
        })
    });

    group.bench_function("create_elevated", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Elevated),
            )
        })
    });

    group.bench_function("create_tonal", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::FilledTonal),
            )
        })
    });

    group.bench_function("create_text", |b| {
        b.iter(|| {
            black_box(MaterialButton::new(black_box("Click Me")).with_variant(ButtonVariant::Text))
        })
    });

    // Full configuration
    group.bench_function("create_full_config", |b| {
        b.iter(|| {
            black_box(
                MaterialButton::new(black_box("Submit"))
                    .with_variant(ButtonVariant::Filled)
                    .disabled(black_box(false))
                    .with_icon(black_box("send")),
            )
        })
    });

    // Batch creation
    group.bench_function("create_batch_10", |b| {
        b.iter(|| {
            let buttons: Vec<_> = (0..10)
                .map(|i| MaterialButton::new(format!("Button {}", i)))
                .collect();
            black_box(buttons)
        })
    });

    group.finish();
}

/// Benchmark checkbox component
fn bench_checkbox(c: &mut Criterion) {
    let mut group = c.benchmark_group("Checkbox Component");

    group.bench_function("create_unchecked", |b| {
        b.iter(|| {
            black_box(MaterialCheckbox::new().with_state(black_box(CheckboxState::Unchecked)))
        })
    });

    group.bench_function("create_checked", |b| {
        b.iter(|| black_box(MaterialCheckbox::new().with_state(black_box(CheckboxState::Checked))))
    });

    group.bench_function("create_indeterminate", |b| {
        b.iter(|| {
            black_box(MaterialCheckbox::new().with_state(black_box(CheckboxState::Indeterminate)))
        })
    });

    group.bench_function("state_toggle", |b| {
        b.iter(|| {
            let state = black_box(CheckboxState::Unchecked);
            let toggled = state.toggle();
            black_box(toggled)
        })
    });

    group.finish();
}

/// Benchmark switch component
fn bench_switch(c: &mut Criterion) {
    let mut group = c.benchmark_group("Switch Component");

    group.bench_function("create_off", |b| {
        b.iter(|| black_box(MaterialSwitch::new().selected(black_box(false))))
    });

    group.bench_function("create_on", |b| {
        b.iter(|| black_box(MaterialSwitch::new().selected(black_box(true))))
    });

    group.bench_function("create_with_icon", |b| {
        b.iter(|| black_box(MaterialSwitch::new().with_icon().selected(black_box(true))))
    });

    group.finish();
}

/// Benchmark radio component
fn bench_radio(c: &mut Criterion) {
    let mut group = c.benchmark_group("Radio Component");

    group.bench_function("create_single", |b| {
        b.iter(|| black_box(MaterialRadio::new().group(black_box("group1"))))
    });

    group.bench_function("create_group_5", |b| {
        b.iter(|| {
            let radios: Vec<_> = (0..5)
                .map(|i| MaterialRadio::new().group("group1").selected(i == 0))
                .collect();
            black_box(radios)
        })
    });

    group.finish();
}

/// Benchmark slider component
fn bench_slider(c: &mut Criterion) {
    let mut group = c.benchmark_group("Slider Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialSlider::new(black_box(0.0), black_box(100.0))))
    });

    group.bench_function("create_with_value", |b| {
        b.iter(|| {
            black_box(
                MaterialSlider::new(black_box(0.0), black_box(100.0)).with_value(black_box(50.0)),
            )
        })
    });

    group.bench_function("create_with_step", |b| {
        b.iter(|| {
            black_box(
                MaterialSlider::new(black_box(0.0), black_box(100.0))
                    .with_value(black_box(50.0))
                    .with_step(black_box(5.0)),
            )
        })
    });

    group.bench_function("create_full_config", |b| {
        b.iter(|| {
            black_box(
                MaterialSlider::new(black_box(0.0), black_box(100.0))
                    .with_value(black_box(50.0))
                    .with_step(black_box(5.0))
                    .show_label()
                    .show_ticks(),
            )
        })
    });

    // Normalize value
    group.bench_function("normalize_value", |b| {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(50.0);
        b.iter(|| black_box(slider.normalized_value()))
    });

    group.finish();
}

/// Benchmark progress indicator
fn bench_progress(c: &mut Criterion) {
    let mut group = c.benchmark_group("Progress Component");

    group.bench_function("create_linear_determinate", |b| {
        b.iter(|| black_box(MaterialLinearProgress::new().with_progress(black_box(0.5))))
    });

    group.bench_function("create_linear_indeterminate", |b| {
        b.iter(|| black_box(MaterialLinearProgress::new().indeterminate()))
    });

    group.bench_function("create_circular", |b| {
        b.iter(|| black_box(MaterialCircularProgress::new().with_progress(black_box(0.75))))
    });

    group.finish();
}

/// Benchmark chip component
fn bench_chip(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chip Component");

    group.bench_function("create_assist", |b| {
        b.iter(|| black_box(MaterialChip::new(black_box("Help")).with_variant(ChipVariant::Assist)))
    });

    group.bench_function("create_filter", |b| {
        b.iter(|| {
            black_box(MaterialChip::new(black_box("Category")).with_variant(ChipVariant::Filter))
        })
    });

    group.bench_function("create_input", |b| {
        b.iter(|| black_box(MaterialChip::new(black_box("Tag")).with_variant(ChipVariant::Input)))
    });

    group.bench_function("create_suggestion", |b| {
        b.iter(|| {
            black_box(
                MaterialChip::new(black_box("Recommended")).with_variant(ChipVariant::Suggestion),
            )
        })
    });

    // Chip group
    group.bench_function("create_filter_group_8", |b| {
        let labels = [
            "All", "Recent", "Starred", "Shared", "Archived", "Drafts", "Sent", "Trash",
        ];
        b.iter(|| {
            let chips: Vec<_> = labels
                .iter()
                .map(|&label| MaterialChip::new(label).with_variant(ChipVariant::Filter))
                .collect();
            black_box(chips)
        })
    });

    group.finish();
}

/// Benchmark design tokens
fn bench_tokens(c: &mut Criterion) {
    let mut group = c.benchmark_group("Design Tokens");

    group.bench_function("spacing_values", |b| {
        b.iter(|| {
            black_box((
                Spacing::EXTRA_SMALL,
                Spacing::SMALL,
                Spacing::MEDIUM,
                Spacing::LARGE,
                Spacing::EXTRA_LARGE,
            ))
        })
    });

    group.bench_function("corner_radius_values", |b| {
        b.iter(|| {
            black_box((
                CornerRadius::NONE,
                CornerRadius::EXTRA_SMALL,
                CornerRadius::SMALL,
                CornerRadius::MEDIUM,
                CornerRadius::LARGE,
                CornerRadius::EXTRA_LARGE,
                CornerRadius::FULL,
            ))
        })
    });

    group.bench_function("elevation_values", |b| {
        b.iter(|| {
            black_box((
                Elevation::Level0,
                Elevation::Level1,
                Elevation::Level2,
                Elevation::Level3,
                Elevation::Level4,
                Elevation::Level5,
            ))
        })
    });

    group.bench_function("duration_values", |b| {
        b.iter(|| {
            black_box((
                Duration::SHORT1,
                Duration::SHORT2,
                Duration::MEDIUM1,
                Duration::MEDIUM2,
                Duration::LONG1,
                Duration::LONG2,
            ))
        })
    });

    group.bench_function("easing_control_points", |b| {
        b.iter(|| {
            black_box((
                Easing::Standard.control_points(),
                Easing::Emphasized.control_points(),
                Easing::EmphasizedDecelerate.control_points(),
            ))
        })
    });

    group.finish();
}

/// Benchmark checkbox state transitions
fn bench_state_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("State Changes");

    // Checkbox state toggles
    group.bench_function("checkbox_state_toggle_100", |b| {
        let mut states: Vec<CheckboxState> = (0..100).map(|_| CheckboxState::Unchecked).collect();
        b.iter(|| {
            for state in states.iter_mut() {
                *state = state.toggle();
            }
            black_box(states.len())
        })
    });

    // Slider normalized value calculations
    group.bench_function("slider_normalize_100", |b| {
        let sliders: Vec<_> = (0..100)
            .map(|i| MaterialSlider::new(0.0, 100.0).with_value(i as f32))
            .collect();
        b.iter(|| {
            for slider in &sliders {
                black_box(slider.normalized_value());
            }
        })
    });

    // Elevation calculations
    group.bench_function("elevation_calculations_100", |b| {
        let elevations: Vec<_> = (0..100)
            .map(|i| match i % 6 {
                0 => Elevation::Level0,
                1 => Elevation::Level1,
                2 => Elevation::Level2,
                3 => Elevation::Level3,
                4 => Elevation::Level4,
                _ => Elevation::Level5,
            })
            .collect();
        b.iter(|| {
            for elevation in &elevations {
                black_box((
                    elevation.dp(),
                    elevation.shadow_opacity(),
                    elevation.shadow_blur(),
                ));
            }
        })
    });

    group.finish();
}

/// Benchmark FAB component creation
fn bench_fab(c: &mut Criterion) {
    let mut group = c.benchmark_group("FAB Component");

    group.bench_function("create_regular", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add"))))
    });

    group.bench_function("create_small", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add")).with_size(FabSize::Small)))
    });

    group.bench_function("create_large", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add")).with_size(FabSize::Large)))
    });

    group.bench_function("create_extended", |b| {
        b.iter(|| black_box(MaterialFab::new(black_box("add")).extended(black_box("Create"))))
    });

    group.finish();
}

/// Benchmark icon button component
fn bench_icon_button(c: &mut Criterion) {
    let mut group = c.benchmark_group("IconButton Component");

    group.bench_function("create_standard", |b| {
        b.iter(|| black_box(MaterialIconButton::new(black_box("home"))))
    });

    group.bench_function("create_filled", |b| {
        b.iter(|| {
            black_box(
                MaterialIconButton::new(black_box("favorite"))
                    .with_variant(IconButtonVariant::Filled),
            )
        })
    });

    group.bench_function("create_outlined", |b| {
        b.iter(|| {
            black_box(
                MaterialIconButton::new(black_box("settings"))
                    .with_variant(IconButtonVariant::Outlined),
            )
        })
    });

    group.bench_function("create_tonal", |b| {
        b.iter(|| {
            black_box(
                MaterialIconButton::new(black_box("share"))
                    .with_variant(IconButtonVariant::FilledTonal),
            )
        })
    });

    group.finish();
}

/// Benchmark card component
fn bench_card(c: &mut Criterion) {
    let mut group = c.benchmark_group("Card Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialCard::new()))
    });

    group.bench_function("create_clickable", |b| {
        b.iter(|| black_box(MaterialCard::new().clickable()))
    });

    group.bench_function("create_draggable", |b| {
        b.iter(|| black_box(MaterialCard::new().draggable()))
    });

    group.finish();
}

/// Benchmark list component
fn bench_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("List Component");

    group.bench_function("create_list_item", |b| {
        b.iter(|| black_box(MaterialListItem::new(black_box("Item"))))
    });

    group.bench_function("create_list_item_with_icon", |b| {
        b.iter(|| {
            black_box(MaterialListItem::new(black_box("Item")).leading_icon(black_box("person")))
        })
    });

    group.bench_function("create_list_item_full", |b| {
        b.iter(|| {
            black_box(
                MaterialListItem::new(black_box("Title"))
                    .leading_icon(black_box("mail"))
                    .supporting_text(black_box("Supporting text"))
                    .trailing_icon(black_box("chevron_right")),
            )
        })
    });

    group.finish();
}

/// Benchmark loading indicator component
fn bench_loading_indicator(c: &mut Criterion) {
    let mut group = c.benchmark_group("LoadingIndicator Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new()))
    });

    group.bench_function("create_small", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new().with_size(24.0)))
    });

    group.bench_function("create_large", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new().with_size(64.0)))
    });

    group.bench_function("create_multi_color", |b| {
        b.iter(|| black_box(MaterialLoadingIndicator::new().multi_color()))
    });

    group.finish();
}

/// Benchmark search bar component
fn bench_search_bar(c: &mut Criterion) {
    let mut group = c.benchmark_group("SearchBar Component");

    group.bench_function("create_default", |b| {
        b.iter(|| black_box(MaterialSearchBar::new(black_box("Search..."))))
    });

    group.bench_function("create_with_text", |b| {
        b.iter(|| {
            black_box(MaterialSearchBar::new(black_box("Search...")).with_text(black_box("test")))
        })
    });

    group.finish();
}

/// Benchmark divider component
fn bench_divider(c: &mut Criterion) {
    let mut group = c.benchmark_group("Divider Component");

    group.bench_function("create_horizontal", |b| {
        b.iter(|| black_box(MaterialDivider::new()))
    });

    group.bench_function("create_vertical", |b| {
        b.iter(|| black_box(MaterialDivider::vertical()))
    });

    group.bench_function("create_inset", |b| {
        b.iter(|| black_box(MaterialDivider::new().inset()))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_button_creation,
    bench_checkbox,
    bench_switch,
    bench_radio,
    bench_slider,
    bench_progress,
    bench_chip,
    bench_fab,
    bench_icon_button,
    bench_card,
    bench_list,
    bench_loading_indicator,
    bench_search_bar,
    bench_divider,
    bench_tokens,
    bench_state_changes,
);

criterion_main!(benches);
