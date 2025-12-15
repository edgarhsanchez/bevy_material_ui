//! App Bar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::{
    ICON_ADD,
    ICON_CHECK,
    ICON_CLOSE,
    ICON_MENU,
    ICON_MORE_VERT,
    ICON_SEARCH,
};

use crate::showcase::common::*;

fn spawn_standard_icon_button_codepoint(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon_font: &Handle<Font>,
    codepoint: char,
) {
    let icon_btn = MaterialIconButton::new(codepoint.to_string()).with_variant(IconButtonVariant::Standard);
    let bg_color = icon_btn.background_color(theme);
    let icon_color = icon_btn.icon_color(theme);

    parent
        .spawn((
            icon_btn,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                width: Val::Px(48.0),
                height: Val::Px(48.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(24.0)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(codepoint.to_string()),
                TextFont {
                    font: icon_font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(icon_color),
            ));
        });
}

/// Spawn the app bar section content
pub fn spawn_app_bar_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, icon_font: Handle<Font>) {
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
                "App Bars",
                "Top and Bottom app bars for navigation and actions"
            );

            // Top App Bar preview
            section.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Top App Bar (Small)"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                ));
                
                // Top app bar
                col.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(64.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                )).with_children(|bar| {
                    spawn_standard_icon_button_codepoint(bar, theme, &icon_font, ICON_MENU);
                    
                    // Title
                    bar.spawn((
                        Text::new("Page Title"),
                        TextFont { font_size: 22.0, ..default() },
                        TextColor(theme.on_surface),
                        Node { flex_grow: 1.0, ..default() },
                    ));
                    
                    // Actions
                    spawn_standard_icon_button_codepoint(bar, theme, &icon_font, ICON_MORE_VERT);
                });
                
                col.spawn((
                    Text::new("Bottom App Bar"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(theme.on_surface),
                    Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
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
                )).with_children(|bar| {
                    // Left actions
                    bar.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        ..default()
                    }).with_children(|actions| {
                        for codepoint in [ICON_MENU, ICON_SEARCH, ICON_CHECK, ICON_CLOSE] {
                            spawn_standard_icon_button_codepoint(actions, theme, &icon_font, codepoint);
                        }
                    });
                    
                    // FAB preview
                    {
                        let fab_btn = MaterialFab::new(ICON_ADD.to_string()).with_size(FabSize::Regular);
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
                            btn.spawn((
                                Text::new(ICON_ADD.to_string()),
                                TextFont {
                                    font: icon_font.clone(),
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(icon_color),
                            ));
                        });
                    }
                });
            });

            spawn_code_block(section, theme,
r#"// Create a top app bar
let app_bar = TopAppBar::new()
    .with_variant(TopAppBarVariant::Small)
    .title("My App")
    .navigation_icon("menu");

commands.spawn((
    app_bar,
    Node { 
        width: Val::Percent(100.0), 
        height: Val::Px(64.0),
        ..default() 
    },
    BackgroundColor(theme.surface),
));

// Create a bottom app bar
let bottom_bar = BottomAppBar::new()
    .actions(vec!["search", "share", "delete"]);"#);
        });
}
