//! Tabs view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;
use crate::showcase::{ComponentSection, TabStateCache};

/// Spawn the tabs section content.
///
/// This demonstrates the library tabs system:
/// - `MaterialTabs` owns the selected index
/// - `MaterialTab` buttons update selection
/// - `TabContent` panels are shown/hidden by the library based on selection
pub fn spawn_tabs_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    tab_cache: &TabStateCache,
) {
    // Restore cached tab selection, default to 0
    let selected_tab = tab_cache
        .selections
        .get(&ComponentSection::Tabs)
        .copied()
        .unwrap_or(0);
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
                "showcase.section.tabs.title",
                "Tabs",
                "showcase.section.tabs.description",
                "Primary tabs with content panels driven by TabContent",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    width: Val::Percent(100.0),
                    max_width: Val::Px(520.0),
                    ..default()
                })
                .with_children(|col| {
                    // Tabs header bar
                    let mut tabs_bar_ec = col.spawn((
                        TestId::new("tabs_primary"),
                        MaterialTabs::new()
                            .with_variant(TabVariant::Primary)
                            .selected(selected_tab),
                        Node {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            border: UiRect::bottom(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface),
                        BorderColor::all(theme.surface_container_highest),
                    ));

                    let tabs_entity = tabs_bar_ec.id();
                    tabs_bar_ec.with_children(|tabs| {
                        spawn_tab_button(tabs, theme, 0, "Home", true);
                        spawn_tab_button(tabs, theme, 1, "Profile", false);
                        spawn_tab_button(tabs, theme, 2, "Settings", false);
                    });

                    // Content panels (visibility is controlled by the library)
                    col.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            min_height: Val::Px(140.0),
                            padding: UiRect::all(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_low),
                        BorderRadius::bottom(Val::Px(12.0)),
                    ))
                    .with_children(|content| {
                        spawn_tab_panel(
                            content,
                            theme,
                            tabs_entity,
                            0,
                            "Home",
                            "Overview content shown when the first tab is selected.",
                        );
                        spawn_tab_panel(
                            content,
                            theme,
                            tabs_entity,
                            1,
                            "Profile",
                            "User profile information and settings.",
                        );
                        spawn_tab_panel(
                            content,
                            theme,
                            tabs_entity,
                            2,
                            "Settings",
                            "Application configuration and preferences.",
                        );
                    });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create tabs with content panels
let tabs_entity = commands
    .spawn((MaterialTabs::new().selected(0), Node::default()))
    .id();

// Tab buttons
commands.entity(tabs_entity).with_children(|parent| {
    parent.spawn((MaterialTab::new(0).selected(true), Button, Node::default()));
    parent.spawn((MaterialTab::new(1), Button, Node::default()));
});

// Content panels: the library shows/hides these via TabContent
commands.spawn((TabContent::new(0, tabs_entity), Visibility::Inherited, Node::default()));
commands.spawn((TabContent::new(1, tabs_entity), Visibility::Hidden, Node::default()));"#,
            );
        });
}

fn spawn_tab_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    index: usize,
    label: &str,
    selected: bool,
) {
    let test_id = format!("tab_{}", index + 1);
    let tab = MaterialTab::new(index, label).selected(selected);

    parent
        .spawn((
            tab,
            TestId::new(test_id),
            Button,
            Interaction::None,
            Node {
                flex_grow: 1.0,
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|tab| {
            tab.spawn((
                TabLabelText,
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(if selected {
                    theme.primary
                } else {
                    theme.on_surface_variant
                }),
            ));
        });
}

fn spawn_tab_panel(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    tabs_entity: Entity,
    index: usize,
    title: &str,
    description: &str,
) {
    let visibility = if index == 0 {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    parent
        .spawn((
            TabContent::new(index, tabs_entity),
            visibility,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                width: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new(title),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
            panel.spawn((
                Text::new(description),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}
