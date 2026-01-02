//! Search bar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_ARROW_BACK, ICON_MENU};
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the search bar section content
pub fn spawn_search_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.search.title",
                "Search",
                "showcase.section.search.description",
                "Search bar for navigation and search functionality",
            );

            // Example search bars
            section
                .spawn(Node {
                    width: Val::Percent(100.0),
                    max_width: Val::Px(560.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Default search bar
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.search.default")
                            .with_default("Default search bar"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        NeedsInternationalFont,
                    ));
                    col.spawn_search_bar(theme, "Search...");

                    // Search bar with navigation
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.search.with_navigation")
                            .with_default("With navigation icon"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },
                        NeedsInternationalFont,
                    ));
                    col.spawn_search_bar_with(
                        theme,
                        SearchBarBuilder::new("Search...").with_navigation(
                            MaterialIcon::from_name(ICON_MENU).expect("menu icon should exist"),
                        ),
                    );

                    // Search bar with text
                    col.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.search.with_text")
                            .with_default("With search text"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                        Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },
                    ));
                    col.spawn_search_bar_with(
                        theme,
                        SearchBarBuilder::new("Search...")
                            .with_navigation(
                                MaterialIcon::from_name(ICON_ARROW_BACK)
                                    .expect("arrow_back icon should exist"),
                            )
                            .with_text("material design"),
                    );
                });

            spawn_code_block(
                section,
                theme,
                r#"// Default search bar
ui.spawn_search_bar(&theme, "Search...");

// With navigation icon
ui.spawn_search_bar_with(
    &theme,
    SearchBarBuilder::new("Search...")
            .with_navigation(MaterialIcon::from_name(ICON_MENU).unwrap()),
);

// With search text
ui.spawn_search_bar_with(
    &theme,
    SearchBarBuilder::new("Search...")
        .with_text("material design"),
);"#,
            );
        });
}
