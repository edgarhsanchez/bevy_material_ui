//! Icon buttons view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the icon buttons section content
pub fn spawn_icon_buttons_section(
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
                "showcase.section.icon_buttons.title",
                "Icon Buttons",
                "showcase.section.icon_buttons.description",
                "Icon-only buttons for actions - Standard, Filled, Tonal, and Outlined variants",
            );
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    // Standard icon button
                    spawn_icon_button_demo(
                        row,
                        theme,
                        "favorite",
                        IconButtonVariant::Standard,
                        "Standard",
                    );
                    // Filled icon button
                    spawn_icon_button_demo(row, theme, "add", IconButtonVariant::Filled, "Filled");
                    // Filled Tonal icon button
                    spawn_icon_button_demo(
                        row,
                        theme,
                        "edit",
                        IconButtonVariant::FilledTonal,
                        "Tonal",
                    );
                    // Outlined icon button
                    spawn_icon_button_demo(
                        row,
                        theme,
                        "delete",
                        IconButtonVariant::Outlined,
                        "Outlined",
                    );
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create an icon button
let icon_btn = MaterialIconButton::new("favorite")
    .with_variant(IconButtonVariant::Filled);

commands.spawn((
    icon_btn,
    Button,
    RippleHost::new(),
    Node {
        width: Val::Px(40.0),
        height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(theme.primary),
    BorderRadius::all(Val::Px(20.0)),
));"#,
            );
        });
}

fn spawn_icon_button_demo(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_name: &str,
    variant: IconButtonVariant,
    label: &str,
) {
    let icon_btn = MaterialIconButton::new(icon_name).with_variant(variant);
    let bg_color = icon_btn.background_color(theme);
    let icon_color = icon_btn.icon_color(theme);
    let has_border = variant == IconButtonVariant::Outlined;

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                IconButtonMarker,
                icon_btn,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(if has_border { 1.0 } else { 0.0 })),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(if has_border {
                    theme.outline
                } else {
                    Color::NONE
                }),
                BorderRadius::all(Val::Px(20.0)),
            ))
            .with_children(|btn| {
                if let Some(icon) =
                    MaterialIcon::from_name(icon_name).or_else(|| MaterialIcon::from_name("star"))
                {
                    btn.spawn(icon.with_size(24.0).with_color(icon_color));
                }
            });

            col.spawn((
                Text::new(label),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}
