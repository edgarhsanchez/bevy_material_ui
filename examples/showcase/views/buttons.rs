//! Buttons view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the buttons section content
pub fn spawn_buttons_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Buttons",
                "MD3 buttons with 5 variants: Filled, Outlined, Text, Elevated, and Tonal",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_interactive_button(row, theme, "Filled", ButtonVariant::Filled);
                    spawn_interactive_button(row, theme, "Outlined", ButtonVariant::Outlined);
                    spawn_interactive_button(row, theme, "Text", ButtonVariant::Text);
                    spawn_interactive_button(row, theme, "Elevated", ButtonVariant::Elevated);
                    spawn_interactive_button(row, theme, "Tonal", ButtonVariant::FilledTonal);
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create a filled button
let button = MaterialButton::new("Click Me")
    .with_variant(ButtonVariant::Filled);

commands.spawn((
    button,
    Button,  // Required for interaction
    RippleHost::new(),
    Node { padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)), ..default() },
    BackgroundColor(theme.primary),
    BorderRadius::all(Val::Px(20.0)),
));"#,
            );
        });
}

fn spawn_interactive_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    variant: ButtonVariant,
) {
    let button = MaterialButton::new(label).with_variant(variant);
    let text_color = button.text_color(theme);
    let bg_color = button.background_color(theme);
    let border_color = button.border_color(theme);
    let has_border = variant == ButtonVariant::Outlined;
    let elevation = button.elevation();

    parent
        .spawn((
            button,
            Button,            // This is key - Bevy's Button component enables interaction
            Interaction::None, // Ensure interaction is initialized
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                border: UiRect::all(Val::Px(if has_border { 1.0 } else { 0.0 })),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
            elevation.to_box_shadow(), // Add shadow for elevated buttons
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}
