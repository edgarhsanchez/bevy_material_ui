//! Motion and animation utilities for Material Design 3
//!
//! This module provides easing functions, spring physics, and animation utilities
//! following the Material Design 3 motion guidelines.
//!
//! Reference: <https://m3.material.io/styles/motion/overview>

use bevy::prelude::*;

use crate::tokens::{Duration, Easing};

/// Plugin for motion and animation systems
pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_state_layers);
    }
}

// ============================================================================
// Easing Functions
// ============================================================================

/// Evaluate a cubic bezier curve at time t
///
/// Control points are (x1, y1, x2, y2) where:
/// - P0 = (0, 0) is the start
/// - P1 = (x1, y1) is the first control point
/// - P2 = (x2, y2) is the second control point
/// - P3 = (1, 1) is the end
pub fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // Binary search for the t value that gives us the x we want
    let mut low = 0.0_f32;
    let mut high = 1.0_f32;
    let mut mid;

    // Find the parameter value that corresponds to our input t (which is actually x)
    for _ in 0..20 {
        mid = (low + high) / 2.0;
        let x = bezier_component(mid, x1, x2);
        if (x - t).abs() < 0.0001 {
            return bezier_component(mid, y1, y2);
        }
        if x < t {
            low = mid;
        } else {
            high = mid;
        }
    }

    bezier_component((low + high) / 2.0, y1, y2)
}

/// Calculate one component of a cubic bezier curve
fn bezier_component(t: f32, p1: f32, p2: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;

    // Cubic bezier: (1-t)³P0 + 3(1-t)²tP1 + 3(1-t)t²P2 + t³P3
    // P0 = 0, P3 = 1
    3.0 * mt2 * t * p1 + 3.0 * mt * t2 * p2 + t3
}

/// Apply an easing curve to a progress value (0.0 to 1.0)
pub fn ease(t: f32, easing: Easing) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let (x1, y1, x2, y2) = easing.control_points();
    cubic_bezier(t, x1, y1, x2, y2)
}

/// Standard easing - for most transitions
pub fn ease_standard(t: f32) -> f32 {
    ease(t, Easing::Standard)
}

/// Standard accelerate - for exiting elements
pub fn ease_standard_accelerate(t: f32) -> f32 {
    ease(t, Easing::StandardAccelerate)
}

/// Standard decelerate - for entering elements
pub fn ease_standard_decelerate(t: f32) -> f32 {
    ease(t, Easing::StandardDecelerate)
}

/// Emphasized easing - for high-emphasis transitions
pub fn ease_emphasized(t: f32) -> f32 {
    ease(t, Easing::Emphasized)
}

/// Emphasized accelerate - for exiting with emphasis
pub fn ease_emphasized_accelerate(t: f32) -> f32 {
    ease(t, Easing::EmphasizedAccelerate)
}

/// Emphasized decelerate - for entering with emphasis
pub fn ease_emphasized_decelerate(t: f32) -> f32 {
    ease(t, Easing::EmphasizedDecelerate)
}

/// Simple ease out cubic (fast start, slow end)
pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

/// Simple ease in cubic (slow start, fast end)
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease in-out cubic (slow start, fast middle, slow end)
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// ============================================================================
// Spring Physics
// ============================================================================

/// Spring animation parameters
#[derive(Debug, Clone, Copy)]
pub struct SpringConfig {
    /// Stiffness of the spring (higher = faster oscillation)
    pub stiffness: f32,
    /// Damping ratio (1.0 = critically damped, <1.0 = oscillates, >1.0 = overdamped)
    pub damping: f32,
    /// Mass of the object
    pub mass: f32,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 300.0,
            damping: 20.0,
            mass: 1.0,
        }
    }
}

impl SpringConfig {
    /// Create a bouncy spring (underdamped)
    pub fn bouncy() -> Self {
        Self {
            stiffness: 400.0,
            damping: 15.0,
            mass: 1.0,
        }
    }

    /// Create a smooth spring (critically damped)
    pub fn smooth() -> Self {
        Self {
            stiffness: 300.0,
            damping: 30.0,
            mass: 1.0,
        }
    }

    /// Create a stiff spring (fast, minimal overshoot)
    pub fn stiff() -> Self {
        Self {
            stiffness: 500.0,
            damping: 40.0,
            mass: 1.0,
        }
    }

    /// Create a gentle spring (slow, smooth)
    pub fn gentle() -> Self {
        Self {
            stiffness: 150.0,
            damping: 20.0,
            mass: 1.0,
        }
    }
}

/// Spring animation state
#[derive(Component, Debug, Clone)]
pub struct SpringAnimation {
    /// Current value
    pub value: f32,
    /// Target value
    pub target: f32,
    /// Current velocity
    pub velocity: f32,
    /// Spring configuration
    pub config: SpringConfig,
    /// Whether the spring has settled
    pub settled: bool,
}

impl SpringAnimation {
    /// Create a new spring animation
    pub fn new(initial: f32, target: f32, config: SpringConfig) -> Self {
        Self {
            value: initial,
            target,
            velocity: 0.0,
            config,
            settled: false,
        }
    }

    /// Update the spring animation
    pub fn update(&mut self, dt: f32) {
        if self.settled {
            return;
        }

        let SpringConfig {
            stiffness,
            damping,
            mass,
        } = self.config;

        // Spring force: F = -kx - cv
        let displacement = self.value - self.target;
        let spring_force = -stiffness * displacement;
        let damping_force = -damping * self.velocity;
        let acceleration = (spring_force + damping_force) / mass;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        // Check if settled (close enough to target with low velocity)
        if displacement.abs() < 0.001 && self.velocity.abs() < 0.001 {
            self.value = self.target;
            self.velocity = 0.0;
            self.settled = true;
        }
    }

    /// Set a new target value
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
        self.settled = false;
    }

    /// Get the current progress (0.0 to 1.0) from initial to target
    pub fn progress(&self) -> f32 {
        self.value
    }
}

// ============================================================================
// State Layer Animation
// ============================================================================

/// State layer for hover/pressed visual feedback
///
/// Material Design 3 uses state layers as semi-transparent overlays
/// to indicate interaction states.
#[derive(Component, Debug, Clone)]
pub struct StateLayer {
    /// Current opacity (0.0 to 1.0, typically 0.0, 0.08, or 0.12)
    pub opacity: f32,
    /// Target opacity based on current state
    pub target_opacity: f32,
    /// Animation timer
    pub timer: Timer,
    /// Whether currently animating
    pub animating: bool,
    /// Base color for the state layer
    pub color: Color,
}

impl Default for StateLayer {
    fn default() -> Self {
        Self {
            opacity: 0.0,
            target_opacity: 0.0,
            timer: Timer::from_seconds(Duration::SHORT3, TimerMode::Once),
            animating: false,
            color: Color::WHITE,
        }
    }
}

impl StateLayer {
    /// Create a new state layer with a specific color
    pub fn new(color: Color) -> Self {
        Self { color, ..default() }
    }

    /// Create a state layer for primary-colored content
    pub fn on_primary(theme_primary: Color) -> Self {
        Self::new(theme_primary)
    }

    /// Set state to hovered (8% opacity)
    pub fn set_hovered(&mut self) {
        self.set_target(Self::HOVER_OPACITY);
    }

    /// Set state to focused (12% opacity)
    pub fn set_focused(&mut self) {
        self.set_target(Self::FOCUS_OPACITY);
    }

    /// Set state to pressed (12% opacity)
    pub fn set_pressed(&mut self) {
        self.set_target(Self::PRESSED_OPACITY);
    }

    /// Set state to dragged (16% opacity)
    pub fn set_dragged(&mut self) {
        self.set_target(Self::DRAGGED_OPACITY);
    }

    /// Clear the state layer
    pub fn clear(&mut self) {
        self.set_target(0.0);
    }

    /// Set a custom target opacity
    pub fn set_target(&mut self, target: f32) {
        if (self.target_opacity - target).abs() > 0.001 {
            self.target_opacity = target;
            self.animating = true;
            self.timer = Timer::from_seconds(Duration::SHORT3, TimerMode::Once);
        }
    }

    /// Update the animation
    pub fn update(&mut self, dt: f32) {
        if !self.animating {
            return;
        }

        self.timer.tick(std::time::Duration::from_secs_f32(dt));
        let progress = ease_standard_decelerate(self.timer.fraction());

        let start = self.opacity;
        let end = self.target_opacity;
        self.opacity = start + (end - start) * progress;

        if self.timer.is_finished() {
            self.opacity = self.target_opacity;
            self.animating = false;
        }
    }

    /// Get the current color with opacity applied
    pub fn current_color(&self) -> Color {
        self.color.with_alpha(self.opacity)
    }

    // Standard opacity values from MD3
    /// Opacity for hover state (8%)
    pub const HOVER_OPACITY: f32 = 0.08;
    /// Opacity for focus state (12%)
    pub const FOCUS_OPACITY: f32 = 0.12;
    /// Opacity for pressed state (12%)
    pub const PRESSED_OPACITY: f32 = 0.12;
    /// Opacity for dragged state (16%)
    pub const DRAGGED_OPACITY: f32 = 0.16;
}

/// System to animate state layers
fn animate_state_layers(
    time: Res<Time>,
    mut state_layers: Query<(&mut StateLayer, Option<&mut BackgroundColor>)>,
) {
    for (mut layer, bg_color) in state_layers.iter_mut() {
        layer.update(time.delta_secs());

        if let Some(mut bg) = bg_color {
            *bg = BackgroundColor(layer.current_color());
        }
    }
}

// ============================================================================
// Animated Value
// ============================================================================

/// A value that animates smoothly between states
#[derive(Component, Debug, Clone)]
pub struct AnimatedValue {
    /// Current value
    pub current: f32,
    /// Target value
    pub target: f32,
    /// Animation duration in seconds
    pub duration: f32,
    /// Elapsed time
    pub elapsed: f32,
    /// Start value (for interpolation)
    pub start: f32,
    /// Easing function to use
    pub easing: Easing,
    /// Whether animation is complete
    pub complete: bool,
}

impl AnimatedValue {
    /// Create a new animated value
    pub fn new(initial: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            duration: Duration::MEDIUM2,
            elapsed: 0.0,
            start: initial,
            easing: Easing::Standard,
            complete: true,
        }
    }

    /// Set the target value
    pub fn set_target(&mut self, target: f32) {
        if (self.target - target).abs() > 0.0001 {
            self.start = self.current;
            self.target = target;
            self.elapsed = 0.0;
            self.complete = false;
        }
    }

    /// Set the animation duration
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Update the animation
    pub fn update(&mut self, dt: f32) {
        if self.complete {
            return;
        }

        self.elapsed += dt;
        let progress = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let eased = ease(progress, self.easing);

        self.current = self.start + (self.target - self.start) * eased;

        if progress >= 1.0 {
            self.current = self.target;
            self.complete = true;
        }
    }

    /// Get the current value
    pub fn value(&self) -> f32 {
        self.current
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_out_cubic() {
        assert!((ease_out_cubic(0.0) - 0.0).abs() < 0.001);
        assert!((ease_out_cubic(1.0) - 1.0).abs() < 0.001);
        // Should be more than linear at 0.5
        assert!(ease_out_cubic(0.5) > 0.5);
    }

    #[test]
    fn test_ease_in_cubic() {
        assert!((ease_in_cubic(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_cubic(1.0) - 1.0).abs() < 0.001);
        // Should be less than linear at 0.5
        assert!(ease_in_cubic(0.5) < 0.5);
    }

    #[test]
    fn test_ease_in_out_cubic() {
        assert!((ease_in_out_cubic(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_cubic(1.0) - 1.0).abs() < 0.001);
        assert!((ease_in_out_cubic(0.5) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_cubic_bezier_linear() {
        // Linear curve: (0, 0, 1, 1)
        let result = cubic_bezier(0.5, 0.0, 0.0, 1.0, 1.0);
        assert!((result - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_spring_animation() {
        let mut spring = SpringAnimation::new(0.0, 1.0, SpringConfig::smooth());

        // Simulate 2 seconds
        for _ in 0..120 {
            spring.update(1.0 / 60.0);
        }

        assert!(spring.settled);
        assert!((spring.value - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_state_layer_opacity() {
        let mut layer = StateLayer::default();
        assert!((layer.opacity - 0.0).abs() < 0.001);

        layer.set_hovered();
        assert!((layer.target_opacity - StateLayer::HOVER_OPACITY).abs() < 0.001);

        layer.set_pressed();
        assert!((layer.target_opacity - StateLayer::PRESSED_OPACITY).abs() < 0.001);
    }

    #[test]
    fn test_animated_value() {
        let mut value = AnimatedValue::new(0.0).with_duration(0.3);
        value.set_target(100.0);

        assert!(!value.complete);

        // Simulate full animation
        for _ in 0..20 {
            value.update(0.02);
        }

        assert!(value.complete);
        assert!((value.current - 100.0).abs() < 0.01);
    }
}
