//! Menus view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_EXPAND_MORE, ICON_MORE_VERT};
use bevy_material_ui::list::{
    ListItemBody, ListItemBuilder, ListItemHeadline, ListItemSupportingText,
};
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the menus section content
pub fn spawn_menus_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
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
                "showcase.section.menus.title",
                "Menus",
                "showcase.section.menus.description",
                "Dropdown menus with selectable items",
            );
            // Menu trigger and dropdown container
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|container| {
                    // Menu trigger button
                    let trigger_button =
                        MaterialButton::new("Options").with_variant(ButtonVariant::Outlined);
                    let trigger_bg = trigger_button.background_color(theme);
                    let trigger_border = trigger_button.border_color(theme);

                    container
                        .spawn((
                            MenuTrigger,
                            trigger_button,
                            Button,
                            Interaction::None,
                            RippleHost::new(),
                            Node {
                                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(trigger_bg),
                            BorderColor::all(trigger_border),
                            BorderRadius::all(Val::Px(8.0)),
                        ))
                        .with_children(|btn| {
                            if let Some(icon) = MaterialIcon::from_name(ICON_MORE_VERT) {
                                btn.spawn(icon.with_size(20.0).with_color(theme.on_surface));
                            }
                            btn.spawn((
                                MenuSelectedText,
                                Text::new("Options"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                            ));
                            if let Some(icon) = MaterialIcon::from_name(ICON_EXPAND_MORE) {
                                btn.spawn(icon.with_size(20.0).with_color(theme.on_surface));
                            }
                        });

                    // Menu dropdown (hidden by default)
                    container
                        .spawn((
                            MenuDropdown,
                            Visibility::Hidden,
                            Node {
                                width: Val::Px(200.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::vertical(Val::Px(8.0)),
                                margin: UiRect::top(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface_container),
                            BorderRadius::all(Val::Px(4.0)),
                            BoxShadow::from(ShadowStyle {
                                color: Color::BLACK.with_alpha(0.2),
                                x_offset: Val::Px(0.0),
                                y_offset: Val::Px(4.0),
                                spread_radius: Val::Px(0.0),
                                blur_radius: Val::Px(8.0),
                            }),
                        ))
                        .with_children(|menu| {
                            spawn_menu_item(menu, theme, "Cut", "Ctrl+X", false);
                            spawn_menu_item(menu, theme, "Copy", "Ctrl+C", false);
                            spawn_menu_item(menu, theme, "Paste", "Ctrl+V", false);
                            // Divider
                            menu.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(1.0),
                                    margin: UiRect::vertical(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(theme.outline_variant),
                            ));
                            spawn_menu_item(menu, theme, "Delete", "", true);
                        });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create a menu
let menu = MaterialMenu::new()
    .anchor(MenuAnchor::BottomLeft)
    .open();

commands.spawn((
    menu,
    Node { width: Val::Px(200.0), ..default() },
    BackgroundColor(theme.surface_container),
));

// Add menu items
let item = MenuItem::new("Copy")
    .shortcut("Ctrl+C");"#,
            );
        });
}

fn spawn_menu_item(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    shortcut: &str,
    is_destructive: bool,
) {
    let headline_color = if is_destructive {
        theme.error
    } else {
        theme.on_surface
    };
    let supporting_color = theme.on_surface_variant;
    let has_supporting = !shortcut.is_empty();

    let builder = if has_supporting {
        ListItemBuilder::new(label).two_line()
    } else {
        ListItemBuilder::new(label)
    };

    parent
        .spawn((
            MenuItemMarker(label.to_string()),
            Interaction::None,
            builder.build(theme),
        ))
        .with_children(|item| {
            item.spawn((
                ListItemBody,
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
            ))
            .with_children(|body| {
                body.spawn((
                    ListItemHeadline,
                    Text::new(label),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(headline_color),
                ));

                if has_supporting {
                    body.spawn((
                        ListItemSupportingText,
                        Text::new(shortcut),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(supporting_color),
                    ));
                }
            });
        });
}
