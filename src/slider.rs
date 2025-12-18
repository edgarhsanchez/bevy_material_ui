//! Material Design 3 Slider component
//!
//! Sliders allow users to select a value from a range.
//! Reference: <https://m3.material.io/components/sliders/overview>

use bevy::prelude::*;
use bevy::ui::UiGlobalTransform;

use crate::theme::MaterialTheme;

/// Slider orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    /// Horizontal slider (left/right)
    #[default]
    Horizontal,
    /// Vertical slider (top/bottom)
    Vertical,
}

/// Direction that values increase along the track.
///
/// - Horizontal: `StartToEnd` = left->right, `EndToStart` = right->left
/// - Vertical: `StartToEnd` = top->bottom, `EndToStart` = bottom->top
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderDirection {
    /// Values increase from the start edge toward the end edge.
    #[default]
    StartToEnd,
    /// Values increase from the end edge toward the start edge.
    EndToStart,
}

/// Plugin for the slider component
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SliderChangeEvent>().add_systems(
            Update,
            (
                slider_interaction_system,
                slider_visual_update_system.after(slider_interaction_system),
                slider_theme_refresh_system.after(slider_visual_update_system),
            ),
        );
    }
}

#[derive(Component, Clone)]
struct SliderParts {
    track: Entity,
    active_track: Entity,
    handle: Entity,
    ticks: Vec<Entity>,
}

#[derive(Component, Clone, Copy)]
struct SliderTick {
    /// Normalized position [0..1] along the track.
    position: f32,
}

/// Slider variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SliderVariant {
    /// Continuous slider - any value in range
    #[default]
    Continuous,
    /// Discrete slider - snaps to steps
    Discrete,
}

/// Material slider component
///
/// Matches properties from Material iOS MDCSlider:
/// - Track colors (background/fill) for different states
/// - Thumb colors and elevation
/// - Tick mark visibility and colors
/// - Value label configuration
/// - Anchor value for filled track start position
#[derive(Component)]
pub struct MaterialSlider {
    /// Current value
    pub value: f32,
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
    /// Step size for discrete sliders (None = continuous)
    pub step: Option<f32>,
    /// Number of discrete values (for discrete sliders)
    pub discrete_value_count: Option<usize>,
    /// Whether to show tick marks
    pub show_ticks: bool,
    /// Tick mark visibility mode
    pub tick_visibility: TickVisibility,
    /// Whether to show value label
    pub show_label: bool,
    /// Whether the slider is disabled
    pub disabled: bool,
    /// Anchor value - where the filled track starts (default: min)
    pub anchor_value: Option<f32>,
    /// Custom track height (default: 4.0)
    pub track_height: f32,
    /// Custom thumb radius (default: 10.0)
    pub thumb_radius: f32,
    /// Thumb elevation when not dragging
    pub thumb_elevation: f32,
    /// Thumb ripple maximum radius
    pub thumb_ripple_radius: f32,
    /// Custom value label formatter
    pub value_formatter: Option<fn(f32) -> String>,
    /// Slider orientation
    pub orientation: SliderOrientation,
    /// Direction values increase along the track
    pub direction: SliderDirection,
    /// Interaction states
    pub dragging: bool,
    pub hovered: bool,
    pub focused: bool,
}

/// Tick mark visibility mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TickVisibility {
    /// Always show tick marks
    Always,
    /// Show tick marks only when dragging
    WhenDragging,
    /// Never show tick marks
    #[default]
    Never,
}

impl MaterialSlider {
    /// Create a new slider
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            value: min,
            min,
            max,
            step: None,
            discrete_value_count: None,
            show_ticks: false,
            tick_visibility: TickVisibility::default(),
            show_label: false,
            disabled: false,
            anchor_value: None,
            track_height: SLIDER_TRACK_HEIGHT,
            thumb_radius: SLIDER_HANDLE_SIZE / 2.0,
            thumb_elevation: 1.0,
            thumb_ripple_radius: SLIDER_HANDLE_SIZE * 1.5,
            value_formatter: None,
            orientation: SliderOrientation::Horizontal,
            direction: SliderDirection::StartToEnd,
            dragging: false,
            hovered: false,
            focused: false,
        }
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Convenience for vertical sliders
    pub fn vertical(self) -> Self {
        self.orientation(SliderOrientation::Vertical)
    }

    /// Set direction
    pub fn direction(mut self, direction: SliderDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Convenience for reversed sliders
    pub fn reversed(self) -> Self {
        self.direction(SliderDirection::EndToStart)
    }

    /// Set the initial value
    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Set the step size (makes it discrete)
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Set the number of discrete values
    pub fn discrete(mut self, count: usize) -> Self {
        self.discrete_value_count = Some(count);
        if count >= 2 {
            self.step = Some((self.max - self.min) / (count - 1) as f32);
        }
        self
    }

    /// Set anchor value (where filled track starts)
    pub fn anchor(mut self, value: f32) -> Self {
        self.anchor_value = Some(value.clamp(self.min, self.max));
        self
    }

    /// Set custom track height
    pub fn track_height(mut self, height: f32) -> Self {
        self.track_height = height;
        self
    }

    /// Set custom thumb radius
    pub fn thumb_radius(mut self, radius: f32) -> Self {
        self.thumb_radius = radius;
        self
    }

    /// Set thumb elevation
    pub fn thumb_elevation(mut self, elevation: f32) -> Self {
        self.thumb_elevation = elevation;
        self
    }

    /// Show tick marks
    pub fn show_ticks(mut self) -> Self {
        self.show_ticks = true;
        self.tick_visibility = TickVisibility::Always;
        self
    }

    /// Set tick visibility mode
    pub fn tick_visibility(mut self, visibility: TickVisibility) -> Self {
        self.tick_visibility = visibility;
        self.show_ticks = visibility != TickVisibility::Never;
        self
    }

    /// Show value label
    pub fn show_label(mut self) -> Self {
        self.show_label = true;
        self
    }

    /// Set custom value formatter for label
    pub fn value_formatter(mut self, formatter: fn(f32) -> String) -> Self {
        self.value_formatter = Some(formatter);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Get the formatted value string for the label
    pub fn formatted_value(&self) -> String {
        if let Some(formatter) = self.value_formatter {
            formatter(self.value)
        } else {
            format!("{:.0}", self.value)
        }
    }

    /// Check if ticks should currently be visible
    pub fn should_show_ticks(&self) -> bool {
        let discrete_ticks = if let Some(count) = self.discrete_value_count {
            count >= 2
        } else if let Some(step) = self.step {
            step > 0.0 && (self.max - self.min) / step >= 1.0
        } else {
            false
        };

        match self.tick_visibility {
            TickVisibility::Always => discrete_ticks,
            TickVisibility::WhenDragging => self.dragging && discrete_ticks,
            TickVisibility::Never => false,
        }
    }

    /// Get the normalized value (0.0 to 1.0)
    pub fn normalized_value(&self) -> f32 {
        (self.value - self.min) / (self.max - self.min)
    }

    /// Get the visual position along the track (0.0 to 1.0), respecting `direction`.
    pub fn position_percent(&self) -> f32 {
        let v = self.normalized_value().clamp(0.0, 1.0);
        match self.direction {
            SliderDirection::StartToEnd => v,
            SliderDirection::EndToStart => 1.0 - v,
        }
    }

    /// Set value from normalized (0.0 to 1.0)
    pub fn set_from_normalized(&mut self, normalized: f32) {
        let raw_value = self.min + normalized * (self.max - self.min);
        self.value = if let Some(step) = self.step {
            (raw_value / step).round() * step
        } else {
            raw_value
        };
        self.value = self.value.clamp(self.min, self.max);
    }

    /// Get the active track color
    pub fn active_track_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.primary
        }
    }

    /// Get the inactive track color
    pub fn inactive_track_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.12)
        } else {
            theme.surface_container_highest
        }
    }

    /// Get the handle (thumb) color
    pub fn handle_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.primary
        }
    }

    /// Get the tick mark color for active section
    pub fn active_tick_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_primary
        }
    }

    /// Get the tick mark color for inactive section
    pub fn inactive_tick_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the label background color
    pub fn label_background_color(&self, theme: &MaterialTheme) -> Color {
        theme.primary
    }

    /// Get the label text color
    pub fn label_text_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_primary
    }
}

impl Default for MaterialSlider {
    fn default() -> Self {
        Self::new(0.0, 100.0)
    }
}

/// Event when slider value changes
#[derive(Event, bevy::prelude::Message)]
pub struct SliderChangeEvent {
    pub entity: Entity,
    pub value: f32,
}

/// Slider dimensions
pub const SLIDER_TRACK_HEIGHT: f32 = 4.0;
pub const SLIDER_TRACK_HEIGHT_ACTIVE: f32 = 6.0;
pub const SLIDER_HANDLE_SIZE: f32 = 20.0;
pub const SLIDER_HANDLE_SIZE_PRESSED: f32 = 24.0;
pub const SLIDER_TICK_SIZE: f32 = 4.0;
pub const SLIDER_LABEL_HEIGHT: f32 = 28.0;

/// System to handle slider interactions
fn slider_interaction_system(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut MaterialSlider,
            &ComputedNode,
            &UiGlobalTransform,
        ),
        With<MaterialSlider>,
    >,
    mut change_events: MessageWriter<SliderChangeEvent>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let Ok(window) = windows.single() else { return };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Bevy UI layout uses physical pixels for `UiGlobalTransform` and `ComputedNode`.
    // Convert the window cursor position (logical coords) into physical pixels.
    let scale_factor = window.scale_factor();
    let cursor_physical = cursor_position * scale_factor;

    for (entity, interaction, mut slider, computed_node, transform) in interaction_query.iter_mut()
    {
        if slider.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                slider.dragging = true;
                slider.hovered = false;
            }
            Interaction::Hovered => {
                if !mouse_button.pressed(MouseButton::Left) {
                    slider.dragging = false;
                }
                slider.hovered = true;
            }
            Interaction::None => {
                if !mouse_button.pressed(MouseButton::Left) {
                    slider.dragging = false;
                }
                slider.hovered = false;
            }
        }

        // Handle dragging
        if slider.dragging {
            let slider_center = transform.translation;
            let slider_size = computed_node.size();

            let position_percent = match slider.orientation {
                SliderOrientation::Horizontal => {
                    // Layout may not be computed yet (or may be zero during first-frame interactions).
                    // Avoid NaNs that would poison the slider value and visuals.
                    if slider_size.x <= 0.0 {
                        continue;
                    }

                    let slider_left = slider_center.x - slider_size.x / 2.0;
                    let relative_x = cursor_physical.x - slider_left;
                    let p = (relative_x / slider_size.x).clamp(0.0, 1.0);
                    if !p.is_finite() {
                        continue;
                    }
                    p
                }
                SliderOrientation::Vertical => {
                    if slider_size.y <= 0.0 {
                        continue;
                    }

                    let slider_top = slider_center.y - slider_size.y / 2.0;
                    let relative_y = cursor_physical.y - slider_top;
                    let p = (relative_y / slider_size.y).clamp(0.0, 1.0);
                    if !p.is_finite() {
                        continue;
                    }
                    p
                }
            };

            // Convert visual position into normalized value (min..max), respecting direction.
            let normalized = match slider.direction {
                SliderDirection::StartToEnd => position_percent,
                SliderDirection::EndToStart => 1.0 - position_percent,
            };
            if !normalized.is_finite() {
                continue;
            }

            let old_value = slider.value;
            slider.set_from_normalized(normalized);

            if (slider.value - old_value).abs() > f32::EPSILON {
                change_events.write(SliderChangeEvent {
                    entity,
                    value: slider.value,
                });
            }
        }
    }
}

fn slider_visual_update_system(
    theme: Option<Res<MaterialTheme>>,
    sliders: Query<(&MaterialSlider, &SliderParts), Changed<MaterialSlider>>,
    mut nodes: Query<&mut Node>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut border_radii: Query<&mut BorderRadius>,
    mut visibilities: Query<&mut Visibility>,
    ticks: Query<&SliderTick>,
) {
    let Some(theme) = theme else { return };

    for (slider, parts) in sliders.iter() {
        update_slider_visuals(
            &theme,
            slider,
            parts,
            &mut nodes,
            &mut bg_colors,
            &mut border_radii,
            &mut visibilities,
            &ticks,
        );
    }
}

/// Refresh all sliders when the theme changes.
fn slider_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    sliders: Query<(&MaterialSlider, &SliderParts)>,
    mut nodes: Query<&mut Node>,
    mut bg_colors: Query<&mut BackgroundColor>,
    mut border_radii: Query<&mut BorderRadius>,
    mut visibilities: Query<&mut Visibility>,
    ticks: Query<&SliderTick>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (slider, parts) in sliders.iter() {
        update_slider_visuals(
            &theme,
            slider,
            parts,
            &mut nodes,
            &mut bg_colors,
            &mut border_radii,
            &mut visibilities,
            &ticks,
        );
    }
}

fn update_slider_visuals(
    theme: &MaterialTheme,
    slider: &MaterialSlider,
    parts: &SliderParts,
    nodes: &mut Query<&mut Node>,
    bg_colors: &mut Query<&mut BackgroundColor>,
    border_radii: &mut Query<&mut BorderRadius>,
    visibilities: &mut Query<&mut Visibility>,
    ticks: &Query<&SliderTick>,
) {
    let _value_percent = slider.normalized_value().clamp(0.0, 1.0);
    let position_percent = slider.position_percent().clamp(0.0, 1.0);

    let track_height = if slider.dragging {
        SLIDER_TRACK_HEIGHT_ACTIVE
    } else {
        slider.track_height
    };

    let track_color = slider.inactive_track_color(theme);
    let active_color = slider.active_track_color(theme);
    let active_tick_color = slider.active_tick_color(theme);
    let inactive_tick_color = slider.inactive_tick_color(theme);
    let handle_color = slider.handle_color(theme);

    // Track base
    if let Ok(mut bg) = bg_colors.get_mut(parts.track) {
        *bg = BackgroundColor(track_color);
    }
    if let Ok(mut node) = nodes.get_mut(parts.track) {
        match slider.orientation {
            SliderOrientation::Horizontal => {
                node.height = Val::Px(track_height);
            }
            SliderOrientation::Vertical => {
                node.width = Val::Px(track_height);
            }
        }
    }
    if let Ok(mut radius) = border_radii.get_mut(parts.track) {
        *radius = BorderRadius::all(Val::Px(track_height / 2.0));
    }

    // Active track
    if let Ok(mut bg) = bg_colors.get_mut(parts.active_track) {
        *bg = BackgroundColor(active_color);
    }
    if let Ok(mut node) = nodes.get_mut(parts.active_track) {
        match slider.orientation {
            SliderOrientation::Horizontal => {
                match slider.direction {
                    SliderDirection::StartToEnd => {
                        node.left = Val::Px(0.0);
                        node.width = Val::Percent(position_percent * 100.0);
                    }
                    SliderDirection::EndToStart => {
                        node.left = Val::Percent(position_percent * 100.0);
                        node.width = Val::Percent((1.0 - position_percent) * 100.0);
                    }
                }
                node.height = Val::Px(track_height);
                node.top = Val::Px((SLIDER_HANDLE_SIZE + 8.0 - track_height) / 2.0);
            }
            SliderOrientation::Vertical => {
                node.width = Val::Px(track_height);
                match slider.direction {
                    SliderDirection::StartToEnd => {
                        node.top = Val::Px(0.0);
                        node.height = Val::Percent(position_percent * 100.0);
                    }
                    SliderDirection::EndToStart => {
                        node.top = Val::Percent(position_percent * 100.0);
                        node.height = Val::Percent((1.0 - position_percent) * 100.0);
                    }
                }
            }
        }
    }
    if let Ok(mut radius) = border_radii.get_mut(parts.active_track) {
        *radius = BorderRadius::all(Val::Px(track_height / 2.0));
    }

    // Handle (thumb)
    let mut handle_radius = slider.thumb_radius;
    if slider.dragging {
        handle_radius = (handle_radius + 2.0).min(SLIDER_HANDLE_SIZE_PRESSED / 2.0);
    }
    if let Ok(mut bg) = bg_colors.get_mut(parts.handle) {
        *bg = BackgroundColor(handle_color);
    }
    if let Ok(mut node) = nodes.get_mut(parts.handle) {
        match slider.orientation {
            SliderOrientation::Horizontal => {
                node.left = Val::Percent(position_percent * 100.0);
                node.margin.left = Val::Px(-handle_radius);
            }
            SliderOrientation::Vertical => {
                node.top = Val::Percent(position_percent * 100.0);
                node.margin.top = Val::Px(-handle_radius);
            }
        }
        node.width = Val::Px(handle_radius * 2.0);
        node.height = Val::Px(handle_radius * 2.0);
    }
    if let Ok(mut radius) = border_radii.get_mut(parts.handle) {
        *radius = BorderRadius::all(Val::Px(handle_radius));
    }

    // Tick marks
    let show_ticks_now = slider.should_show_ticks();
    for &tick_entity in &parts.ticks {
        if let Ok(mut vis) = visibilities.get_mut(tick_entity) {
            *vis = if show_ticks_now {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        let Ok(tick) = ticks.get(tick_entity) else {
            continue;
        };
        let tick_active = match slider.direction {
            SliderDirection::StartToEnd => tick.position <= position_percent,
            SliderDirection::EndToStart => tick.position >= position_percent,
        };
        let tick_color = if tick_active {
            active_tick_color
        } else {
            inactive_tick_color
        };
        if let Ok(mut bg) = bg_colors.get_mut(tick_entity) {
            *bg = BackgroundColor(tick_color);
        }
    }
}

/// Spawn a standalone slider control (no label wrapper) as a child.
///
/// Returns the slider entity id so callers can attach marker components.
pub fn spawn_slider_control(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    slider: MaterialSlider,
) -> Entity {
    spawn_slider_control_with(parent, theme, slider, ())
}

/// Spawn a standalone slider control (no label wrapper) as a child, inserting extra components.
///
/// Returns the slider entity id so callers can reference it later.
pub fn spawn_slider_control_with<E: Bundle>(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    slider: MaterialSlider,
    extra: E,
) -> Entity {
    let orientation = slider.orientation;
    let direction = slider.direction;
    let track_color = slider.inactive_track_color(theme);
    let active_color = slider.active_track_color(theme);
    let handle_color = slider.handle_color(theme);

    let value_percent = slider.normalized_value().clamp(0.0, 1.0);
    let position_percent = slider.position_percent().clamp(0.0, 1.0);

    let show_ticks = slider.show_ticks;
    let show_ticks_now = slider.should_show_ticks();
    let step = slider.step;
    let min = slider.min;
    let max = slider.max;
    let track_height = slider.track_height;
    let thumb_radius = slider.thumb_radius;
    let active_tick_color = slider.active_tick_color(theme);
    let inactive_tick_color = slider.inactive_tick_color(theme);

    let mut slider_ec = parent.spawn((
        slider,
        Button,
        Interaction::None,
        extra,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            // Allow flexbox to shrink this item on compact widths.
            min_width: Val::Px(0.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: match orientation {
                SliderOrientation::Horizontal => FlexDirection::Row,
                SliderOrientation::Vertical => FlexDirection::Column,
            },
            ..default()
        },
        BackgroundColor(Color::NONE),
    ));

    let mut parts_track: Option<Entity> = None;
    let mut parts_active_track: Option<Entity> = None;
    let mut parts_handle: Option<Entity> = None;
    let mut parts_ticks: Vec<Entity> = Vec::new();

    slider_ec.with_children(|slider_area| {
        // Track
        let track_node = match orientation {
            SliderOrientation::Horizontal => Node {
                width: Val::Percent(100.0),
                height: Val::Px(track_height),
                ..default()
            },
            SliderOrientation::Vertical => Node {
                width: Val::Px(track_height),
                height: Val::Percent(100.0),
                ..default()
            },
        };
        let track_entity = slider_area
            .spawn((
                SliderTrack,
                track_node,
                BackgroundColor(track_color),
                BorderRadius::all(Val::Px(track_height / 2.0)),
            ))
            .id();
        parts_track = Some(track_entity);

        // Active track
        let active_node = match orientation {
            SliderOrientation::Horizontal => {
                let (left, width) = match direction {
                    SliderDirection::StartToEnd => {
                        (Val::Px(0.0), Val::Percent(position_percent * 100.0))
                    }
                    SliderDirection::EndToStart => (
                        Val::Percent(position_percent * 100.0),
                        Val::Percent((1.0 - position_percent) * 100.0),
                    ),
                };
                Node {
                    position_type: PositionType::Absolute,
                    left,
                    top: Val::Px((SLIDER_HANDLE_SIZE + 8.0 - track_height) / 2.0),
                    width,
                    height: Val::Px(track_height),
                    ..default()
                }
            }
            SliderOrientation::Vertical => {
                let (top, height) = match direction {
                    SliderDirection::StartToEnd => {
                        (Val::Px(0.0), Val::Percent(position_percent * 100.0))
                    }
                    SliderDirection::EndToStart => (
                        Val::Percent(position_percent * 100.0),
                        Val::Percent((1.0 - position_percent) * 100.0),
                    ),
                };
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    margin: UiRect::left(Val::Px(-track_height / 2.0)),
                    top,
                    width: Val::Px(track_height),
                    height,
                    ..default()
                }
            }
        };
        let active_track_entity = slider_area
            .spawn((
                SliderActiveTrack {
                    track: track_entity,
                },
                active_node,
                BackgroundColor(active_color),
                BorderRadius::all(Val::Px(track_height / 2.0)),
            ))
            .id();
        parts_active_track = Some(active_track_entity);

        // Handle
        let handle_node = match orientation {
            SliderOrientation::Horizontal => Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(position_percent * 100.0),
                margin: UiRect::left(Val::Px(-thumb_radius)),
                width: Val::Px(thumb_radius * 2.0),
                height: Val::Px(thumb_radius * 2.0),
                ..default()
            },
            SliderOrientation::Vertical => Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(position_percent * 100.0),
                left: Val::Percent(50.0),
                margin: UiRect {
                    left: Val::Px(-thumb_radius),
                    right: Val::Px(0.0),
                    top: Val::Px(-thumb_radius),
                    bottom: Val::Px(0.0),
                },
                width: Val::Px(thumb_radius * 2.0),
                height: Val::Px(thumb_radius * 2.0),
                ..default()
            },
        };
        let handle_entity = slider_area
            .spawn((
                SliderHandle {
                    min,
                    max,
                    value: min + value_percent * (max - min),
                    track: track_entity,
                    step,
                },
                handle_node,
                BackgroundColor(handle_color),
                BorderRadius::all(Val::Px(thumb_radius)),
            ))
            .id();
        parts_handle = Some(handle_entity);

        // Tick marks (discrete)
        if show_ticks {
            if let Some(step_size) = step {
                let num_ticks = ((max - min) / step_size) as usize + 1;
                for i in 0..num_ticks {
                    let pos = i as f32 / (num_ticks - 1) as f32;
                    let tick_active = match direction {
                        SliderDirection::StartToEnd => pos <= position_percent,
                        SliderDirection::EndToStart => pos >= position_percent,
                    };
                    let tick_color = if tick_active {
                        active_tick_color
                    } else {
                        inactive_tick_color
                    };

                    let tick_node = match orientation {
                        SliderOrientation::Horizontal => Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(pos * 100.0),
                            margin: UiRect::left(Val::Px(-1.0)),
                            top: Val::Px((SLIDER_HANDLE_SIZE + 8.0 + track_height) / 2.0),
                            width: Val::Px(2.0),
                            height: Val::Px(4.0),
                            ..default()
                        },
                        SliderOrientation::Vertical => Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(pos * 100.0),
                            margin: UiRect::top(Val::Px(-1.0)),
                            left: Val::Percent(50.0),
                            width: Val::Px(4.0),
                            height: Val::Px(2.0),
                            ..default()
                        },
                    };

                    let tick_entity = slider_area
                        .spawn((
                            SliderTick { position: pos },
                            tick_node,
                            BackgroundColor(tick_color),
                            if show_ticks_now {
                                Visibility::Visible
                            } else {
                                Visibility::Hidden
                            },
                        ))
                        .id();
                    parts_ticks.push(tick_entity);
                }
            }
        }
    });

    if let (Some(track), Some(active_track), Some(handle)) =
        (parts_track, parts_active_track, parts_handle)
    {
        slider_ec.insert(SliderParts {
            track,
            active_track,
            handle,
            ticks: parts_ticks,
        });
    }

    slider_ec.id()
}

/// Builder for sliders
pub struct SliderBuilder {
    slider: MaterialSlider,
    width: Val,
}

impl SliderBuilder {
    /// Create a new slider builder
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            slider: MaterialSlider::new(min, max),
            width: Val::Px(200.0),
        }
    }

    /// Set initial value
    pub fn value(mut self, value: f32) -> Self {
        self.slider.value = value.clamp(self.slider.min, self.slider.max);
        self
    }

    /// Set step size
    pub fn step(mut self, step: f32) -> Self {
        self.slider.step = Some(step);
        self
    }

    /// Show tick marks
    pub fn ticks(mut self) -> Self {
        self.slider.show_ticks = true;
        self.slider.tick_visibility = TickVisibility::Always;
        self
    }

    /// Show value label
    pub fn label(mut self) -> Self {
        self.slider.show_label = true;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.slider.disabled = disabled;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the slider bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let _bg_color = self.slider.inactive_track_color(theme);

        (
            self.slider,
            Button,
            Node {
                width: self.width,
                height: Val::Px(SLIDER_HANDLE_SIZE + 8.0), // Extra space for handle
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
        )
    }
}

/// Marker component for slider track
#[derive(Component)]
pub struct SliderTrack;

/// Marker component for slider active track (fill portion showing current value)
#[derive(Component)]
pub struct SliderActiveTrack {
    /// Reference to the parent track entity
    pub track: Entity,
}

/// Marker component for slider handle (thumb)
#[derive(Component)]
pub struct SliderHandle {
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
    /// Current value
    pub value: f32,
    /// Reference to the parent track entity
    pub track: Entity,
    /// Optional step for discrete sliders
    pub step: Option<f32>,
}

/// Marker component for slider value label
#[derive(Component)]
pub struct SliderLabel {
    /// Reference to the parent track entity
    pub track: Entity,
}

/// Extension trait to spawn sliders with full visual hierarchy
pub trait SpawnSliderChild {
    /// Spawn a continuous slider with a label
    fn spawn_slider(
        &mut self,
        theme: &MaterialTheme,
        min: f32,
        max: f32,
        value: f32,
        label: Option<&str>,
    );

    /// Spawn a discrete slider with tick marks
    fn spawn_discrete_slider(
        &mut self,
        theme: &MaterialTheme,
        min: f32,
        max: f32,
        value: f32,
        step: f32,
        label: Option<&str>,
    );

    /// Spawn a slider using a builder for more control
    fn spawn_slider_with(
        &mut self,
        theme: &MaterialTheme,
        slider: MaterialSlider,
        label: Option<&str>,
    );
}

impl SpawnSliderChild for ChildSpawnerCommands<'_> {
    fn spawn_slider(
        &mut self,
        theme: &MaterialTheme,
        min: f32,
        max: f32,
        value: f32,
        label: Option<&str>,
    ) {
        let slider = MaterialSlider::new(min, max).with_value(value);
        self.spawn_slider_with(theme, slider, label);
    }

    fn spawn_discrete_slider(
        &mut self,
        theme: &MaterialTheme,
        min: f32,
        max: f32,
        value: f32,
        step: f32,
        label: Option<&str>,
    ) {
        let mut slider = MaterialSlider::new(min, max)
            .with_value(value)
            .with_step(step);
        slider.show_ticks = true;
        slider.tick_visibility = TickVisibility::Always;
        self.spawn_slider_with(theme, slider, label);
    }

    fn spawn_slider_with(
        &mut self,
        theme: &MaterialTheme,
        slider: MaterialSlider,
        label: Option<&str>,
    ) {
        let label_color = theme.on_surface;
        // Container row with optional label
        self.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(16.0),
            row_gap: Val::Px(8.0),
            flex_wrap: FlexWrap::Wrap,
            width: Val::Percent(100.0),
            ..default()
        })
        .with_children(|row| {
            // Optional left label
            if let Some(label_text) = label {
                row.spawn((
                    Text::new(label_text),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(label_color),
                ));
            }

            // Slider container: caller can size the row child; slider fills 100%.
            row.spawn(Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                min_width: Val::Px(0.0),
                height: Val::Px(SLIDER_HANDLE_SIZE + 8.0),
                ..default()
            })
            .with_children(|slot| {
                spawn_slider_control(slot, theme, slider);
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // SliderVariant Tests
    // ============================================================================

    #[test]
    fn test_slider_variant_default() {
        assert_eq!(SliderVariant::default(), SliderVariant::Continuous);
    }

    #[test]
    fn test_slider_variants_distinct() {
        assert_ne!(SliderVariant::Continuous, SliderVariant::Discrete);
    }

    // ============================================================================
    // TickVisibility Tests
    // ============================================================================

    #[test]
    fn test_tick_visibility_default() {
        assert_eq!(TickVisibility::default(), TickVisibility::Never);
    }

    #[test]
    fn test_tick_visibility_all_modes() {
        let modes = [
            TickVisibility::Always,
            TickVisibility::WhenDragging,
            TickVisibility::Never,
        ];

        for i in 0..modes.len() {
            for j in (i + 1)..modes.len() {
                assert_ne!(modes[i], modes[j]);
            }
        }
    }

    // ============================================================================
    // MaterialSlider Tests
    // ============================================================================

    #[test]
    fn test_slider_new_defaults() {
        let slider = MaterialSlider::new(0.0, 100.0);
        assert_eq!(slider.value, 0.0);
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
        assert!(slider.step.is_none());
        assert!(slider.discrete_value_count.is_none());
        assert!(!slider.show_ticks);
        assert_eq!(slider.tick_visibility, TickVisibility::Never);
        assert!(!slider.show_label);
        assert!(!slider.disabled);
        assert!(slider.anchor_value.is_none());
        assert!(!slider.dragging);
        assert!(!slider.hovered);
        assert!(!slider.focused);
    }

    #[test]
    fn test_slider_with_value() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(50.0);
        assert_eq!(slider.value, 50.0);
    }

    #[test]
    fn test_slider_with_value_clamped_min() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(-10.0);
        assert_eq!(slider.value, 0.0);
    }

    #[test]
    fn test_slider_with_value_clamped_max() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(150.0);
        assert_eq!(slider.value, 100.0);
    }

    #[test]
    fn test_slider_with_step() {
        let slider = MaterialSlider::new(0.0, 100.0).with_step(10.0);
        assert_eq!(slider.step, Some(10.0));
    }

    #[test]
    fn test_slider_discrete() {
        let slider = MaterialSlider::new(0.0, 100.0).discrete(5);
        assert_eq!(slider.discrete_value_count, Some(5));
        assert_eq!(slider.step, Some(25.0)); // (100-0)/(5-1) = 25
    }

    #[test]
    fn test_slider_anchor() {
        let slider = MaterialSlider::new(0.0, 100.0).anchor(50.0);
        assert_eq!(slider.anchor_value, Some(50.0));
    }

    #[test]
    fn test_slider_anchor_clamped() {
        let slider = MaterialSlider::new(0.0, 100.0).anchor(150.0);
        assert_eq!(slider.anchor_value, Some(100.0));
    }

    #[test]
    fn test_slider_track_height() {
        let slider = MaterialSlider::new(0.0, 100.0).track_height(8.0);
        assert_eq!(slider.track_height, 8.0);
    }

    #[test]
    fn test_slider_thumb_radius() {
        let slider = MaterialSlider::new(0.0, 100.0).thumb_radius(12.0);
        assert_eq!(slider.thumb_radius, 12.0);
    }

    #[test]
    fn test_slider_thumb_elevation() {
        let slider = MaterialSlider::new(0.0, 100.0).thumb_elevation(4.0);
        assert_eq!(slider.thumb_elevation, 4.0);
    }

    #[test]
    fn test_slider_show_ticks() {
        let slider = MaterialSlider::new(0.0, 100.0).show_ticks();
        assert!(slider.show_ticks);
        assert_eq!(slider.tick_visibility, TickVisibility::Always);
    }

    #[test]
    fn test_slider_tick_visibility() {
        let slider = MaterialSlider::new(0.0, 100.0).tick_visibility(TickVisibility::WhenDragging);
        assert_eq!(slider.tick_visibility, TickVisibility::WhenDragging);
        assert!(slider.show_ticks);
    }

    #[test]
    fn test_slider_tick_visibility_never() {
        let slider = MaterialSlider::new(0.0, 100.0).tick_visibility(TickVisibility::Never);
        assert_eq!(slider.tick_visibility, TickVisibility::Never);
        assert!(!slider.show_ticks);
    }

    #[test]
    fn test_slider_show_label() {
        let slider = MaterialSlider::new(0.0, 100.0).show_label();
        assert!(slider.show_label);
    }

    #[test]
    fn test_slider_disabled() {
        let slider = MaterialSlider::new(0.0, 100.0).disabled(true);
        assert!(slider.disabled);
    }

    #[test]
    fn test_slider_formatted_value_default() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(42.7);
        assert_eq!(slider.formatted_value(), "43"); // Rounded
    }

    #[test]
    fn test_slider_formatted_value_custom() {
        fn custom_formatter(v: f32) -> String {
            format!("{}%", v as i32)
        }
        let slider = MaterialSlider::new(0.0, 100.0)
            .with_value(75.0)
            .value_formatter(custom_formatter);
        assert_eq!(slider.formatted_value(), "75%");
    }

    #[test]
    fn test_slider_normalized_value() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(50.0);
        assert_eq!(slider.normalized_value(), 0.5);
    }

    #[test]
    fn test_slider_normalized_value_min() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(0.0);
        assert_eq!(slider.normalized_value(), 0.0);
    }

    #[test]
    fn test_slider_normalized_value_max() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(100.0);
        assert_eq!(slider.normalized_value(), 1.0);
    }

    #[test]
    fn test_slider_set_from_normalized() {
        let mut slider = MaterialSlider::new(0.0, 100.0);
        slider.set_from_normalized(0.5);
        assert_eq!(slider.value, 50.0);
    }

    #[test]
    fn test_slider_set_from_normalized_with_step() {
        let mut slider = MaterialSlider::new(0.0, 100.0).with_step(10.0);
        slider.set_from_normalized(0.55); // Should snap to 60
        assert_eq!(slider.value, 60.0);
    }

    #[test]
    fn test_slider_should_show_ticks_always() {
        let slider = MaterialSlider::new(0.0, 100.0)
            .discrete(5)
            .tick_visibility(TickVisibility::Always);
        assert!(slider.should_show_ticks());
    }

    #[test]
    fn test_slider_should_show_ticks_when_dragging_not_dragging() {
        let slider = MaterialSlider::new(0.0, 100.0)
            .discrete(5)
            .tick_visibility(TickVisibility::WhenDragging);
        assert!(!slider.should_show_ticks());
    }

    #[test]
    fn test_slider_should_show_ticks_when_dragging_and_dragging() {
        let mut slider = MaterialSlider::new(0.0, 100.0)
            .discrete(5)
            .tick_visibility(TickVisibility::WhenDragging);
        slider.dragging = true;
        assert!(slider.should_show_ticks());
    }

    #[test]
    fn test_slider_should_show_ticks_never() {
        let slider = MaterialSlider::new(0.0, 100.0)
            .discrete(5)
            .tick_visibility(TickVisibility::Never);
        assert!(!slider.should_show_ticks());
    }

    // ============================================================================
    // Constants Tests
    // ============================================================================

    #[test]
    fn test_slider_track_height_constant() {
        assert_eq!(SLIDER_TRACK_HEIGHT, 4.0);
    }

    #[test]
    fn test_slider_handle_size_constant() {
        assert_eq!(SLIDER_HANDLE_SIZE, 20.0);
    }

    // ============================================================================
    // SliderBuilder Tests
    // ============================================================================

    #[test]
    fn test_slider_builder_new() {
        let builder = SliderBuilder::new(0.0, 100.0);
        assert_eq!(builder.slider.min, 0.0);
        assert_eq!(builder.slider.max, 100.0);
    }

    #[test]
    fn test_slider_builder_value() {
        let builder = SliderBuilder::new(0.0, 100.0).value(25.0);
        assert_eq!(builder.slider.value, 25.0);
    }

    #[test]
    fn test_slider_builder_step() {
        let builder = SliderBuilder::new(0.0, 100.0).step(5.0);
        assert_eq!(builder.slider.step, Some(5.0));
    }

    #[test]
    fn test_slider_builder_ticks() {
        let builder = SliderBuilder::new(0.0, 100.0).ticks();
        assert!(builder.slider.show_ticks);
    }

    #[test]
    fn test_slider_builder_label() {
        let builder = SliderBuilder::new(0.0, 100.0).label();
        assert!(builder.slider.show_label);
    }

    #[test]
    fn test_slider_builder_disabled() {
        let builder = SliderBuilder::new(0.0, 100.0).disabled(true);
        assert!(builder.slider.disabled);
    }

    #[test]
    fn test_slider_builder_full_chain() {
        let builder = SliderBuilder::new(0.0, 100.0)
            .value(50.0)
            .step(10.0)
            .ticks()
            .label()
            .disabled(false);

        assert_eq!(builder.slider.value, 50.0);
        assert_eq!(builder.slider.step, Some(10.0));
        assert!(builder.slider.show_ticks);
        assert!(builder.slider.show_label);
        assert!(!builder.slider.disabled);
    }
}
