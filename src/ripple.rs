//! Ripple effect for Material Design 3
//!
//! The ripple provides visual feedback when users interact with components.
//! Reference: <https://m3.material.io/foundations/interaction/states/overview>

use bevy::prelude::*;

use crate::tokens::Duration;

/// Plugin for the ripple effect system
pub struct RipplePlugin;

impl Plugin for RipplePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnRipple>()
            .add_systems(Update, (spawn_ripple_system, animate_ripple_system));
    }
}

/// Component that enables ripple effects on an entity
#[derive(Component, Default)]
pub struct RippleHost {
    /// Color of the ripple effect
    pub color: Option<Color>,
    /// Whether ripple is unbounded (extends beyond container)
    pub unbounded: bool,
}

impl RippleHost {
    /// Create a new ripple host with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ripple color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Make the ripple unbounded
    pub fn unbounded(mut self) -> Self {
        self.unbounded = true;
        self
    }
}

/// Event to spawn a ripple effect
#[derive(Event, bevy::prelude::Message)]
pub struct SpawnRipple {
    /// The entity that hosts the ripple
    pub host: Entity,
    /// The position within the host where the ripple should start
    pub position: Vec2,
}

/// Component for active ripple animations
#[derive(Component)]
pub struct Ripple {
    /// Current scale of the ripple (0.0 to 1.0)
    pub scale: f32,
    /// Current opacity of the ripple
    pub opacity: f32,
    /// Animation timer
    pub timer: Timer,
    /// Whether the ripple is in the fade-out phase
    pub fading_out: bool,
    /// Maximum radius of the ripple
    pub max_radius: f32,
    /// Center position of the ripple
    pub center: Vec2,
    /// Color of the ripple
    pub color: Color,
}

impl Ripple {
    /// Create a new ripple
    pub fn new(center: Vec2, max_radius: f32, color: Color) -> Self {
        Self {
            scale: 0.0,
            opacity: 0.12,
            timer: Timer::from_seconds(Duration::MEDIUM4, TimerMode::Once),
            fading_out: false,
            max_radius,
            center,
            color,
        }
    }

    /// Start the fade-out phase
    pub fn start_fade_out(&mut self) {
        self.fading_out = true;
        self.timer = Timer::from_seconds(Duration::SHORT4, TimerMode::Once);
    }

    /// Check if the ripple animation is complete
    pub fn is_complete(&self) -> bool {
        self.fading_out && self.timer.is_finished()
    }
}

/// System to spawn ripple effects
fn spawn_ripple_system(
    mut commands: Commands,
    mut events: MessageReader<SpawnRipple>,
    hosts: Query<(&RippleHost, &ComputedNode, &GlobalTransform)>,
) {
    for event in events.read() {
        if let Ok((host, computed_node, _transform)) = hosts.get(event.host) {
            let size = computed_node.size();
            let max_radius = (size.x.powi(2) + size.y.powi(2)).sqrt();

            let color = host.color.unwrap_or(Color::srgba(1.0, 1.0, 1.0, 0.12));

            commands.entity(event.host).with_children(|parent| {
                parent.spawn((
                    Ripple::new(event.position, max_radius, color),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(event.position.x),
                        top: Val::Px(event.position.y),
                        width: Val::Px(0.0),
                        height: Val::Px(0.0),
                        ..default()
                    },
                    BackgroundColor(color),
                    BorderRadius::all(Val::Percent(50.0)),
                ));
            });
        }
    }
}

/// System to animate ripple effects
fn animate_ripple_system(
    mut commands: Commands,
    time: Res<Time>,
    mut ripples: Query<(Entity, &mut Ripple, &mut Node, &mut BackgroundColor)>,
) {
    for (entity, mut ripple, mut node, mut bg_color) in ripples.iter_mut() {
        ripple.timer.tick(time.delta());

        let progress = ripple.timer.fraction();

        if ripple.fading_out {
            // Fade out phase - reduce opacity
            ripple.opacity = 0.12 * (1.0 - ease_out(progress));
        } else {
            // Expand phase - grow the ripple
            ripple.scale = ease_out(progress);
        }

        // Update visual properties
        let current_radius = ripple.max_radius * ripple.scale;
        let diameter = current_radius * 2.0;

        node.width = Val::Px(diameter);
        node.height = Val::Px(diameter);
        node.left = Val::Px(ripple.center.x - current_radius);
        node.top = Val::Px(ripple.center.y - current_radius);

        *bg_color = BackgroundColor(ripple.color.with_alpha(ripple.opacity));

        // Check if expansion is complete, start fade out
        if !ripple.fading_out && ripple.timer.is_finished() {
            ripple.start_fade_out();
        }

        // Remove completed ripples
        if ripple.is_complete() {
            commands.entity(entity).despawn();
        }
    }
}

/// Ease out cubic function for smooth deceleration
fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
