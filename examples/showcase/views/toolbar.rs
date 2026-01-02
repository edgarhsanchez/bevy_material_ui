//! Toolbar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::icons::{ICON_MENU, ICON_MORE_VERT, ICON_SEARCH};
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the toolbar section content
pub fn spawn_toolbar_section(
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
                "showcase.section.toolbar.title",
                "Toolbars",
                "showcase.section.toolbar.description",
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
                    fn spawn_standard_icon_button(
                        parent: &mut ChildSpawnerCommands,
                        theme: &MaterialTheme,
                        icon_name: &str,
                    ) {
                        let icon_btn = MaterialIconButton::new(icon_name.to_string())
                            .with_variant(IconButtonVariant::Standard);
                        let icon_color = icon_btn.icon_color(theme);

                        parent
                            .spawn((IconButtonBuilder::new(icon_name.to_string())
                                .standard()
                                .build(theme),))
                            .with_children(|btn| {
                                if let Some(icon) =
                                    bevy_material_ui::icons::MaterialIcon::from_name(icon_name)
                                {
                                    btn.spawn(
                                        icon.with_size(TOOLBAR_ICON_SIZE).with_color(icon_color),
                                    );
                                }
                            });
                    }

                    spawn_standard_icon_button(toolbar, theme, ICON_MENU);

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

                    spawn_standard_icon_button(toolbar, theme, ICON_SEARCH);
                    spawn_standard_icon_button(toolbar, theme, ICON_MORE_VERT);
                });

            spawn_code_block(
                section,
                theme,
                r#"// Spawn a toolbar
ui.spawn_toolbar_with(
    &theme,
    ToolbarBuilder::new("Inventory")
            .navigation_icon_name(ICON_MENU)
            .action_name(ICON_SEARCH, "search")
            .action_name(ICON_MORE_VERT, "more"),
);"#,
            );
        });
}
