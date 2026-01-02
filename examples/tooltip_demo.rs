//! Tooltip Demo
//!
//! Demonstrates attaching tooltips via `TooltipTrigger` and the `SpawnTooltipChild` helpers.

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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("tooltip_demo/root", &telemetry)
        .with_children(|root| {
            let label = "Hover Me";
            let btn = MaterialButton::new(label).with_variant(ButtonVariant::Filled);
            let label_color = btn.text_color(&theme);

            root.spawn((
                Interaction::None,
                TooltipTrigger::new("Hover to see tooltip!").with_position(TooltipPosition::Bottom),
                MaterialButtonBuilder::new(label).filled().build(&theme),
            ))
            .insert_test_id("tooltip_demo/button", &telemetry)
            .with_children(|b| {
                b.spawn((
                    ButtonLabel,
                    Text::new(label),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(label_color),
                ));
            });

            // Spawn a simple icon-like node with a tooltip attached via helper.
            let help_box = root
                .spawn((
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container),
                    BorderRadius::all(Val::Px(8.0)),
                    TooltipTrigger::new("Help tooltip"),
                ))
                .id();

            root.commands().entity(help_box).with_children(|n| {
                n.spawn((
                    Text::new("?"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));
            });
        });
}
