//! Navigation sidebar component for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::list::{ListItemClickEvent, ListItemHeadline, MaterialListItem};
use bevy_material_ui::prelude::*;

use super::common::*;

// ============================================================================
// Navigation Components
// ============================================================================

/// Marker for navigation list items - stores the ComponentSection this item represents
#[derive(Component)]
pub struct NavItem(pub ComponentSection);

// ============================================================================
// Navigation Spawn Functions
// ============================================================================

/// Spawn a navigation item in the sidebar using MaterialListItem
pub fn spawn_nav_item(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
    is_selected: bool,
) {
    // Create list item with proper selected state
    let item = MaterialListItem::new(section.display_name()).selected(is_selected);
    let text_color = item.headline_color(theme);
    let bg_color = item.background_color(theme);

    // Create test ID from section name (e.g., "nav_buttons", "nav_sliders")
    let test_id = format!(
        "nav_{}",
        section.display_name().to_lowercase().replace(" ", "_")
    );

    // Spawn with MaterialListItem + NavItem marker + TestId
    parent
        .spawn((
            NavItem(section),
            TestId::new(test_id),
            item,
            Button,
            Interaction::None,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(48.0), // Slightly smaller for navigation
                padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|item_container| {
            // Item content - headline text with ListItemHeadline marker for automatic color updates
            item_container.spawn((
                ListItemHeadline,
                Text::new(section.display_name()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}

/// Spawn a navigation item intended for a horizontal (bottom) navigation surface.
///
/// Keeps the same `TestId` format as `spawn_nav_item` so automation doesn't need
/// special-casing.
pub fn spawn_nav_item_horizontal(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
    is_selected: bool,
) {
    let item = MaterialListItem::new(section.display_name()).selected(is_selected);
    let text_color = item.headline_color(theme);
    let bg_color = item.background_color(theme);

    let test_id = format!(
        "nav_{}",
        section.display_name().to_lowercase().replace(" ", "_")
    );

    parent
        .spawn((
            NavItem(section),
            TestId::new(test_id),
            item,
            Button,
            Interaction::None,
            Node {
                // Fixed width so many items can live in a horizontal scroller.
                width: Val::Px(160.0),
                height: Val::Px(48.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(12.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|item_container| {
            item_container.spawn((
                ListItemHeadline,
                Text::new(section.display_name()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}

// ============================================================================
// Navigation Systems
// ============================================================================

/// Handle navigation item clicks
pub fn handle_nav_clicks(
    mut selected: ResMut<SelectedSection>,
    mut click_events: MessageReader<ListItemClickEvent>,
    nav_items: Query<&NavItem>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for event in click_events.read() {
        // Check if this is a navigation item click
        if let Ok(nav_item) = nav_items.get(event.entity) {
            if selected.current != nav_item.0 {
                selected.current = nav_item.0;
                info!("📍 Selected section: {:?}", nav_item.0);
                telemetry.log_event(&format!("Nav selected: {:?}", nav_item.0));
                telemetry.states.insert(
                    "selected_section".to_string(),
                    nav_item.0.telemetry_name().to_string(),
                );
            }
        }
    }
}

/// Update navigation item highlights based on selection
pub fn update_nav_highlights(
    selected: Res<SelectedSection>,
    mut nav_items: Query<(&NavItem, &mut MaterialListItem, &mut BackgroundColor)>,
    theme: Res<MaterialTheme>,
) {
    if selected.is_changed() {
        // Update MaterialListItem selected state and background color
        for (nav_item, mut list_item, mut bg) in nav_items.iter_mut() {
            let is_selected = nav_item.0 == selected.current;
            list_item.selected = is_selected;
            // Update background color based on selection
            bg.0 = if is_selected {
                theme.secondary_container
            } else {
                Color::NONE
            };
        }
    }
}
