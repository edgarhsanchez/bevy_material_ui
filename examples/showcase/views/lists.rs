//! Lists view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::chip::{ChipBuilder, ChipLabel};
use bevy_material_ui::icons::ICON_EMAIL;
use bevy_material_ui::list::ListItemBuilder;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the list section content
pub fn spawn_list_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Handle<Font>,
) {
    let theme_clone = theme.clone();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                &theme_clone,
                "Lists (with Selection)",
                "Scrollable list with single or multi-select - click items to select",
            );

            // Selection mode options
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Selection Mode:"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme_clone.on_surface),
                        Node {
                            margin: UiRect::right(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                    spawn_list_mode_option(
                        row,
                        &theme_clone,
                        "Single",
                        ListSelectionMode::Single,
                        true,
                    );
                    spawn_list_mode_option(
                        row,
                        &theme_clone,
                        "Multi",
                        ListSelectionMode::Multi,
                        false,
                    );
                });

            let icon_font_clone = icon_font.clone();
            // Container for list with scrollbar
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start, // Track aligns to top of list
                    width: Val::Percent(100.0),
                    max_width: Val::Px(400.0),
                    ..default()
                })
                .with_children(|container| {
                    // Scrollable list using the new API
                    let scroll_area_id = container
                        .spawn((
                            ListDemoRoot,
                            TestId::new("list_scroll_area"),
                            bevy_material_ui::list::ListBuilder::new()
                                .max_visible_items_variant(
                                    4,
                                    bevy_material_ui::list::ListItemVariant::TwoLine,
                                )
                                .build_scrollable(),
                            BackgroundColor(theme_clone.surface_container_low),
                            BorderRadius::all(Val::Px(12.0)),
                            Interaction::None, // Enable hover detection
                        ))
                        .with_children(|list| {
                            // 10 list items
                            let items = [
                                ("Inbox", "Primary inbox for emails"),
                                ("Starred", "Important messages"),
                                ("Sent", "Outgoing messages"),
                                ("Drafts", "Unfinished messages"),
                                ("Spam", "Filtered junk mail"),
                                ("Trash", "Deleted items"),
                                ("Archive", "Stored messages"),
                                ("Labels", "Organized categories"),
                                ("Settings", "Configuration options"),
                                ("Help", "Support and documentation"),
                            ];

                            for (i, (headline, supporting)) in items.iter().enumerate() {
                                let icon_for_item = icon_font_clone.clone();
                                list.spawn((
                                    SelectableListItem,
                                    TestId::new(format!("list_item_{}", i)),
                                    ListItemBuilder::new(*headline)
                                        .two_line()
                                        .supporting_text(*supporting)
                                        .build(&theme_clone),
                                ))
                                .with_children(|item| {
                                    // Leading icon with proper font
                                    item.spawn((
                                        Text::new(ICON_EMAIL.to_string()),
                                        TextFont {
                                            font: icon_for_item,
                                            font_size: 24.0,
                                            ..default()
                                        },
                                        TextColor(theme_clone.on_surface_variant),
                                        Node {
                                            margin: UiRect::right(Val::Px(16.0)),
                                            ..default()
                                        },
                                    ));

                                    // Body with text
                                    item.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        flex_grow: 1.0,
                                        ..default()
                                    })
                                    .with_children(|body| {
                                        body.spawn((
                                            Text::new(*headline),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(theme_clone.on_surface),
                                        ));
                                        body.spawn((
                                            Text::new(*supporting),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(theme_clone.on_surface_variant),
                                        ));
                                    });
                                });
                            }

                            // Framework scrollbars (track + thumb) driven by ScrollPlugin
                            spawn_scrollbars(list, &theme_clone, ScrollDirection::Vertical);
                        })
                        .id();

                    // Keep the entity id around for future selection/scroll interactions.
                    let _ = scroll_area_id;
                });

            spawn_code_block(
                section,
                &theme_clone,
                r#"// Scrollable list with selection modes
// Single select clears previous selection
// Multi select allows multiple items to be selected
commands.spawn((
    ListBuilder::new()
        .max_visible_items_variant(4, ListItemVariant::TwoLine)
        .selection_mode(ListSelectionMode::Multi)  // or Single
        .build_scrollable(),
    BackgroundColor(theme.surface_container_low),
)).with_children(|list| {
    for (headline, supporting) in items {
        list.spawn((
            SelectableListItem,
            ListItemBuilder::new(headline)
                .two_line()
                .supporting_text(supporting)
                .build(&theme)
        ));
    }
});"#,
            );
        });
}

fn spawn_list_mode_option(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    mode: ListSelectionMode,
    is_selected: bool,
) {
    let chip_for_color = MaterialChip::filter(label).with_selected(is_selected);
    let label_color = chip_for_color.label_color(theme);

    parent
        .spawn((
            ListSelectionModeOption(mode),
            Interaction::None,
            ChipBuilder::filter(label)
                .selected(is_selected)
                .build(theme),
        ))
        .with_children(|chip| {
            chip.spawn((
                ChipLabel,
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(label_color),
            ));
        });
}
