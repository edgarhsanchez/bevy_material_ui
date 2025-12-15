//! Toolbar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::{ICON_MENU, ICON_MORE_VERT, ICON_SEARCH};

use crate::showcase::common::*;

/// Spawn the toolbar section content
pub fn spawn_toolbar_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "Toolbars",
                "Compact top row with navigation, title, and actions",
            );

            // Example toolbar (deterministic icon rendering)
            section
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        max_width: Val::Px(560.0),
                        height: Val::Px(TOOLBAR_HEIGHT),
                        padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(Spacing::MEDIUM),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                ))
                .with_children(|toolbar| {
                    fn spawn_standard_icon_button_codepoint(
                        parent: &mut ChildSpawnerCommands,
                        theme: &MaterialTheme,
                        icon_font: &Handle<Font>,
                        codepoint: char,
                    ) {
                        let icon_btn = MaterialIconButton::new(codepoint.to_string())
                            .with_variant(IconButtonVariant::Standard);
                        let bg_color = icon_btn.background_color(theme);
                        let icon_color = icon_btn.icon_color(theme);

                        parent
                            .spawn((
                                icon_btn,
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                Node {
                                    width: Val::Px(ICON_BUTTON_SIZE),
                                    height: Val::Px(ICON_BUTTON_SIZE),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                                BorderRadius::all(Val::Px(CornerRadius::FULL)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new(codepoint.to_string()),
                                    TextFont {
                                        font: icon_font.clone(),
                                        font_size: TOOLBAR_ICON_SIZE,
                                        ..default()
                                    },
                                    TextColor(icon_color),
                                ));
                            });
                    }

                    spawn_standard_icon_button_codepoint(toolbar, theme, &icon_font, ICON_MENU);

                    toolbar.spawn((
                        Text::new("Inventory"),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                        Node {
                            flex_grow: 1.0,
                            ..default()
                        },
                    ));

                    spawn_standard_icon_button_codepoint(toolbar, theme, &icon_font, ICON_SEARCH);
                    spawn_standard_icon_button_codepoint(toolbar, theme, &icon_font, ICON_MORE_VERT);
                });

            spawn_code_block(
                section,
                theme,
                r#"// Spawn a toolbar
ui.spawn_toolbar_with(
    &theme,
    ToolbarBuilder::new("Inventory")
        .navigation_icon(MaterialIcon::new(ICON_MENU))
        .action(MaterialIcon::new(ICON_SEARCH), "search")
        .action(MaterialIcon::new(ICON_MORE_VERT), "more"),
);"#,
            );
        });
}
