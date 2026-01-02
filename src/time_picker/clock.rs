//! Clock face rendering and layout logic

use bevy::prelude::*;
use std::f32::consts::PI;

/// Clock face layout helper
pub struct ClockFaceLayout {
    pub radius: f32,
    pub inner_radius: f32, // For 24H dual-level
}

impl ClockFaceLayout {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            inner_radius: radius * 0.6,
        }
    }

    /// Calculate position for a clock number
    pub fn number_position(&self, value: u8, total: u8, is_inner: bool) -> Vec2 {
        let angle = (value as f32 / total as f32) * 2.0 * PI - PI / 2.0;
        let r = if is_inner {
            self.inner_radius
        } else {
            self.radius * 0.85 // Slightly inside the edge
        };

        Vec2::new(r * angle.cos(), r * angle.sin())
    }

    /// Calculate clock hand rotation and length
    pub fn hand_transform(&self, value: u8, total: u8, is_inner: bool) -> (f32, f32) {
        let angle = (value as f32 / total as f32) * 2.0 * PI - PI / 2.0;
        let length = if is_inner {
            self.inner_radius
        } else {
            self.radius * 0.7
        };

        (angle, length)
    }
}

/// Clock hand component
#[derive(Component)]
pub struct ClockHand {
    pub angle: f32,
    pub length: f32,
}

impl ClockHand {
    pub fn new(angle: f32, length: f32) -> Self {
        Self { angle, length }
    }

    pub fn for_hour(hour: u8, is_24h: bool) -> Self {
        let (value, total) = if is_24h {
            (hour % 24, 24)
        } else {
            (hour % 12, 12)
        };

        let angle = (value as f32 / total as f32) * 2.0 * PI - PI / 2.0;
        Self::new(angle, 100.0) // Length will be scaled
    }

    pub fn for_minute(minute: u8) -> Self {
        let angle = (minute as f32 / 60.0) * 2.0 * PI - PI / 2.0;
        Self::new(angle, 120.0)
    }
}
