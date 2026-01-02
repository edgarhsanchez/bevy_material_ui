use bevy::prelude::*;
use bevy_material_ui::{
    loading_indicator::{LoadingIndicatorBuilder, ShapeMorphMaterial, SpawnLoadingIndicatorChild},
    telemetry::TelemetryPlugin,
    theme::MaterialTheme,
    MaterialUiPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialUiPlugin, TelemetryPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(40.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Material Design 3 Loading Indicators"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            // Container for loading indicators
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(60.0),
                    ..default()
                })
                .with_children(|parent| {
                    // Standard loading indicator
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(16.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Standard"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));
                            parent.spawn_loading_indicator(&theme, &mut materials);
                        });

                    // Multi-color loading indicator
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(16.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Multi-Color"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));
                            parent.spawn_loading_indicator_with(
                                &theme,
                                &mut materials,
                                LoadingIndicatorBuilder::new().multi_color(),
                            );
                        });

                    // Larger size
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(16.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Large (72px)"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));
                            parent.spawn_loading_indicator_with(
                                &theme,
                                &mut materials,
                                LoadingIndicatorBuilder::new().size(72.0),
                            );
                        });

                    // Faster animation
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(16.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Fast (2x speed)"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));
                            parent.spawn_loading_indicator_with(
                                &theme,
                                &mut materials,
                                LoadingIndicatorBuilder::new().speed(2.0),
                            );
                        });
                });
        });
}
