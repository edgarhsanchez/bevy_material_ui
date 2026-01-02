//! Search Demo
//!
//! Demonstrates the Material search bar.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_ARROW_BACK, ICON_MENU};
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (log_search_events_system,))
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("search_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Text::new("Search Bars"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            root.spawn(Node {
                width: Val::Px(560.0),
                ..default()
            })
            .with_children(|col| {
                col.spawn_search_bar(&theme, "Search...");

                col.spawn_search_bar_with(
                    &theme,
                    SearchBarBuilder::new("Search...").with_navigation(
                        MaterialIcon::from_name(ICON_MENU).expect("menu icon should exist"),
                    ),
                );

                col.spawn_search_bar_with(
                    &theme,
                    SearchBarBuilder::new("Search...")
                        .with_navigation(
                            MaterialIcon::from_name(ICON_ARROW_BACK)
                                .expect("arrow_back icon should exist"),
                        )
                        .with_text("material design"),
                );
            });
        });
}

fn log_search_events_system(
    mut clicks: MessageReader<SearchBarClickEvent>,
    mut queries: MessageReader<SearchQueryEvent>,
) {
    for ev in clicks.read() {
        info!("Search bar clicked: {:?}", ev.search_bar);
    }

    for ev in queries.read() {
        info!("Search query: '{}'", ev.query);
    }
}
