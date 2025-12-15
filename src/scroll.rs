//! Scrollable container component for Material Design 3
//!
//! Provides mouse wheel scrolling and scrollbar visuals using Bevy's native scroll system.
//! Uses Bevy's `ScrollPosition` component and `Overflow::scroll_y()` for actual scrolling.
//!
//! Usage:
//! ```ignore
//! commands.spawn((
//!     ScrollContainer::vertical(),
//!     ScrollPosition::default(),
//!     Node { 
//!         height: Val::Px(400.0), 
//!         overflow: Overflow::scroll_y(), // Use Bevy's native scroll
//!         ..default() 
//!     },
//! )).with_children(|parent| {
//!     // Important: `ScrollContent` is the *scroll wrapper* node (created automatically
//!     // by the plugin if you don't spawn it). Your actual content should be children
//!     // of that wrapper, not the wrapper node itself.
//!     parent.spawn(Node { ..default() })
//!         .with_children(|content| {
//!             // Your scrollable content here
//!         });
//! });
//! ```

use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::picking::Pickable;

use std::collections::HashSet;

use crate::theme::MaterialTheme;
use crate::telemetry::{InsertTestIdIfExists, TestId};

/// Plugin for scroll container functionality
pub struct ScrollPlugin;

impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ensure_scroll_content_wrapper_system,
                ensure_scrollbars_system,
                assign_scrollbar_test_ids_system,
                sync_scroll_state_system,
                mouse_wheel_scroll_system,
                scrollbar_thumb_drag_system,
                sync_scroll_position_to_content_system,
                update_scrollbars,
            )
                .chain(),
        );
    }
}

fn ensure_scrollbars_system(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    containers: Query<(Entity, &ScrollContainer, Option<&Children>), With<ScrollContainer>>,
    track_v: Query<(), With<ScrollbarTrackVertical>>,
    track_h: Query<(), With<ScrollbarTrackHorizontal>>,
) {
    for (entity, container, children) in containers.iter() {
        let (mut has_v, mut has_h) = (false, false);
        let mut existing_tracks_v: Vec<Entity> = Vec::new();
        let mut existing_tracks_h: Vec<Entity> = Vec::new();

        if let Some(children) = children {
            for child in children.iter() {
                if track_v.get(child).is_ok() {
                    has_v = true;
                    existing_tracks_v.push(child);
                }
                if track_h.get(child).is_ok() {
                    has_h = true;
                    existing_tracks_h.push(child);
                }
            }
        }

        let wants_v = container.show_scrollbars
            && matches!(container.direction, ScrollDirection::Vertical | ScrollDirection::Both);
        let wants_h = container.show_scrollbars
            && matches!(container.direction, ScrollDirection::Horizontal | ScrollDirection::Both);

        // Spawn missing scrollbars.
        if wants_v || wants_h {
            // If both are desired, spawn with correct corner reservations.
            if matches!(container.direction, ScrollDirection::Both) {
                if !has_v {
                    commands.entity(entity).with_children(|c| {
                        spawn_scrollbar_vertical(c, &theme, true);
                    });
                }
                if !has_h {
                    commands.entity(entity).with_children(|c| {
                        spawn_scrollbar_horizontal(c, &theme, true);
                    });
                }
            } else if wants_v && !has_v {
                commands.entity(entity).with_children(|c| {
                    spawn_scrollbar_vertical(c, &theme, false);
                });
            } else if wants_h && !has_h {
                commands.entity(entity).with_children(|c| {
                    spawn_scrollbar_horizontal(c, &theme, false);
                });
            }
        }

        // Toggle visibility based on current container settings.
        // We do this via commands so we don't require Visibility to already exist.
        for track in existing_tracks_v {
            commands
                .entity(track)
                .insert(if wants_v { Visibility::Visible } else { Visibility::Hidden });
        }
        for track in existing_tracks_h {
            commands
                .entity(track)
                .insert(if wants_h { Visibility::Visible } else { Visibility::Hidden });
        }
    }
}

fn assign_scrollbar_test_ids_system(
    mut commands: Commands,
    parents: Query<&ChildOf>,
    test_ids: Query<&TestId>,
    tracks_v: Query<Entity, (With<ScrollbarTrackVertical>, Without<TestId>)>,
    thumbs_v: Query<Entity, (With<ScrollbarThumbVertical>, Without<TestId>)>,
    tracks_h: Query<Entity, (With<ScrollbarTrackHorizontal>, Without<TestId>)>,
    thumbs_h: Query<Entity, (With<ScrollbarThumbHorizontal>, Without<TestId>)>,
) {
    fn ancestor_test_id(
        start: Entity,
        parents: &Query<&ChildOf>,
        test_ids: &Query<&TestId>,
    ) -> Option<String> {
        let mut current = Some(start);
        for _ in 0..32 {
            let Some(entity) = current else { break };
            if let Ok(id) = test_ids.get(entity) {
                return Some(id.id().to_string());
            }
            current = parents.get(entity).ok().map(|p| p.0);
        }
        None
    }

    for entity in tracks_v.iter() {
        if let Some(prefix) = ancestor_test_id(entity, &parents, &test_ids) {
            commands.queue(InsertTestIdIfExists {
                entity,
                id: format!("{prefix}_scroll_track_v"),
            });
        }
    }

    for entity in thumbs_v.iter() {
        if let Some(prefix) = ancestor_test_id(entity, &parents, &test_ids) {
            commands.queue(InsertTestIdIfExists {
                entity,
                id: format!("{prefix}_scroll_thumb_v"),
            });
        }
    }

    for entity in tracks_h.iter() {
        if let Some(prefix) = ancestor_test_id(entity, &parents, &test_ids) {
            commands.queue(InsertTestIdIfExists {
                entity,
                id: format!("{prefix}_scroll_track_h"),
            });
        }
    }

    for entity in thumbs_h.iter() {
        if let Some(prefix) = ancestor_test_id(entity, &parents, &test_ids) {
            commands.queue(InsertTestIdIfExists {
                entity,
                id: format!("{prefix}_scroll_thumb_h"),
            });
        }
    }
}

/// Ensures each `ScrollContainer` has a `ScrollContent` child that is the actual Bevy scroll node.
/// This prevents overlay UI (scrollbar rails/thumb) from moving with scrolled content.
fn ensure_scroll_content_wrapper_system(
    mut commands: Commands,
    containers: Query<
        (Entity, &Children, &Node, &ScrollContainer, Option<&ScrollPosition>),
        With<ScrollContainer>,
    >,
    is_scroll_content: Query<(), With<ScrollContent>>,
    is_scrollbar_part: Query<
        (),
        Or<(
            With<ScrollbarTrackVertical>,
            With<ScrollbarTrackHorizontal>,
            With<ScrollbarThumbVertical>,
            With<ScrollbarThumbHorizontal>,
        )>,
    >,
) {
    for (container_entity, children, node, container, scroll_pos) in containers.iter() {
        let has_scroll_content_child = children
            .iter()
            .any(|child| is_scroll_content.get(child).is_ok());
        if has_scroll_content_child {
            continue;
        }

        // Reserve space so overlay scrollbars do not overlap content (e.g. list items).
        // This matches the thickness used by `spawn_scrollbars`.
        let reserve_right = if container.show_scrollbars
            && matches!(container.direction, ScrollDirection::Vertical | ScrollDirection::Both)
        {
            SCROLLBAR_THICKNESS
        } else {
            0.0
        };
        let reserve_bottom = if container.show_scrollbars
            && matches!(container.direction, ScrollDirection::Horizontal | ScrollDirection::Both)
        {
            SCROLLBAR_THICKNESS
        } else {
            0.0
        };

        let content_width = match node.width {
            Val::Auto => Val::Auto,
            _ => Val::Percent(100.0),
        };
        let content_height = match node.height {
            Val::Auto => Val::Auto,
            _ => Val::Percent(100.0),
        };

        fn add_px(base: Val, extra: f32) -> Val {
            if extra == 0.0 {
                return base;
            }
            match base {
                Val::Px(v) => Val::Px(v + extra),
                Val::Auto => Val::Px(extra),
                other => other,
            }
        }

        // Create the wrapper that will actually scroll its children.
        //
        // Important: if the container uses `height: Auto` (common for lists that should size
        // themselves to their children), forcing `height: 100%` on the scroll node can collapse
        // it to 0 because the parent's computed height isn't definite.
        let mut content_node = node.clone();
        content_node.width = content_width;
        content_node.height = content_height;
        content_node.margin = UiRect::default();
        content_node.border = UiRect::default();
        // Important for scroll containers inside flex columns:
        // allow shrinking so overflow/scrolling can happen.
        content_node.min_height = Val::Px(0.0);
        content_node.overflow = node.overflow;
        content_node.padding = UiRect {
            right: add_px(node.padding.right, reserve_right),
            bottom: add_px(node.padding.bottom, reserve_bottom),
            ..node.padding
        };

        let initial_scroll = scroll_pos
            .map(|p| ScrollPosition(**p))
            .unwrap_or_default();
        let content_entity = commands
            .spawn((
                ScrollContent,
                initial_scroll,
                content_node,
                // The scrollable content fills the container; don't let it block pointer
                // interaction with overlay UI (scrollbar tracks/thumb).
                Pickable::IGNORE,
            ))
            .id();

        // Attach the ScrollContent wrapper under the container, but do it safely.
        // The container may be despawned in the same frame (e.g. showcase view rebuild).
        commands.queue(move |world: &mut World| {
            if world.get_entity(container_entity).is_ok() {
                if let Ok(mut content) = world.get_entity_mut(content_entity) {
                    content.insert(ChildOf(container_entity));
                }
            }
        });

        // Move all non-scrollbar children into the ScrollContent wrapper.
        //
        // Important: also guard on the container still existing at apply time.
        // If the container is despawned (e.g. view rebuild) and we reparent anyway,
        // we'd effectively detach the child subtree under an unparented wrapper,
        // making it disappear.
        for child in children.iter() {
            if is_scrollbar_part.get(child).is_ok() {
                continue;
            }
            if is_scroll_content.get(child).is_ok() {
                continue;
            }

            // Use a deferred command to safely reparent the child, handling the case where
            // the child entity may have been despawned in the meantime.
            commands.queue(move |world: &mut World| {
                if world.get_entity(container_entity).is_err() {
                    return;
                }
                if world.get_entity(content_entity).is_err() {
                    return;
                }
                if let Ok(mut child_entity) = world.get_entity_mut(child) {
                    child_entity.insert(ChildOf(content_entity));
                }
            });
        }

        let mut container_node = node.clone();
        // The container becomes a clip/overlay host; padding must live on the ScrollContent node
        // so it applies to the scrolled children.
        container_node.padding = UiRect::all(Val::Px(0.0));
        container_node.overflow = Overflow::clip();
        
        // The container itself should clip; the ScrollContent child scrolls.
        // (We intentionally do not try to preserve scroll_y vs scroll_x here;
        // the child inherited the original overflow, and the parent just clips.)
        commands.queue(move |world: &mut World| {
            if let Ok(mut container) = world.get_entity_mut(container_entity) {
                container.insert(container_node);
            }
        });
    }
}

/// Keep the scroll offset in sync between the public `ScrollPosition` on the `ScrollContainer`
/// and the internal `ScrollContent` scroll node.
fn sync_scroll_position_to_content_system(
    containers: Query<(&ScrollPosition, &Children), (With<ScrollContainer>, Without<ScrollContent>)>,
    mut content_scroll: Query<&mut ScrollPosition, (With<ScrollContent>, Without<ScrollContainer>)>,
) {
    for (container_scroll, children) in containers.iter() {
        for child in children.iter() {
            if let Ok(mut scroll) = content_scroll.get_mut(child) {
                **scroll = **container_scroll;
                break;
            }
        }
    }
}

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ScrollDirection {
    /// Vertical scrolling only
    #[default]
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both directions
    Both,
}

/// Scroll container component
#[derive(Component)]
pub struct ScrollContainer {
    /// Scroll direction
    pub direction: ScrollDirection,
    /// Current scroll offset (pixels)
    pub offset: Vec2,
    /// Target scroll offset for smooth scrolling
    pub target_offset: Vec2,
    /// Maximum scroll offset
    pub max_offset: Vec2,
    /// Content size
    pub content_size: Vec2,
    /// Container size
    pub container_size: Vec2,
    /// Scroll sensitivity (pixels per scroll unit)
    pub sensitivity: f32,
    /// Whether smooth scrolling is enabled
    pub smooth: bool,
    /// Smooth scrolling speed (0.0-1.0, higher = faster)
    pub smooth_speed: f32,
    /// Whether the container is currently being dragged
    pub dragging: bool,
    /// Last drag position
    pub last_drag_pos: Option<Vec2>,
    /// Whether to show scrollbars
    pub show_scrollbars: bool,
    /// Scrollbar width
    pub scrollbar_width: f32,
}

impl Default for ScrollContainer {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            offset: Vec2::ZERO,
            target_offset: Vec2::ZERO,
            max_offset: Vec2::ZERO,
            content_size: Vec2::ZERO,
            container_size: Vec2::ZERO,
            sensitivity: 40.0,
            smooth: true,
            smooth_speed: 0.2,
            dragging: false,
            last_drag_pos: None,
            show_scrollbars: true,
            scrollbar_width: 8.0,
        }
    }
}

impl ScrollContainer {
    /// Create a new vertical scroll container
    pub fn vertical() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            ..default()
        }
    }

    /// Create a new horizontal scroll container
    pub fn horizontal() -> Self {
        Self {
            direction: ScrollDirection::Horizontal,
            ..default()
        }
    }

    /// Create a scroll container that scrolls in both directions
    pub fn both() -> Self {
        Self {
            direction: ScrollDirection::Both,
            ..default()
        }
    }

    /// Set scroll sensitivity
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Enable or disable smooth scrolling
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    /// Show or hide scrollbars
    pub fn with_scrollbars(mut self, show: bool) -> Self {
        self.show_scrollbars = show;
        self
    }

    /// Set scrollbar width
    pub fn with_scrollbar_width(mut self, width: f32) -> Self {
        self.scrollbar_width = width;
        self
    }

    /// Scroll by a delta amount
    pub fn scroll_by(&mut self, delta: Vec2) {
        match self.direction {
            ScrollDirection::Vertical => {
                self.target_offset.y += delta.y;
            }
            ScrollDirection::Horizontal => {
                self.target_offset.x += delta.x;
            }
            ScrollDirection::Both => {
                self.target_offset += delta;
            }
        }
    }

    /// Check if scrolling is needed in x direction
    pub fn needs_scroll_x(&self) -> bool {
        self.max_offset.x > 0.0 && matches!(self.direction, ScrollDirection::Horizontal | ScrollDirection::Both)
    }

    /// Check if scrolling is needed in y direction
    pub fn needs_scroll_y(&self) -> bool {
        self.max_offset.y > 0.0 && matches!(self.direction, ScrollDirection::Vertical | ScrollDirection::Both)
    }

    /// Get scrollbar thumb size for vertical scrollbar
    pub fn vertical_thumb_size(&self) -> f32 {
        if self.content_size.y <= 0.0 || self.container_size.y <= 0.0 {
            return 0.0;
        }
        let ratio = self.container_size.y / self.content_size.y;
        (self.container_size.y * ratio).max(30.0).min(self.container_size.y)
    }

    /// Get scrollbar thumb position for vertical scrollbar (0.0 to 1.0)
    pub fn vertical_thumb_position(&self) -> f32 {
        if self.max_offset.y <= 0.0 {
            return 0.0;
        }
        self.offset.y / self.max_offset.y
    }

    /// Get scrollbar thumb size for horizontal scrollbar
    pub fn horizontal_thumb_size(&self) -> f32 {
        if self.content_size.x <= 0.0 || self.container_size.x <= 0.0 {
            return 0.0;
        }
        let ratio = self.container_size.x / self.content_size.x;
        (self.container_size.x * ratio).max(30.0).min(self.container_size.x)
    }

    /// Get scrollbar thumb position for horizontal scrollbar (0.0 to 1.0)
    pub fn horizontal_thumb_position(&self) -> f32 {
        if self.max_offset.x <= 0.0 {
            return 0.0;
        }
        self.offset.x / self.max_offset.x
    }
}

/// Marker component for scroll content (the inner scrollable element)
#[derive(Component, Default)]
pub struct ScrollContent;

/// Marker for vertical scrollbar track
#[derive(Component)]
pub struct ScrollbarTrackVertical;

/// Marker for vertical scrollbar thumb
#[derive(Component)]
pub struct ScrollbarThumbVertical;

/// Marker for horizontal scrollbar track
#[derive(Component)]
pub struct ScrollbarTrackHorizontal;

/// Marker for horizontal scrollbar thumb
#[derive(Component)]
pub struct ScrollbarThumbHorizontal;

/// Component to track scrollbar thumb dragging state
#[derive(Component, Default)]
pub struct ScrollbarDragging {
    /// Whether the thumb is being dragged
    pub is_dragging: bool,
    /// Starting mouse position when drag began
    pub drag_start_pos: Option<Vec2>,
    /// Starting scroll offset when drag began
    pub drag_start_offset: f32,
}

/// Line height for scroll calculations
const LINE_HEIGHT: f32 = 21.0;

/// Thickness (in logical px) of the visual scrollbars spawned by `spawn_scrollbars`.
/// Also used to reserve space in `ScrollContent` so scrollbars do not overlap content.
const SCROLLBAR_THICKNESS: f32 = 10.0;

/// System to handle mouse wheel scrolling
/// This follows Bevy's pattern: read mouse wheel, find hovered entities, update their ScrollPosition
#[allow(deprecated)] // EventReader renamed to MessageReader in Bevy 0.17
fn mouse_wheel_scroll_system(
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    parents: Query<&ChildOf>,
    mut scrollable_query: Query<(&mut ScrollPosition, &ScrollContainer), With<ScrollContainer>>,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        // Calculate scroll delta.
        // - Mouse wheels typically report `Line` deltas with the sign opposite of the desired
        //   scroll offset direction (wheel down should increase scroll offset), so we negate.
        // - Touchpads often report `Pixel` deltas already aligned with the user's gesture, so we
        //   keep the sign as-is to avoid inverted scrolling.
        let mut delta = match mouse_wheel.unit {
            MouseScrollUnit::Line => Vec2::new(-mouse_wheel.x, -mouse_wheel.y),
            MouseScrollUnit::Pixel => Vec2::new(mouse_wheel.x, mouse_wheel.y),
        };
        
        // Convert line units to pixels
        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }
        
        // Shift key swaps x/y for horizontal scrolling
        if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }
        
        // Find entities under the cursor and scroll their nearest ScrollContainer ancestor.
        // (This allows scrolling when hovering list items/content, not just the container root.)
        let mut scrolled_containers: HashSet<Entity> = HashSet::new();
        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                // Walk up the parent hierarchy until we find a ScrollContainer.
                let mut current = Some(entity);
                let mut container_entity = None;
                for _ in 0..32 {
                    let Some(e) = current else { break };
                    if scrollable_query.get(e).is_ok() {
                        container_entity = Some(e);
                        break;
                    }
                    current = parents.get(e).ok().map(|p| p.0);
                }

                let Some(container_entity) = container_entity else { continue };
                if !scrolled_containers.insert(container_entity) {
                    continue;
                }

                if let Ok((mut scroll_position, container)) = scrollable_query.get_mut(container_entity) {
                    let max_offset = container.max_offset;
                    
                    // Handle vertical scroll
                    if matches!(container.direction, ScrollDirection::Vertical | ScrollDirection::Both)
                        && max_offset.y > 0.0
                        && delta.y != 0.0
                    {
                        let at_max = if delta.y > 0.0 {
                            scroll_position.y >= max_offset.y
                        } else {
                            scroll_position.y <= 0.0
                        };
                        
                        if !at_max {
                            scroll_position.y = (scroll_position.y + delta.y).clamp(0.0, max_offset.y);
                        }
                    }
                    
                    // Handle horizontal scroll
                    if matches!(container.direction, ScrollDirection::Horizontal | ScrollDirection::Both)
                        && max_offset.x > 0.0
                        && delta.x != 0.0
                    {
                        let at_max = if delta.x > 0.0 {
                            scroll_position.x >= max_offset.x
                        } else {
                            scroll_position.x <= 0.0
                        };
                        
                        if !at_max {
                            scroll_position.x = (scroll_position.x + delta.x).clamp(0.0, max_offset.x);
                        }
                    }
                }
            }
        }
    }
}

/// System to sync ScrollContainer state with Bevy's native ScrollPosition
/// This reads the ScrollPosition (managed by Bevy's scroll system) and updates our ScrollContainer
fn sync_scroll_state_system(
    mut containers: Query<(&mut ScrollContainer, &ScrollPosition, &ComputedNode, &Children)>,
    content_nodes: Query<&ComputedNode, With<ScrollContent>>,
) {
    for (mut container, scroll_pos, computed, children) in containers.iter_mut() {
        // The actual scrollable extents come from the internal ScrollContent node.
        // Fall back to the container node if the wrapper doesn't exist yet.
        let mut viewport_size_phys = computed.size();
        let mut content_size_phys = computed.content_size();
        let mut inv = computed.inverse_scale_factor();

        for child in children.iter() {
            if let Ok(content_computed) = content_nodes.get(child) {
                viewport_size_phys = content_computed.size();
                content_size_phys = content_computed.content_size();
                inv = content_computed.inverse_scale_factor();
                break;
            }
        }

        container.container_size = viewport_size_phys * inv;
        container.content_size = content_size_phys * inv;
        container.max_offset = (container.content_size - container.container_size).max(Vec2::ZERO);

        // Sync offset from Bevy's ScrollPosition (logical pixels)
        container.offset = **scroll_pos;
        container.target_offset = **scroll_pos;
    }
}

/// System to handle scrollbar thumb dragging
fn scrollbar_thumb_drag_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut thumb_v: Query<
        (&Interaction, &mut ScrollbarDragging, &ChildOf, &ComputedNode),
        (With<ScrollbarThumbVertical>, Without<ScrollbarThumbHorizontal>),
    >,
    mut thumb_h: Query<
        (&Interaction, &mut ScrollbarDragging, &ChildOf, &ComputedNode),
        (With<ScrollbarThumbHorizontal>, Without<ScrollbarThumbVertical>),
    >,
    track_v: Query<(&ComputedNode, &ChildOf), With<ScrollbarTrackVertical>>,
    track_h: Query<(&ComputedNode, &ChildOf), With<ScrollbarTrackHorizontal>>,
    mut containers: Query<(&ScrollContainer, &mut ScrollPosition, &ComputedNode)>,
) {
    let Ok(window) = windows.single() else { return };
    let cursor_pos = window.cursor_position();
    let window_scale_factor = window.scale_factor();
    
    // Handle vertical scrollbar thumb dragging
    for (interaction, mut drag_state, track_parent, thumb_node) in thumb_v.iter_mut() {
        // Start dragging on press
        // Note: `Interaction::Pressed` is updated by Bevy's UI systems and may not
        // become `Pressed` until a frame after the mouse button press is registered.
        // If we also require `just_pressed`, we can miss the drag start entirely.
        if *interaction == Interaction::Pressed && !drag_state.is_dragging {
            if let Some(pos) = cursor_pos {
                // Find the container through the track's parent
                // track_parent is the thumb's ChildOf (points to track)
                // scroll_parent is the track's ChildOf (points to scroll container)
                if let Ok((track_node, scroll_parent)) = track_v.get(track_parent.0) {
                    if let Ok((container, scroll_pos, computed)) = containers.get(scroll_parent.0) {
                        drag_state.is_dragging = true;
                        drag_state.drag_start_pos = Some(pos);
                        drag_state.drag_start_offset = scroll_pos.y;

                        let inv = computed.inverse_scale_factor();
                        let phys_scale = if inv > 0.0 { 1.0 / inv } else { 0.0 };
                        let pos_phys = pos * phys_scale;
                        bevy::log::info!(
                            "SBAR DragVStart: cursor_log=({:.1},{:.1}) cursor_phys=({:.1},{:.1}) win_scale={:.3} inv={:.4} phys_scale={:.4} track_h={:.1} thumb_comp_h={:.1} thumb_req={:.1} max_y={:.1} start_off_y={:.1}",
                            pos.x,
                            pos.y,
                            pos_phys.x,
                            pos_phys.y,
                            window_scale_factor,
                            inv,
                            phys_scale,
                            track_node.size().y * inv,
                            thumb_node.size().y * inv,
                            container.vertical_thumb_size(),
                            container.max_offset.y,
                            scroll_pos.y
                        );
                    }
                }
            }
        }
        
        // Stop dragging on release
        if mouse_button.just_released(MouseButton::Left) {
            drag_state.is_dragging = false;
            drag_state.drag_start_pos = None;

            if let Some(pos) = cursor_pos {
                bevy::log::info!("SBAR DragVEnd: cursor_log=({:.1},{:.1})", pos.x, pos.y);
            } else {
                bevy::log::info!("SBAR DragVEnd: cursor_log=(none)");
            }
        }
        
        // Update scroll position while dragging
        if drag_state.is_dragging {
            if let (Some(start_pos), Some(current_pos)) = (drag_state.drag_start_pos, cursor_pos) {
                if let Ok((track_node, scroll_parent)) = track_v.get(track_parent.0) {
                    if let Ok((container, mut scroll_pos, computed)) = containers.get_mut(scroll_parent.0) {
                        // Use logical pixels consistently (cursor positions, ScrollPosition, Node style values).
                        // track_node.size() is physical, so convert to logical.
                        let inv = computed.inverse_scale_factor();
                        let track_height = track_node.size().y * inv;

                        // Clamp thumb size to the actual track height (important when both scrollbars are visible
                        // and the vertical track is shortened to avoid the bottom-right corner overlap).
                        let thumb_size = container
                            .vertical_thumb_size()
                            .max(30.0)
                            .min(track_height);
                        let thumb_travel = track_height - thumb_size;
                        
                        let max_offset_y = container.max_offset.y;
                        
                        if max_offset_y > 0.0 && thumb_travel > 0.0 {
                            // Treat cursor Y as increasing downward for drag purposes.
                            // Dragging the mouse down should increase the thumb/scroll offset.
                            let drag_delta_y = current_pos.y - start_pos.y;

                            // Convert thumb movement to scroll
                            let scroll_delta = (drag_delta_y / thumb_travel) * max_offset_y;
                            let new_y = (drag_state.drag_start_offset + scroll_delta)
                                .clamp(0.0, max_offset_y);

                            let phys_scale = if inv > 0.0 { 1.0 / inv } else { 0.0 };
                            let start_phys = start_pos * phys_scale;
                            let cur_phys = current_pos * phys_scale;
                            bevy::log::info!(
                                "SBAR DragV: start_log=({:.1},{:.1}) cur_log=({:.1},{:.1}) start_phys=({:.1},{:.1}) cur_phys=({:.1},{:.1}) d_log_y={:.1} d_phys_y={:.1} track_h={:.1} thumb_comp_h={:.1} thumb_req={:.1} travel={:.1} max_y={:.1} start_off_y={:.1} new_y={:.1} win_scale={:.3} inv={:.4} phys_scale={:.4}",
                                start_pos.x,
                                start_pos.y,
                                current_pos.x,
                                current_pos.y,
                                start_phys.x,
                                start_phys.y,
                                cur_phys.x,
                                cur_phys.y,
                                current_pos.y - start_pos.y,
                                drag_delta_y,
                                track_height,
                                thumb_node.size().y * inv,
                                thumb_size,
                                thumb_travel,
                                max_offset_y,
                                drag_state.drag_start_offset,
                                new_y,
                                window_scale_factor,
                                inv,
                                phys_scale
                            );

                            scroll_pos.y = new_y;
                        }
                    }
                }
            }
        }
    }
    
    // Handle horizontal scrollbar thumb dragging
    for (interaction, mut drag_state, track_parent, thumb_node) in thumb_h.iter_mut() {
        // Start dragging on press
        if *interaction == Interaction::Pressed && !drag_state.is_dragging {
            if let Some(pos) = cursor_pos {
                // Find the container through the track's parent
                if let Ok((_track_node, scroll_parent)) = track_h.get(track_parent.0) {
                    if let Ok((container, scroll_pos, computed)) = containers.get(scroll_parent.0) {
                        drag_state.is_dragging = true;
                        drag_state.drag_start_pos = Some(pos);
                        drag_state.drag_start_offset = scroll_pos.x;

                        let inv = computed.inverse_scale_factor();
                        let phys_scale = if inv > 0.0 { 1.0 / inv } else { 0.0 };
                        let pos_phys = pos * phys_scale;
                        bevy::log::info!(
                            "SBAR DragHStart: cursor_log=({:.1},{:.1}) cursor_phys=({:.1},{:.1}) win_scale={:.3} inv={:.4} phys_scale={:.4} track_w={:.1} thumb_comp_w={:.1} thumb_req={:.1} max_x={:.1} start_off_x={:.1}",
                            pos.x,
                            pos.y,
                            pos_phys.x,
                            pos_phys.y,
                            window_scale_factor,
                            inv,
                            phys_scale,
                            container.container_size.x,
                            thumb_node.size().x * inv,
                            container.horizontal_thumb_size(),
                            container.max_offset.x,
                            scroll_pos.x
                        );
                    }
                }
            }
        }
        
        // Stop dragging on release
        if mouse_button.just_released(MouseButton::Left) {
            drag_state.is_dragging = false;
            drag_state.drag_start_pos = None;

            if let Some(pos) = cursor_pos {
                bevy::log::info!("SBAR DragHEnd: cursor_log=({:.1},{:.1})", pos.x, pos.y);
            } else {
                bevy::log::info!("SBAR DragHEnd: cursor_log=(none)");
            }
        }
        
        // Update scroll position while dragging
        if drag_state.is_dragging {
            if let (Some(start_pos), Some(current_pos)) = (drag_state.drag_start_pos, cursor_pos) {
                if let Ok((track_node, scroll_parent)) = track_h.get(track_parent.0) {
                    if let Ok((container, mut scroll_pos, computed)) = containers.get_mut(scroll_parent.0) {
                        // Use logical pixels consistently.
                        let inv = computed.inverse_scale_factor();
                        let track_width = track_node.size().x * inv;
                        let thumb_size = container
                            .horizontal_thumb_size()
                            .max(30.0)
                            .min(track_width);
                        let thumb_travel = track_width - thumb_size;
                        
                        let max_offset_x = container.max_offset.x;
                        
                        if max_offset_x > 0.0 && thumb_travel > 0.0 {
                            let drag_delta_x = current_pos.x - start_pos.x;
                            
                            // Convert thumb movement to scroll
                            let scroll_delta = (drag_delta_x / thumb_travel) * max_offset_x;
                            let new_x = (drag_state.drag_start_offset + scroll_delta)
                                .clamp(0.0, max_offset_x);

                            let phys_scale = if inv > 0.0 { 1.0 / inv } else { 0.0 };
                            let start_phys = start_pos * phys_scale;
                            let cur_phys = current_pos * phys_scale;
                            bevy::log::info!(
                                "SBAR DragH: start_log=({:.1},{:.1}) cur_log=({:.1},{:.1}) start_phys=({:.1},{:.1}) cur_phys=({:.1},{:.1}) d_log_x={:.1} d_phys_x={:.1} track_w={:.1} thumb_comp_w={:.1} thumb_req={:.1} travel={:.1} max_x={:.1} start_off_x={:.1} new_x={:.1} win_scale={:.3} inv={:.4} phys_scale={:.4}",
                                start_pos.x,
                                start_pos.y,
                                current_pos.x,
                                current_pos.y,
                                start_phys.x,
                                start_phys.y,
                                cur_phys.x,
                                cur_phys.y,
                                current_pos.x - start_pos.x,
                                drag_delta_x,
                                track_width,
                                thumb_node.size().x * inv,
                                thumb_size,
                                thumb_travel,
                                max_offset_x,
                                drag_state.drag_start_offset,
                                new_x,
                                window_scale_factor,
                                inv,
                                phys_scale
                            );

                            scroll_pos.x = new_x;
                        }
                    }
                }
            }
        }
    }
}

/// System to update scrollbar visuals
fn update_scrollbars(
    containers: Query<(&ScrollContainer, &ScrollPosition, &Children)>,
    track_v_nodes: Query<&ComputedNode, With<ScrollbarTrackVertical>>,
    track_h_nodes: Query<&ComputedNode, With<ScrollbarTrackHorizontal>>,
    mut thumb_v: Query<&mut Node, (With<ScrollbarThumbVertical>, Without<ScrollbarThumbHorizontal>)>,
    mut thumb_h: Query<&mut Node, (With<ScrollbarThumbHorizontal>, Without<ScrollbarThumbVertical>)>,
    mut track_v_vis: Query<&mut Visibility, (With<ScrollbarTrackVertical>, Without<ScrollbarTrackHorizontal>)>,
    mut track_h_vis: Query<&mut Visibility, (With<ScrollbarTrackHorizontal>, Without<ScrollbarTrackVertical>)>,
    children_query: Query<&Children>,
) {
    for (container, scroll_pos, children) in containers.iter() {
        // Find scrollbar elements in children
        for child in children.iter() {
            // Check for vertical track
            if let Ok(mut vis) = track_v_vis.get_mut(child) {
                *vis = if container.needs_scroll_y() && container.show_scrollbars {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
            
            // Check for horizontal track
            if let Ok(mut vis) = track_h_vis.get_mut(child) {
                *vis = if container.needs_scroll_x() && container.show_scrollbars {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }

            // Look for thumbs in track children
            if let Ok(track_children) = children_query.get(child) {
                for track_child in track_children.iter() {
                    // Update vertical thumb - get track height from track's ComputedNode
                    if let Ok(mut node) = thumb_v.get_mut(track_child) {
                        if let Ok(track_computed) = track_v_nodes.get(child) {
                            let track_height = track_computed.size().y * track_computed.inverse_scale_factor();
                            let thumb_size = container
                                .vertical_thumb_size()
                                .max(30.0)
                                .min(track_height);
                            // scroll_ratio: 0 = top, 1 = bottom
                            let scroll_ratio = if container.max_offset.y > 0.0 {
                                (scroll_pos.y / container.max_offset.y).clamp(0.0, 1.0)
                            } else {
                                0.0
                            };
                            let thumb_travel = (track_height - thumb_size).max(0.0);
                            let top_value = scroll_ratio * thumb_travel;

                            // Log only when something is out of sync (thumb outside rails, etc.)
                            if thumb_size > track_height + 0.5
                                || top_value < -0.5
                                || top_value > thumb_travel + 0.5
                            {
                                bevy::log::info!(
                                    "SBAR ThumbV_OOB: scroll_y={:.2} max_y={:.2} ratio={:.4} track_h={:.2} thumb={:.2} travel={:.2} top={:.2}",
                                    scroll_pos.y,
                                    container.max_offset.y,
                                    scroll_ratio,
                                    track_height,
                                    thumb_size,
                                    thumb_travel,
                                    top_value
                                );
                            }
                            
                            node.height = Val::Px(thumb_size);
                            node.top = Val::Px(top_value);
                        }
                    }
                    
                    // Update horizontal thumb - get track width from track's ComputedNode
                    if let Ok(mut node) = thumb_h.get_mut(track_child) {
                        if let Ok(track_computed) = track_h_nodes.get(child) {
                            let track_width = track_computed.size().x * track_computed.inverse_scale_factor();
                            let thumb_size = container
                                .horizontal_thumb_size()
                                .max(30.0)
                                .min(track_width);
                            let scroll_ratio = if container.max_offset.x > 0.0 {
                                (scroll_pos.x / container.max_offset.x).clamp(0.0, 1.0)
                            } else {
                                0.0
                            };
                            let thumb_travel = (track_width - thumb_size).max(0.0);
                            let left_value = scroll_ratio * thumb_travel;

                            if thumb_size > track_width + 0.5
                                || left_value < -0.5
                                || left_value > thumb_travel + 0.5
                            {
                                bevy::log::info!(
                                    "SBAR ThumbH_OOB: scroll_x={:.2} max_x={:.2} ratio={:.4} track_w={:.2} thumb={:.2} travel={:.2} left={:.2}",
                                    scroll_pos.x,
                                    container.max_offset.x,
                                    scroll_ratio,
                                    track_width,
                                    thumb_size,
                                    thumb_travel,
                                    left_value
                                );
                            }
                            
                            node.width = Val::Px(thumb_size);
                            node.left = Val::Px(left_value);
                        }
                    }
                }
            }
        }
    }
}

/// Spawn scrollbars for a scroll container
/// Call this after spawning ScrollContainer to add visual scrollbars
pub fn spawn_scrollbars(commands: &mut ChildSpawnerCommands, theme: &MaterialTheme, direction: ScrollDirection) {
    if matches!(direction, ScrollDirection::Vertical | ScrollDirection::Both) {
        spawn_scrollbar_vertical(commands, theme, matches!(direction, ScrollDirection::Both));
    }
    if matches!(direction, ScrollDirection::Horizontal | ScrollDirection::Both) {
        spawn_scrollbar_horizontal(commands, theme, matches!(direction, ScrollDirection::Both));
    }
}

fn spawn_scrollbar_vertical(
    commands: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    reserve_bottom_corner: bool,
) {
    let scrollbar_width = SCROLLBAR_THICKNESS;
    let track_color = theme.surface_container_highest.with_alpha(0.5);
    let thumb_color = theme.primary.with_alpha(0.7);

    commands
        .spawn((
            ScrollbarTrackVertical,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(if reserve_bottom_corner { scrollbar_width } else { 0.0 }),
                width: Val::Px(scrollbar_width),
                ..default()
            },
            BackgroundColor(track_color),
            BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
        ))
        .with_children(|track| {
            track.spawn((
                ScrollbarThumbVertical,
                ScrollbarDragging::default(),
                Button,
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(scrollbar_width),
                    height: Val::Px(50.0), // Will be updated by system
                    top: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(thumb_color),
                BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
            ));
        });
}

fn spawn_scrollbar_horizontal(
    commands: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    reserve_right_corner: bool,
) {
    let scrollbar_width = SCROLLBAR_THICKNESS;
    let track_color = theme.surface_container_highest.with_alpha(0.5);
    let thumb_color = theme.primary.with_alpha(0.7);

    commands
        .spawn((
            ScrollbarTrackHorizontal,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(if reserve_right_corner { scrollbar_width } else { 0.0 }),
                height: Val::Px(scrollbar_width),
                ..default()
            },
            BackgroundColor(track_color),
            BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
        ))
        .with_children(|track| {
            track.spawn((
                ScrollbarThumbHorizontal,
                ScrollbarDragging::default(),
                Button,
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(scrollbar_width),
                    width: Val::Px(50.0), // Will be updated by system
                    left: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(thumb_color),
                BorderRadius::all(Val::Px(scrollbar_width / 2.0)),
            ));
        });
}

/// Builder for scroll containers
pub struct ScrollContainerBuilder {
    direction: ScrollDirection,
    sensitivity: f32,
    smooth: bool,
    smooth_speed: f32,
    show_scrollbars: bool,
}

impl Default for ScrollContainerBuilder {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            sensitivity: 40.0,
            smooth: true,
            smooth_speed: 0.2,
            show_scrollbars: true,
        }
    }
}

impl ScrollContainerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertical(mut self) -> Self {
        self.direction = ScrollDirection::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.direction = ScrollDirection::Horizontal;
        self
    }

    pub fn both(mut self) -> Self {
        self.direction = ScrollDirection::Both;
        self
    }

    pub fn sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    pub fn with_scrollbars(mut self, show: bool) -> Self {
        self.show_scrollbars = show;
        self
    }

    pub fn build(self) -> ScrollContainer {
        ScrollContainer {
            direction: self.direction,
            sensitivity: self.sensitivity,
            smooth: self.smooth,
            smooth_speed: self.smooth_speed,
            show_scrollbars: self.show_scrollbars,
            ..default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_container_default() {
        let container = ScrollContainer::default();
        assert!(matches!(container.direction, ScrollDirection::Vertical));
        assert_eq!(container.offset, Vec2::ZERO);
        assert!(container.smooth);
        assert!(container.show_scrollbars);
    }

    #[test]
    fn test_scroll_container_vertical() {
        let container = ScrollContainer::vertical();
        assert!(!container.needs_scroll_y()); // No content yet
    }

    #[test]
    fn test_scroll_by() {
        let mut container = ScrollContainer::vertical();
        container.scroll_by(Vec2::new(10.0, 20.0));
        assert_eq!(container.target_offset.y, 20.0);
        assert_eq!(container.target_offset.x, 0.0);
    }

    #[test]
    fn test_scroll_builder() {
        let container = ScrollContainerBuilder::new()
            .vertical()
            .sensitivity(50.0)
            .smooth(false)
            .with_scrollbars(false)
            .build();
        
        assert_eq!(container.sensitivity, 50.0);
        assert!(!container.smooth);
        assert!(!container.show_scrollbars);
    }

    #[test]
    fn test_thumb_calculations() {
        let mut container = ScrollContainer::vertical();
        container.container_size = Vec2::new(100.0, 400.0);
        container.content_size = Vec2::new(100.0, 1000.0);
        container.max_offset = Vec2::new(0.0, 600.0);
        container.offset = Vec2::new(0.0, 300.0);

        let thumb_size = container.vertical_thumb_size();
        assert!(thumb_size > 0.0);
        assert!(thumb_size < container.container_size.y);

        let thumb_pos = container.vertical_thumb_position();
        assert_eq!(thumb_pos, 0.5); // 300 / 600
    }
}
