//! Layout scaffolds showcase demonstrating navigation and pane patterns.

use bevy::prelude::*;
use bevy_material_ui::icons::icon_by_name;
use bevy_material_ui::layout::{self, PaneEntities};
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

pub fn spawn_layouts_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Handle<Font>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.layouts.title",
                "Layouts",
                "showcase.section.layouts.description",
                "Canonical Material 3 scaffolds: bottom navigation, rail, modal drawer, list-detail, and supporting panes.",
            );

            spawn_navigation_examples(section, theme.clone(), icon_font.clone());
            spawn_adaptive_examples(section, theme.clone(), icon_font.clone());
            spawn_modal_drawer_example(section, theme.clone());
            spawn_panes_examples(section, theme.clone());

            spawn_code_block(section, theme,
r#"// Navigation bar scaffold (Material 3)
let scaffold = NavigationBarScaffold::default();
spawn_navigation_bar_scaffold(parent, theme, &scaffold,
    |content| {
        // main content children here
    },
    |nav| {
        // bottom bar items here
    },
);

// List-detail scaffold
let scaffold = ListDetailScaffold::default();
spawn_list_detail_scaffold(parent, theme, &scaffold,
    |primary| {
        // list pane
    },
    |secondary| {
        // detail pane
    },
);

// Supporting panes scaffold (three-pane)
let scaffold = SupportingPanesScaffold::default();
spawn_supporting_panes_scaffold(parent, theme, &scaffold,
    |primary| { /* primary */ },
    |secondary| { /* secondary */ },
    |supporting| { /* supporting */ },
);

// Navigation suite scaffold (Material 3)
let size_class = WindowSizeClass::new(window_width_px, window_height_px);
let config = NavigationSuiteScaffold::default();
spawn_navigation_suite_scaffold(parent, theme, &size_class,
    |nav| { /* nav items */ },
    |content| { /* main content */ },
);
"#);
        });
}

fn spawn_navigation_examples(
    parent: &mut ChildSpawnerCommands,
    theme: MaterialTheme,
    _icon_font: Handle<Font>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Navigation scaffolds"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            // Row of nav scaffold previews
            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            })
            .with_children(|row| {
                spawn_bottom_nav_card(row, &theme, _icon_font.clone());
                spawn_nav_rail_card(row, &theme, _icon_font.clone());
            });
        });
}

fn spawn_adaptive_examples(
    parent: &mut ChildSpawnerCommands,
    theme: MaterialTheme,
    _icon_font: Handle<Font>,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Adaptive navigation (by window class)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            let phone = WindowSizeClass::new(480.0, 800.0); // Compact width -> bottom nav
            let tablet = WindowSizeClass::new(900.0, 900.0); // Medium/Expanded -> rail
            let desktop = WindowSizeClass::new(1400.0, 900.0); // Large -> drawer

            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            })
            .with_children(|row| {
                spawn_adaptive_card(
                    row,
                    &theme,
                    _icon_font.clone(),
                    "Phone (Compact)",
                    phone,
                    "layout_adaptive_phone",
                );
                spawn_adaptive_card(
                    row,
                    &theme,
                    _icon_font.clone(),
                    "Tablet (Rail)",
                    tablet,
                    "layout_adaptive_tablet",
                );
                spawn_adaptive_card(
                    row,
                    &theme,
                    _icon_font.clone(),
                    "Desktop (Drawer)",
                    desktop,
                    "layout_adaptive_desktop",
                );
            });
        });
}

fn spawn_adaptive_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
    label: &str,
    size_class: WindowSizeClass,
    test_prefix: &str,
) {
    let config = layout::NavigationSuiteScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(320.0),
            height: Val::Px(220.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|card| {
            card.spawn((
                Text::new(label),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            card.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            })
            .with_children(|root| {
                let _entities = layout::spawn_navigation_suite_scaffold(
                    root,
                    theme,
                    &size_class,
                    &config,
                    |nav| {
                        for (i, icon_name) in ["home", "search", "person"].iter().enumerate() {
                            nav.spawn((
                                TestId::new(format!("{}_nav_{}", test_prefix, i)),
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                Node {
                                    flex_grow: 1.0,
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|btn| {
                                if let Some(id) = icon_by_name(icon_name) {
                                    btn.spawn(
                                        bevy_material_ui::icons::MaterialIcon::new(id)
                                            .with_size(18.0)
                                            .with_color(theme.on_surface),
                                    );
                                }
                            });
                        }
                    },
                    |content| {
                        content
                            .spawn((
                                TestId::new(format!("{}_content", test_prefix)),
                                Node {
                                    flex_grow: 1.0,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|c| {
                                c.spawn((
                                    Text::new("Content"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                    },
                );
            });
        });
}

fn spawn_bottom_nav_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
) {
    let config = layout::NavigationBarScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(320.0),
            height: Val::Px(220.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|card| {
            // Use a mini scaffold to preview the structure
            card.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|root| {
                let _entities = layout::spawn_navigation_bar_scaffold(
                    root,
                    theme,
                    &config,
                    |content| {
                        content
                            .spawn((
                                TestId::new("layout_bottom_content"),
                                Node {
                                    flex_grow: 1.0,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|c| {
                                c.spawn((
                                    Text::new("Content"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                    },
                    |nav| {
                        for (i, icon_name) in ["home", "search", "person"].iter().enumerate() {
                            nav.spawn((
                                TestId::new(format!("layout_bottom_nav_{}", i)),
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                Node {
                                    flex_grow: 1.0,
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|btn| {
                                if let Some(id) = icon_by_name(icon_name) {
                                    btn.spawn(
                                        bevy_material_ui::icons::MaterialIcon::new(id)
                                            .with_size(18.0)
                                            .with_color(theme.on_surface),
                                    );
                                }
                            });
                        }
                    },
                );
            });
        });
}

fn spawn_nav_rail_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
) {
    let config = layout::NavigationRailScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(360.0),
            height: Val::Px(220.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|card| {
            card.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|root| {
                let _entities = layout::spawn_navigation_rail_scaffold(
                    root,
                    theme,
                    &config,
                    |nav| {
                        for (i, icon_name) in ["menu", "favorite", "more_vert"].iter().enumerate() {
                            nav.spawn((
                                TestId::new(format!("layout_rail_nav_{}", i)),
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(56.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|btn| {
                                if let Some(id) = icon_by_name(icon_name) {
                                    btn.spawn(
                                        bevy_material_ui::icons::MaterialIcon::new(id)
                                            .with_size(20.0)
                                            .with_color(theme.on_surface),
                                    );
                                }
                            });
                        }
                    },
                    |content| {
                        content
                            .spawn((
                                TestId::new("layout_rail_content"),
                                Node {
                                    flex_grow: 1.0,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|c| {
                                c.spawn((
                                    Text::new("Content"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                    },
                );
            });
        });
}

fn spawn_modal_drawer_example(parent: &mut ChildSpawnerCommands, theme: MaterialTheme) {
    let config = layout::ModalDrawerScaffold::default();
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Modal drawer scaffold"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            col.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(240.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            })
            .with_children(|root| {
                let _entities = layout::spawn_modal_drawer_scaffold(
                    root,
                    &theme,
                    &config,
                    |drawer| {
                        drawer
                            .spawn((
                                TestId::new("layout_drawer_list"),
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.0),
                                    ..default()
                                },
                            ))
                            .with_children(|list| {
                                for (i, label) in ["Inbox", "Starred", "Archive"].iter().enumerate()
                                {
                                    list.spawn((
                                        TestId::new(format!("layout_drawer_item_{}", i)),
                                        Button,
                                        Interaction::None,
                                        RippleHost::new(),
                                        Node {
                                            height: Val::Px(40.0),
                                            align_items: AlignItems::Center,
                                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                            ..default()
                                        },
                                        BackgroundColor(theme.surface_container_high),
                                        BorderRadius::all(Val::Px(8.0)),
                                    ))
                                    .with_children(|item| {
                                        item.spawn((
                                            Text::new((*label).to_string()),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(theme.on_surface),
                                        ));
                                    });
                                }
                            });
                    },
                    |content| {
                        content
                            .spawn((
                                TestId::new("layout_drawer_content"),
                                Node {
                                    flex_grow: 1.0,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                            ))
                            .with_children(|c| {
                                c.spawn((
                                    Text::new("Main content"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                    },
                );
            });
        });
}

fn spawn_panes_examples(parent: &mut ChildSpawnerCommands, theme: MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Pane scaffolds"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(12.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            })
            .with_children(|row| {
                spawn_list_detail_card(row, &theme);
                spawn_supporting_panes_card(row, &theme);
            });
        });
}

fn spawn_list_detail_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::ListDetailScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(360.0),
            height: Val::Px(220.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|card| {
            card.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                overflow: Overflow::clip(),
                ..default()
            })
            .with_children(|root| {
                let _entities: PaneEntities = layout::spawn_list_detail_scaffold(
                    root,
                    theme,
                    &config,
                    |primary| {
                        primary
                            .spawn((
                                TestId::new("layout_list_primary"),
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.0),
                                    padding: UiRect::all(Val::Px(12.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|list| {
                                for (i, label) in
                                    ["Email A", "Email B", "Email C"].iter().enumerate()
                                {
                                    list.spawn((
                                        TestId::new(format!("layout_list_item_{}", i)),
                                        Node {
                                            height: Val::Px(32.0),
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(theme.surface_container_high),
                                        BorderRadius::all(Val::Px(6.0)),
                                    ))
                                    .with_children(|item| {
                                        item.spawn((
                                            Text::new((*label).to_string()),
                                            TextFont {
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(theme.on_surface),
                                        ));
                                    });
                                }
                            });
                    },
                    |secondary| {
                        secondary
                            .spawn((
                                TestId::new("layout_list_detail"),
                                Node {
                                    flex_grow: 1.0,
                                    padding: UiRect::all(Val::Px(16.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|detail| {
                                detail.spawn((
                                    Text::new("Detail content"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                    },
                );
            });
        });
}

fn spawn_supporting_panes_card(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let config = layout::SupportingPanesScaffold::default();
    parent
        .spawn(Node {
            width: Val::Px(440.0),
            height: Val::Px(220.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|card| {
            card.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            })
            .with_children(|root| {
                let _entities: PaneEntities = layout::spawn_supporting_panes_scaffold(
                    root,
                    theme,
                    &config,
                    |primary| {
                        primary
                            .spawn((
                                TestId::new("layout_support_primary"),
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.0),
                                    padding: UiRect::all(Val::Px(12.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|list| {
                                for (i, label) in
                                    ["Thread A", "Thread B", "Thread C"].iter().enumerate()
                                {
                                    list.spawn((
                                        TestId::new(format!("layout_support_item_{}", i)),
                                        Node {
                                            height: Val::Px(32.0),
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(theme.surface_container_high),
                                        BorderRadius::all(Val::Px(6.0)),
                                    ))
                                    .with_children(|item| {
                                        item.spawn((
                                            Text::new((*label).to_string()),
                                            TextFont {
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(theme.on_surface),
                                        ));
                                    });
                                }
                            });
                    },
                    |secondary| {
                        secondary
                            .spawn((
                                TestId::new("layout_support_secondary"),
                                Node {
                                    flex_grow: 1.0,
                                    padding: UiRect::all(Val::Px(14.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|detail| {
                                detail.spawn((
                                    Text::new("Secondary content"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                ));
                            });
                    },
                    |supporting| {
                        supporting
                            .spawn((
                                TestId::new("layout_supporting"),
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::all(Val::Px(12.0)),
                                    row_gap: Val::Px(8.0),
                                    ..default()
                                },
                            ))
                            .with_children(|support| {
                                support.spawn((
                                    Text::new("Supporting actions"),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));

                                support
                                    .spawn((
                                        Button,
                                        Interaction::None,
                                        RippleHost::new(),
                                        TestId::new("layout_supporting_action"),
                                        Node {
                                            height: Val::Px(36.0),
                                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(1.0)),
                                            ..default()
                                        },
                                        BackgroundColor(theme.secondary_container),
                                        BorderColor::all(theme.secondary),
                                        BorderRadius::all(Val::Px(8.0)),
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new("Action"),
                                            TextFont {
                                                font_size: 13.0,
                                                ..default()
                                            },
                                            TextColor(theme.on_secondary_container),
                                        ));
                                    });
                            });
                    },
                );
            });
        });
}
