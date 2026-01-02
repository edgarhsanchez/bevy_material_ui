//! App Bar Demo
//!
//! Demonstrates Material Design 3 top and bottom app bars.

use bevy::prelude::*;
use bevy_material_ui::app_bar::spawn_top_app_bar_with_right_content;
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

    // Root container
    let root_id = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("app_bar_demo/root", &telemetry)
        .id();

    // Top app bar
    let top = spawn_top_app_bar_with_right_content(
        &mut commands,
        &theme,
        TopAppBarBuilder::new("Page Title")
            .small()
            .with_navigation("menu")
            .add_action("search", "search")
            .add_action("more_vert", "more"),
        |_right| {},
    );

    commands
        .entity(top)
        .insert_test_id("app_bar_demo/top", &telemetry);
    commands.entity(root_id).add_child(top);

    // Spacer
    let spacer = commands
        .spawn(Node {
            height: Val::Px(16.0),
            ..default()
        })
        .id();
    commands.entity(root_id).add_child(spacer);

    // Main body content
    let body = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("app_bar_demo/body", &telemetry)
        .id();

    let body_text = commands
        .spawn((
            Text::new("Top + Bottom App Bars"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(theme.on_surface),
        ))
        .id();

    commands.entity(body).add_child(body_text);
    commands.entity(root_id).add_child(body);

    // Bottom app bar (absolute positioned at bottom)
    let bottom = commands
        .spawn(
            BottomAppBarBuilder::new()
                .add_action("home", "home")
                .add_action("favorite", "favorite")
                .with_fab("add")
                .elevated()
                .build(&theme),
        )
        .insert_test_id("app_bar_demo/bottom", &telemetry)
        .id();

    commands.entity(root_id).add_child(bottom);
}
