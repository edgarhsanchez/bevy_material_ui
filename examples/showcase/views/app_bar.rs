//! App Bar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::app_bar::SpawnTopAppBarWithRightContentChild;
use bevy_material_ui::icons::{ICON_ADD, ICON_CHECK, ICON_CLOSE, ICON_MENU, ICON_SEARCH};
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::{spawn_text_field_control, InputType};

use crate::showcase::common::*;

fn spawn_standard_icon_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_name: &str,
) {
    let icon_btn =
        MaterialIconButton::new(icon_name.to_string()).with_variant(IconButtonVariant::Standard);
    let icon_color = icon_btn.icon_color(theme);

    parent
        .spawn(
            IconButtonBuilder::new(icon_name.to_string())
                .standard()
                .build(theme),
        )
        .with_children(|btn| {
            if let Some(icon) = bevy_material_ui::icons::MaterialIcon::from_name(icon_name) {
                btn.spawn(icon.with_size(24.0).with_color(icon_color));
            }
        });
}

/// Spawn the app bar section content
pub fn spawn_app_bar_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: Handle<Font>,
) {
    let _ = icon_font;
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
                "showcase.section.app_bar.title",
                "App Bars",
                "showcase.section.app_bar.description",
                "Top and Bottom app bars for navigation and actions",
            );

            // Top App Bar preview
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((
                        Text::new("Top App Bar (Small)"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    // Top app bar (real library implementation) with a right-side slot.
                    col.spawn_top_app_bar_with_right_content(
                        theme,
                        TopAppBarBuilder::new("Page Title")
                            .small()
                            .with_navigation("menu")
                            .add_action("more_vert", "more"),
                        |right| {
                            right
                                .spawn(Node {
                                    width: Val::Px(240.0),
                                    ..default()
                                })
                                .with_children(|slot| {
                                    spawn_text_field_control(
                                        slot,
                                        theme,
                                        TextFieldBuilder::new()
                                            .label("Search")
                                            .placeholder("Search")
                                            .input_type(InputType::Text)
                                            .outlined()
                                            .width(Val::Percent(100.0)),
                                    );
                                });
                        },
                    );

                    col.spawn((
                        Text::new("Bottom App Bar"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        Node {
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },
                    ));

                    // Bottom app bar
                    col.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(80.0),
                            padding: UiRect::horizontal(Val::Px(16.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        BackgroundColor(theme.surface_container),
                    ))
                    .with_children(|bar| {
                        // Left actions
                        bar.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|actions| {
                            for icon_name in [ICON_MENU, ICON_SEARCH, ICON_CHECK, ICON_CLOSE] {
                                spawn_standard_icon_button(actions, theme, icon_name);
                            }
                        });

                        // FAB preview
                        {
                            let fab_btn =
                                MaterialFab::new(ICON_ADD.to_string()).with_size(FabSize::Regular);
                            let bg_color = fab_btn.background_color(theme);
                            let icon_color = fab_btn.content_color(theme);

                            bar.spawn((
                                fab_btn,
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                Node {
                                    width: Val::Px(56.0),
                                    height: Val::Px(56.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                                BorderRadius::all(Val::Px(16.0)),
                            ))
                            .with_children(|btn| {
                                if let Some(icon) =
                                    bevy_material_ui::icons::MaterialIcon::from_name(ICON_ADD)
                                {
                                    btn.spawn(icon.with_size(24.0).with_color(icon_color));
                                }
                            });
                        }
                    });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Top App Bar with navigation, actions, and right-side content
parent.spawn_top_app_bar_with_right_content(
    theme,
    TopAppBarBuilder::new("Page Title")
        .small()
        .with_navigation("menu")
        .add_action("more_vert", "more"),
    |right| {
        // Spawn any extra widgets here (e.g. a search field)
    },
);

// Bottom App Bar
parent.spawn_bottom_app_bar(theme, |bar| {
    // Spawn actions + optional FAB
});"#,
            );
        });
}
