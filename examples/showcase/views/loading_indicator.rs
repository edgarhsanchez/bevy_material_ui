//! Loading indicator view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::{
    loading_indicator::{LoadingIndicatorBuilder, ShapeMorphMaterial, SpawnLoadingIndicatorChild},
    prelude::*,
};

use crate::showcase::common::*;

/// Spawn the loading indicator section content
pub fn spawn_loading_indicator_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    materials: &mut Assets<ShapeMorphMaterial>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.loading_indicator.title",
                "Loading Indicator",
                "showcase.section.loading_indicator.description",
                "Material Design 3 loading indicators with morphing shapes",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(420.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Default
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.loading.default")
                            .with_default("Loading indicator"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        NeedsInternationalFont,
                    ));
                    col.spawn_loading_indicator(theme, materials);

                    // Contained (with container background)
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.loading.with_container")
                            .with_default("Loading indicator with container"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        NeedsInternationalFont,
                    ));
                    col.spawn_loading_indicator_with(
                        theme,
                        materials,
                        LoadingIndicatorBuilder::new().contained(),
                    );

                    // Multiple colors
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.loading.multi_color")
                            .with_default("Loading indicator with multiple colors"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        NeedsInternationalFont,
                    ));
                    col.spawn_loading_indicator_with(
                        theme,
                        materials,
                        LoadingIndicatorBuilder::new().multi_color(),
                    );

                    // Small size
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.loading.small")
                            .with_default("Small loading indicator"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        NeedsInternationalFont,
                    ));
                    col.spawn_loading_indicator_with(
                        theme,
                        materials,
                        LoadingIndicatorBuilder::new().size(36.0),
                    );

                    // Large and fast
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.loading.large_fast")
                            .with_default("Large loading indicator (fast)"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        NeedsInternationalFont,
                    ));
                    col.spawn_loading_indicator_with(
                        theme,
                        materials,
                        LoadingIndicatorBuilder::new().size(64.0).speed(2.0),
                    );
                });

            spawn_code_block(
                section,
                theme,
                r#"// Default loading indicator with shape morphing
parent.spawn_loading_indicator(&theme, &mut materials);

// Contained (with container background)
parent.spawn_loading_indicator_with(
    &theme,
    &mut materials,
    LoadingIndicatorBuilder::new().contained(),
);

// Multiple colors
parent.spawn_loading_indicator_with(
    &theme,
    &mut materials,
    LoadingIndicatorBuilder::new().multi_color(),
);

// Custom size
parent.spawn_loading_indicator_with(
    &theme,
    &mut materials,
    LoadingIndicatorBuilder::new().size(36.0),
);

// Large and fast
parent.spawn_loading_indicator_with(
    &theme,
    &mut materials,
    LoadingIndicatorBuilder::new().size(64.0).speed(2.0),
);"#,
            );
        });
}
