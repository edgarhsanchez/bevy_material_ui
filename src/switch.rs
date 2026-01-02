//! Material Design 3 Switch component
//!
//! Switches toggle the state of a single item on or off.
//! Reference: <https://m3.material.io/components/switch/overview>
//!
//! # Example
//! ```ignore
//! // Using the spawn extension trait (recommended)
//! commands.spawn_switch(&theme, false, "Enable notifications");
//!
//! // Or using the builder for more control
//! parent.spawn_switch_with(&theme, SwitchBuilder::new().selected(true).with_icon(), "Label");
//! ```

use bevy::prelude::*;

use crate::{ripple::RippleHost, theme::MaterialTheme, tokens::CornerRadius};

/// Marker component for switch state layer
#[derive(Component)]
pub struct SwitchStateLayer;

/// Plugin for the switch component
pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SwitchChangeEvent>().add_systems(
            Update,
            (
                switch_interaction_system,
                switch_style_system,
                switch_theme_refresh_system,
            ),
        );
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
    }
}

/// Material switch component
#[derive(Component)]
pub struct MaterialSwitch {
    /// Whether the switch is on
    pub selected: bool,
    /// Whether the switch is disabled
    pub disabled: bool,
    /// Whether the switch has icons
    pub with_icon: bool,
    /// Animation progress (0.0 = off, 1.0 = on)
    pub animation_progress: f32,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialSwitch {
    /// Create a new switch
    pub fn new() -> Self {
        Self {
            selected: false,
            disabled: false,
            with_icon: false,
            animation_progress: 0.0,
            pressed: false,
            hovered: false,
        }
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self.animation_progress = if selected { 1.0 } else { 0.0 };
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable icons in the switch
    pub fn with_icon(mut self) -> Self {
        self.with_icon = true;
        self
    }

    /// Get the track color
    pub fn track_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            if self.selected {
                return theme.on_surface.with_alpha(0.12);
            } else {
                return theme.surface_container_highest.with_alpha(0.12);
            }
        }

        if self.selected {
            theme.primary
        } else {
            theme.surface_container_highest
        }
    }

    /// Get the track outline color
    pub fn track_outline_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        if self.selected {
            Color::NONE
        } else {
            theme.outline
        }
    }

    /// Get the handle (thumb) color
    pub fn handle_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            if self.selected {
                return theme.surface;
            } else {
                return theme.on_surface.with_alpha(0.38);
            }
        }

        if self.selected {
            theme.on_primary
        } else if self.pressed || self.hovered {
            theme.on_surface_variant
        } else {
            theme.outline
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            if self.selected {
                return theme.on_surface.with_alpha(0.38);
            } else {
                return theme.surface_container_highest.with_alpha(0.38);
            }
        }

        if self.selected {
            theme.on_primary_container
        } else {
            theme.surface_container_highest
        }
    }

    /// Get the handle size based on state
    pub fn handle_size(&self) -> f32 {
        if self.pressed {
            SWITCH_HANDLE_SIZE_PRESSED
        } else if self.selected || self.with_icon {
            SWITCH_HANDLE_SIZE_SELECTED
        } else {
            SWITCH_HANDLE_SIZE_UNSELECTED
        }
    }

    /// Get the handle position (0.0 to 1.0)
    pub fn handle_position(&self) -> f32 {
        self.animation_progress
    }
}

impl Default for MaterialSwitch {
    fn default() -> Self {
        Self::new()
    }
}

/// Event when switch state changes
#[derive(Event, bevy::prelude::Message)]
pub struct SwitchChangeEvent {
    pub entity: Entity,
    pub selected: bool,
}

/// Switch dimensions
pub const SWITCH_TRACK_WIDTH: f32 = 52.0;
pub const SWITCH_TRACK_HEIGHT: f32 = 32.0;
pub const SWITCH_HANDLE_SIZE_UNSELECTED: f32 = 16.0;
pub const SWITCH_HANDLE_SIZE_SELECTED: f32 = 24.0;
pub const SWITCH_HANDLE_SIZE_PRESSED: f32 = 28.0;

/// System to handle switch interactions
fn switch_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialSwitch),
        (Changed<Interaction>, With<MaterialSwitch>),
    >,
    mut change_events: MessageWriter<SwitchChangeEvent>,
) {
    for (entity, interaction, mut switch) in interaction_query.iter_mut() {
        if switch.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                switch.pressed = true;
                switch.hovered = false;
                switch.selected = !switch.selected;
                change_events.write(SwitchChangeEvent {
                    entity,
                    selected: switch.selected,
                });
            }
            Interaction::Hovered => {
                switch.pressed = false;
                switch.hovered = true;
            }
            Interaction::None => {
                switch.pressed = false;
                switch.hovered = false;
            }
        }
    }
}

/// System to update switch visual styles when state changes
fn switch_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut switches: Query<
        (
            &MaterialSwitch,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Node,
            &Children,
        ),
        Changed<MaterialSwitch>,
    >,
    mut handles: Query<
        (&mut BackgroundColor, &mut Node, &mut BorderRadius),
        (With<SwitchHandle>, Without<MaterialSwitch>),
    >,
) {
    let Some(theme) = theme else { return };

    for (switch, mut bg_color, mut border_color, mut node, children) in switches.iter_mut() {
        // Update track
        *bg_color = BackgroundColor(switch.track_color(&theme));
        *border_color = BorderColor::all(switch.track_outline_color(&theme));

        // Update track layout for handle position
        node.justify_content = if switch.selected {
            JustifyContent::FlexEnd
        } else {
            JustifyContent::FlexStart
        };
        node.border = UiRect::all(Val::Px(if switch.selected { 0.0 } else { 2.0 }));

        // Update handle
        let handle_color = switch.handle_color(&theme);
        let handle_size = switch.handle_size();

        for child in children.iter() {
            if let Ok((mut handle_bg, mut handle_node, mut handle_radius)) = handles.get_mut(child)
            {
                *handle_bg = BackgroundColor(handle_color);
                handle_node.width = Val::Px(handle_size);
                handle_node.height = Val::Px(handle_size);
                *handle_radius = BorderRadius::all(Val::Px(handle_size / 2.0));
            }
        }
    }
}

/// Refresh switch visuals when the theme changes.
fn switch_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut switches: Query<(
        &MaterialSwitch,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Node,
        &Children,
    )>,
    mut handles: Query<
        (&mut BackgroundColor, &mut Node, &mut BorderRadius),
        (With<SwitchHandle>, Without<MaterialSwitch>),
    >,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (switch, mut bg_color, mut border_color, mut node, children) in switches.iter_mut() {
        *bg_color = BackgroundColor(switch.track_color(&theme));
        *border_color = BorderColor::all(switch.track_outline_color(&theme));

        node.justify_content = if switch.selected {
            JustifyContent::FlexEnd
        } else {
            JustifyContent::FlexStart
        };
        node.border = UiRect::all(Val::Px(if switch.selected { 0.0 } else { 2.0 }));

        let handle_color = switch.handle_color(&theme);
        let handle_size = switch.handle_size();

        for child in children.iter() {
            if let Ok((mut handle_bg, mut handle_node, mut handle_radius)) = handles.get_mut(child)
            {
                *handle_bg = BackgroundColor(handle_color);
                handle_node.width = Val::Px(handle_size);
                handle_node.height = Val::Px(handle_size);
                *handle_radius = BorderRadius::all(Val::Px(handle_size / 2.0));
            }
        }
    }
}

/// Builder for switches
pub struct SwitchBuilder {
    switch: MaterialSwitch,
}

impl SwitchBuilder {
    /// Create a new switch builder
    pub fn new() -> Self {
        Self {
            switch: MaterialSwitch::new(),
        }
    }

    /// Set initial state
    pub fn selected(mut self, selected: bool) -> Self {
        self.switch.selected = selected;
        self.switch.animation_progress = if selected { 1.0 } else { 0.0 };
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.switch.disabled = disabled;
        self
    }

    /// Enable icon display
    pub fn with_icon(mut self) -> Self {
        self.switch.with_icon = true;
        self
    }

    /// Build the switch bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.switch.track_color(theme);
        let border_color = self.switch.track_outline_color(theme);
        let has_border = !self.switch.selected;

        (
            self.switch,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Px(SWITCH_TRACK_WIDTH),
                height: Val::Px(SWITCH_TRACK_HEIGHT),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(2.0)),
                border: UiRect::all(Val::Px(if has_border { 2.0 } else { 0.0 })),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )
    }
}

impl Default for SwitchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker component for the switch handle
#[derive(Component)]
pub struct SwitchHandle;

/// Extension trait to spawn switches with full visual hierarchy
pub trait SpawnSwitch {
    /// Spawn a switch with a label
    fn spawn_switch(&mut self, theme: &MaterialTheme, selected: bool, label: &str) -> Entity;

    /// Spawn a switch using a builder for more control
    fn spawn_switch_with(
        &mut self,
        theme: &MaterialTheme,
        builder: SwitchBuilder,
        label: &str,
    ) -> Entity;
}

impl SpawnSwitch for Commands<'_, '_> {
    fn spawn_switch(&mut self, theme: &MaterialTheme, selected: bool, label: &str) -> Entity {
        let builder = SwitchBuilder::new().selected(selected);
        self.spawn_switch_with(theme, builder, label)
    }

    fn spawn_switch_with(
        &mut self,
        theme: &MaterialTheme,
        builder: SwitchBuilder,
        label: &str,
    ) -> Entity {
        let label_color = theme.on_surface;
        let label_text = label.to_string();
        let switch = builder.switch;
        let bg_color = switch.track_color(theme);
        let border_color = switch.track_outline_color(theme);
        let handle_color = switch.handle_color(theme);
        let handle_size = switch.handle_size();
        let has_border = !switch.selected;
        let justify = if switch.selected {
            JustifyContent::FlexEnd
        } else {
            JustifyContent::FlexStart
        };

        self.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Switch track (the main touch target)
            row.spawn((
                switch,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(SWITCH_TRACK_WIDTH),
                    height: Val::Px(SWITCH_TRACK_HEIGHT),
                    justify_content: justify,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(2.0)),
                    border: UiRect::all(Val::Px(if has_border { 2.0 } else { 0.0 })),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(border_color),
                BorderRadius::all(Val::Px(CornerRadius::FULL)),
            ))
            .with_children(|track| {
                // Handle (thumb)
                track.spawn((
                    SwitchHandle,
                    Node {
                        width: Val::Px(handle_size),
                        height: Val::Px(handle_size),
                        ..default()
                    },
                    BackgroundColor(handle_color),
                    BorderRadius::all(Val::Px(handle_size / 2.0)),
                ));
            });

            // Label
            row.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(label_color),
            ));
        })
        .id()
    }
}

/// Extension trait to spawn switches within a ChildSpawnerCommands context
pub trait SpawnSwitchChild {
    /// Spawn a switch with a label
    fn spawn_switch(&mut self, theme: &MaterialTheme, selected: bool, label: &str);

    /// Spawn a switch using a builder for more control
    fn spawn_switch_with(&mut self, theme: &MaterialTheme, builder: SwitchBuilder, label: &str);
}

impl SpawnSwitchChild for ChildSpawnerCommands<'_> {
    fn spawn_switch(&mut self, theme: &MaterialTheme, selected: bool, label: &str) {
        let builder = SwitchBuilder::new().selected(selected);
        self.spawn_switch_with(theme, builder, label);
    }

    fn spawn_switch_with(&mut self, theme: &MaterialTheme, builder: SwitchBuilder, label: &str) {
        let label_color = theme.on_surface;
        let label_text = label.to_string();
        let switch = builder.switch;
        let bg_color = switch.track_color(theme);
        let border_color = switch.track_outline_color(theme);
        let handle_color = switch.handle_color(theme);
        let handle_size = switch.handle_size();
        let has_border = !switch.selected;
        let justify = if switch.selected {
            JustifyContent::FlexEnd
        } else {
            JustifyContent::FlexStart
        };

        self.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Switch track
            row.spawn((
                switch,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(SWITCH_TRACK_WIDTH),
                    height: Val::Px(SWITCH_TRACK_HEIGHT),
                    justify_content: justify,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(2.0)),
                    border: UiRect::all(Val::Px(if has_border { 2.0 } else { 0.0 })),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(border_color),
                BorderRadius::all(Val::Px(CornerRadius::FULL)),
            ))
            .with_children(|track| {
                // Handle (thumb)
                track.spawn((
                    SwitchHandle,
                    Node {
                        width: Val::Px(handle_size),
                        height: Val::Px(handle_size),
                        ..default()
                    },
                    BackgroundColor(handle_color),
                    BorderRadius::all(Val::Px(handle_size / 2.0)),
                ));
            });

            // Label
            row.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(label_color),
            ));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // MaterialSwitch Tests
    // ============================================================================

    #[test]
    fn test_switch_new_defaults() {
        let switch = MaterialSwitch::new();
        assert!(!switch.selected);
        assert!(!switch.disabled);
        assert!(!switch.with_icon);
        assert_eq!(switch.animation_progress, 0.0);
        assert!(!switch.pressed);
        assert!(!switch.hovered);
    }

    #[test]
    fn test_switch_default_trait() {
        let switch = MaterialSwitch::default();
        assert!(!switch.selected);
        assert!(!switch.disabled);
    }

    #[test]
    fn test_switch_selected_true() {
        let switch = MaterialSwitch::new().selected(true);
        assert!(switch.selected);
        assert_eq!(switch.animation_progress, 1.0);
    }

    #[test]
    fn test_switch_selected_false() {
        let switch = MaterialSwitch::new().selected(false);
        assert!(!switch.selected);
        assert_eq!(switch.animation_progress, 0.0);
    }

    #[test]
    fn test_switch_disabled() {
        let switch = MaterialSwitch::new().disabled(true);
        assert!(switch.disabled);

        let switch = MaterialSwitch::new().disabled(false);
        assert!(!switch.disabled);
    }

    #[test]
    fn test_switch_with_icon() {
        let switch = MaterialSwitch::new().with_icon();
        assert!(switch.with_icon);
    }

    #[test]
    fn test_switch_handle_size_unselected() {
        let switch = MaterialSwitch::new();
        assert_eq!(switch.handle_size(), SWITCH_HANDLE_SIZE_UNSELECTED);
    }

    #[test]
    fn test_switch_handle_size_selected() {
        let switch = MaterialSwitch::new().selected(true);
        assert_eq!(switch.handle_size(), SWITCH_HANDLE_SIZE_SELECTED);
    }

    #[test]
    fn test_switch_handle_size_with_icon() {
        let switch = MaterialSwitch::new().with_icon();
        assert_eq!(switch.handle_size(), SWITCH_HANDLE_SIZE_SELECTED);
    }

    #[test]
    fn test_switch_handle_size_pressed() {
        let mut switch = MaterialSwitch::new();
        switch.pressed = true;
        assert_eq!(switch.handle_size(), SWITCH_HANDLE_SIZE_PRESSED);
    }

    #[test]
    fn test_switch_handle_position_off() {
        let switch = MaterialSwitch::new().selected(false);
        assert_eq!(switch.handle_position(), 0.0);
    }

    #[test]
    fn test_switch_handle_position_on() {
        let switch = MaterialSwitch::new().selected(true);
        assert_eq!(switch.handle_position(), 1.0);
    }

    #[test]
    fn test_switch_builder_chain() {
        let switch = MaterialSwitch::new()
            .selected(true)
            .disabled(false)
            .with_icon();

        assert!(switch.selected);
        assert!(!switch.disabled);
        assert!(switch.with_icon);
    }

    // ============================================================================
    // SwitchBuilder Tests
    // ============================================================================

    #[test]
    fn test_switch_builder_new() {
        let builder = SwitchBuilder::new();
        assert!(!builder.switch.selected);
        assert!(!builder.switch.disabled);
    }

    #[test]
    fn test_switch_builder_default() {
        let builder = SwitchBuilder::default();
        assert!(!builder.switch.selected);
    }

    #[test]
    fn test_switch_builder_selected() {
        let builder = SwitchBuilder::new().selected(true);
        assert!(builder.switch.selected);
        assert_eq!(builder.switch.animation_progress, 1.0);
    }

    #[test]
    fn test_switch_builder_disabled() {
        let builder = SwitchBuilder::new().disabled(true);
        assert!(builder.switch.disabled);
    }

    #[test]
    fn test_switch_builder_with_icon() {
        let builder = SwitchBuilder::new().with_icon();
        assert!(builder.switch.with_icon);
    }

    #[test]
    fn test_switch_builder_full_chain() {
        let builder = SwitchBuilder::new()
            .selected(true)
            .disabled(false)
            .with_icon();

        assert!(builder.switch.selected);
        assert!(!builder.switch.disabled);
        assert!(builder.switch.with_icon);
    }

    // ============================================================================
    // Constants Tests
    // ============================================================================

    #[test]
    fn test_switch_track_width() {
        assert_eq!(SWITCH_TRACK_WIDTH, 52.0);
    }

    #[test]
    fn test_switch_track_height() {
        assert_eq!(SWITCH_TRACK_HEIGHT, 32.0);
    }

    #[test]
    fn test_switch_handle_sizes() {
        assert_eq!(SWITCH_HANDLE_SIZE_UNSELECTED, 16.0);
        assert_eq!(SWITCH_HANDLE_SIZE_SELECTED, 24.0);
        assert_eq!(SWITCH_HANDLE_SIZE_PRESSED, 28.0);
    }

    #[test]
    fn test_switch_handle_size_ordering() {
        // Pressed should be largest, then selected, then unselected
        use std::hint::black_box;
        assert!(black_box(SWITCH_HANDLE_SIZE_PRESSED) > black_box(SWITCH_HANDLE_SIZE_SELECTED));
        assert!(black_box(SWITCH_HANDLE_SIZE_SELECTED) > black_box(SWITCH_HANDLE_SIZE_UNSELECTED));
    }

    #[test]
    fn test_switch_track_dimensions() {
        // Track should be wider than tall
        use std::hint::black_box;
        assert!(black_box(SWITCH_TRACK_WIDTH) > black_box(SWITCH_TRACK_HEIGHT));
    }
}
