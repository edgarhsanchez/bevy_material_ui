//! Tabs Demo
//!
//! Demonstrates Material Design 3 tabs and content panels.

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
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .with_children(|root| {
            let tabs_entity = root
                .spawn(TabsBuilder::new().primary().selected(0).build(&theme))
                .insert_test_id("tabs_demo/tabs", &telemetry)
                .with_children(|tabs| {
                    for (index, label) in ["Home", "Explore", "Settings"].into_iter().enumerate() {
                        let selected = index == 0;
                        let builder = TabBuilder::new(index, label)
                            .variant(TabVariant::Primary)
                            .selected(selected);

                        let tab_component = MaterialTab::new(index, label)
                            .selected(selected)
                            .disabled(false);
                        let content_color =
                            tab_component.content_color(&theme, TabVariant::Primary);

                        tabs.spawn(builder.build(&theme)).with_children(|tab| {
                            tab.spawn((
                                TabLabelText,
                                Text::new(label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(content_color),
                            ));

                            if selected {
                                tab.spawn(create_tab_indicator(&theme, TabVariant::Primary));
                            }
                        });
                    }
                })
                .id();

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
            ))
            .with_children(|content| {
                for (index, label) in ["Home", "Explore", "Settings"].into_iter().enumerate() {
                    content
                        .spawn((
                            TabContent::new(index, tabs_entity),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|panel| {
                            panel.spawn((
                                Text::new(format!("{label} content")),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                        });
                }
            });
        });
}
