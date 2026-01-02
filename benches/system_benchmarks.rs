//! System Benchmarks
//!
//! Measures the performance of Bevy ECS systems for UI components.
//! These benchmarks use actual Bevy World and systems.

use bevy::prelude::*;
use bevy_material_ui::{
    button::{ButtonVariant, MaterialButton},
    card::MaterialCard,
    checkbox::{CheckboxState, MaterialCheckbox},
    divider::MaterialDivider,
    elevation::{Elevation, ElevationShadow},
    fab::MaterialFab,
    focus::Focusable,
    icon_button::MaterialIconButton,
    list::MaterialListItem,
    loading_indicator::MaterialLoadingIndicator,
    radio::MaterialRadio,
    ripple::Ripple,
    search::MaterialSearchBar,
    slider::MaterialSlider,
    switch::MaterialSwitch,
    theme::MaterialTheme,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Setup a minimal Bevy App for benchmarking
fn setup_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

/// Benchmark entity spawning with UI components
fn bench_entity_spawning(c: &mut Criterion) {
    let mut group = c.benchmark_group("Entity Spawning");

    // Spawn button entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("buttons", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialButton::new(format!("Button {}", i)),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn checkbox entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("checkboxes", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|_| {
                    (
                        MaterialCheckbox::new().with_state(CheckboxState::Unchecked),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn switch entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("switches", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialSwitch::new(), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn slider entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("sliders", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|_| {
                    (
                        MaterialSlider::new(0.0, 100.0).with_value(50.0),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn radio entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("radios", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialRadio::new()
                            .selected(i % 2 == 0)
                            .group("test_group"),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn FAB entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("fabs", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialFab::new("add"), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn icon button entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("icon_buttons", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.world_mut().spawn_batch(
                        (0..count).map(|_| (MaterialIconButton::new("home"), Node::default())),
                    );
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    // Spawn card entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("cards", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialCard::new(), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn list item entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("list_items", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialListItem::new(format!("Item {}", i)),
                        Node::default(),
                    )
                }));
                black_box(app.world().entities().len())
            })
        });
    }

    // Spawn loading indicator entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("loading_indicators", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.world_mut().spawn_batch(
                        (0..count).map(|_| (MaterialLoadingIndicator::new(), Node::default())),
                    );
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    // Spawn search bar entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("search_bars", count),
            count,
            |b, &count| {
                b.iter(|| {
                    let mut app = setup_app();
                    app.world_mut().spawn_batch(
                        (0..count).map(|_| (MaterialSearchBar::new("Search..."), Node::default())),
                    );
                    black_box(app.world().entities().len())
                })
            },
        );
    }

    // Spawn divider entities
    for count in [10, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("dividers", count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_app();
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (MaterialDivider::new(), Node::default())));
                black_box(app.world().entities().len())
            })
        });
    }

    group.finish();
}

/// Benchmark component queries
fn bench_component_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("Component Queries");

    // Query buttons
    for count in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("query_buttons", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialButton::new(format!("Button {}", i)),
                        Node::default(),
                    )
                }));

                b.iter(|| {
                    let mut query_count = 0;
                    let world = app.world_mut();
                    let mut query = world.query::<&MaterialButton>();
                    for button in query.iter(world) {
                        black_box(button);
                        query_count += 1;
                    }
                    black_box(query_count)
                })
            },
        );
    }

    // Query with filters
    for count in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("query_with_filter", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities, half with different variants
                app.world_mut().spawn_batch((0..count).map(|i| {
                    (
                        MaterialButton::new(format!("Button {}", i)).with_variant(if i % 2 == 0 {
                            ButtonVariant::Filled
                        } else {
                            ButtonVariant::Outlined
                        }),
                        Node::default(),
                    )
                }));

                b.iter(|| {
                    let mut query_count = 0;
                    let world = app.world_mut();
                    let mut query = world.query_filtered::<&MaterialButton, With<Node>>();
                    for button in query.iter(world) {
                        black_box(button);
                        query_count += 1;
                    }
                    black_box(query_count)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark ripple effect updates
fn bench_ripple_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Ripple Updates");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("update_ripples", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities with ripples
                app.world_mut().spawn_batch((0..count).map(|_| {
                    (
                        Ripple::new(Vec2::ZERO, 100.0, Color::WHITE),
                        Node::default(),
                    )
                }));

                b.iter(|| {
                    // Simulate ripple update system
                    let world = app.world_mut();
                    let mut query = world.query::<&mut Ripple>();
                    for mut ripple in query.iter_mut(world) {
                        // Simulate animation progress
                        ripple.scale = (ripple.scale + 0.016).min(1.0);
                        if ripple.scale >= 1.0 && !ripple.fading_out {
                            ripple.start_fade_out();
                        }
                        black_box(&ripple);
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark focus ring updates
fn bench_focus_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("Focus Updates");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("update_focus", count),
            count,
            |b, &count| {
                let mut app = setup_app();
                // Spawn entities with focusable components
                app.world_mut()
                    .spawn_batch((0..count).map(|_| (Focusable::new(), Node::default())));

                b.iter(|| {
                    let world = app.world_mut();
                    let mut query = world.query::<&mut Focusable>();
                    for mut focusable in query.iter_mut(world) {
                        // Simulate focus toggle
                        focusable.focused = !focusable.focused;
                        focusable.focus_visible = focusable.focused;
                        black_box(&focusable);
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark elevation shadow calculations
fn bench_elevation_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Elevation Calculations");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("calculate_shadows", count),
            count,
            |b, &count| {
                let elevations: Vec<Elevation> = (0..count)
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
                        let shadow = ElevationShadow::from_elevation(*elevation);
                        black_box(shadow);
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark theme resource access
fn bench_theme_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("Theme Access");

    group.bench_function("get_theme_colors", |b| {
        let mut app = setup_app();
        app.insert_resource(MaterialTheme::default());

        b.iter(|| {
            let theme = app.world().resource::<MaterialTheme>();
            black_box((
                theme.primary,
                theme.secondary,
                theme.tertiary,
                theme.surface,
                theme.on_surface,
            ))
        })
    });

    group.finish();
}

/// Benchmark mixed component workload
fn bench_mixed_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mixed Workload");

    // Simulate a realistic UI with various components
    group.bench_function("realistic_ui_100_components", |b| {
        b.iter(|| {
            let mut app = setup_app();

            // Spawn a mix of components (typical form)
            // 20 buttons
            app.world_mut().spawn_batch((0..20).map(|i| {
                (
                    MaterialButton::new(format!("Button {}", i)),
                    Node::default(),
                )
            }));

            // 30 checkboxes
            app.world_mut().spawn_batch((0..30).map(|_| {
                (
                    MaterialCheckbox::new().with_state(CheckboxState::Unchecked),
                    Node::default(),
                )
            }));

            // 10 switches
            app.world_mut()
                .spawn_batch((0..10).map(|_| (MaterialSwitch::new(), Node::default())));

            // 20 sliders
            app.world_mut().spawn_batch((0..20).map(|i| {
                (
                    MaterialSlider::new(0.0, 100.0).with_value((i as f32 * 5.0) % 100.0),
                    Node::default(),
                )
            }));

            // 20 focusables
            app.world_mut()
                .spawn_batch((0..20).map(|_| (Focusable::new(), Node::default())));

            black_box(app.world().entities().len())
        })
    });

    // Simulate querying all interactive components
    group.bench_function("query_all_interactive", |b| {
        let mut app = setup_app();

        // Setup entities
        app.world_mut().spawn_batch((0..50).map(|i| {
            (
                MaterialButton::new(format!("Button {}", i)),
                Node::default(),
                Interaction::None,
            )
        }));

        app.world_mut().spawn_batch((0..50).map(|_| {
            (
                MaterialCheckbox::new().with_state(CheckboxState::Unchecked),
                Node::default(),
                Interaction::None,
            )
        }));

        b.iter(|| {
            let mut count = 0;
            let world = app.world_mut();

            // Query buttons
            let mut button_query = world.query::<(&MaterialButton, &Interaction)>();
            for (button, interaction) in button_query.iter(world) {
                black_box((button, interaction));
                count += 1;
            }

            // Query checkboxes
            let mut checkbox_query = world.query::<(&MaterialCheckbox, &Interaction)>();
            for (checkbox, interaction) in checkbox_query.iter(world) {
                black_box((checkbox, interaction));
                count += 1;
            }

            black_box(count)
        })
    });

    group.finish();
}

/// Benchmark scroll container operations
fn bench_scroll_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scroll Operations");

    // Benchmark scroll offset calculation
    group.bench_function("calculate_scroll_offset", |b| {
        let content_size: f32 = 2000.0;
        let visible_size: f32 = 400.0;
        let mut offset: f32 = 0.0;
        let delta: f32 = 20.0;

        b.iter(|| {
            // Simulate scroll update
            offset = (offset + delta).clamp(0.0, content_size - visible_size);
            black_box(offset)
        })
    });

    // Benchmark scrollbar thumb position calculation
    group.bench_function("calculate_thumb_position", |b| {
        let content_size: f32 = 2000.0;
        let visible_size: f32 = 400.0;
        let track_length: f32 = 380.0;
        let thumb_size = track_length * (visible_size / content_size);
        let offset: f32 = 100.0;

        b.iter(|| {
            let usable_track = track_length - thumb_size;
            let scroll_ratio = offset / (content_size - visible_size);
            let thumb_pos = scroll_ratio * usable_track;
            black_box(thumb_pos)
        })
    });

    // Benchmark normalized scroll (like Bevy's approach)
    group.bench_function("calculate_normalized_scroll", |b| {
        let content_size: f32 = 2000.0;
        let visible_size: f32 = 400.0;
        let track_length: f32 = 380.0;
        let offset: f32 = 100.0;

        b.iter(|| {
            let thumb_size = track_length * (visible_size / content_size);
            let usable_track = track_length - thumb_size;
            let max_scroll = content_size - visible_size;
            let thumb_pos = offset * usable_track / max_scroll;
            black_box((thumb_pos, thumb_size))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_entity_spawning,
    bench_component_queries,
    bench_ripple_updates,
    bench_focus_updates,
    bench_elevation_calculations,
    bench_theme_access,
    bench_mixed_workload,
    bench_scroll_operations,
);

criterion_main!(benches);
