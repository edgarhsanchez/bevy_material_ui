//! Material Design 3 List component
//!
//! Lists are continuous, vertical indexes of text and images.
//! Reference: <https://m3.material.io/components/lists/overview>

use bevy::prelude::*;
use bevy::ui::ScrollPosition;

use crate::{
    icons::{icon_by_name, IconStyle, MaterialIcon},
    ripple::RippleHost,
    scroll::ScrollContainerBuilder,
    theme::{blend_state_layer, MaterialTheme},
    tokens::Spacing,
};

/// Maximum depth to traverse when searching for ancestor entities.
/// This prevents infinite loops in case of circular references or pathological entity hierarchies.
const MAX_ANCESTOR_DEPTH: usize = 32;

fn resolve_icon_id(icon: &str) -> Option<crate::icons::material_icons::IconId> {
    let icon = icon.trim();
    if icon.is_empty() {
        return None;
    }

    icon_by_name(icon)
}

/// Plugin for the list component
pub struct ListPlugin;

impl Plugin for ListPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<ListItemClickEvent>().add_systems(
            Update,
            (
                list_item_interaction_system,
                list_selection_system,
                list_item_style_system,
                list_item_text_style_system,
            ),
        );
    }
}

/// Selection behavior for a list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListSelectionMode {
    /// Clicking items does not change their `selected` state.
    #[default]
    None,
    /// Exactly one item is selected at a time.
    Single,
    /// Multiple items may be selected.
    Multi,
}

/// List item variants based on content
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ListItemVariant {
    /// One line of text
    #[default]
    OneLine,
    /// Two lines of text
    TwoLine,
    /// Three lines of text
    ThreeLine,
}

impl ListItemVariant {
    /// Get the height for this variant
    pub fn height(&self) -> f32 {
        match self {
            ListItemVariant::OneLine => 56.0,
            ListItemVariant::TwoLine => 72.0,
            ListItemVariant::ThreeLine => 88.0,
        }
    }
}

/// Material list container
#[derive(Component, Default)]
pub struct MaterialList {
    pub selection_mode: ListSelectionMode,
}

impl MaterialList {
    /// Create a new list
    pub fn new() -> Self {
        Self {
            selection_mode: ListSelectionMode::None,
        }
    }

    pub fn with_selection_mode(mut self, mode: ListSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }
}

/// Material list item component
#[derive(Component)]
pub struct MaterialListItem {
    /// Item variant
    pub variant: ListItemVariant,
    /// Whether the item is disabled
    pub disabled: bool,
    /// Whether the item is selected
    pub selected: bool,
    /// Headline text (primary text)
    pub headline: String,
    /// Supporting text (secondary text)
    pub supporting_text: Option<String>,
    /// Trailing supporting text
    pub trailing_text: Option<String>,
    /// Leading icon
    pub leading_icon: Option<String>,
    /// Trailing icon
    pub trailing_icon: Option<String>,
    /// Leading avatar/image URL
    pub leading_avatar: Option<String>,
    /// Leading video thumbnail URL
    pub leading_video: Option<String>,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialListItem {
    /// Create a new list item
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            variant: ListItemVariant::default(),
            disabled: false,
            selected: false,
            headline: headline.into(),
            supporting_text: None,
            trailing_text: None,
            leading_icon: None,
            trailing_icon: None,
            leading_avatar: None,
            leading_video: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: ListItemVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set trailing text
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
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

    /// Set leading avatar
    pub fn leading_avatar(mut self, url: impl Into<String>) -> Self {
        self.leading_avatar = Some(url.into());
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

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        let base = if self.selected {
            theme.secondary_container
        } else {
            Color::NONE
        };

        // Apply state layer
        let state_opacity = self.state_layer_opacity();
        if state_opacity > 0.0 {
            let state_color = theme.on_surface;
            if base == Color::NONE {
                state_color.with_alpha(state_opacity)
            } else {
                blend_state_layer(base, state_color, state_opacity)
            }
        } else {
            base
        }
    }

    /// Get the headline color
    pub fn headline_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the supporting text color
    pub fn supporting_text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
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

    /// Get the state layer opacity
    pub fn state_layer_opacity(&self) -> f32 {
        if self.disabled {
            0.0
        } else if self.pressed {
            0.12
        } else if self.hovered {
            0.08
        } else {
            0.0
        }
    }
}

/// Event when list item is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct ListItemClickEvent {
    pub entity: Entity,
}

fn list_selection_system(
    mut click_events: MessageReader<ListItemClickEvent>,
    parents: Query<&ChildOf>,
    lists: Query<&MaterialList>,
    children_query: Query<&Children>,
    mut items: Query<&mut MaterialListItem>,
) {
    for event in click_events.read() {
        // Find the nearest ancestor that is a MaterialList.
        let mut current = Some(event.entity);
        let mut list_entity = None;
        for _ in 0..MAX_ANCESTOR_DEPTH {
            let Some(e) = current else { break };
            if lists.get(e).is_ok() {
                list_entity = Some(e);
                break;
            }
            current = parents.get(e).ok().map(|p| p.0);
        }

        let Some(list_entity) = list_entity else {
            continue;
        };
        let Ok(list) = lists.get(list_entity) else {
            continue;
        };

        match list.selection_mode {
            ListSelectionMode::None => {}
            ListSelectionMode::Multi => {
                if let Ok(mut clicked) = items.get_mut(event.entity) {
                    clicked.selected = !clicked.selected;
                }
            }
            ListSelectionMode::Single => {
                // Select the clicked item and clear any other selected items in this list.
                // Traverse the list subtree to support wrappers (e.g., scroll content).
                let mut stack: Vec<Entity> = vec![list_entity];
                while let Some(node) = stack.pop() {
                    if let Ok(children) = children_query.get(node) {
                        for child in children.iter() {
                            if let Ok(mut item) = items.get_mut(child) {
                                item.selected = child == event.entity;
                            }
                            stack.push(child);
                        }
                    }
                }

                // If the clicked entity isn't under the list (unexpected), still force it selected.
                if let Ok(mut clicked) = items.get_mut(event.entity) {
                    clicked.selected = true;
                }
            }
        }
    }
}

/// System to handle list item interactions
fn list_item_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialListItem),
        (Changed<Interaction>, With<MaterialListItem>),
    >,
    mut click_events: MessageWriter<ListItemClickEvent>,
) {
    for (entity, interaction, mut item) in interaction_query.iter_mut() {
        if item.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                item.pressed = true;
                item.hovered = false;
                click_events.write(ListItemClickEvent { entity });
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

/// System to update list item styles
fn list_item_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut items: Query<(&MaterialListItem, &mut BackgroundColor), Changed<MaterialListItem>>,
) {
    let Some(theme) = theme else { return };

    for (item, mut bg_color) in items.iter_mut() {
        *bg_color = BackgroundColor(item.background_color(&theme));
    }
}

/// System to update list item text colors when item state changes
fn list_item_text_style_system(
    theme: Option<Res<MaterialTheme>>,
    changed_items: Query<(&MaterialListItem, &Children), Changed<MaterialListItem>>,
    mut headline_texts: Query<&mut TextColor, With<ListItemHeadline>>,
    mut supporting_texts: Query<
        &mut TextColor,
        (With<ListItemSupportingText>, Without<ListItemHeadline>),
    >,
    children_query: Query<&Children>,
) {
    let Some(theme) = theme else { return };

    for (item, children) in changed_items.iter() {
        let headline_color = item.headline_color(&theme);
        let supporting_color = item.supporting_text_color(&theme);

        // Update direct children
        for child in children.iter() {
            if let Ok(mut text_color) = headline_texts.get_mut(child) {
                *text_color = TextColor(headline_color);
            }
            if let Ok(mut text_color) = supporting_texts.get_mut(child) {
                *text_color = TextColor(supporting_color);
            }

            // Check nested children (for body containers)
            if let Ok(grandchildren) = children_query.get(child) {
                for grandchild in grandchildren.iter() {
                    if let Ok(mut text_color) = headline_texts.get_mut(grandchild) {
                        *text_color = TextColor(headline_color);
                    }
                    if let Ok(mut text_color) = supporting_texts.get_mut(grandchild) {
                        *text_color = TextColor(supporting_color);
                    }
                }
            }
        }
    }
}

/// Builder for lists
pub struct ListBuilder {
    /// Maximum height before scrolling (None = no limit)
    max_height: Option<f32>,
    /// Whether to show scrollbar
    show_scrollbar: bool,
    /// Selection behavior
    selection_mode: ListSelectionMode,
}

impl ListBuilder {
    /// Create a new list builder
    pub fn new() -> Self {
        Self {
            max_height: None,
            show_scrollbar: true,
            selection_mode: ListSelectionMode::None,
        }
    }

    /// Set list selection behavior.
    pub fn selection_mode(mut self, mode: ListSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Set maximum height (enables scrolling)
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Set maximum visible items (enables scrolling based on item count)
    /// Uses one-line item height (56px) as reference
    pub fn max_visible_items(mut self, count: usize) -> Self {
        self.max_height = Some(count as f32 * 56.0);
        self
    }

    /// Set maximum visible items with specific variant height
    pub fn max_visible_items_variant(mut self, count: usize, variant: ListItemVariant) -> Self {
        self.max_height = Some(count as f32 * variant.height());
        self
    }

    /// Hide the scrollbar (content still scrollable)
    pub fn hide_scrollbar(mut self) -> Self {
        self.show_scrollbar = false;
        self
    }

    /// Build the list bundle (non-scrollable)
    pub fn build(self) -> impl Bundle {
        (
            MaterialList::new().with_selection_mode(self.selection_mode),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                padding: UiRect::vertical(Val::Px(Spacing::SMALL)),
                ..default()
            },
        )
    }

    /// Build a scrollable list bundle
    /// Uses scroll_y() for native scrolling - ensure scroll position is clamped externally
    pub fn build_scrollable(self) -> impl Bundle {
        let height = self.max_height.map(Val::Px).unwrap_or(Val::Auto);
        (
            MaterialList::new().with_selection_mode(self.selection_mode),
            ScrollableList,
            ScrollContainerBuilder::new()
                .vertical()
                .with_scrollbars(self.show_scrollbar)
                .build(),
            ScrollPosition::default(),
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height,
                max_height: height,
                // Important for scroll containers inside flex columns:
                // allow shrinking so overflow/scrolling can happen.
                min_height: Val::Px(0.0),
                padding: UiRect::vertical(Val::Px(Spacing::SMALL)),
                // Bevy's scroll system expects both axes to be `Scroll`.
                // Actual scroll direction is controlled by `ScrollContainer.direction`.
                overflow: Overflow::scroll(),
                ..default()
            },
        )
    }
}

/// Marker for scrollable lists
#[derive(Component)]
pub struct ScrollableList;

impl Default for ListBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for list items
pub struct ListItemBuilder {
    item: MaterialListItem,
}

impl ListItemBuilder {
    /// Create a new list item builder
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            item: MaterialListItem::new(headline),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: ListItemVariant) -> Self {
        self.item.variant = variant;
        self
    }

    /// Make one-line item
    pub fn one_line(self) -> Self {
        self.variant(ListItemVariant::OneLine)
    }

    /// Make two-line item
    pub fn two_line(self) -> Self {
        self.variant(ListItemVariant::TwoLine)
    }

    /// Make three-line item
    pub fn three_line(self) -> Self {
        self.variant(ListItemVariant::ThreeLine)
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.item.supporting_text = Some(text.into());
        self
    }

    /// Set trailing text
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.item.trailing_text = Some(text.into());
        self
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

    /// Set leading avatar
    pub fn leading_avatar(mut self, url: impl Into<String>) -> Self {
        self.item.leading_avatar = Some(url.into());
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

    /// Build the list item bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.item.background_color(theme);
        let height = self.item.variant.height();

        (
            self.item,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(height),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::SMALL)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::LARGE),
                ..default()
            },
            BackgroundColor(bg_color),
        )
    }
}

/// Marker for leading content area
#[derive(Component)]
pub struct ListItemLeading;

/// Marker for content/body area
#[derive(Component)]
pub struct ListItemBody;

/// Marker for headline text in list item
#[derive(Component)]
pub struct ListItemHeadline;

/// Marker for supporting text in list item
#[derive(Component)]
pub struct ListItemSupportingText;

/// Marker for trailing content area
#[derive(Component)]
pub struct ListItemTrailing;

/// Marker for list divider
#[derive(Component)]
pub struct ListDivider;

/// Create a list divider
pub fn create_list_divider(theme: &MaterialTheme, inset: bool) -> impl Bundle {
    (
        ListDivider,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: if inset {
                UiRect::left(Val::Px(Spacing::LARGE + 56.0)) // Account for leading element
            } else {
                UiRect::ZERO
            },
            ..default()
        },
        BackgroundColor(theme.outline_variant),
    )
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material lists and list items as children
///
/// This trait provides a clean API for spawning lists within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_list(&theme, |list| {
///         list.spawn_list_item(&theme, "Item 1", None);
///         list.spawn_list_item(&theme, "Item 2", Some("Supporting text"));
///     });
/// });
/// ```
pub trait SpawnListChild {
    /// Spawn a list container
    fn spawn_list(&mut self, with_children: impl FnOnce(&mut ChildSpawnerCommands));

    /// Spawn a list item with headline and optional supporting text
    fn spawn_list_item(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        supporting: Option<impl Into<String>>,
    );

    /// Spawn a list item with full builder control
    fn spawn_list_item_with(&mut self, theme: &MaterialTheme, builder: ListItemBuilder);

    /// Spawn a list divider
    fn spawn_list_divider(&mut self, theme: &MaterialTheme, inset: bool);
}

impl SpawnListChild for ChildSpawnerCommands<'_> {
    fn spawn_list(&mut self, with_children: impl FnOnce(&mut ChildSpawnerCommands)) {
        self.spawn(ListBuilder::new().build())
            .with_children(with_children);
    }

    fn spawn_list_item(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        supporting: Option<impl Into<String>>,
    ) {
        let headline_str = headline.into();
        let supporting_str = supporting.map(|s| s.into());
        let has_supporting = supporting_str.is_some();

        let builder = if has_supporting {
            ListItemBuilder::new(&headline_str).two_line()
        } else {
            ListItemBuilder::new(&headline_str)
        };

        let headline_color = theme.on_surface;
        let supporting_color = theme.on_surface_variant;

        self.spawn(builder.build(theme)).with_children(|item| {
            // Body content
            item.spawn((
                ListItemBody,
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
            ))
            .with_children(|body| {
                // Headline
                body.spawn((
                    ListItemHeadline,
                    Text::new(&headline_str),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(headline_color),
                ));

                // Supporting text (if provided)
                if let Some(ref supporting) = supporting_str {
                    body.spawn((
                        ListItemSupportingText,
                        Text::new(supporting),
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

    fn spawn_list_item_with(&mut self, theme: &MaterialTheme, builder: ListItemBuilder) {
        let headline = builder.item.headline.clone();
        let supporting_text = builder.item.supporting_text.clone();
        let trailing_text = builder.item.trailing_text.clone();
        let leading_icon = builder.item.leading_icon.clone();
        let trailing_icon = builder.item.trailing_icon.clone();

        let headline_color = builder.item.headline_color(theme);
        let supporting_color = builder.item.supporting_text_color(theme);
        let icon_color = builder.item.icon_color(theme);

        self.spawn(builder.build(theme)).with_children(|item| {
            // Leading content
            if let Some(icon_str) = leading_icon.as_deref() {
                if let Some(icon_id) = resolve_icon_id(icon_str) {
                    item.spawn((
                        ListItemLeading,
                        Node {
                            width: Val::Px(56.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|leading| {
                        leading.spawn((
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });
                }
            }

            // Body
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
                    Text::new(&headline),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(headline_color),
                ));

                if let Some(ref supporting) = supporting_text {
                    body.spawn((
                        ListItemSupportingText,
                        Text::new(supporting),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(supporting_color),
                    ));
                }
            });

            // Trailing content
            if trailing_text.is_some() || trailing_icon.is_some() {
                item.spawn((
                    ListItemTrailing,
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(Spacing::MEDIUM),
                        ..default()
                    },
                ))
                .with_children(|trailing| {
                    if let Some(ref text) = trailing_text {
                        trailing.spawn((
                            Text::new(text),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(supporting_color),
                        ));
                    }

                    if let Some(icon_str) = trailing_icon.as_deref() {
                        if let Some(icon_id) = resolve_icon_id(icon_str) {
                            trailing.spawn((
                                MaterialIcon::new(icon_id),
                                IconStyle::outlined().with_color(icon_color).with_size(24.0),
                            ));
                        }
                    }
                });
            }
        });
    }

    fn spawn_list_divider(&mut self, theme: &MaterialTheme, inset: bool) {
        self.spawn(create_list_divider(theme, inset));
    }
}
