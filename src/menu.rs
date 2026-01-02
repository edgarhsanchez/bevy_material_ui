//! Material Design 3 Menu component
//!
//! Menus display a list of choices on a temporary surface.
//! This module leverages native `BoxShadow` for elevation shadows.
//!
//! Reference: <https://m3.material.io/components/menus/overview>

use bevy::prelude::*;
use bevy::ui::BoxShadow;

use std::collections::HashMap;

use crate::{
    elevation::Elevation,
    ripple::RippleHost,
    telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId},
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the menu component
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<MenuOpenEvent>()
            .add_message::<MenuCloseEvent>()
            .add_message::<MenuItemSelectEvent>()
            .add_systems(
                Update,
                (
                    menu_visibility_system,
                    menu_shadow_system,
                    menu_item_interaction_system,
                    menu_item_style_system,
                    menu_telemetry_system,
                ),
            );
    }
}

fn sanitize_test_id_component(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        let c = ch.to_ascii_lowercase();
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if (c.is_ascii_whitespace() || c == '-') && !out.ends_with('_') {
            out.push('_');
        }
    }

    while out.ends_with('_') {
        out.pop();
    }

    if out.is_empty() {
        "item".to_string()
    } else {
        out
    }
}

fn menu_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    menus: Query<(&TestId, &Children), With<MaterialMenu>>,
    children_query: Query<&Children>,
    menu_items: Query<&MaterialMenuItem>,
    menu_dividers: Query<(), With<MenuDivider>>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    for (test_id, children) in menus.iter() {
        let base = test_id.id();
        let mut item_counts: HashMap<String, u32> = HashMap::new();
        let mut divider_index: u32 = 0;

        let mut stack: Vec<Entity> = children.iter().collect();
        while let Some(entity) = stack.pop() {
            if let Ok(item) = menu_items.get(entity) {
                let slug = sanitize_test_id_component(item.label.as_str());
                let count = item_counts.entry(slug.clone()).or_insert(0);
                *count += 1;
                let unique = if *count == 1 {
                    slug
                } else {
                    format!("{slug}_{}", *count)
                };

                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/item/{unique}"),
                });
            }

            if menu_dividers.get(entity).is_ok() {
                divider_index += 1;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/divider/{divider_index}"),
                });
            }

            if let Ok(children) = children_query.get(entity) {
                stack.extend(children.iter());
            }
        }
    }
}

/// Material menu component
#[derive(Component)]
pub struct MaterialMenu {
    /// Whether the menu is currently open
    pub open: bool,
    /// Anchor corner for positioning
    pub anchor: MenuAnchor,
    /// Whether clicking outside closes the menu
    pub close_on_click_outside: bool,
}

impl MaterialMenu {
    /// Create a new menu
    pub fn new() -> Self {
        Self {
            open: false,
            anchor: MenuAnchor::default(),
            close_on_click_outside: true,
        }
    }

    /// Set anchor position
    pub fn anchor(mut self, anchor: MenuAnchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Start open
    pub fn open(mut self) -> Self {
        self.open = true;
        self
    }

    /// Keep open when clicking outside
    pub fn no_close_on_outside(mut self) -> Self {
        self.close_on_click_outside = false;
        self
    }

    /// Get the surface color
    pub fn surface_color(&self, theme: &MaterialTheme) -> Color {
        theme.surface_container
    }

    /// Get the elevation
    pub fn elevation(&self) -> Elevation {
        Elevation::Level2
    }
}

impl Default for MaterialMenu {
    fn default() -> Self {
        Self::new()
    }
}

/// Menu anchor position
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MenuAnchor {
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner (default dropdown position)
    #[default]
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
}

/// Material menu item
#[derive(Component)]
pub struct MaterialMenuItem {
    /// Item label text
    pub label: String,
    /// Leading icon
    pub leading_icon: Option<String>,
    /// Trailing icon
    pub trailing_icon: Option<String>,
    /// Trailing text (e.g., keyboard shortcut)
    pub trailing_text: Option<String>,
    /// Whether this item opens a submenu
    pub has_submenu: bool,
    /// Whether the item is disabled
    pub disabled: bool,
    /// Whether the item is selected/checked
    pub selected: bool,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialMenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            leading_icon: None,
            trailing_icon: None,
            trailing_text: None,
            has_submenu: false,
            disabled: false,
            selected: false,
            pressed: false,
            hovered: false,
        }
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set trailing text (keyboard shortcut)
    pub fn shortcut(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    /// Mark as having a submenu
    pub fn submenu(mut self) -> Self {
        self.has_submenu = true;
        self.trailing_icon = Some("chevron_right".into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Get the text color
    pub fn text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the background color
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.selected {
            theme.secondary_container
        } else {
            Color::NONE
        }
    }
}

/// Event to open a menu
#[derive(Event, bevy::prelude::Message)]
pub struct MenuOpenEvent {
    pub entity: Entity,
}

/// Event when menu is closed
#[derive(Event, bevy::prelude::Message)]
pub struct MenuCloseEvent {
    pub entity: Entity,
}

/// Event when menu item is selected
#[derive(Event, bevy::prelude::Message)]
pub struct MenuItemSelectEvent {
    pub menu_entity: Entity,
    pub item_entity: Entity,
}

/// Menu dimensions
pub const MENU_MIN_WIDTH: f32 = 112.0;
pub const MENU_MAX_WIDTH: f32 = 280.0;
pub const MENU_ITEM_HEIGHT: f32 = 48.0;

/// System to handle menu visibility
fn menu_visibility_system(mut menus: Query<(&MaterialMenu, &mut Node), Changed<MaterialMenu>>) {
    for (menu, mut node) in menus.iter_mut() {
        node.display = if menu.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// System to update menu shadows using native BoxShadow
fn menu_shadow_system(mut menus: Query<(&MaterialMenu, &mut BoxShadow), Changed<MaterialMenu>>) {
    for (menu, mut shadow) in menus.iter_mut() {
        // Only show shadow when menu is open
        if menu.open {
            *shadow = menu.elevation().to_box_shadow();
        } else {
            *shadow = BoxShadow::default();
        }
    }
}

/// System to handle menu item interactions
fn menu_item_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialMenuItem, &ChildOf),
        (Changed<Interaction>, With<MaterialMenuItem>),
    >,
    menus: Query<Entity, With<MaterialMenu>>,
    mut select_events: MessageWriter<MenuItemSelectEvent>,
) {
    for (entity, interaction, mut item, parent) in interaction_query.iter_mut() {
        if item.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                item.pressed = true;
                item.hovered = false;

                if !item.has_submenu {
                    // Find the menu ancestor
                    if let Ok(menu_entity) = menus.get(parent.parent()) {
                        select_events.write(MenuItemSelectEvent {
                            menu_entity,
                            item_entity: entity,
                        });
                    }
                }
            }
            Interaction::Hovered => {
                item.pressed = false;
                item.hovered = true;
            }
            Interaction::None => {
                item.pressed = false;
                item.hovered = false;
            }
        }
    }
}

/// System to update menu item styles
fn menu_item_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut items: Query<(&MaterialMenuItem, &mut BackgroundColor), Changed<MaterialMenuItem>>,
) {
    let Some(theme) = theme else { return };

    for (item, mut bg_color) in items.iter_mut() {
        *bg_color = BackgroundColor(item.background_color(&theme));
    }
}

/// Builder for menus
pub struct MenuBuilder {
    menu: MaterialMenu,
}

impl MenuBuilder {
    /// Create a new menu builder
    pub fn new() -> Self {
        Self {
            menu: MaterialMenu::new(),
        }
    }

    /// Set anchor position
    pub fn anchor(mut self, anchor: MenuAnchor) -> Self {
        self.menu.anchor = anchor;
        self
    }

    /// Start open
    pub fn open(mut self) -> Self {
        self.menu.open = true;
        self
    }

    /// Keep open when clicking outside
    pub fn no_close_on_outside(mut self) -> Self {
        self.menu.close_on_click_outside = false;
        self
    }

    /// Build the menu bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.menu.surface_color(theme);

        (
            self.menu,
            Node {
                display: Display::None, // Hidden by default
                position_type: PositionType::Absolute,
                min_width: Val::Px(MENU_MIN_WIDTH),
                max_width: Val::Px(MENU_MAX_WIDTH),
                flex_direction: FlexDirection::Column,
                padding: UiRect::vertical(Val::Px(Spacing::SMALL)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
            // Native Bevy 0.17 shadow support (starts hidden since menu is closed)
            BoxShadow::default(),
        )
    }
}

impl Default for MenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for menu items
pub struct MenuItemBuilder {
    item: MaterialMenuItem,
}

impl MenuItemBuilder {
    /// Create a new menu item builder
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            item: MaterialMenuItem::new(label),
        }
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.item.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.item.trailing_icon = Some(icon.into());
        self
    }

    /// Set keyboard shortcut
    pub fn shortcut(mut self, text: impl Into<String>) -> Self {
        self.item.trailing_text = Some(text.into());
        self
    }

    /// Mark as submenu trigger
    pub fn submenu(mut self) -> Self {
        self.item.has_submenu = true;
        self.item.trailing_icon = Some("chevron_right".into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.item.disabled = disabled;
        self
    }

    /// Set selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.item.selected = selected;
        self
    }

    /// Build the menu item bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.item.background_color(theme);

        (
            self.item,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(MENU_ITEM_HEIGHT),
                padding: UiRect::horizontal(Val::Px(Spacing::MEDIUM)),
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::MEDIUM),
                ..default()
            },
            BackgroundColor(bg_color),
        )
    }
}

/// Marker for menu divider
#[derive(Component)]
pub struct MenuDivider;

/// Create a menu divider
pub fn create_menu_divider(theme: &MaterialTheme) -> impl Bundle {
    (
        MenuDivider,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(Spacing::SMALL)),
            ..default()
        },
        BackgroundColor(theme.outline_variant),
    )
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material menus as children
pub trait SpawnMenuChild {
    /// Spawn a menu container
    fn spawn_menu(
        &mut self,
        theme: &MaterialTheme,
        with_items: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a menu item
    fn spawn_menu_item(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn a menu item with full builder control
    fn spawn_menu_item_with(&mut self, theme: &MaterialTheme, builder: MenuItemBuilder);

    /// Spawn a menu divider
    fn spawn_menu_divider(&mut self, theme: &MaterialTheme);
}

impl SpawnMenuChild for ChildSpawnerCommands<'_> {
    fn spawn_menu(
        &mut self,
        theme: &MaterialTheme,
        with_items: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn(MenuBuilder::new().build(theme))
            .with_children(with_items);
    }

    fn spawn_menu_item(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        let label_str = label.into();
        let label_color = theme.on_surface;

        self.spawn(MenuItemBuilder::new(&label_str).build(theme))
            .with_children(|item| {
                item.spawn((
                    Text::new(&label_str),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(label_color),
                ));
            });
    }

    fn spawn_menu_item_with(&mut self, theme: &MaterialTheme, builder: MenuItemBuilder) {
        let label_str = builder.item.label.clone();
        let label_color = theme.on_surface;

        self.spawn(builder.build(theme)).with_children(|item| {
            item.spawn((
                Text::new(&label_str),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(label_color),
            ));
        });
    }

    fn spawn_menu_divider(&mut self, theme: &MaterialTheme) {
        self.spawn(create_menu_divider(theme));
    }
}
