//! Switch Demo
//!
//! Demonstrates Material Design 3 switches with icons and without.

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

    let root_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("switch_demo/root", &telemetry)
        .id();

    commands.entity(root_entity).with_children(|root| {
        root.spawn((
            Text::new("Switches"),
            TextFont {
                font_size: 28.0,
                ..default()
            },
            TextColor(theme.on_surface),
        ));
    });

    let switch_default =
        commands.spawn_switch_with(&theme, SwitchBuilder::new().selected(false), "Default");
    commands
        .entity(switch_default)
        .insert_test_id("switch_demo/switch/default", &telemetry);
    commands.entity(root_entity).add_child(switch_default);

    let switch_with_icon = commands.spawn_switch_with(
        &theme,
        SwitchBuilder::new().selected(true).with_icon(),
        "With icon",
    );
    commands
        .entity(switch_with_icon)
        .insert_test_id("switch_demo/switch/with_icon", &telemetry);
    commands.entity(root_entity).add_child(switch_with_icon);
}
