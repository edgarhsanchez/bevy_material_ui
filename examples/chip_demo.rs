//! Chip Demo
//!
//! Demonstrates assist, filter, input, and suggestion chips.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (log_chip_events_system,))
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
        .insert_test_id("chip_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                ..default()
            },))
                .with_children(|row| {
                    row.spawn_chip_with(&theme, ChipBuilder::assist("Assist"));

                    row.spawn_chip_with(&theme, ChipBuilder::filter("Filter").selected(true));

                    row.spawn_chip_with(&theme, ChipBuilder::input("Input").deletable(true));

                    row.spawn_chip_with(&theme, ChipBuilder::suggestion("Suggestion"));
                });
        });
}

fn log_chip_events_system(
    mut clicks: MessageReader<ChipClickEvent>,
    mut deletes: MessageReader<ChipDeleteEvent>,
) {
    for ev in clicks.read() {
        info!("Chip clicked: {}", ev.value.as_deref().unwrap_or("(none)"));
    }

    for ev in deletes.read() {
        info!(
            "Chip delete clicked: {}",
            ev.value.as_deref().unwrap_or("(none)")
        );
    }
}
