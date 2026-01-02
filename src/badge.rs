//! Material Design 3 Badge component
//!
//! Badges show notifications, counts, or status information on navigation items and icons.
//!
//! Reference: <https://m3.material.io/components/badges/overview>

use bevy::prelude::*;

use crate::theme::MaterialTheme;

/// Plugin for the badge component
pub struct BadgePlugin;

impl Plugin for BadgePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_systems(Update, (badge_style_system, badge_theme_refresh_system));
    }
}

// ============================================================================
// Types
// ============================================================================

/// Badge size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeSize {
    /// Small dot badge (no content)
    Small,
    /// Large badge (with content)
    #[default]
    Large,
}

// ============================================================================
// Components
// ============================================================================

/// Material badge component
#[derive(Component)]
pub struct MaterialBadge {
    /// Badge size
    pub size: BadgeSize,
    /// Content (number or text) - None for small dot badge
    pub content: Option<String>,
    /// Maximum number to display (shows "99+" if exceeded)
    pub max: u32,
    /// Whether the badge is visible
    pub visible: bool,
}

impl MaterialBadge {
    /// Create a small dot badge
    pub fn dot() -> Self {
        Self {
            size: BadgeSize::Small,
            content: None,
            max: 999,
            visible: true,
        }
    }

    /// Create a large badge with a number
    pub fn count(count: u32) -> Self {
        Self {
            size: BadgeSize::Large,
            content: Some(Self::format_count(count, 999)),
            max: 999,
            visible: true,
        }
    }

    /// Create a large badge with text
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            size: BadgeSize::Large,
            content: Some(text.into()),
            max: 999,
            visible: true,
        }
    }

    /// Set the maximum count before showing "+"
    pub fn with_max(mut self, max: u32) -> Self {
        self.max = max;
        if let Some(ref content) = self.content {
            if let Ok(count) = content.trim_end_matches('+').parse::<u32>() {
                self.content = Some(Self::format_count(count, max));
            }
        }
        self
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Update the count
    pub fn set_count(&mut self, count: u32) {
        self.size = BadgeSize::Large;
        self.content = Some(Self::format_count(count, self.max));
    }

    /// Update the text
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.size = BadgeSize::Large;
        self.content = Some(text.into());
    }

    /// Convert to dot badge
    pub fn set_dot(&mut self) {
        self.size = BadgeSize::Small;
        self.content = None;
    }

    /// Get the display text
    pub fn display_text(&self) -> Option<&str> {
        self.content.as_deref()
    }

    /// Format a count with max limit
    fn format_count(count: u32, max: u32) -> String {
        if count > max {
            format!("{}+", max)
        } else {
            count.to_string()
        }
    }

    /// Get the badge color
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        theme.error
    }

    /// Get the content color
    pub fn content_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_error
    }

    /// Get the badge width
    pub fn width(&self) -> f32 {
        match self.size {
            BadgeSize::Small => BADGE_SIZE_SMALL,
            BadgeSize::Large => {
                // Minimum width is BADGE_SIZE_LARGE, grows with content
                if let Some(ref content) = self.content {
                    let char_count = content.len();
                    if char_count <= 1 {
                        BADGE_SIZE_LARGE
                    } else {
                        BADGE_SIZE_LARGE + (char_count as f32 - 1.0) * 6.0
                    }
                } else {
                    BADGE_SIZE_LARGE
                }
            }
        }
    }

    /// Get the badge height
    pub fn height(&self) -> f32 {
        match self.size {
            BadgeSize::Small => BADGE_SIZE_SMALL,
            BadgeSize::Large => BADGE_SIZE_LARGE,
        }
    }
}

impl Default for MaterialBadge {
    fn default() -> Self {
        Self::dot()
    }
}

/// Marker for badge content text
#[derive(Component)]
pub struct BadgeContent;

// ============================================================================
// Dimensions
// ============================================================================

/// Small badge (dot) size
pub const BADGE_SIZE_SMALL: f32 = 6.0;
/// Large badge minimum size
pub const BADGE_SIZE_LARGE: f32 = 16.0;
/// Badge horizontal padding
pub const BADGE_PADDING: f32 = 4.0;
/// Badge offset from parent edge
pub const BADGE_OFFSET: f32 = -4.0;

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating badges
pub struct BadgeBuilder {
    badge: MaterialBadge,
}

impl BadgeBuilder {
    /// Create a dot badge
    pub fn dot() -> Self {
        Self {
            badge: MaterialBadge::dot(),
        }
    }

    /// Create a count badge
    pub fn count(count: u32) -> Self {
        Self {
            badge: MaterialBadge::count(count),
        }
    }

    /// Create a text badge
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            badge: MaterialBadge::text(text),
        }
    }

    /// Set maximum count
    pub fn max(mut self, max: u32) -> Self {
        self.badge = self.badge.with_max(max);
        self
    }

    /// Build the badge bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.badge.background_color(theme);
        let width = self.badge.width();
        let height = self.badge.height();

        (
            self.badge,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(BADGE_OFFSET),
                right: Val::Px(BADGE_OFFSET),
                width: Val::Px(width),
                height: Val::Px(height),
                min_width: Val::Px(width),
                min_height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(BADGE_PADDING), Val::Px(0.0)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(height / 2.0)),
        )
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material badges as children
pub trait SpawnBadgeChild {
    /// Spawn a small badge (dot indicator)
    fn spawn_small_badge(&mut self, theme: &MaterialTheme);

    /// Spawn a large badge with count
    fn spawn_badge_count(&mut self, theme: &MaterialTheme, count: u32);

    /// Spawn a badge with text
    fn spawn_badge_text(&mut self, theme: &MaterialTheme, text: impl Into<String>);

    /// Spawn a badge with full builder control
    fn spawn_badge_with(&mut self, theme: &MaterialTheme, builder: BadgeBuilder);
}

impl SpawnBadgeChild for ChildSpawnerCommands<'_> {
    fn spawn_small_badge(&mut self, theme: &MaterialTheme) {
        // Small badge is just the dot indicator without content
        self.spawn(BadgeBuilder::count(0).build(theme));
    }

    fn spawn_badge_count(&mut self, theme: &MaterialTheme, count: u32) {
        self.spawn_badge_with(theme, BadgeBuilder::count(count));
    }

    fn spawn_badge_text(&mut self, theme: &MaterialTheme, text: impl Into<String>) {
        self.spawn_badge_with(theme, BadgeBuilder::text(text));
    }

    fn spawn_badge_with(&mut self, theme: &MaterialTheme, builder: BadgeBuilder) {
        let content = builder.badge.content.clone();
        let content_color = builder.badge.content_color(theme);

        self.spawn(builder.build(theme)).with_children(|badge| {
            // Content text (for large badges)
            if let Some(ref text) = content {
                badge.spawn((
                    BadgeContent,
                    Text::new(text),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor(content_color),
                ));
            }
        });
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a badge with content
pub fn spawn_badge(commands: &mut Commands, theme: &MaterialTheme, badge: MaterialBadge) -> Entity {
    let content = badge.content.clone();
    let content_color = badge.content_color(theme);
    let bg_color = badge.background_color(theme);
    let width = badge.width();
    let height = badge.height();
    let visible = badge.visible;

    let mut entity = commands.spawn((
        badge,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(BADGE_OFFSET),
            right: Val::Px(BADGE_OFFSET),
            width: Val::Px(width),
            height: Val::Px(height),
            min_width: Val::Px(width),
            min_height: Val::Px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(BADGE_PADDING), Val::Px(0.0)),
            display: if visible {
                Display::Flex
            } else {
                Display::None
            },
            ..default()
        },
        BackgroundColor(bg_color),
        BorderRadius::all(Val::Px(height / 2.0)),
    ));

    // Add text content for large badges
    if let Some(text) = content {
        entity.with_children(|parent| {
            parent.spawn((
                BadgeContent,
                Text::new(text),
                TextFont {
                    font_size: 11.0,
                    ..default()
                },
                TextColor(content_color),
            ));
        });
    }

    entity.id()
}

/// Spawn a badge attached to a parent (like an icon button)
pub fn spawn_badge_on(
    commands: &mut Commands,
    theme: &MaterialTheme,
    badge: MaterialBadge,
    parent: Entity,
) -> Entity {
    let entity = spawn_badge(commands, theme, badge);
    commands.entity(entity).insert(ChildOf(parent));
    entity
}

// ============================================================================
// Systems
// ============================================================================

/// System to update badge styles
fn badge_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut badges: Query<
        (
            &MaterialBadge,
            &mut Node,
            &mut BackgroundColor,
            &mut BorderRadius,
        ),
        Changed<MaterialBadge>,
    >,
    mut badge_texts: Query<(&ChildOf, &mut Text, &mut TextColor), With<BadgeContent>>,
) {
    let Some(theme) = theme else { return };

    for (badge, mut node, mut bg_color, mut border_radius) in badges.iter_mut() {
        let width = badge.width();
        let height = badge.height();

        node.width = Val::Px(width);
        node.height = Val::Px(height);
        node.min_width = Val::Px(width);
        node.min_height = Val::Px(height);
        node.display = if badge.visible {
            Display::Flex
        } else {
            Display::None
        };

        *bg_color = BackgroundColor(badge.background_color(&theme));
        *border_radius = BorderRadius::all(Val::Px(height / 2.0));
    }

    // Update text content
    for (parent, mut text, mut color) in badge_texts.iter_mut() {
        if let Ok((badge, _, _, _)) = badges.get(parent.parent()) {
            if let Some(content) = &badge.content {
                **text = content.clone();
            }
            color.0 = badge.content_color(&theme);
        }
    }
}

/// Refresh badge visuals when the theme changes.
fn badge_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut badges: Query<(
        &MaterialBadge,
        &mut Node,
        &mut BackgroundColor,
        &mut BorderRadius,
    )>,
    mut badge_texts: Query<(&ChildOf, &mut Text, &mut TextColor), With<BadgeContent>>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (badge, mut node, mut bg_color, mut border_radius) in badges.iter_mut() {
        let width = badge.width();
        let height = badge.height();

        node.width = Val::Px(width);
        node.height = Val::Px(height);
        node.min_width = Val::Px(width);
        node.min_height = Val::Px(height);
        node.display = if badge.visible {
            Display::Flex
        } else {
            Display::None
        };

        *bg_color = BackgroundColor(badge.background_color(&theme));
        *border_radius = BorderRadius::all(Val::Px(height / 2.0));
    }

    for (parent, mut text, mut color) in badge_texts.iter_mut() {
        if let Ok(badge) = badges.get(parent.parent()).map(|(b, _, _, _)| b) {
            if let Some(content) = &badge.content {
                **text = content.clone();
            }
            color.0 = badge.content_color(&theme);
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
    fn test_dot_badge() {
        let badge = MaterialBadge::dot();
        assert_eq!(badge.size, BadgeSize::Small);
        assert!(badge.content.is_none());
        assert_eq!(badge.width(), BADGE_SIZE_SMALL);
    }

    #[test]
    fn test_count_badge() {
        let badge = MaterialBadge::count(5);
        assert_eq!(badge.size, BadgeSize::Large);
        assert_eq!(badge.content, Some("5".to_string()));
    }

    #[test]
    fn test_count_badge_max() {
        let badge = MaterialBadge::count(150).with_max(99);
        assert_eq!(badge.content, Some("99+".to_string()));
    }

    #[test]
    fn test_text_badge() {
        let badge = MaterialBadge::text("NEW");
        assert_eq!(badge.content, Some("NEW".to_string()));
    }

    #[test]
    fn test_badge_width() {
        // Single digit
        let badge1 = MaterialBadge::count(5);
        assert_eq!(badge1.width(), BADGE_SIZE_LARGE);

        // Double digit
        let badge2 = MaterialBadge::count(25);
        assert!(badge2.width() > BADGE_SIZE_LARGE);

        // Triple digit
        let badge3 = MaterialBadge::count(100);
        assert!(badge3.width() > badge2.width());
    }

    #[test]
    fn test_badge_visibility() {
        let mut badge = MaterialBadge::dot();
        assert!(badge.visible);

        badge = badge.visible(false);
        assert!(!badge.visible);
    }

    #[test]
    fn test_badge_update() {
        let mut badge = MaterialBadge::dot();
        assert_eq!(badge.size, BadgeSize::Small);

        badge.set_count(10);
        assert_eq!(badge.size, BadgeSize::Large);
        assert_eq!(badge.content, Some("10".to_string()));

        badge.set_dot();
        assert_eq!(badge.size, BadgeSize::Small);
        assert!(badge.content.is_none());
    }
}
