//! Material Design 3 Icon Button component
//!
//! Icon buttons display actions using icons.
//! Reference: <https://m3.material.io/components/icon-buttons/overview>

use bevy::prelude::*;

use crate::{
    icons::MaterialIcon,
    ripple::RippleHost,
    theme::{blend_state_layer, MaterialTheme},
    tokens::CornerRadius,
};

/// Plugin for the icon button component
pub struct IconButtonPlugin;

impl Plugin for IconButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<IconButtonClickEvent>().add_systems(
            Update,
            (
                icon_button_interaction_system,
                icon_button_style_system,
                icon_button_content_style_system,
                icon_button_theme_refresh_system,
            ),
        );
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
    }
}

/// Icon button variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum IconButtonVariant {
    /// Standard icon button
    #[default]
    Standard,
    /// Filled icon button
    Filled,
    /// Filled tonal icon button
    FilledTonal,
    /// Outlined icon button
    Outlined,
}

/// Material icon button component
#[derive(Component)]
pub struct MaterialIconButton {
    /// Button variant style
    pub variant: IconButtonVariant,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Whether the button is selected/toggled
    pub selected: bool,
    /// Whether the button supports toggle behavior
    pub toggle: bool,
    /// Icon identifier
    pub icon: String,
    /// Whether this button is pressed
    pub pressed: bool,
    /// Whether this button is hovered
    pub hovered: bool,
}

impl MaterialIconButton {
    /// Create a new icon button
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            variant: IconButtonVariant::default(),
            disabled: false,
            selected: false,
            toggle: false,
            icon: icon.into(),
            pressed: false,
            hovered: false,
        }
    }

    /// Set the button variant
    pub fn with_variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set whether the button is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable toggle behavior
    pub fn toggleable(mut self) -> Self {
        self.toggle = true;
        self
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return match self.variant {
                IconButtonVariant::Standard => Color::NONE,
                IconButtonVariant::Filled | IconButtonVariant::FilledTonal => {
                    theme.on_surface.with_alpha(0.12)
                }
                IconButtonVariant::Outlined => Color::NONE,
            };
        }

        let base = match self.variant {
            IconButtonVariant::Standard => Color::NONE,
            IconButtonVariant::Filled => {
                if self.selected {
                    theme.primary
                } else {
                    theme.surface_container_highest
                }
            }
            IconButtonVariant::FilledTonal => {
                if self.selected {
                    theme.secondary_container
                } else {
                    theme.surface_container_highest
                }
            }
            IconButtonVariant::Outlined => {
                if self.selected {
                    theme.inverse_surface
                } else {
                    Color::NONE
                }
            }
        };

        // Apply state layer
        let state_opacity = self.state_layer_opacity();
        if state_opacity > 0.0 {
            let state_color = self.icon_color(theme);
            if base == Color::NONE {
                state_color.with_alpha(state_opacity)
            } else {
                blend_state_layer(base, state_color, state_opacity)
            }
        } else {
            base
        }
    }

    /// Get the state layer opacity
    fn state_layer_opacity(&self) -> f32 {
        if self.disabled {
            0.0
        } else if self.pressed {
            0.12
        } else if self.hovered {
            0.08
        } else {
            0.0
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        match self.variant {
            IconButtonVariant::Standard => {
                if self.selected {
                    theme.primary
                } else {
                    theme.on_surface_variant
                }
            }
            IconButtonVariant::Filled => {
                if self.selected {
                    theme.on_primary
                } else {
                    theme.primary
                }
            }
            IconButtonVariant::FilledTonal => {
                if self.selected {
                    theme.on_secondary_container
                } else {
                    theme.on_surface_variant
                }
            }
            IconButtonVariant::Outlined => {
                if self.selected {
                    theme.inverse_on_surface
                } else {
                    theme.on_surface_variant
                }
            }
        }
    }

    /// Get the border color
    pub fn border_color(&self, theme: &MaterialTheme) -> Color {
        if self.variant != IconButtonVariant::Outlined {
            return Color::NONE;
        }

        if self.disabled {
            theme.on_surface.with_alpha(0.12)
        } else if self.selected {
            Color::NONE
        } else {
            theme.outline
        }
    }
}

/// Event fired when an icon button is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct IconButtonClickEvent {
    /// The button entity
    pub entity: Entity,
    /// Whether the button is now selected (for toggle buttons)
    pub selected: bool,
}

/// System to handle icon button interactions
fn icon_button_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialIconButton),
        (Changed<Interaction>, With<MaterialIconButton>),
    >,
    mut click_events: MessageWriter<IconButtonClickEvent>,
) {
    for (entity, interaction, mut button) in interaction_query.iter_mut() {
        if button.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                button.pressed = true;
                button.hovered = false;

                if button.toggle {
                    button.selected = !button.selected;
                }

                click_events.write(IconButtonClickEvent {
                    entity,
                    selected: button.selected,
                });
            }
            Interaction::Hovered => {
                button.pressed = false;
                button.hovered = true;
            }
            Interaction::None => {
                button.pressed = false;
                button.hovered = false;
            }
        }
    }
}

/// System to update icon button styles
fn icon_button_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut buttons: Query<
        (&MaterialIconButton, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialIconButton>,
    >,
) {
    let Some(theme) = theme else { return };

    for (button, mut bg_color, mut border_color) in buttons.iter_mut() {
        *bg_color = BackgroundColor(button.background_color(&theme));
        *border_color = BorderColor::all(button.border_color(&theme));
    }
}

/// System to update the child icon color when the icon button state changes.
fn icon_button_content_style_system(
    theme: Option<Res<MaterialTheme>>,
    buttons: Query<(Entity, &MaterialIconButton), Changed<MaterialIconButton>>,
    children_q: Query<&Children>,
    mut icons: Query<&mut MaterialIcon>,
) {
    let Some(theme) = theme else { return };

    for (entity, button) in buttons.iter() {
        let Ok(children) = children_q.get(entity) else {
            continue;
        };
        let icon_color = button.icon_color(&theme);
        for child in children.iter() {
            if let Ok(mut icon) = icons.get_mut(child) {
                icon.color = icon_color;
            }
        }
    }
}

/// Refresh icon button visuals when the theme resource changes.
fn icon_button_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut buttons: Query<(
        Entity,
        &MaterialIconButton,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    children_q: Query<&Children>,
    mut icons: Query<&mut MaterialIcon>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (entity, button, mut bg_color, mut border_color) in buttons.iter_mut() {
        *bg_color = BackgroundColor(button.background_color(&theme));
        *border_color = BorderColor::all(button.border_color(&theme));

        let Ok(children) = children_q.get(entity) else {
            continue;
        };
        let icon_color = button.icon_color(&theme);
        for child in children.iter() {
            if let Ok(mut icon) = icons.get_mut(child) {
                icon.color = icon_color;
            }
        }
    }
}

/// Standard icon button size
pub const ICON_BUTTON_SIZE: f32 = 40.0;
/// Icon size within button
pub const ICON_SIZE: f32 = 24.0;

/// Builder for icon buttons
pub struct IconButtonBuilder {
    button: MaterialIconButton,
}

impl IconButtonBuilder {
    /// Create a new icon button builder
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            button: MaterialIconButton::new(icon),
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.button.variant = variant;
        self
    }

    /// Make standard variant
    pub fn standard(self) -> Self {
        self.variant(IconButtonVariant::Standard)
    }

    /// Make filled variant
    pub fn filled(self) -> Self {
        self.variant(IconButtonVariant::Filled)
    }

    /// Make filled tonal variant
    pub fn filled_tonal(self) -> Self {
        self.variant(IconButtonVariant::FilledTonal)
    }

    /// Make outlined variant
    pub fn outlined(self) -> Self {
        self.variant(IconButtonVariant::Outlined)
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button.disabled = disabled;
        self
    }

    /// Enable toggle mode
    pub fn toggle(mut self) -> Self {
        self.button.toggle = true;
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.button.selected = selected;
        self
    }

    /// Build the button bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.button.background_color(theme);
        let border_color = self.button.border_color(theme);
        let border_width = if self.button.variant == IconButtonVariant::Outlined {
            1.0
        } else {
            0.0
        };

        (
            self.button,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Px(ICON_BUTTON_SIZE),
                height: Val::Px(ICON_BUTTON_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(border_width)),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material icon buttons as children
///
/// This trait provides a clean API for spawning icon buttons within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_icon_button(&theme, "favorite", IconButtonVariant::Standard);
///     children.spawn_filled_icon_button(&theme, "add");
/// });
/// ```
pub trait SpawnIconButtonChild {
    /// Spawn an icon button with specified variant
    fn spawn_icon_button(
        &mut self,
        theme: &MaterialTheme,
        icon: impl Into<String>,
        variant: IconButtonVariant,
    );

    /// Spawn a standard icon button
    fn spawn_standard_icon_button(&mut self, theme: &MaterialTheme, icon: impl Into<String>);

    /// Spawn a filled icon button
    fn spawn_filled_icon_button(&mut self, theme: &MaterialTheme, icon: impl Into<String>);

    /// Spawn an outlined icon button
    fn spawn_outlined_icon_button(&mut self, theme: &MaterialTheme, icon: impl Into<String>);

    /// Spawn an icon button with full builder control
    fn spawn_icon_button_with(&mut self, theme: &MaterialTheme, button: MaterialIconButton);
}

impl SpawnIconButtonChild for ChildSpawnerCommands<'_> {
    fn spawn_icon_button(
        &mut self,
        theme: &MaterialTheme,
        icon: impl Into<String>,
        variant: IconButtonVariant,
    ) {
        let icon_name = icon.into();
        let builder = IconButtonBuilder::new(icon_name.clone()).variant(variant);
        let icon_color = builder.button.icon_color(theme);

        self.spawn(builder.build(theme)).with_children(|button| {
            if let Some(icon) = MaterialIcon::from_name(&icon_name) {
                button.spawn(icon.with_color(icon_color).with_size(ICON_SIZE));
            }
        });
    }

    fn spawn_standard_icon_button(&mut self, theme: &MaterialTheme, icon: impl Into<String>) {
        self.spawn_icon_button(theme, icon, IconButtonVariant::Standard);
    }

    fn spawn_filled_icon_button(&mut self, theme: &MaterialTheme, icon: impl Into<String>) {
        self.spawn_icon_button(theme, icon, IconButtonVariant::Filled);
    }

    fn spawn_outlined_icon_button(&mut self, theme: &MaterialTheme, icon: impl Into<String>) {
        self.spawn_icon_button(theme, icon, IconButtonVariant::Outlined);
    }

    fn spawn_icon_button_with(&mut self, theme: &MaterialTheme, button: MaterialIconButton) {
        let icon_color = button.icon_color(theme);
        let icon_name = button.icon.clone();
        let builder = IconButtonBuilder { button };

        self.spawn(builder.build(theme)).with_children(|btn| {
            if let Some(icon) = MaterialIcon::from_name(&icon_name) {
                btn.spawn(icon.with_color(icon_color).with_size(ICON_SIZE));
            }
        });
    }
}
