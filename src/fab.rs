//! Material Design 3 Floating Action Button (FAB) component
//!
//! FABs represent the primary action on a screen.
//! Reference: <https://m3.material.io/components/floating-action-button/overview>
//!
//! ## Bevy 0.17 Improvements
//!
//! This module now leverages native `BoxShadow` for elevation shadows.

use bevy::prelude::*;
use bevy::ui::BoxShadow;

use crate::{
    elevation::Elevation,
    icons::IconStyle,
    ripple::RippleHost,
    theme::{blend_state_layer, MaterialTheme},
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the FAB component
pub struct FabPlugin;

impl Plugin for FabPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FabClickEvent>().add_systems(
            Update,
            (
                fab_interaction_system,
                fab_style_system,
                fab_content_style_system,
                fab_theme_refresh_system,
                fab_shadow_system,
            ),
        );
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
    }
}

/// FAB size variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FabSize {
    /// Small FAB: 40dp
    Small,
    /// Regular FAB: 56dp
    #[default]
    Regular,
    /// Large FAB: 96dp
    Large,
}

impl FabSize {
    /// Get the size in pixels
    pub fn size(&self) -> f32 {
        match self {
            FabSize::Small => 40.0,
            FabSize::Regular => 56.0,
            FabSize::Large => 96.0,
        }
    }

    /// Get the icon size for this FAB size
    pub fn icon_size(&self) -> f32 {
        match self {
            FabSize::Small => 24.0,
            FabSize::Regular => 24.0,
            FabSize::Large => 36.0,
        }
    }

    /// Get the corner radius for this FAB size
    pub fn corner_radius(&self) -> f32 {
        match self {
            FabSize::Small => CornerRadius::MEDIUM,
            FabSize::Regular => CornerRadius::LARGE,
            FabSize::Large => CornerRadius::EXTRA_LARGE,
        }
    }
}

/// FAB color variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FabColor {
    /// Primary container color (default)
    #[default]
    Primary,
    /// Surface color
    Surface,
    /// Secondary container color
    Secondary,
    /// Tertiary container color
    Tertiary,
}

/// Material FAB component
#[derive(Component)]
pub struct MaterialFab {
    /// FAB size
    pub size: FabSize,
    /// FAB color variant
    pub color: FabColor,
    /// Whether the FAB is lowered (reduced elevation)
    pub lowered: bool,
    /// Icon identifier
    pub icon: String,
    /// Optional label for extended FAB
    pub label: Option<String>,
    /// Interaction state
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialFab {
    /// Create a new FAB
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            size: FabSize::default(),
            color: FabColor::default(),
            lowered: false,
            icon: icon.into(),
            label: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the FAB size
    pub fn with_size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// Set the FAB color
    pub fn with_color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    /// Make this a lowered FAB
    pub fn lowered(mut self) -> Self {
        self.lowered = true;
        self
    }

    /// Make this an extended FAB with a label
    pub fn extended(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        let base = match self.color {
            FabColor::Primary => theme.primary_container,
            FabColor::Surface => theme.surface_container_high,
            FabColor::Secondary => theme.secondary_container,
            FabColor::Tertiary => theme.tertiary_container,
        };

        // Apply state layer
        let state_opacity = self.state_layer_opacity();
        if state_opacity > 0.0 {
            let state_color = self.content_color(theme);
            blend_state_layer(base, state_color, state_opacity)
        } else {
            base
        }
    }

    /// Get the state layer opacity
    fn state_layer_opacity(&self) -> f32 {
        if self.pressed {
            0.12
        } else if self.hovered {
            0.08
        } else {
            0.0
        }
    }

    /// Get the icon/content color
    pub fn content_color(&self, theme: &MaterialTheme) -> Color {
        match self.color {
            FabColor::Primary => theme.on_primary_container,
            FabColor::Surface => theme.primary,
            FabColor::Secondary => theme.on_secondary_container,
            FabColor::Tertiary => theme.on_tertiary_container,
        }
    }

    /// Get the elevation
    pub fn elevation(&self) -> Elevation {
        if self.lowered {
            if self.pressed {
                Elevation::Level1
            } else if self.hovered {
                Elevation::Level2
            } else {
                Elevation::Level1
            }
        } else if self.pressed {
            Elevation::Level3
        } else if self.hovered {
            Elevation::Level4
        } else {
            Elevation::Level3
        }
    }

    /// Check if this is an extended FAB
    pub fn is_extended(&self) -> bool {
        self.label.is_some()
    }
}

/// Event when FAB is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct FabClickEvent {
    pub entity: Entity,
}

/// System to handle FAB interactions
fn fab_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialFab),
        (Changed<Interaction>, With<MaterialFab>),
    >,
    mut click_events: MessageWriter<FabClickEvent>,
) {
    for (entity, interaction, mut fab) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                fab.pressed = true;
                fab.hovered = false;
                click_events.write(FabClickEvent { entity });
            }
            Interaction::Hovered => {
                fab.pressed = false;
                fab.hovered = true;
            }
            Interaction::None => {
                fab.pressed = false;
                fab.hovered = false;
            }
        }
    }
}

/// System to update FAB styles
fn fab_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut fabs: Query<(&MaterialFab, &mut BackgroundColor), Changed<MaterialFab>>,
) {
    let Some(theme) = theme else { return };

    for (fab, mut bg_color) in fabs.iter_mut() {
        *bg_color = BackgroundColor(fab.background_color(&theme));
    }
}

/// System to update FAB label and icon colors when FAB state changes.
fn fab_content_style_system(
    theme: Option<Res<MaterialTheme>>,
    fabs: Query<(Entity, &MaterialFab), Changed<MaterialFab>>,
    children_q: Query<&Children>,
    mut icon_styles: Query<&mut IconStyle>,
    mut labels: Query<&mut TextColor, With<FabLabel>>,
) {
    let Some(theme) = theme else { return };

    for (entity, fab) in fabs.iter() {
        let Ok(children) = children_q.get(entity) else {
            continue;
        };
        let content_color = fab.content_color(&theme);

        for child in children.iter() {
            if let Ok(mut style) = icon_styles.get_mut(child) {
                style.color = content_color;
            }
            if let Ok(mut color) = labels.get_mut(child) {
                color.0 = content_color;
            }
        }
    }
}

/// Refresh FAB visuals when the theme changes.
fn fab_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut fabs: Query<(Entity, &MaterialFab, &mut BackgroundColor)>,
    children_q: Query<&Children>,
    mut icon_styles: Query<&mut IconStyle>,
    mut labels: Query<&mut TextColor, With<FabLabel>>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (entity, fab, mut bg_color) in fabs.iter_mut() {
        *bg_color = BackgroundColor(fab.background_color(&theme));

        let Ok(children) = children_q.get(entity) else {
            continue;
        };
        let content_color = fab.content_color(&theme);
        for child in children.iter() {
            if let Ok(mut style) = icon_styles.get_mut(child) {
                style.color = content_color;
            }
            if let Ok(mut color) = labels.get_mut(child) {
                color.0 = content_color;
            }
        }
    }
}

/// System to update FAB shadows using native BoxShadow
fn fab_shadow_system(mut fabs: Query<(&MaterialFab, &mut BoxShadow), Changed<MaterialFab>>) {
    for (fab, mut box_shadow) in fabs.iter_mut() {
        let elevation = fab.elevation();
        *box_shadow = elevation.to_box_shadow();
    }
}

/// Builder for FABs
pub struct FabBuilder {
    fab: MaterialFab,
}

impl FabBuilder {
    /// Create a new FAB builder
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            fab: MaterialFab::new(icon),
        }
    }

    /// Set size
    pub fn size(mut self, size: FabSize) -> Self {
        self.fab.size = size;
        self
    }

    /// Make small FAB
    pub fn small(self) -> Self {
        self.size(FabSize::Small)
    }

    /// Make large FAB
    pub fn large(self) -> Self {
        self.size(FabSize::Large)
    }

    /// Set color
    pub fn color(mut self, color: FabColor) -> Self {
        self.fab.color = color;
        self
    }

    /// Make surface FAB
    pub fn surface(self) -> Self {
        self.color(FabColor::Surface)
    }

    /// Make secondary FAB
    pub fn secondary(self) -> Self {
        self.color(FabColor::Secondary)
    }

    /// Make tertiary FAB
    pub fn tertiary(self) -> Self {
        self.color(FabColor::Tertiary)
    }

    /// Make lowered FAB
    pub fn lowered(mut self) -> Self {
        self.fab.lowered = true;
        self
    }

    /// Make extended FAB
    pub fn extended(mut self, label: impl Into<String>) -> Self {
        self.fab.label = Some(label.into());
        self
    }

    /// Build the FAB bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.fab.background_color(theme);
        let size = self.fab.size.size();
        let corner_radius = self.fab.size.corner_radius();
        let is_extended = self.fab.is_extended();
        let elevation = self.fab.elevation();

        (
            self.fab,
            Button,
            RippleHost::new(),
            Node {
                width: if is_extended {
                    Val::Auto
                } else {
                    Val::Px(size)
                },
                height: Val::Px(size),
                min_width: if is_extended {
                    Val::Px(80.0)
                } else {
                    Val::Auto
                },
                padding: if is_extended {
                    UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::LARGE))
                } else {
                    UiRect::all(Val::Px(0.0))
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: if is_extended {
                    Val::Px(Spacing::SMALL)
                } else {
                    Val::Px(0.0)
                },
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(corner_radius)),
            // Native Bevy 0.17 shadow support
            elevation.to_box_shadow(),
        )
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

use crate::icons::MaterialIcon;

/// Marker component for FAB label text
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FabLabel;

/// Extension trait to spawn FABs as children
///
/// This trait provides a clean API for spawning FABs within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_fab(&theme, "add", FabSize::Regular);
///     children.spawn_small_fab(&theme, "edit");
///     children.spawn_extended_fab(&theme, "add", "Create New");
/// });
/// ```
pub trait SpawnFabChild {
    /// Spawn a FAB with specified size
    fn spawn_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>, size: FabSize);

    /// Spawn a small FAB
    fn spawn_small_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>);

    /// Spawn a regular FAB
    fn spawn_regular_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>);

    /// Spawn a large FAB
    fn spawn_large_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>);

    /// Spawn an extended FAB with icon and label
    fn spawn_extended_fab(
        &mut self,
        theme: &MaterialTheme,
        icon: impl Into<String>,
        label: impl Into<String>,
    );

    /// Spawn a FAB with full builder control
    fn spawn_fab_with(&mut self, theme: &MaterialTheme, fab: MaterialFab);
}

impl SpawnFabChild for ChildSpawnerCommands<'_> {
    fn spawn_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>, size: FabSize) {
        let icon_name = icon.into();
        let builder = FabBuilder::new(icon_name.clone()).size(size);
        let icon_color = builder.fab.content_color(theme);
        let icon_size = builder.fab.size.icon_size();

        self.spawn(builder.build(theme)).with_children(|fab| {
            if let Some(icon) = MaterialIcon::from_name(&icon_name) {
                fab.spawn((
                    icon,
                    IconStyle::outlined()
                        .with_color(icon_color)
                        .with_size(icon_size),
                ));
            }
        });
    }

    fn spawn_small_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>) {
        self.spawn_fab(theme, icon, FabSize::Small);
    }

    fn spawn_regular_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>) {
        self.spawn_fab(theme, icon, FabSize::Regular);
    }

    fn spawn_large_fab(&mut self, theme: &MaterialTheme, icon: impl Into<String>) {
        self.spawn_fab(theme, icon, FabSize::Large);
    }

    fn spawn_extended_fab(
        &mut self,
        theme: &MaterialTheme,
        icon: impl Into<String>,
        label: impl Into<String>,
    ) {
        let icon_name = icon.into();
        let label_str = label.into();
        let builder = FabBuilder::new(icon_name.clone()).extended(label_str.clone());
        let icon_color = builder.fab.content_color(theme);
        let text_color = builder.fab.content_color(theme);
        let icon_size = builder.fab.size.icon_size();

        self.spawn(builder.build(theme)).with_children(|fab| {
            if let Some(icon) = MaterialIcon::from_name(&icon_name) {
                fab.spawn((
                    icon,
                    IconStyle::outlined()
                        .with_color(icon_color)
                        .with_size(icon_size),
                ));
            }
            fab.spawn((
                FabLabel,
                Text::new(label_str),
                TextColor(text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
    }

    fn spawn_fab_with(&mut self, theme: &MaterialTheme, fab: MaterialFab) {
        let icon_color = fab.content_color(theme);
        let text_color = fab.content_color(theme);
        let icon_name = fab.icon.clone();
        let label_text = fab.label.clone();
        let icon_size = fab.size.icon_size();
        let builder = FabBuilder { fab };

        self.spawn(builder.build(theme)).with_children(|fab_inner| {
            if let Some(icon) = MaterialIcon::from_name(&icon_name) {
                fab_inner.spawn((
                    icon,
                    IconStyle::outlined()
                        .with_color(icon_color)
                        .with_size(icon_size),
                ));
            }
            if let Some(label) = label_text {
                fab_inner.spawn((
                    FabLabel,
                    Text::new(label),
                    TextColor(text_color),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                ));
            }
        });
    }
}
