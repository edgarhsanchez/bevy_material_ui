//! Material Design 3 Snackbar component
//!
//! Snackbars provide brief messages about app processes at the bottom of the screen.
//! They can contain an optional action.
//! This module leverages native `BoxShadow` for elevation shadows.
//!
//! Reference: <https://m3.material.io/components/snackbar/overview>

use bevy::picking::Pickable;
use bevy::prelude::*;

use crate::{
    elevation::Elevation,
    icons::{icon_by_name, IconStyle, MaterialIcon, ICON_CLOSE},
    motion::{ease_standard_accelerate, ease_standard_decelerate},
    theme::MaterialTheme,
    tokens::{CornerRadius, Duration, Spacing},
};

/// Plugin for the snackbar component
pub struct SnackbarPlugin;

impl Plugin for SnackbarPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<ShowSnackbar>()
            .add_message::<DismissSnackbar>()
            .add_message::<SnackbarActionEvent>()
            .init_resource::<SnackbarQueue>()
            .add_systems(
                Update,
                (
                    snackbar_queue_system,
                    snackbar_animation_system,
                    snackbar_timeout_system,
                    snackbar_action_system,
                    snackbar_close_system,
                    snackbar_close_button_style_system,
                    snackbar_cleanup_system,
                ),
            );
    }
}

// ============================================================================
// Types
// ============================================================================

/// Snackbar position on screen
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SnackbarPosition {
    /// Bottom center (default Material Design position)
    #[default]
    BottomCenter,
    /// Bottom left
    BottomLeft,
    /// Bottom right
    BottomRight,
    /// Top center
    TopCenter,
    /// Top left
    TopLeft,
    /// Top right
    TopRight,
}

// ============================================================================
// Events
// ============================================================================

/// Event to show a snackbar
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct ShowSnackbar {
    /// The message to display
    pub message: String,
    /// Optional action button text
    pub action: Option<String>,
    /// Duration to show (None = use default)
    pub duration: Option<f32>,
    /// Whether this snackbar can be dismissed by swiping
    pub dismissible: bool,
    /// Position on screen
    pub position: SnackbarPosition,
}

impl ShowSnackbar {
    /// Create a simple snackbar with just a message
    pub fn message(text: impl Into<String>) -> Self {
        Self {
            message: text.into(),
            action: None,
            duration: None,
            dismissible: true,
            position: SnackbarPosition::default(),
        }
    }

    /// Create a snackbar with an action button
    pub fn with_action(text: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            message: text.into(),
            action: Some(action.into()),
            duration: None,
            dismissible: true,
            position: SnackbarPosition::default(),
        }
    }

    /// Set the duration
    pub fn duration(mut self, seconds: f32) -> Self {
        self.duration = Some(seconds);
        self
    }

    /// Set whether dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set position
    pub fn position(mut self, position: SnackbarPosition) -> Self {
        self.position = position;
        self
    }

    /// Position at bottom left
    pub fn bottom_left(self) -> Self {
        self.position(SnackbarPosition::BottomLeft)
    }

    /// Position at bottom right
    pub fn bottom_right(self) -> Self {
        self.position(SnackbarPosition::BottomRight)
    }

    /// Position at top center
    pub fn top_center(self) -> Self {
        self.position(SnackbarPosition::TopCenter)
    }

    /// Position at top left
    pub fn top_left(self) -> Self {
        self.position(SnackbarPosition::TopLeft)
    }

    /// Position at top right
    pub fn top_right(self) -> Self {
        self.position(SnackbarPosition::TopRight)
    }
}

/// Event to dismiss the current snackbar
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct DismissSnackbar;

/// Event fired when a snackbar action is clicked
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct SnackbarActionEvent {
    /// The snackbar entity
    pub entity: Entity,
    /// The action text
    pub action: String,
}

// ============================================================================
// Resources
// ============================================================================

/// Queue of pending snackbars
#[derive(Resource, Default)]
pub struct SnackbarQueue {
    /// Queued snackbars waiting to be shown
    pub queue: Vec<ShowSnackbar>,
    /// Currently active snackbar entity
    pub active: Option<Entity>,
}

// ============================================================================
// Components
// ============================================================================

/// Snackbar container component
#[derive(Component)]
pub struct Snackbar {
    /// The message text
    pub message: String,
    /// Optional action text
    pub action: Option<String>,
    /// Duration to display (in seconds)
    pub duration: f32,
    /// Whether dismissible
    pub dismissible: bool,
    /// Position on screen
    pub position: SnackbarPosition,
    /// Current animation state
    pub animation_state: SnackbarAnimationState,
    /// Time remaining before auto-dismiss
    pub time_remaining: f32,
    /// Animation progress (0.0 = hidden, 1.0 = visible)
    pub animation_progress: f32,
}

/// Animation state for snackbar
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SnackbarAnimationState {
    /// Snackbar is entering
    #[default]
    Entering,
    /// Snackbar is visible
    Visible,
    /// Snackbar is exiting
    Exiting,
    /// Snackbar has been dismissed
    Dismissed,
}

impl Snackbar {
    /// Default duration for snackbars (4 seconds)
    pub const DEFAULT_DURATION: f32 = 4.0;
    /// Short duration (2 seconds)
    pub const SHORT_DURATION: f32 = 2.0;
    /// Long duration (10 seconds)
    pub const LONG_DURATION: f32 = 10.0;
    /// Indefinite duration (must be manually dismissed)
    pub const INDEFINITE: f32 = f32::MAX;

    /// Create a new snackbar from a ShowSnackbar event
    pub fn from_event(event: &ShowSnackbar) -> Self {
        Self {
            message: event.message.clone(),
            action: event.action.clone(),
            duration: event.duration.unwrap_or(Self::DEFAULT_DURATION),
            dismissible: event.dismissible,
            position: event.position,
            animation_state: SnackbarAnimationState::Entering,
            time_remaining: event.duration.unwrap_or(Self::DEFAULT_DURATION),
            animation_progress: 0.0,
        }
    }

    /// Start the exit animation
    pub fn dismiss(&mut self) {
        if self.animation_state != SnackbarAnimationState::Exiting {
            self.animation_state = SnackbarAnimationState::Exiting;
            self.animation_progress = 1.0;
        }
    }

    /// Check if the snackbar has been fully dismissed
    pub fn is_dismissed(&self) -> bool {
        self.animation_state == SnackbarAnimationState::Dismissed
    }
}

/// Marker for snackbar action button
#[derive(Component)]
pub struct SnackbarAction;

/// Marker for snackbar message text
#[derive(Component)]
pub struct SnackbarMessage;

/// Marker for snackbar close button
#[derive(Component)]
pub struct SnackbarCloseButton;

/// Snackbar host - container that holds snackbars
#[derive(Component)]
pub struct SnackbarHost;

/// Tracks the default position for snackbars in this host
#[derive(Component, Clone, Copy)]
pub struct SnackbarHostPosition(pub SnackbarPosition);

// ============================================================================
// Dimensions
// ============================================================================

/// Minimum width for snackbar
pub const SNACKBAR_MIN_WIDTH: f32 = 288.0;
/// Maximum width for snackbar
pub const SNACKBAR_MAX_WIDTH: f32 = 560.0;
/// Height for single-line snackbar
pub const SNACKBAR_HEIGHT_SINGLE: f32 = 48.0;
/// Height for two-line snackbar
pub const SNACKBAR_HEIGHT_DOUBLE: f32 = 68.0;
/// Bottom margin from screen edge
pub const SNACKBAR_MARGIN_BOTTOM: f32 = 16.0;

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating snackbar hosts
pub struct SnackbarHostBuilder;

impl SnackbarHostBuilder {
    /// Build the snackbar host - full screen overlay for positioning snackbars
    /// By default positions at bottom center
    pub fn build() -> impl Bundle {
        Self::build_with_position(SnackbarPosition::BottomCenter)
    }

    /// Build the snackbar host with a specific default position
    pub fn build_with_position(position: SnackbarPosition) -> impl Bundle {
        // For Column flex direction:
        // - justify_content controls vertical (main axis) - FlexEnd = bottom, FlexStart = top
        // - align_items controls horizontal (cross axis) - Center = centered, FlexStart = left, FlexEnd = right
        let (justify, align, flex_direction, padding) = match position {
            SnackbarPosition::BottomCenter => (
                JustifyContent::FlexEnd, // Bottom
                AlignItems::Center,      // Horizontally centered
                FlexDirection::Column,
                UiRect::bottom(Val::Px(SNACKBAR_MARGIN_BOTTOM)),
            ),
            SnackbarPosition::BottomLeft => (
                JustifyContent::FlexEnd, // Bottom
                AlignItems::FlexStart,   // Left
                FlexDirection::Column,
                UiRect::new(
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                    Val::Auto,
                    Val::Auto,
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                ),
            ),
            SnackbarPosition::BottomRight => (
                JustifyContent::FlexEnd, // Bottom
                AlignItems::FlexEnd,     // Right
                FlexDirection::Column,
                UiRect::new(
                    Val::Auto,
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                    Val::Auto,
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                ),
            ),
            SnackbarPosition::TopCenter => (
                JustifyContent::FlexStart, // Top
                AlignItems::Center,        // Horizontally centered
                FlexDirection::Column,
                UiRect::top(Val::Px(SNACKBAR_MARGIN_BOTTOM)),
            ),
            SnackbarPosition::TopLeft => (
                JustifyContent::FlexStart, // Top
                AlignItems::FlexStart,     // Left
                FlexDirection::Column,
                UiRect::new(
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                    Val::Auto,
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                    Val::Auto,
                ),
            ),
            SnackbarPosition::TopRight => (
                JustifyContent::FlexStart, // Top
                AlignItems::FlexEnd,       // Right
                FlexDirection::Column,
                UiRect::new(
                    Val::Auto,
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                    Val::Px(SNACKBAR_MARGIN_BOTTOM),
                    Val::Auto,
                ),
            ),
        };

        (
            SnackbarHost,
            SnackbarHostPosition(position),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                flex_direction,
                justify_content: justify,
                align_items: align,
                padding,
                ..default()
            },
            // Make it not block mouse events on the overlay itself
            Pickable::IGNORE,
            GlobalZIndex(999),
        )
    }
}

/// Builder for creating snackbars
pub struct SnackbarBuilder {
    snackbar: Snackbar,
}

impl SnackbarBuilder {
    /// Create a new snackbar builder
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            snackbar: Snackbar {
                message: message.into(),
                action: None,
                duration: Snackbar::DEFAULT_DURATION,
                dismissible: true,
                position: SnackbarPosition::default(),
                animation_state: SnackbarAnimationState::Entering,
                time_remaining: Snackbar::DEFAULT_DURATION,
                animation_progress: 0.0,
            },
        }
    }

    /// Add an action button
    pub fn action(mut self, text: impl Into<String>) -> Self {
        self.snackbar.action = Some(text.into());
        self
    }

    /// Set the duration
    pub fn duration(mut self, seconds: f32) -> Self {
        self.snackbar.duration = seconds;
        self.snackbar.time_remaining = seconds;
        self
    }

    /// Set short duration
    pub fn short(self) -> Self {
        self.duration(Snackbar::SHORT_DURATION)
    }

    /// Set long duration
    pub fn long(self) -> Self {
        self.duration(Snackbar::LONG_DURATION)
    }

    /// Set indefinite duration
    pub fn indefinite(self) -> Self {
        self.duration(Snackbar::INDEFINITE)
    }

    /// Build the snackbar bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = theme.inverse_surface;

        (
            self.snackbar,
            Node {
                min_width: Val::Px(SNACKBAR_MIN_WIDTH),
                max_width: Val::Px(SNACKBAR_MAX_WIDTH),
                min_height: Val::Px(SNACKBAR_HEIGHT_SINGLE),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::SMALL),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
            // Native Bevy 0.17 shadow support (MD3 snackbars are Level 3 elevation)
            Elevation::Level3.to_box_shadow(),
        )
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material snackbars as children
pub trait SpawnSnackbarChild {
    /// Spawn a snackbar host container
    fn spawn_snackbar_host(&mut self, position: SnackbarPosition);

    /// Spawn a snackbar with message
    fn spawn_snackbar_message(&mut self, theme: &MaterialTheme, message: impl Into<String>);

    /// Spawn a snackbar with message and action
    fn spawn_snackbar_with_action(
        &mut self,
        theme: &MaterialTheme,
        message: impl Into<String>,
        action: impl Into<String>,
    );

    /// Spawn a snackbar with full builder control
    fn spawn_snackbar_with(&mut self, theme: &MaterialTheme, builder: SnackbarBuilder);
}

impl SpawnSnackbarChild for ChildSpawnerCommands<'_> {
    fn spawn_snackbar_host(&mut self, position: SnackbarPosition) {
        self.spawn(SnackbarHostBuilder::build_with_position(position));
    }

    fn spawn_snackbar_message(&mut self, theme: &MaterialTheme, message: impl Into<String>) {
        let msg = message.into();
        self.spawn_snackbar_with(theme, SnackbarBuilder::new(msg));
    }

    fn spawn_snackbar_with_action(
        &mut self,
        theme: &MaterialTheme,
        message: impl Into<String>,
        action: impl Into<String>,
    ) {
        let msg = message.into();
        let act = action.into();
        self.spawn_snackbar_with(theme, SnackbarBuilder::new(msg).action(act));
    }

    fn spawn_snackbar_with(&mut self, theme: &MaterialTheme, builder: SnackbarBuilder) {
        let message_text = builder.snackbar.message.clone();
        let action_text = builder.snackbar.action.clone();
        let message_color = theme.inverse_on_surface;
        let action_color = theme.inverse_primary;
        let close_color = theme.inverse_on_surface;

        self.spawn(builder.build(theme)).with_children(|snackbar| {
            // Message
            snackbar.spawn((
                SnackbarMessage,
                Text::new(&message_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(message_color),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // Action button (if present)
            if let Some(ref action) = action_text {
                snackbar
                    .spawn((
                        SnackbarAction,
                        Button,
                        Node {
                            padding: UiRect::axes(
                                Val::Px(Spacing::SMALL),
                                Val::Px(Spacing::EXTRA_SMALL),
                            ),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(action),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(action_color),
                        ));
                    });
            }

            // Close button (X icon)
            snackbar
                .spawn((
                    SnackbarCloseButton,
                    Button,
                    Interaction::None,
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::left(Val::Px(Spacing::SMALL)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        MaterialIcon::new(
                            icon_by_name(ICON_CLOSE).expect("embedded icon 'close' not found"),
                        ),
                        IconStyle::outlined()
                            .with_color(close_color)
                            .with_size(24.0),
                    ));
                });
        });
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a snackbar entity as a child of the host
pub fn spawn_snackbar(
    commands: &mut Commands,
    theme: &MaterialTheme,
    event: &ShowSnackbar,
    host: Entity,
) -> Entity {
    let snackbar = Snackbar::from_event(event);
    let message = snackbar.message.clone();
    let action = snackbar.action.clone();

    let snackbar_entity = commands
        .spawn((
            snackbar,
            Node {
                min_width: Val::Px(SNACKBAR_MIN_WIDTH),
                max_width: Val::Px(SNACKBAR_MAX_WIDTH),
                min_height: Val::Px(SNACKBAR_HEIGHT_SINGLE),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::SMALL),
                ..default()
            },
            Transform::default(), // Required for animation system
            BackgroundColor(theme.inverse_surface),
            BorderRadius::all(Val::Px(CornerRadius::EXTRA_SMALL)),
            // Native Bevy 0.17 shadow support
            Elevation::Level3.to_box_shadow(),
            GlobalZIndex(1000), // Ensure snackbar is on top
        ))
        .with_children(|parent| {
            // Message text
            parent.spawn((
                SnackbarMessage,
                Text::new(&message),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.inverse_on_surface),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // Action button (if provided)
            if let Some(action_text) = &action {
                parent
                    .spawn((
                        SnackbarAction,
                        Button,
                        Node {
                            padding: UiRect::axes(
                                Val::Px(Spacing::SMALL),
                                Val::Px(Spacing::EXTRA_SMALL),
                            ),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(action_text),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(theme.inverse_primary),
                        ));
                    });
            }

            // Close button (X icon) - always shown for easy dismissal
            let inverse_on_surface = theme.inverse_on_surface;
            parent
                .spawn((
                    SnackbarCloseButton,
                    Button,
                    Interaction::None,
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::left(Val::Px(Spacing::SMALL)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                ))
                .with_children(move |btn| {
                    btn.spawn((
                        MaterialIcon::new(
                            icon_by_name(ICON_CLOSE).expect("embedded icon 'close' not found"),
                        ),
                        IconStyle::outlined()
                            .with_color(inverse_on_surface)
                            .with_size(24.0),
                    ));
                });
        })
        .id();

    // Make the snackbar a child of the host for proper z-ordering
    commands.entity(host).add_children(&[snackbar_entity]);

    snackbar_entity
}

fn snackbar_close_button_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SnackbarCloseButton>),
    >,
) {
    let Some(theme) = theme else { return };

    for (interaction, mut bg) in buttons.iter_mut() {
        let color = match *interaction {
            Interaction::Pressed => theme.inverse_on_surface.with_alpha(0.12),
            Interaction::Hovered => theme.inverse_on_surface.with_alpha(0.08),
            Interaction::None => Color::NONE,
        };
        *bg = BackgroundColor(color);
    }
}

// ============================================================================
// Systems
// ============================================================================

/// System to process the snackbar queue
fn snackbar_queue_system(
    mut commands: Commands,
    mut events: MessageReader<ShowSnackbar>,
    theme: Option<Res<MaterialTheme>>,
    mut queue: ResMut<SnackbarQueue>,
    mut hosts: Query<(Entity, &mut Node, &mut SnackbarHostPosition), With<SnackbarHost>>,
    snackbars: Query<&Snackbar>,
) {
    let Some(theme) = theme else { return };

    // If the previously active snackbar entity was despawned (for any reason),
    // don't let the queue get stuck forever.
    if let Some(active) = queue.active {
        if snackbars.get(active).is_err() {
            queue.active = None;
        }
    }

    // Add new events to the queue
    for event in events.read() {
        queue.queue.push(event.clone());
    }

    // Check if we can show a snackbar
    let can_show = match queue.active {
        Some(entity) => {
            // Check if active snackbar is dismissed
            snackbars.get(entity).is_ok_and(|s| s.is_dismissed())
        }
        None => true,
    };

    // Show next snackbar if queue has items and we can show
    if can_show && !queue.queue.is_empty() {
        if let Some(event) = queue.queue.first().cloned() {
            if let Some((host, mut host_node, mut host_pos)) = hosts.iter_mut().next() {
                // Update host layout if position changed
                if host_pos.0 != event.position {
                    host_pos.0 = event.position;
                    // For Column flex direction:
                    // - justify_content controls vertical (main axis) - FlexEnd = bottom, FlexStart = top
                    // - align_items controls horizontal (cross axis) - Center = centered, FlexStart = left, FlexEnd = right
                    let (justify, align, flex_direction, padding) = match event.position {
                        SnackbarPosition::BottomCenter => (
                            JustifyContent::FlexEnd, // Bottom
                            AlignItems::Center,      // Horizontally centered
                            FlexDirection::Column,
                            UiRect::bottom(Val::Px(SNACKBAR_MARGIN_BOTTOM)),
                        ),
                        SnackbarPosition::BottomLeft => (
                            JustifyContent::FlexEnd, // Bottom
                            AlignItems::FlexStart,   // Left
                            FlexDirection::Column,
                            UiRect::new(
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                                Val::Auto,
                                Val::Auto,
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                            ),
                        ),
                        SnackbarPosition::BottomRight => (
                            JustifyContent::FlexEnd, // Bottom
                            AlignItems::FlexEnd,     // Right
                            FlexDirection::Column,
                            UiRect::new(
                                Val::Auto,
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                                Val::Auto,
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                            ),
                        ),
                        SnackbarPosition::TopCenter => (
                            JustifyContent::FlexStart, // Top
                            AlignItems::Center,        // Horizontally centered
                            FlexDirection::Column,
                            UiRect::top(Val::Px(SNACKBAR_MARGIN_BOTTOM)),
                        ),
                        SnackbarPosition::TopLeft => (
                            JustifyContent::FlexStart, // Top
                            AlignItems::FlexStart,     // Left
                            FlexDirection::Column,
                            UiRect::new(
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                                Val::Auto,
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                                Val::Auto,
                            ),
                        ),
                        SnackbarPosition::TopRight => (
                            JustifyContent::FlexStart, // Top
                            AlignItems::FlexEnd,       // Right
                            FlexDirection::Column,
                            UiRect::new(
                                Val::Auto,
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                                Val::Px(SNACKBAR_MARGIN_BOTTOM),
                                Val::Auto,
                            ),
                        ),
                    };
                    host_node.justify_content = justify;
                    host_node.align_items = align;
                    host_node.flex_direction = flex_direction;
                    host_node.padding = padding;
                }

                let entity = spawn_snackbar(&mut commands, &theme, &event, host);
                queue.active = Some(entity);
                queue.queue.remove(0);
            }
        }
    }
}

/// System to animate snackbars using transform for slide animation
fn snackbar_animation_system(
    time: Res<Time>,
    mut snackbars: Query<(&mut Snackbar, &mut Transform)>,
) {
    for (mut snackbar, mut transform) in snackbars.iter_mut() {
        let dt = time.delta_secs();

        match snackbar.animation_state {
            SnackbarAnimationState::Entering => {
                snackbar.animation_progress += dt / Duration::MEDIUM2;
                if snackbar.animation_progress >= 1.0 {
                    snackbar.animation_progress = 1.0;
                    snackbar.animation_state = SnackbarAnimationState::Visible;
                }

                // Slide animation using transform
                let progress = ease_standard_decelerate(snackbar.animation_progress);
                let offset = (1.0 - progress) * (SNACKBAR_HEIGHT_SINGLE + SNACKBAR_MARGIN_BOTTOM);
                // Positive Y moves down in UI coordinates, so we use positive offset for bottom snackbars
                transform.translation.y = -offset;
            }
            SnackbarAnimationState::Visible => {
                // Ensure fully visible
                transform.translation.y = 0.0;
            }
            SnackbarAnimationState::Exiting => {
                snackbar.animation_progress -= dt / Duration::MEDIUM2;
                if snackbar.animation_progress <= 0.0 {
                    snackbar.animation_progress = 0.0;
                    snackbar.animation_state = SnackbarAnimationState::Dismissed;
                }

                // Slide animation using transform
                let progress = ease_standard_accelerate(snackbar.animation_progress);
                let offset = (1.0 - progress) * (SNACKBAR_HEIGHT_SINGLE + SNACKBAR_MARGIN_BOTTOM);
                transform.translation.y = -offset;
            }
            SnackbarAnimationState::Dismissed => {
                // Will be cleaned up
            }
        }
    }
}

/// System to handle snackbar timeout
fn snackbar_timeout_system(
    time: Res<Time>,
    mut snackbars: Query<&mut Snackbar>,
    mut queue: ResMut<SnackbarQueue>,
) {
    for mut snackbar in snackbars.iter_mut() {
        if snackbar.animation_state == SnackbarAnimationState::Visible {
            snackbar.time_remaining -= time.delta_secs();

            if snackbar.time_remaining <= 0.0 {
                snackbar.dismiss();
            }
        }
    }

    // If the active snackbar is dismissed or was despawned, clear the gate.
    if let Some(entity) = queue.active {
        match snackbars.get(entity) {
            Ok(snackbar) => {
                if snackbar.is_dismissed() {
                    queue.active = None;
                }
            }
            Err(_) => {
                queue.active = None;
            }
        }
    }
}

/// System to despawn snackbars that have completed their exit animation
fn snackbar_cleanup_system(
    mut commands: Commands,
    mut queue: ResMut<SnackbarQueue>,
    snackbars: Query<(Entity, &Snackbar)>,
) {
    for (entity, snackbar) in snackbars.iter() {
        if snackbar.is_dismissed() {
            if queue.active == Some(entity) {
                queue.active = None;
            }
            // In Bevy 0.17, despawn() removes the entity and all children via ChildOf relationship
            commands.entity(entity).despawn();
        }
    }
}

/// System to handle snackbar action clicks
fn snackbar_action_system(
    interactions: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<SnackbarAction>)>,
    mut snackbars: Query<(Entity, &mut Snackbar)>,
    mut events: MessageWriter<SnackbarActionEvent>,
) {
    for (interaction, parent) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((entity, mut snackbar)) = snackbars.get_mut(parent.parent()) {
                if let Some(action) = &snackbar.action {
                    events.write(SnackbarActionEvent {
                        entity,
                        action: action.clone(),
                    });
                }
                snackbar.dismiss();
            }
        }
    }
}

/// System to handle snackbar close button clicks
fn snackbar_close_system(
    interactions: Query<
        (&Interaction, &ChildOf),
        (Changed<Interaction>, With<SnackbarCloseButton>),
    >,
    mut snackbars: Query<&mut Snackbar>,
) {
    for (interaction, child_of) in interactions.iter() {
        #[cfg(debug_assertions)]
        bevy::log::debug!("Snackbar close button interaction: {:?}", interaction);

        if *interaction == Interaction::Pressed {
            let parent_entity = child_of.parent();
            #[cfg(debug_assertions)]
            bevy::log::debug!(
                "Snackbar close button pressed, looking for parent: {:?}",
                parent_entity
            );

            if let Ok(mut snackbar) = snackbars.get_mut(parent_entity) {
                #[cfg(debug_assertions)]
                bevy::log::info!("Dismissing snackbar via close button");
                snackbar.dismiss();
            } else {
                #[cfg(debug_assertions)]
                bevy::log::warn!("Could not find snackbar parent entity: {:?}", parent_entity);
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
    fn test_snackbar_creation() {
        let snackbar = Snackbar::from_event(&ShowSnackbar::message("Test"));
        assert_eq!(snackbar.message, "Test");
        assert!(snackbar.action.is_none());
        assert!((snackbar.duration - Snackbar::DEFAULT_DURATION).abs() < 0.001);
    }

    #[test]
    fn test_snackbar_with_action() {
        let snackbar = Snackbar::from_event(&ShowSnackbar::with_action("Error", "Retry"));
        assert_eq!(snackbar.message, "Error");
        assert_eq!(snackbar.action, Some("Retry".to_string()));
    }

    #[test]
    fn test_snackbar_dismiss() {
        let mut snackbar = Snackbar::from_event(&ShowSnackbar::message("Test"));
        assert_eq!(snackbar.animation_state, SnackbarAnimationState::Entering);

        snackbar.dismiss();
        assert_eq!(snackbar.animation_state, SnackbarAnimationState::Exiting);
    }

    #[test]
    fn test_show_snackbar_builder() {
        let event = ShowSnackbar::message("Hello")
            .duration(5.0)
            .dismissible(false);

        assert_eq!(event.message, "Hello");
        assert_eq!(event.duration, Some(5.0));
        assert!(!event.dismissible);
    }

    #[test]
    fn test_snackbar_close_button_marker() {
        // Verify SnackbarCloseButton can be created as a marker component
        let _close_button = SnackbarCloseButton;
        // The marker is used to identify close buttons in the UI hierarchy
    }

    #[test]
    fn test_snackbar_double_dismiss() {
        // Test that dismissing twice doesn't cause issues
        let mut snackbar = Snackbar::from_event(&ShowSnackbar::message("Test"));
        snackbar.dismiss();
        assert_eq!(snackbar.animation_state, SnackbarAnimationState::Exiting);

        // Second dismiss should not change state
        snackbar.dismiss();
        assert_eq!(snackbar.animation_state, SnackbarAnimationState::Exiting);
    }
}
