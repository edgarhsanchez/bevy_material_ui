//! Material Design 3 Animation System
//!
//! Provides animation utilities for Material Design transitions and transformations.
//! Reference: <https://m3.material.io/styles/motion/overview>

use bevy::prelude::*;

use crate::tokens::{Duration, Easing};

/// Plugin for animation system
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animation_system, spring_animation_system));
    }
}

// ============================================================================
// Components
// ============================================================================

/// Animated value that interpolates over time
#[derive(Component)]
pub struct AnimatedValue<T: Clone> {
    pub start: T,
    pub target: T,
    pub current: T,
    pub progress: f32,
    pub duration: f32,
    pub easing: Easing,
}

impl<T: Clone> AnimatedValue<T> {
    pub fn new(start: T, target: T, duration: f32) -> Self {
        Self {
            current: start.clone(),
            start,
            target,
            progress: 0.0,
            duration,
            easing: Easing::Standard,
        }
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

/// Spring-based animation for natural motion
#[derive(Component)]
pub struct SpringAnimation {
    pub value: f32,
    pub velocity: f32,
    pub target: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl SpringAnimation {
    pub fn new(value: f32, target: f32) -> Self {
        Self {
            value,
            velocity: 0.0,
            target,
            stiffness: 200.0,
            damping: 0.6,
        }
    }

    pub fn with_stiffness(mut self, stiffness: f32) -> Self {
        self.stiffness = stiffness;
        self
    }

    pub fn with_damping(mut self, damping: f32) -> Self {
        self.damping = damping;
        self
    }

    pub fn animate_to(&mut self, target: f32) {
        self.target = target;
    }
}

/// Morph animation between shapes (for loading indicator)
#[derive(Component)]
pub struct MorphAnimation {
    pub morph_fraction: f32,
    pub rotation: f32,
    pub color_index: usize,
    pub speed: f32,
}

impl Default for MorphAnimation {
    fn default() -> Self {
        Self {
            morph_fraction: 0.0,
            rotation: 0.0,
            color_index: 0,
            speed: 1.0,
        }
    }
}

// ============================================================================
// Systems
// ============================================================================

fn animation_system(time: Res<Time>, mut animations: Query<&mut AnimatedValue<f32>>) {
    for mut anim in animations.iter_mut() {
        if anim.is_complete() {
            continue;
        }

        anim.progress += time.delta_secs() / anim.duration;
        anim.progress = anim.progress.min(1.0);

        // Apply easing
        let t = apply_easing(anim.progress, anim.easing);
        anim.current = anim.start + (anim.target - anim.start) * t;
    }
}

fn spring_animation_system(time: Res<Time>, mut springs: Query<&mut SpringAnimation>) {
    let dt = time.delta_secs();

    for mut spring in springs.iter_mut() {
        // Spring physics
        let displacement = spring.value - spring.target;
        let spring_force = -spring.stiffness * displacement;
        let damping_force = -spring.damping * spring.velocity;

        spring.velocity += (spring_force + damping_force) * dt;
        spring.value += spring.velocity * dt;

        // Clamp to prevent oscillation around target
        if displacement.abs() < 0.001 && spring.velocity.abs() < 0.001 {
            spring.value = spring.target;
            spring.velocity = 0.0;
        }
    }
}

fn apply_easing(t: f32, easing: Easing) -> f32 {
    let (_c1, c2, _c3, c4) = easing.control_points();

    // Simplified cubic bezier approximation
    // For production, use a proper cubic bezier solver
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    mt3 * 0.0 + 3.0 * mt2 * t * c2 + 3.0 * mt * t2 * c4 + t3 * 1.0
}

// ============================================================================
// FAB Transformation
// ============================================================================

/// FAB transformation state
#[derive(Component)]
pub struct FabTransformation {
    pub state: FabTransformState,
    pub progress: f32,
    pub duration: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FabTransformState {
    Collapsed,
    Expanding,
    Expanded,
    Collapsing,
}

impl FabTransformation {
    pub fn new() -> Self {
        Self {
            state: FabTransformState::Collapsed,
            progress: 0.0,
            duration: Duration::MEDIUM2,
        }
    }

    pub fn expand(&mut self) {
        if self.state == FabTransformState::Collapsed {
            self.state = FabTransformState::Expanding;
            self.progress = 0.0;
        }
    }

    pub fn collapse(&mut self) {
        if self.state == FabTransformState::Expanded {
            self.state = FabTransformState::Collapsing;
            self.progress = 0.0;
        }
    }
}

impl Default for FabTransformation {
    fn default() -> Self {
        Self::new()
    }
}
