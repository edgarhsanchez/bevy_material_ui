//! Scroll Demo
//!
//! Demonstrates Material UI scroll containers and scrollbar orientations.
//!
//! ## Simple Scroll Pattern
//!
//! Creating a scrollable container requires three components:
//!
//! 1. **ScrollContainerBuilder** - Configure scroll direction and behavior
//!    - `.vertical()`, `.horizontal()`, or `.both()`
//!    - `.with_scrollbars(true)` - Scrollbars spawn automatically (default: true)
//!    - `.sensitivity(40.0)` - Mouse wheel scroll speed
//!
//! 2. **ScrollPosition::default()** - Tracks current scroll offset
//!
//! 3. **Node with overflow** - Defines the scrollable area
//!    - `overflow: Overflow::scroll()` - **Both axes must be Scroll for Bevy's scroll system**
//!    - The `ScrollContainer.direction` field controls which direction actually scrolls
//!    - For vertical-only: use `.vertical()` + `Overflow::scroll()`
//!    - For horizontal-only: use `.horizontal()` + `Overflow::scroll()`
//!    - Both overflow and size (width/height) are required
//!
//! The ScrollPlugin automatically:
//! - Creates an internal ScrollContent wrapper
//! - Spawns scrollbars (when show_scrollbars=true)
//! - Syncs scroll position
//! - Handles mouse wheel and scrollbar dragging
//!
//! No manual scrollbar spawning needed!

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("scroll_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Scroll Containers"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            // Vertical scroller
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|section| {
                section.spawn((
                    Text::new("Vertical scrollbar"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                section
                    .spawn((
                        // 1. Configure scroll direction and behavior
                        // Scrollbars spawn automatically (default: with_scrollbars=true)
                        ScrollContainerBuilder::new().vertical().build(),
                        // 2. Track scroll position
                        ScrollPosition::default(),
                        // 3. Define scrollable area with overflow and size
                        Node {
                            width: Val::Px(420.0),
                            height: Val::Px(160.0),
                            overflow: Overflow::scroll(), // Both axes must be Scroll
                            padding: UiRect::all(Val::Px(12.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_low),
                        BorderRadius::all(Val::Px(12.0)),
                    ))
                    .insert_test_id("scroll_demo/scroll/vertical", &telemetry)
                    .with_children(|content| {
                        for i in 1..=30 {
                            content.spawn((
                                Text::new(format!("Item {i}")),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        }
                    });
            });

            // Horizontal scroller
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|section| {
                section.spawn((
                    Text::new("Horizontal scrollbar"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                section
                    .spawn((
                        ScrollContainerBuilder::new().horizontal().build(),
                        ScrollPosition::default(),
                        Node {
                            width: Val::Px(420.0),
                            height: Val::Px(120.0),
                            // Both axes must be Scroll for Bevy's scroll system
                            // ScrollContainer.direction controls which direction actually scrolls
                            overflow: Overflow::scroll(),
                            padding: UiRect::all(Val::Px(12.0)),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(12.0),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_low),
                        BorderRadius::all(Val::Px(12.0)),
                    ))
                    .insert_test_id("scroll_demo/scroll/horizontal", &telemetry)
                    .with_children(|content| {
                        for i in 1..=20 {
                            content.spawn((
                                Node {
                                    width: Val::Px(96.0),
                                    height: Val::Px(72.0),
                                    ..default()
                                },
                                BackgroundColor(if i % 2 == 0 {
                                    theme.secondary_container
                                } else {
                                    theme.primary_container
                                }),
                                BorderRadius::all(Val::Px(12.0)),
                            ));
                        }
                    });
            });
        });
}
