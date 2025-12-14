//! App Bar view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the app bar section content
pub fn spawn_app_bar_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, _icon_font: Handle<Font>) {
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
                    bar.spawn_standard_icon_button(theme, "menu");
                    
                    // Title
                    bar.spawn((
                        Text::new("Page Title"),
                        TextFont { font_size: 22.0, ..default() },
                        TextColor(theme.on_surface),
                        Node { flex_grow: 1.0, ..default() },
                    ));
                    
                    // Actions
                    bar.spawn_standard_icon_button(theme, "more_vert");
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
                        for icon in ["menu", "search", "check", "close"] {
                            actions.spawn_standard_icon_button(theme, icon);
                        }
                    });
                    
                    bar.spawn_regular_fab(theme, "add");
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
