//! Material Design 3 Tabs component
//!
//! Tabs organize content across different screens, data sets, and other interactions.
//! Reference: <https://m3.material.io/components/tabs/overview>

use bevy::prelude::*;

use crate::{
    ripple::RippleHost,
    telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId},
    theme::MaterialTheme,
    tokens::Spacing,
};

/// Plugin for the tabs component
pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<TabChangeEvent>()
            .add_systems(Update, tab_interaction_system)
            .add_systems(Update, tab_style_system)
            .add_systems(Update, sync_tabs_selection_system)
            .add_systems(Update, tab_label_and_indicator_system)
            .add_systems(Update, tab_content_visibility_system)
            .add_systems(
                Update,
                tabs_telemetry_system.after(tab_label_and_indicator_system),
            );
    }
}

fn tabs_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    tabs_query: Query<(Entity, &TestId, &Children), With<MaterialTabs>>,
    tab_query: Query<(&MaterialTab, &Children), With<MaterialTab>>,
    label_query: Query<(), With<TabLabelText>>,
    indicator_query: Query<(), With<TabIndicator>>,
    content_query: Query<(Entity, &TabContent), Without<TestId>>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    use std::collections::HashMap;
    let mut tabs_ids: HashMap<Entity, String> = HashMap::new();

    for (tabs_entity, tabs_id, children) in tabs_query.iter() {
        let tabs_id = tabs_id.id();
        tabs_ids.insert(tabs_entity, tabs_id.to_owned());

        for child in children.iter() {
            let Ok((tab, tab_children)) = tab_query.get(child) else {
                continue;
            };

            commands.queue(InsertTestIdIfExists {
                entity: child,
                id: format!("{tabs_id}/tab/{}", tab.index),
            });

            for tab_child in tab_children.iter() {
                if label_query.get(tab_child).is_ok() {
                    commands.queue(InsertTestIdIfExists {
                        entity: tab_child,
                        id: format!("{tabs_id}/tab/{}/label", tab.index),
                    });
                }
                if indicator_query.get(tab_child).is_ok() {
                    commands.queue(InsertTestIdIfExists {
                        entity: tab_child,
                        id: format!("{tabs_id}/tab/{}/indicator", tab.index),
                    });
                }
            }
        }
    }

    for (entity, content) in content_query.iter() {
        let Some(tabs_id) = tabs_ids.get(&content.tabs_entity) else {
            continue;
        };

        commands.queue(InsertTestIdIfExists {
            entity,
            id: format!("{tabs_id}/content/{}", content.index),
        });
    }
}

/// Tab variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TabVariant {
    /// Primary tabs - For primary destinations
    #[default]
    Primary,
    /// Secondary tabs - For secondary destinations or subpages
    Secondary,
}

/// Material tabs container
#[derive(Component)]
pub struct MaterialTabs {
    /// Tab variant
    pub variant: TabVariant,
    /// Currently selected tab index
    pub selected: usize,
}

impl MaterialTabs {
    /// Create a new tabs container
    pub fn new() -> Self {
        Self {
            variant: TabVariant::default(),
            selected: 0,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set initially selected tab
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }
}

impl Default for MaterialTabs {
    fn default() -> Self {
        Self::new()
    }
}

/// Material tab item
#[derive(Component)]
pub struct MaterialTab {
    /// Tab index in parent container
    pub index: usize,
    /// Tab label text
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Whether the tab is disabled
    pub disabled: bool,
    /// Whether this tab is currently selected
    pub selected: bool,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialTab {
    /// Create a new tab
    pub fn new(index: usize, label: impl Into<String>) -> Self {
        Self {
            index,
            label: label.into(),
            icon: None,
            disabled: false,
            selected: index == 0,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
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

    /// Get the content color
    pub fn content_color(&self, theme: &MaterialTheme, variant: TabVariant) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.selected {
            match variant {
                TabVariant::Primary => theme.primary,
                TabVariant::Secondary => theme.on_surface,
            }
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the indicator color
    pub fn indicator_color(&self, theme: &MaterialTheme, variant: TabVariant) -> Color {
        match variant {
            TabVariant::Primary => theme.primary,
            TabVariant::Secondary => theme.primary,
        }
    }
}

/// Event when tab selection changes
#[derive(Event, bevy::prelude::Message)]
pub struct TabChangeEvent {
    /// The tabs container entity
    pub tabs_entity: Entity,
    /// The selected tab entity
    pub tab_entity: Entity,
    /// The selected tab index
    pub index: usize,
}

/// Tab dimensions
pub const TAB_HEIGHT_PRIMARY: f32 = 64.0;
pub const TAB_HEIGHT_PRIMARY_ICON_ONLY: f32 = 48.0;
pub const TAB_HEIGHT_SECONDARY: f32 = 48.0;
pub const TAB_INDICATOR_HEIGHT: f32 = 3.0;

/// Marker for tab label text, so the tabs systems can reliably update the label color.
#[derive(Component)]
pub struct TabLabelText;

/// System to handle tab interactions
fn tab_interaction_system(
    mut tab_queries: ParamSet<(
        Query<
            (Entity, &Interaction, &mut MaterialTab, &ChildOf),
            (Changed<Interaction>, With<MaterialTab>),
        >,
        Query<&mut MaterialTab>,
    )>,
    mut tabs_query: Query<(Entity, &mut MaterialTabs)>,
    children_query: Query<&Children>,
    mut change_events: MessageWriter<TabChangeEvent>,
) {
    // Two-phase approach:
    // 1) Read interactions and update the interacted tab + parent selection.
    // 2) Sync sibling `MaterialTab.selected` flags via a separate query.
    // This avoids Bevy's runtime borrow conflict checks for overlapping `&mut MaterialTab` queries.
    let mut pending_selection_updates: Vec<(Entity, Entity, usize)> = Vec::new();

    for (entity, interaction, mut tab, parent) in tab_queries.p0().iter_mut() {
        if tab.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                tab.pressed = true;
                tab.hovered = false;

                let tab_index = tab.index;

                // Update tabs container
                if let Ok((tabs_entity, mut tabs)) = tabs_query.get_mut(parent.parent()) {
                    if tabs.selected != tab_index {
                        tabs.selected = tab_index;
                    }

                    pending_selection_updates.push((tabs_entity, entity, tab_index));

                    change_events.write(TabChangeEvent {
                        tabs_entity,
                        tab_entity: entity,
                        index: tab_index,
                    });
                }
            }
            Interaction::Hovered => {
                tab.pressed = false;
                tab.hovered = true;
            }
            Interaction::None => {
                tab.pressed = false;
                tab.hovered = false;
            }
        }
    }

    // Apply selection updates to all sibling tabs.
    for (tabs_entity, pressed_tab_entity, tab_index) in pending_selection_updates {
        if let Ok(children) = children_query.get(tabs_entity) {
            for child in children.iter() {
                if let Ok(mut sibling_tab) = tab_queries.p1().get_mut(child) {
                    sibling_tab.selected = sibling_tab.index == tab_index;
                    if child == pressed_tab_entity {
                        sibling_tab.pressed = true;
                        sibling_tab.hovered = false;
                    } else {
                        sibling_tab.pressed = false;
                        sibling_tab.hovered = false;
                    }
                }
            }
        }
    }
}

/// System to update tab styles
fn tab_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut tabs: Query<(&MaterialTab, &mut BackgroundColor), Changed<MaterialTab>>,
) {
    let Some(theme) = theme else { return };

    for (tab, mut bg_color) in tabs.iter_mut() {
        // Keep styling minimal; primary feedback is the indicator + label color.
        // Use a subtle container tint on hover/press.
        bg_color.0 = if tab.pressed {
            theme.surface_container_high
        } else if tab.hovered {
            theme.surface_container_highest
        } else {
            Color::NONE
        };
    }
}

/// Ensure the `MaterialTab.selected` flags match the parent `MaterialTabs.selected`.
/// This keeps programmatic changes and startup state consistent.
fn sync_tabs_selection_system(
    tabs_query: Query<(&MaterialTabs, &Children), Changed<MaterialTabs>>,
    mut tab_query: Query<&mut MaterialTab>,
) {
    for (tabs, children) in tabs_query.iter() {
        for child in children.iter() {
            if let Ok(mut tab) = tab_query.get_mut(child) {
                tab.selected = tab.index == tabs.selected;
            }
        }
    }
}

/// Update tab label colors and ensure the selected tab has an indicator.
fn tab_label_and_indicator_system(
    mut commands: Commands,
    theme: Option<Res<MaterialTheme>>,
    tabs_query: Query<&MaterialTabs>,
    mut tab_query: Query<(Entity, &MaterialTab, &Children, &ChildOf), Changed<MaterialTab>>,
    mut label_query: Query<&mut TextColor, With<TabLabelText>>,
    indicator_query: Query<(), With<TabIndicator>>,
) {
    let Some(theme) = theme else { return };

    for (tab_entity, tab, children, parent) in tab_query.iter_mut() {
        let Ok(tabs) = tabs_query.get(parent.parent()) else {
            continue;
        };
        let label_color = tab.content_color(&theme, tabs.variant);

        let mut has_indicator = false;
        for child in children.iter() {
            if let Ok(mut tc) = label_query.get_mut(child) {
                tc.0 = label_color;
            }

            if indicator_query.get(child).is_ok() {
                has_indicator = true;
                if !tab.selected {
                    // Indicator has no children in our bundles; plain despawn is sufficient.
                    commands.entity(child).despawn();
                }
            }
        }

        if tab.selected && !has_indicator {
            commands.entity(tab_entity).with_children(|c| {
                c.spawn(create_tab_indicator(&theme, tabs.variant));
            });
        }
    }
}

/// System to update tab content visibility based on selected tab
///
/// This system looks for `TabContent` components that are siblings of `MaterialTab` components
/// within a `MaterialTabs` container, or content panels that are children of the tabs container.
fn tab_content_visibility_system(
    tabs_query: Query<(Entity, &MaterialTabs)>,
    mut content_query: Query<(&TabContent, &mut Visibility, Option<&mut Node>)>,
) {
    for (tabs_entity, tabs) in tabs_query.iter() {
        for (content, mut visibility, node) in content_query.iter_mut() {
            if content.tabs_entity != tabs_entity {
                continue;
            }

            let is_selected = content.index == tabs.selected;
            *visibility = if is_selected {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };

            // `Visibility::Hidden` still participates in layout in Bevy UI.
            // Toggle `Display` so only the selected panel affects layout.
            if let Some(mut node) = node {
                node.display = if is_selected {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

/// Builder for tabs container
pub struct TabsBuilder {
    tabs: MaterialTabs,
}

impl TabsBuilder {
    /// Create a new tabs builder
    pub fn new() -> Self {
        Self {
            tabs: MaterialTabs::new(),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.tabs.variant = variant;
        self
    }

    /// Make primary tabs
    pub fn primary(self) -> Self {
        self.variant(TabVariant::Primary)
    }

    /// Make secondary tabs
    pub fn secondary(self) -> Self {
        self.variant(TabVariant::Secondary)
    }

    /// Set initially selected tab
    pub fn selected(mut self, index: usize) -> Self {
        self.tabs.selected = index;
        self
    }

    /// Build the tabs bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let height = match self.tabs.variant {
            TabVariant::Primary => TAB_HEIGHT_PRIMARY,
            TabVariant::Secondary => TAB_HEIGHT_SECONDARY,
        };

        (
            self.tabs,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(height),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                ..default()
            },
            BackgroundColor(theme.surface),
        )
    }
}

impl Default for TabsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for individual tabs
pub struct TabBuilder {
    tab: MaterialTab,
    variant: TabVariant,
}

impl TabBuilder {
    /// Create a new tab builder
    pub fn new(index: usize, label: impl Into<String>) -> Self {
        Self {
            tab: MaterialTab::new(index, label),
            variant: TabVariant::Primary,
        }
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.tab.icon = Some(icon.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.tab.disabled = disabled;
        self
    }

    /// Set selected
    pub fn selected(mut self, selected: bool) -> Self {
        self.tab.selected = selected;
        self
    }

    /// Set the variant (inherited from parent tabs usually)
    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Build the tab bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let _content_color = self.tab.content_color(theme, self.variant);

        (
            self.tab,
            Button,
            RippleHost::new(),
            Node {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        )
    }
}

/// Marker for tab indicator (the active line)
#[derive(Component)]
pub struct TabIndicator;

/// Tab content panel that shows/hides based on tab selection
///
/// Add this to content containers that should be shown/hidden when tabs change.
/// The content's visibility will be managed automatically based on the parent
/// `MaterialTabs` container's selected tab.
#[derive(Component)]
pub struct TabContent {
    /// Index of the tab this content corresponds to
    pub index: usize,

    /// Entity of the `MaterialTabs` container this content belongs to
    pub tabs_entity: Entity,
}

impl TabContent {
    /// Create a new tab content for the given tab index and tabs container
    pub fn new(index: usize, tabs_entity: Entity) -> Self {
        Self { index, tabs_entity }
    }
}

/// Create a tab indicator
pub fn create_tab_indicator(theme: &MaterialTheme, _variant: TabVariant) -> impl Bundle {
    (
        TabIndicator,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            height: Val::Px(TAB_INDICATOR_HEIGHT),
            ..default()
        },
        BackgroundColor(theme.primary),
        BorderRadius::top(Val::Px(TAB_INDICATOR_HEIGHT)),
    )
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material tabs as children
pub trait SpawnTabsChild {
    /// Spawn a tab bar container with tabs
    fn spawn_tab_bar(
        &mut self,
        theme: &MaterialTheme,
        variant: TabVariant,
        with_tabs: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a single tab
    fn spawn_tab(&mut self, theme: &MaterialTheme, label: impl Into<String>, selected: bool);

    /// Spawn a tab with full builder control
    fn spawn_tab_with(&mut self, theme: &MaterialTheme, builder: TabBuilder);
}

impl SpawnTabsChild for ChildSpawnerCommands<'_> {
    fn spawn_tab_bar(
        &mut self,
        theme: &MaterialTheme,
        variant: TabVariant,
        with_tabs: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn(TabsBuilder::new().variant(variant).build(theme))
            .with_children(with_tabs);
    }

    fn spawn_tab(&mut self, theme: &MaterialTheme, label: impl Into<String>, selected: bool) {
        let label_str = label.into();
        let builder = TabBuilder::new(0, &label_str).selected(selected);
        let content_color = builder.tab.content_color(theme, builder.variant);

        self.spawn(builder.build(theme)).with_children(|tab| {
            tab.spawn((
                TabLabelText,
                Text::new(&label_str),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(content_color),
            ));

            if selected {
                tab.spawn(create_tab_indicator(theme, TabVariant::Primary));
            }
        });
    }

    fn spawn_tab_with(&mut self, theme: &MaterialTheme, builder: TabBuilder) {
        let label_str = builder.tab.label.clone();
        let selected = builder.tab.selected;
        let variant = builder.variant;
        let content_color = builder.tab.content_color(theme, variant);

        self.spawn(builder.build(theme)).with_children(|tab| {
            tab.spawn((
                TabLabelText,
                Text::new(&label_str),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(content_color),
            ));

            if selected {
                tab.spawn(create_tab_indicator(theme, variant));
            }
        });
    }
}
