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
    _icon_font: Handle<Font>,
) {
    let theme_clone = theme.clone();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            align_items: AlignItems::Stretch,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                &theme_clone,
                "showcase.section.lists.title",
                "Lists (with Selection)",
                "showcase.section.lists.description",
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
                        Text::new(""),
                        LocalizedText::new("showcase.lists.selection_mode")
                            .with_default("Selection Mode:"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme_clone.on_surface),
                        Node {
                            margin: UiRect::right(Val::Px(8.0)),
                            ..default()
                        },
                        NeedsInternationalFont,
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
            // Container for list with scrollbar
            section
                .spawn(Node {
                    width: Val::Px(420.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
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
                                .selection_mode(ListSelectionMode::Single)
                                .build_scrollable(),
                            BackgroundColor(theme_clone.surface),
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
                                list.spawn((
                                    SelectableListItem,
                                    TestId::new(format!("list_item_{}", i)),
                                    ListItemBuilder::new(*headline)
                                        .two_line()
                                        .supporting_text(*supporting)
                                        .build(&theme_clone),
                                    Interaction::None,
                                ))
                                .with_children(|item| {
                                    // Leading (match library default list item layout)
                                    item.spawn((
                                        bevy_material_ui::list::ListItemLeading,
                                        Node {
                                            width: Val::Px(56.0),
                                            height: Val::Px(56.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                    ))
                                    .with_children(
                                        |leading| {
                                            if let Some(icon) = MaterialIcon::from_name(ICON_EMAIL)
                                            {
                                                leading
                                                    .spawn(icon.with_size(24.0).with_color(
                                                        theme_clone.on_surface_variant,
                                                    ));
                                            }
                                        },
                                    );

                                    // Body (match library markers so styling/selection systems can work)
                                    item.spawn((
                                        bevy_material_ui::list::ListItemBody,
                                        Node {
                                            flex_direction: FlexDirection::Column,
                                            flex_grow: 1.0,
                                            ..default()
                                        },
                                    ))
                                    .with_children(|body| {
                                        body.spawn((
                                            bevy_material_ui::list::ListItemHeadline,
                                            Text::new(*headline),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(theme_clone.on_surface),
                                        ));
                                        body.spawn((
                                            bevy_material_ui::list::ListItemSupportingText,
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
                            // Note: Scrollbars spawn automatically via ScrollPlugin's ensure_scrollbars_system
                            // because ScrollContainerBuilder defaults to show_scrollbars=true.
                            // No manual spawn_scrollbars() call needed!
                        })
                        .id();

                    // Keep the entity id around for future selection/scroll interactions.
                    let _ = scroll_area_id;
                });

            // Explicit scrollbar orientation demos (vertical/horizontal/both)
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    margin: UiRect::top(Val::Px(16.0)),
                    ..default()
                })
                .with_children(|demo| {
                    demo.spawn((
                        Text::new("Scrollbar orientations"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme_clone.on_surface),
                    ));

                    demo.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    })
                    .with_children(|row| {
                        // Horizontal scrollbar
                        row.spawn((
                            TestId::new("scroll_demo_horizontal"),
                            ScrollContainerBuilder::new().horizontal().build(),
                            ScrollPosition::default(),
                            Node {
                                width: Val::Px(400.0),
                                height: Val::Px(120.0),
                                // Both axes must be Scroll (direction controlled by ScrollContainer)
                                overflow: Overflow::scroll(),
                                padding: UiRect::all(Val::Px(12.0)),
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(12.0),
                                ..default()
                            },
                            BackgroundColor(theme_clone.surface_container_low),
                            BorderRadius::all(Val::Px(12.0)),
                            Interaction::None,
                        ))
                        .with_children(|scroller| {
                            for i in 1..=18 {
                                scroller.spawn((
                                    Node {
                                        width: Val::Px(84.0),
                                        height: Val::Px(72.0),
                                        ..default()
                                    },
                                    BackgroundColor(if i % 2 == 0 {
                                        theme_clone.secondary_container
                                    } else {
                                        theme_clone.primary_container
                                    }),
                                    BorderRadius::all(Val::Px(12.0)),
                                ));
                            }
                            // Scrollbars spawn automatically (show_scrollbars=true by default)
                        });

                        // Both directions
                        row.spawn((
                            TestId::new("scroll_demo_both"),
                            ScrollContainerBuilder::new().both().build(),
                            ScrollPosition::default(),
                            Node {
                                width: Val::Px(400.0),
                                height: Val::Px(180.0),
                                // Both axes must be Scroll
                                overflow: Overflow::scroll(),
                                padding: UiRect::all(Val::Px(12.0)),
                                ..default()
                            },
                            BackgroundColor(theme_clone.surface_container_low),
                            BorderRadius::all(Val::Px(12.0)),
                            Interaction::None,
                        ))
                        .with_children(|scroller| {
                            scroller
                                .spawn(Node {
                                    width: Val::Px(760.0),
                                    height: Val::Px(380.0),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    row_gap: Val::Px(12.0),
                                    column_gap: Val::Px(12.0),
                                    ..default()
                                })
                                .with_children(|content| {
                                    for i in 1..=30 {
                                        content.spawn((
                                            Node {
                                                width: Val::Px(120.0),
                                                height: Val::Px(72.0),
                                                ..default()
                                            },
                                            BackgroundColor(if i % 3 == 0 {
                                                theme_clone.tertiary_container
                                            } else if i % 2 == 0 {
                                                theme_clone.secondary_container
                                            } else {
                                                theme_clone.primary_container
                                            }),
                                            BorderRadius::all(Val::Px(12.0)),
                                        ));
                                    }
                                });
                            // Scrollbars spawn automatically
                        });
                    });
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
                Text::new(""),
                LocalizedText::new(match label {
                    "Single" => "showcase.lists.mode_single",
                    "Multi" => "showcase.lists.mode_multi",
                    _ => label,
                })
                .with_default(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(label_color),
                NeedsInternationalFont,
            ));
        });
}
