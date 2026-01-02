//! Material Design 3 App Bar components
//!
//! App bars display information and actions at the top of a screen.
//! This module includes Top App Bar and Bottom App Bar.
//!
//! Reference: <https://m3.material.io/components/app-bars/overview>

use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::{
    i18n::LocalizedText,
    icons::{IconStyle, MaterialIcon},
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Conventional maximum depth to traverse when searching for ancestor entities.
///
/// In typical Material Design–style UI layouts (see module-level reference), the widget
/// hierarchy is only a handful of levels deep (usually well under 10). A value of 32 is
/// therefore a conservative upper bound: it is far larger than any realistic app bar
/// hierarchy, but still finite, and can be used by traversal helpers to guard against
/// circular references or otherwise corrupted/pathological entity hierarchies.
const MAX_ANCESTOR_DEPTH: usize = 32;

/// Plugin for app bar components
pub struct AppBarPlugin;

impl Plugin for AppBarPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<AppBarNavigationEvent>()
            .add_message::<AppBarActionEvent>()
            .add_systems(
                Update,
                (top_app_bar_scroll_system, app_bar_interaction_system),
            );
    }
}

// ============================================================================
// Events
// ============================================================================

/// Event fired when the navigation icon is clicked
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct AppBarNavigationEvent {
    /// The app bar entity
    pub app_bar: Entity,
}

/// Event fired when an action button is clicked
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct AppBarActionEvent {
    /// The app bar entity
    pub app_bar: Entity,
    /// The action identifier
    pub action: String,
}

// ============================================================================
// Types
// ============================================================================

/// Top App Bar variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TopAppBarVariant {
    /// Small - Standard height, no scrolling behavior
    #[default]
    Small,
    /// Center-aligned - Title centered
    CenterAligned,
    /// Medium - Larger, collapses on scroll
    Medium,
    /// Large - Largest, collapses to medium then small
    Large,
}

/// Scroll behavior for Top App Bar
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TopAppBarScrollBehavior {
    /// Fixed - Always visible
    #[default]
    Fixed,
    /// Scroll - Scrolls with content
    Scroll,
    /// Enter/Exit - Hides on scroll down, shows on scroll up
    EnterExit,
    /// Collapse - Collapses as content is scrolled
    Collapse,
}

/// Bottom App Bar layout
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BottomAppBarLayout {
    /// Standard - Action buttons and optional FAB
    #[default]
    Standard,
    /// With FAB - Includes a FAB cutout
    WithFab,
}

// ============================================================================
// Components
// ============================================================================

/// Top App Bar component
#[derive(Component)]
pub struct TopAppBar {
    /// Variant
    pub variant: TopAppBarVariant,
    /// Title text
    pub title: String,
    /// Navigation icon (usually menu or back)
    pub navigation_icon: Option<String>,
    /// Action icons (right side)
    pub actions: Vec<AppBarAction>,
    /// Scroll behavior
    pub scroll_behavior: TopAppBarScrollBehavior,
    /// Current scroll offset for collapse behavior
    pub scroll_offset: f32,
    /// Whether elevated (has shadow)
    pub elevated: bool,
}

/// An action button for the app bar
#[derive(Debug, Clone)]
pub struct AppBarAction {
    /// Icon name
    pub icon: String,
    /// Action identifier
    pub id: String,
    /// Whether disabled
    pub disabled: bool,
}

impl AppBarAction {
    /// Create a new action
    pub fn new(icon: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            id: id.into(),
            disabled: false,
        }
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl TopAppBar {
    /// Create a new top app bar
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            variant: TopAppBarVariant::default(),
            title: title.into(),
            navigation_icon: None,
            actions: Vec::new(),
            scroll_behavior: TopAppBarScrollBehavior::default(),
            scroll_offset: 0.0,
            elevated: false,
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: TopAppBarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the navigation icon
    pub fn with_navigation(mut self, icon: impl Into<String>) -> Self {
        self.navigation_icon = Some(icon.into());
        self
    }

    /// Add a back button
    pub fn with_back_button(self) -> Self {
        self.with_navigation("arrow_back")
    }

    /// Add a menu button
    pub fn with_menu_button(self) -> Self {
        self.with_navigation("menu")
    }

    /// Add an action
    pub fn add_action(mut self, action: AppBarAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Set scroll behavior
    pub fn with_scroll_behavior(mut self, behavior: TopAppBarScrollBehavior) -> Self {
        self.scroll_behavior = behavior;
        self
    }

    /// Set elevated
    pub fn elevated(mut self) -> Self {
        self.elevated = true;
        self
    }

    /// Get the height based on variant and scroll state
    pub fn height(&self) -> f32 {
        match self.variant {
            TopAppBarVariant::Small | TopAppBarVariant::CenterAligned => TOP_APP_BAR_HEIGHT_SMALL,
            TopAppBarVariant::Medium => {
                let collapsed = (self.scroll_offset / 100.0).clamp(0.0, 1.0);
                TOP_APP_BAR_HEIGHT_MEDIUM
                    - collapsed * (TOP_APP_BAR_HEIGHT_MEDIUM - TOP_APP_BAR_HEIGHT_SMALL)
            }
            TopAppBarVariant::Large => {
                let collapsed = (self.scroll_offset / 100.0).clamp(0.0, 1.0);
                TOP_APP_BAR_HEIGHT_LARGE
                    - collapsed * (TOP_APP_BAR_HEIGHT_LARGE - TOP_APP_BAR_HEIGHT_SMALL)
            }
        }
    }

    /// Get the background color
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.elevated {
            theme.surface_container
        } else {
            theme.surface
        }
    }

    /// Get the title color
    pub fn title_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_surface
    }
}

impl Default for TopAppBar {
    fn default() -> Self {
        Self::new("")
    }
}

/// Bottom App Bar component
#[derive(Component)]
pub struct BottomAppBar {
    /// Layout variant
    pub layout: BottomAppBarLayout,
    /// Action icons
    pub actions: Vec<AppBarAction>,
    /// Whether to show FAB
    pub has_fab: bool,
    /// FAB icon
    pub fab_icon: Option<String>,
    /// Whether elevated
    pub elevated: bool,
}

impl BottomAppBar {
    /// Create a new bottom app bar
    pub fn new() -> Self {
        Self {
            layout: BottomAppBarLayout::default(),
            actions: Vec::new(),
            has_fab: false,
            fab_icon: None,
            elevated: false,
        }
    }

    /// Add an action
    pub fn add_action(mut self, action: AppBarAction) -> Self {
        self.actions.push(action);
        self
    }

    /// Add a FAB
    pub fn with_fab(mut self, icon: impl Into<String>) -> Self {
        self.has_fab = true;
        self.fab_icon = Some(icon.into());
        self.layout = BottomAppBarLayout::WithFab;
        self
    }

    /// Set elevated
    pub fn elevated(mut self) -> Self {
        self.elevated = true;
        self
    }

    /// Get the background color
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.elevated {
            theme.surface_container
        } else {
            theme.surface_container_low
        }
    }
}

impl Default for BottomAppBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker for navigation button
#[derive(Component)]
pub struct AppBarNavigation;

/// Marker for action button with its ID
#[derive(Component)]
pub struct AppBarActionButton {
    pub id: String,
}

/// Marker for app bar title
#[derive(Component)]
pub struct AppBarTitle;

// ============================================================================
// Dimensions
// ============================================================================

/// Small top app bar height
pub const TOP_APP_BAR_HEIGHT_SMALL: f32 = 64.0;
/// Medium top app bar height
pub const TOP_APP_BAR_HEIGHT_MEDIUM: f32 = 112.0;
/// Large top app bar height
pub const TOP_APP_BAR_HEIGHT_LARGE: f32 = 152.0;
/// Bottom app bar height
pub const BOTTOM_APP_BAR_HEIGHT: f32 = 80.0;

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating Top App Bars
pub struct TopAppBarBuilder {
    app_bar: TopAppBar,
    title_key: Option<String>,
}

impl TopAppBarBuilder {
    /// Create a new builder
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            app_bar: TopAppBar::new(title),
            title_key: None,
        }
    }

    /// Set title from an i18n key.
    pub fn title_key(mut self, key: impl Into<String>) -> Self {
        self.app_bar.title = String::new();
        self.title_key = Some(key.into());
        self
    }

    /// Set small variant
    pub fn small(mut self) -> Self {
        self.app_bar.variant = TopAppBarVariant::Small;
        self
    }

    /// Set center-aligned variant
    pub fn center_aligned(mut self) -> Self {
        self.app_bar.variant = TopAppBarVariant::CenterAligned;
        self
    }

    /// Set medium variant
    pub fn medium(mut self) -> Self {
        self.app_bar.variant = TopAppBarVariant::Medium;
        self
    }

    /// Set large variant
    pub fn large(mut self) -> Self {
        self.app_bar.variant = TopAppBarVariant::Large;
        self
    }

    /// Add navigation (back button)
    pub fn with_back(mut self) -> Self {
        self.app_bar.navigation_icon = Some("←".to_string());
        self
    }

    /// Add navigation (menu button)
    pub fn with_menu(mut self) -> Self {
        self.app_bar.navigation_icon = Some("☰".to_string());
        self
    }

    /// Add custom navigation icon
    pub fn with_navigation(mut self, icon: impl Into<String>) -> Self {
        self.app_bar.navigation_icon = Some(icon.into());
        self
    }

    /// Add an action
    pub fn add_action(mut self, icon: impl Into<String>, id: impl Into<String>) -> Self {
        self.app_bar.actions.push(AppBarAction::new(icon, id));
        self
    }

    /// Set elevated
    pub fn elevated(mut self) -> Self {
        self.app_bar.elevated = true;
        self
    }

    /// Build the app bar bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let height = self.app_bar.height();
        let bg_color = self.app_bar.background_color(theme);

        (
            self.app_bar,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(height),
                padding: UiRect::axes(Val::Px(Spacing::EXTRA_SMALL), Val::Px(0.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(bg_color),
        )
    }
}

/// Builder for creating Bottom App Bars
pub struct BottomAppBarBuilder {
    app_bar: BottomAppBar,
}

impl BottomAppBarBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            app_bar: BottomAppBar::new(),
        }
    }

    /// Add an action
    pub fn add_action(mut self, icon: impl Into<String>, id: impl Into<String>) -> Self {
        self.app_bar.actions.push(AppBarAction::new(icon, id));
        self
    }

    /// Add a FAB
    pub fn with_fab(mut self, icon: impl Into<String>) -> Self {
        self.app_bar.has_fab = true;
        self.app_bar.fab_icon = Some(icon.into());
        self
    }

    /// Set elevated
    pub fn elevated(mut self) -> Self {
        self.app_bar.elevated = true;
        self
    }

    /// Build the app bar bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.app_bar.background_color(theme);

        (
            self.app_bar,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(BOTTOM_APP_BAR_HEIGHT),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            BackgroundColor(bg_color),
        )
    }
}

impl Default for BottomAppBarBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material app bars as children
pub trait SpawnAppBarChild {
    /// Spawn a top app bar
    fn spawn_top_app_bar(
        &mut self,
        theme: &MaterialTheme,
        title: impl Into<String>,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a top app bar with full builder control
    fn spawn_top_app_bar_with(
        &mut self,
        theme: &MaterialTheme,
        builder: TopAppBarBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a bottom app bar
    fn spawn_bottom_app_bar(
        &mut self,
        theme: &MaterialTheme,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a bottom app bar with full builder control
    fn spawn_bottom_app_bar_with(
        &mut self,
        theme: &MaterialTheme,
        builder: BottomAppBarBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );
}

/// Extension trait to spawn Top App Bars with a right-side custom content slot.
///
/// This keeps navigation/actions wired through `AppBarNavigationEvent` and `AppBarActionEvent`,
/// while letting callers inject extra widgets (e.g. a search box) into the right section.
pub trait SpawnTopAppBarWithRightContentChild {
    /// Spawn a top app bar and inject additional widgets into the right section *before* actions.
    fn spawn_top_app_bar_with_right_content(
        &mut self,
        theme: &MaterialTheme,
        builder: TopAppBarBuilder,
        with_right_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) -> Entity;
}

impl SpawnAppBarChild for ChildSpawnerCommands<'_> {
    fn spawn_top_app_bar(
        &mut self,
        theme: &MaterialTheme,
        title: impl Into<String>,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_top_app_bar_with(theme, TopAppBarBuilder::new(title), with_content);
    }

    fn spawn_top_app_bar_with(
        &mut self,
        theme: &MaterialTheme,
        builder: TopAppBarBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        let title_text = builder.app_bar.title.clone();
        let title_key = builder.title_key.clone();
        let title_color = builder.app_bar.title_color(theme);

        self.spawn(builder.build(theme)).with_children(|bar| {
            // Title
            if let Some(key) = title_key.as_deref() {
                bar.spawn((
                    Text::new(""),
                    LocalizedText::new(key),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(title_color),
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                ));
            } else {
                bar.spawn((
                    Text::new(&title_text),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(title_color),
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                ));
            }

            // Additional content
            with_content(bar);
        });
    }

    fn spawn_bottom_app_bar(
        &mut self,
        theme: &MaterialTheme,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_bottom_app_bar_with(theme, BottomAppBarBuilder::new(), with_content);
    }

    fn spawn_bottom_app_bar_with(
        &mut self,
        theme: &MaterialTheme,
        builder: BottomAppBarBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn(builder.build(theme)).with_children(with_content);
    }
}

impl SpawnTopAppBarWithRightContentChild for ChildSpawnerCommands<'_> {
    fn spawn_top_app_bar_with_right_content(
        &mut self,
        theme: &MaterialTheme,
        builder: TopAppBarBuilder,
        with_right_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) -> Entity {
        let title = builder.app_bar.title.clone();
        let title_key = builder.title_key.clone();
        let title_color = builder.app_bar.title_color(theme);
        let nav_icon = builder.app_bar.navigation_icon.clone();
        let actions = builder.app_bar.actions.clone();
        let variant = builder.app_bar.variant;

        self.spawn(builder.build(theme))
            .with_children(|parent| {
                // Left section (navigation + title for small)
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(Spacing::EXTRA_SMALL),
                        ..default()
                    })
                    .with_children(|left| {
                        // Navigation icon
                        if let Some(icon) = &nav_icon {
                            left.spawn((
                                AppBarNavigation,
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
                                BackgroundColor(Color::NONE),
                                BorderRadius::all(Val::Px(CornerRadius::FULL)),
                            ))
                            .with_children(|btn| {
                                if let Some(icon) = MaterialIcon::from_name(icon) {
                                    btn.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(theme.on_surface)
                                            .with_size(24.0),
                                    ));
                                }
                            });
                        }

                        // Title (for Small variant)
                        if variant == TopAppBarVariant::Small {
                            if let Some(key) = title_key.as_deref() {
                                left.spawn((
                                    AppBarTitle,
                                    Text::new(""),
                                    LocalizedText::new(key),
                                    TextFont {
                                        font_size: 22.0,
                                        ..default()
                                    },
                                    TextColor(title_color),
                                ));
                            } else {
                                left.spawn((
                                    AppBarTitle,
                                    Text::new(&title),
                                    TextFont {
                                        font_size: 22.0,
                                        ..default()
                                    },
                                    TextColor(title_color),
                                ));
                            }
                        }
                    });

                // Center section (title for center-aligned)
                if variant == TopAppBarVariant::CenterAligned {
                    if let Some(key) = title_key.as_deref() {
                        parent.spawn((
                            AppBarTitle,
                            Text::new(""),
                            LocalizedText::new(key),
                            TextFont {
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(title_color),
                            Node {
                                flex_grow: 1.0,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                        ));
                    } else {
                        parent.spawn((
                            AppBarTitle,
                            Text::new(&title),
                            TextFont {
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(title_color),
                            Node {
                                flex_grow: 1.0,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                        ));
                    }
                }

                // Right section (custom content + actions)
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(Spacing::EXTRA_SMALL),
                        ..default()
                    })
                    .with_children(|right| {
                        // Injected widgets come first (so actions stay right-most).
                        with_right_content(right);

                        for action in &actions {
                            right
                                .spawn((
                                    AppBarActionButton {
                                        id: action.id.clone(),
                                    },
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
                                    BackgroundColor(Color::NONE),
                                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                ))
                                .with_children(|btn| {
                                    if let Some(icon) = MaterialIcon::from_name(&action.icon) {
                                        btn.spawn((
                                            icon,
                                            IconStyle::outlined()
                                                .with_color(theme.on_surface_variant)
                                                .with_size(24.0),
                                        ));
                                    }
                                });
                        }
                    });
            })
            .id()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a top app bar with all children
pub fn spawn_top_app_bar(
    commands: &mut Commands,
    theme: &MaterialTheme,
    builder: TopAppBarBuilder,
) -> Entity {
    let title = builder.app_bar.title.clone();
    let title_color = builder.app_bar.title_color(theme);
    let nav_icon = builder.app_bar.navigation_icon.clone();
    let actions = builder.app_bar.actions.clone();
    let variant = builder.app_bar.variant;

    commands
        .spawn(builder.build(theme))
        .with_children(|parent| {
            // Left section (navigation + title for small/center)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(Spacing::EXTRA_SMALL),
                    ..default()
                })
                .with_children(|left| {
                    // Navigation icon
                    if let Some(icon) = &nav_icon {
                        left.spawn((
                            AppBarNavigation,
                            Button,
                            RippleHost::new(),
                            Node {
                                width: Val::Px(48.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                        ))
                        .with_children(|btn| {
                            if let Some(icon) = MaterialIcon::from_name(icon) {
                                btn.spawn((
                                    icon,
                                    IconStyle::outlined()
                                        .with_color(theme.on_surface)
                                        .with_size(24.0),
                                ));
                            }
                        });
                    }

                    // Title (for Small variant)
                    if variant == TopAppBarVariant::Small {
                        left.spawn((
                            AppBarTitle,
                            Text::new(&title),
                            TextFont {
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(title_color),
                        ));
                    }
                });

            // Center section (title for center-aligned)
            if variant == TopAppBarVariant::CenterAligned {
                parent.spawn((
                    AppBarTitle,
                    Text::new(&title),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(title_color),
                    Node {
                        flex_grow: 1.0,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ));
            }

            // Right section (actions)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(Spacing::EXTRA_SMALL),
                    ..default()
                })
                .with_children(|right| {
                    for action in &actions {
                        right
                            .spawn((
                                AppBarActionButton {
                                    id: action.id.clone(),
                                },
                                Button,
                                RippleHost::new(),
                                Node {
                                    width: Val::Px(48.0),
                                    height: Val::Px(48.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderRadius::all(Val::Px(CornerRadius::FULL)),
                            ))
                            .with_children(|btn| {
                                if let Some(icon) = MaterialIcon::from_name(&action.icon) {
                                    btn.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(theme.on_surface_variant)
                                            .with_size(24.0),
                                    ));
                                }
                            });
                    }
                });
        })
        .id()
}

/// Spawn a top app bar with a right-side custom content slot.
///
/// The injected widgets are spawned *before* the action buttons.
pub fn spawn_top_app_bar_with_right_content(
    commands: &mut Commands,
    theme: &MaterialTheme,
    builder: TopAppBarBuilder,
    with_right_content: impl FnOnce(&mut ChildSpawnerCommands),
) -> Entity {
    let title = builder.app_bar.title.clone();
    let title_color = builder.app_bar.title_color(theme);
    let nav_icon = builder.app_bar.navigation_icon.clone();
    let actions = builder.app_bar.actions.clone();
    let variant = builder.app_bar.variant;

    commands
        .spawn(builder.build(theme))
        .with_children(|parent| {
            // Left section (navigation + title for small/center)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(Spacing::EXTRA_SMALL),
                    ..default()
                })
                .with_children(|left| {
                    // Make the navigation area (icon + title) one large clickable target.
                    // This matches user expectations that clicking the title behaves like "Back",
                    // and it provides a clear hover target.
                    if nav_icon.is_some() {
                        left.spawn((
                            AppBarNavigation,
                            Button,
                            Interaction::None,
                            RippleHost::new(),
                            GlobalZIndex(1002),
                            Node {
                                height: Val::Px(48.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(8.0)),
                                column_gap: Val::Px(Spacing::EXTRA_SMALL),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                        ))
                        .with_children(|btn| {
                            if let Some(icon_name) = &nav_icon {
                                if let Some(icon) = MaterialIcon::from_name(icon_name) {
                                    btn.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(theme.on_surface)
                                            .with_size(24.0),
                                    ));
                                }
                            }

                            if variant == TopAppBarVariant::Small {
                                btn.spawn((
                                    AppBarTitle,
                                    Text::new(&title),
                                    TextFont {
                                        font_size: 22.0,
                                        ..default()
                                    },
                                    TextColor(title_color),
                                ));
                            }
                        });
                    } else if variant == TopAppBarVariant::Small {
                        // No navigation: just show the title.
                        left.spawn((
                            AppBarTitle,
                            Text::new(&title),
                            TextFont {
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(title_color),
                        ));
                    }
                });

            // Center section (title for center-aligned)
            if variant == TopAppBarVariant::CenterAligned {
                parent.spawn((
                    AppBarTitle,
                    Text::new(&title),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(title_color),
                    Node {
                        flex_grow: 1.0,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                ));
            }

            // Right section (custom content + actions)
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(Spacing::EXTRA_SMALL),
                    ..default()
                })
                .with_children(|right| {
                    with_right_content(right);

                    for action in &actions {
                        right
                            .spawn((
                                AppBarActionButton {
                                    id: action.id.clone(),
                                },
                                Button,
                                Interaction::None,
                                RippleHost::new(),
                                GlobalZIndex(1002),
                                Node {
                                    width: Val::Px(48.0),
                                    height: Val::Px(48.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderRadius::all(Val::Px(CornerRadius::FULL)),
                            ))
                            .with_children(|btn| {
                                if let Some(icon) = MaterialIcon::from_name(&action.icon) {
                                    btn.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(theme.on_surface_variant)
                                            .with_size(24.0),
                                    ));
                                }
                            });
                    }
                });
        })
        .id()
}

// ============================================================================
// Systems
// ============================================================================

/// System to handle scroll behavior for top app bars
fn top_app_bar_scroll_system(
    mut _app_bars: Query<(&mut TopAppBar, &mut Node)>,
    // In a real implementation, this would listen to scroll events
) {
    // Placeholder for scroll behavior implementation
    // This would track scroll position and update the app bar height/style accordingly
}

/// System to handle app bar interactions
fn app_bar_interaction_system(
    theme: Res<MaterialTheme>,
    nav_buttons: Query<(Entity, &Interaction), (Changed<Interaction>, With<AppBarNavigation>)>,
    action_buttons: Query<(Entity, &Interaction, &AppBarActionButton), Changed<Interaction>>,
    parents: Query<&ChildOf>,
    app_bars: Query<Entity, With<TopAppBar>>,
    mut bgs: Query<&mut BackgroundColor>,
    mut nav_events: MessageWriter<AppBarNavigationEvent>,
    mut action_events: MessageWriter<AppBarActionEvent>,
) {
    let find_app_bar_ancestor = |mut cursor: Entity| {
        for _ in 0..MAX_ANCESTOR_DEPTH {
            if app_bars.get(cursor).is_ok() {
                return Some(cursor);
            }
            if let Ok(parent) = parents.get(cursor) {
                cursor = parent.get();
            } else {
                break;
            }
        }
        None
    };

    // Handle navigation clicks
    for (entity, interaction) in nav_buttons.iter() {
        if let Ok(mut bg) = bgs.get_mut(entity) {
            *bg = match interaction {
                Interaction::Hovered | Interaction::Pressed => {
                    BackgroundColor(theme.surface_container_highest)
                }
                Interaction::None => BackgroundColor(Color::NONE),
            };
        }

        if *interaction == Interaction::Pressed {
            if let Some(app_bar) = find_app_bar_ancestor(entity) {
                nav_events.write(AppBarNavigationEvent { app_bar });
            }
        }
    }

    // Handle action clicks
    for (entity, interaction, action) in action_buttons.iter() {
        if let Ok(mut bg) = bgs.get_mut(entity) {
            *bg = match interaction {
                Interaction::Hovered | Interaction::Pressed => {
                    BackgroundColor(theme.surface_container_highest)
                }
                Interaction::None => BackgroundColor(Color::NONE),
            };
        }

        if *interaction == Interaction::Pressed {
            if let Some(app_bar) = find_app_bar_ancestor(entity) {
                action_events.write(AppBarActionEvent {
                    app_bar,
                    action: action.id.clone(),
                });
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_app_bar_creation() {
        let app_bar = TopAppBar::new("Title")
            .with_back_button()
            .add_action(AppBarAction::new("search", "search"));

        assert_eq!(app_bar.title, "Title");
        assert!(app_bar.navigation_icon.is_some());
        assert_eq!(app_bar.actions.len(), 1);
    }

    #[test]
    fn test_top_app_bar_variants() {
        let small = TopAppBar::new("Small").with_variant(TopAppBarVariant::Small);
        assert_eq!(small.height(), TOP_APP_BAR_HEIGHT_SMALL);

        let medium = TopAppBar::new("Medium").with_variant(TopAppBarVariant::Medium);
        assert_eq!(medium.height(), TOP_APP_BAR_HEIGHT_MEDIUM);

        let large = TopAppBar::new("Large").with_variant(TopAppBarVariant::Large);
        assert_eq!(large.height(), TOP_APP_BAR_HEIGHT_LARGE);
    }

    #[test]
    fn test_bottom_app_bar_creation() {
        let app_bar = BottomAppBar::new()
            .add_action(AppBarAction::new("home", "home"))
            .with_fab("add");

        assert_eq!(app_bar.actions.len(), 1);
        assert!(app_bar.has_fab);
        assert_eq!(app_bar.fab_icon, Some("add".to_string()));
    }

    #[test]
    fn test_app_bar_builder() {
        let builder = TopAppBarBuilder::new("My App")
            .small()
            .with_back()
            .add_action("⋮", "more");

        assert_eq!(builder.app_bar.title, "My App");
        assert_eq!(builder.app_bar.variant, TopAppBarVariant::Small);
    }
}
