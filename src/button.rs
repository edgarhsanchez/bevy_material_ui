//! Material Design 3 Button component
//!
//! Buttons communicate actions that users can take.
//! Reference: <https://m3.material.io/components/buttons/overview>
//!
//! ## Bevy 0.17 Improvements
//!
//! This module now leverages:
//! - Native `BoxShadow` for elevation shadows
//! - `children!` macro for declarative child spawning
//! - Modern bundle patterns

use bevy::prelude::*;
use bevy::ui::{BoxShadow, Val};

use crate::{
    elevation::Elevation,
    ripple::RippleHost,
    theme::{blend_state_layer, MaterialTheme},
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the button component
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<ButtonClickEvent>().add_systems(
            Update,
            (
                button_interaction_system,
                button_style_system,
                button_label_style_system,
                button_theme_refresh_system,
                button_shadow_system,
            ),
        );
    }
}

/// Button variants following Material Design 3
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    /// Elevated button - Use for emphasis on surfaces
    Elevated,
    /// Filled button - High emphasis actions
    #[default]
    Filled,
    /// Filled tonal button - Medium emphasis
    FilledTonal,
    /// Outlined button - Medium emphasis, secondary actions
    Outlined,
    /// Text button - Low emphasis actions
    Text,
}

/// Icon gravity - determines where the icon is positioned relative to the label
/// Matches the reference `MaterialButton.IconGravity`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum IconGravity {
    /// Icon at the start of the button
    #[default]
    Start,
    /// Icon at the start of the text (next to text, not button edge)
    TextStart,
    /// Icon at the end of the button
    End,
    /// Icon at the end of the text (next to text, not button edge)
    TextEnd,
    /// Icon at the top of the button (for vertical layout)
    Top,
    /// Icon at the top of the text (next to text, not button edge)
    TextTop,
}

/// Material button component
///
/// Matches properties from the reference `MaterialButton`:
/// - Multiple variants (filled, outlined, text, elevated, tonal)
/// - State-based styling (normal, pressed, hovered, focused, disabled)
/// - Corner radius customization
/// - Custom colors per state
/// - Icon support with gravity control (start, end, top, text-relative)
/// - Icon padding, size, and tint
/// - Stroke width and color (for outlined buttons)
/// - Checkable/checked states
#[derive(Component, Clone)]
pub struct MaterialButton {
    /// Button variant style
    pub variant: ButtonVariant,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Button label text
    pub label: String,
    /// Optional leading icon
    pub icon: Option<String>,
    /// Optional trailing icon
    pub trailing_icon: Option<String>,
    /// Icon gravity (positioning relative to label)
    pub icon_gravity: IconGravity,
    /// Icon padding (space between icon and label)
    pub icon_padding: f32,
    /// Icon size (0 = use intrinsic size)
    pub icon_size: f32,
    /// Custom corner radius (None = use default for variant)
    pub corner_radius: Option<f32>,
    /// Custom minimum width
    pub min_width: Option<f32>,
    /// Custom minimum height
    pub min_height: Option<f32>,
    /// Custom background color override (for all states)
    pub custom_background_color: Option<Color>,
    /// Custom text color override
    pub custom_text_color: Option<Color>,
    /// Stroke width (for outlined variant)
    pub stroke_width: f32,
    /// Custom stroke color (for outlined variant)
    pub stroke_color: Option<Color>,
    /// Whether this button is checkable (toggle button)
    pub checkable: bool,
    /// Whether this button is checked (for toggle buttons)
    pub checked: bool,
    /// Whether this button is in a pressed state
    pub pressed: bool,
    /// Whether this button is hovered
    pub hovered: bool,
    /// Whether this button is focused
    pub focused: bool,
}

impl MaterialButton {
    /// Create a new material button
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            variant: ButtonVariant::default(),
            disabled: false,
            label: label.into(),
            icon: None,
            trailing_icon: None,
            icon_gravity: IconGravity::default(),
            icon_padding: 8.0,
            icon_size: 18.0,
            corner_radius: None,
            min_width: None,
            min_height: None,
            custom_background_color: None,
            custom_text_color: None,
            stroke_width: 1.0,
            stroke_color: None,
            checkable: false,
            checked: false,
            pressed: false,
            hovered: false,
            focused: false,
        }
    }

    /// Set the button variant
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set whether the button is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the leading icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the trailing icon
    pub fn with_trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set icon gravity (positioning relative to label)
    pub fn icon_gravity(mut self, gravity: IconGravity) -> Self {
        self.icon_gravity = gravity;
        self
    }

    /// Set icon padding (space between icon and label)
    pub fn icon_padding(mut self, padding: f32) -> Self {
        self.icon_padding = padding;
        self
    }

    /// Set icon size
    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Set custom corner radius
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Set minimum width
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Set minimum height
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    /// Set custom background color (overrides theme)
    pub fn custom_background_color(mut self, color: Color) -> Self {
        self.custom_background_color = Some(color);
        self
    }

    /// Set custom text color (overrides theme)
    pub fn custom_text_color(mut self, color: Color) -> Self {
        self.custom_text_color = Some(color);
        self
    }

    /// Set stroke width (for outlined variant)
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    /// Set custom stroke color (for outlined variant)
    pub fn stroke_color(mut self, color: Color) -> Self {
        self.stroke_color = Some(color);
        self
    }

    /// Set whether button is checkable (toggle button)
    pub fn checkable(mut self, checkable: bool) -> Self {
        self.checkable = checkable;
        self
    }

    /// Set checked state (for toggle buttons)
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Toggle the checked state
    pub fn toggle(&mut self) {
        if self.checkable {
            self.checked = !self.checked;
        }
    }

    /// Get the effective corner radius
    pub fn effective_corner_radius(&self) -> f32 {
        self.corner_radius.unwrap_or(CornerRadius::FULL)
    }

    /// Get the background color based on state and theme
    ///
    /// MD3 uses state layers to indicate hover/pressed states.
    /// The state layer is a semi-transparent overlay of the "on" color.
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        // Use custom background color if set
        if let Some(custom_color) = self.custom_background_color {
            return custom_color;
        }

        let state_opacity = self.state_layer_opacity();

        match self.variant {
            ButtonVariant::Elevated => {
                // State layer uses primary color on elevated buttons
                blend_state_layer(theme.surface_container_low, theme.primary, state_opacity)
            }
            ButtonVariant::Filled => {
                // State layer uses on_primary color
                blend_state_layer(theme.primary, theme.on_primary, state_opacity)
            }
            ButtonVariant::FilledTonal => {
                // State layer uses on_secondary_container color
                blend_state_layer(
                    theme.secondary_container,
                    theme.on_secondary_container,
                    state_opacity,
                )
            }
            ButtonVariant::Outlined => {
                // Transparent background with primary state layer
                if state_opacity > 0.0 {
                    theme.primary.with_alpha(state_opacity)
                } else {
                    Color::NONE
                }
            }
            ButtonVariant::Text => {
                // Transparent background with primary state layer
                if state_opacity > 0.0 {
                    theme.primary.with_alpha(state_opacity)
                } else {
                    Color::NONE
                }
            }
        }
    }

    /// Get the text color based on state and theme
    pub fn text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        // Use custom text color if set
        if let Some(custom_color) = self.custom_text_color {
            return custom_color;
        }

        match self.variant {
            ButtonVariant::Elevated => theme.primary,
            ButtonVariant::Filled => theme.on_primary,
            ButtonVariant::FilledTonal => theme.on_secondary_container,
            ButtonVariant::Outlined => theme.primary,
            ButtonVariant::Text => theme.primary,
        }
    }

    /// Get the border color based on state and theme
    pub fn border_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        match self.variant {
            ButtonVariant::Outlined => theme.outline,
            _ => Color::NONE,
        }
    }

    /// Get the elevation for this button variant
    pub fn elevation(&self) -> Elevation {
        if self.disabled {
            return Elevation::Level0;
        }

        match self.variant {
            ButtonVariant::Elevated => {
                if self.pressed {
                    Elevation::Level1
                } else if self.hovered {
                    Elevation::Level2
                } else {
                    Elevation::Level1
                }
            }
            ButtonVariant::Filled | ButtonVariant::FilledTonal => {
                if self.pressed || self.hovered {
                    Elevation::Level1
                } else {
                    Elevation::Level0
                }
            }
            _ => Elevation::Level0,
        }
    }

    /// Get the state layer opacity
    pub fn state_layer_opacity(&self) -> f32 {
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
}

/// Event fired when a button is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct ButtonClickEvent {
    /// The button entity that was clicked
    pub entity: Entity,
}

/// System to handle button interactions
fn button_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialButton),
        (Changed<Interaction>, With<MaterialButton>),
    >,
    mut click_events: MessageWriter<ButtonClickEvent>,
) {
    for (entity, interaction, mut button) in interaction_query.iter_mut() {
        if button.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                button.pressed = true;
                button.hovered = false;
                click_events.write(ButtonClickEvent { entity });
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

/// System to update button visual styles based on state
fn button_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut buttons: Query<
        (&MaterialButton, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialButton>,
    >,
) {
    let Some(theme) = theme else { return };

    for (button, mut bg_color, mut border_color) in buttons.iter_mut() {
        *bg_color = BackgroundColor(button.background_color(&theme));
        *border_color = BorderColor::all(button.border_color(&theme));
    }
}

/// System to update button label text colors when button state changes.
fn button_label_style_system(
    theme: Option<Res<MaterialTheme>>,
    buttons: Query<(&MaterialButton, &Children), Changed<MaterialButton>>,
    mut labels: Query<&mut TextColor, With<ButtonLabel>>,
) {
    let Some(theme) = theme else { return };

    for (button, children) in buttons.iter() {
        let label_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = labels.get_mut(child) {
                color.0 = label_color;
            }
        }
    }
}

/// System to refresh button visuals when the theme resource changes.
///
/// Theme changes are expected to be rare, so it is OK to update all buttons in one pass.
fn button_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut buttons: Query<(
        &MaterialButton,
        &Children,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut labels: Query<&mut TextColor, With<ButtonLabel>>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (button, children, mut bg_color, mut border_color) in buttons.iter_mut() {
        *bg_color = BackgroundColor(button.background_color(&theme));
        *border_color = BorderColor::all(button.border_color(&theme));

        let label_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = labels.get_mut(child) {
                color.0 = label_color;
            }
        }
    }
}

/// System to update button shadows using Bevy's native BoxShadow
///
/// This leverages Bevy 0.17's GPU-accelerated shadow rendering.
fn button_shadow_system(
    mut buttons: Query<(&MaterialButton, &mut BoxShadow), Changed<MaterialButton>>,
) {
    for (button, mut box_shadow) in buttons.iter_mut() {
        let elevation = button.elevation();
        *box_shadow = elevation.to_box_shadow();
    }
}

/// Builder for creating Material buttons with proper styling
///
/// ## Example with Bevy 0.17's `children!` macro:
///
/// ```ignore
/// commands.spawn((
///     MaterialButtonBuilder::new("Click Me").filled().build(&theme),
///     children![
///         (Text::new("Click Me"), TextColor(theme.on_primary)),
///     ],
/// ));
/// ```
pub struct MaterialButtonBuilder {
    button: MaterialButton,
}

impl MaterialButtonBuilder {
    /// Create a new button builder
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            button: MaterialButton::new(label),
        }
    }

    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.button.variant = variant;
        self
    }

    /// Make this an elevated button
    pub fn elevated(self) -> Self {
        self.variant(ButtonVariant::Elevated)
    }

    /// Make this a filled button
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }

    /// Make this a filled tonal button
    pub fn filled_tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }

    /// Make this an outlined button
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }

    /// Make this a text button
    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }

    /// Set the button as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button.disabled = disabled;
        self
    }

    /// Set whether the button is checkable (toggle button)
    pub fn checkable(mut self, checkable: bool) -> Self {
        self.button.checkable = checkable;
        self
    }

    /// Set whether the button is checked (for toggle buttons)
    pub fn checked(mut self, checked: bool) -> Self {
        self.button.checked = checked;
        self
    }

    /// Add an icon to the button
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.button.icon = Some(icon.into());
        self
    }

    /// Build the button bundle with native BoxShadow support
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.button.background_color(theme);
        let border_color = self.button.border_color(theme);
        let border_width = if self.button.variant == ButtonVariant::Outlined {
            1.0
        } else {
            0.0
        };
        let elevation = self.button.elevation();
        let corner_radius = self.button.effective_corner_radius();

        (
            self.button,
            Button,
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(Spacing::EXTRA_LARGE), Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(border_width)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(corner_radius)),
            // Native Bevy 0.17 shadow support
            elevation.to_box_shadow(),
        )
    }

    /// Build the button bundle without shadow (for layered UIs)
    pub fn build_without_shadow(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.button.background_color(theme);
        let border_color = self.button.border_color(theme);
        let border_width = if self.button.variant == ButtonVariant::Outlined {
            1.0
        } else {
            0.0
        };
        let corner_radius = self.button.effective_corner_radius();

        (
            self.button,
            Button,
            RippleHost::new(),
            Node {
                padding: UiRect::axes(Val::Px(Spacing::EXTRA_LARGE), Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(border_width)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(corner_radius)),
        )
    }
}

/// Helper function to spawn a material button with a text child
///
/// This function uses Bevy 0.17's native BoxShadow for elevation rendering.
pub fn spawn_material_button(
    commands: &mut Commands,
    theme: &MaterialTheme,
    label: impl Into<String>,
    variant: ButtonVariant,
) -> Entity {
    let label_text = label.into();
    let builder = MaterialButtonBuilder::new(label_text.clone()).variant(variant);
    let button = builder.button.clone();
    let text_color = button.text_color(theme);

    commands
        .spawn(
            MaterialButtonBuilder::new(label_text.clone())
                .variant(variant)
                .build(theme),
        )
        .with_children(|parent| {
            parent.spawn((
                Text::new(label_text),
                TextColor(text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            ));
        })
        .id()
}

/// Spawn a material button using the `children!` macro pattern
///
/// This is the recommended approach for Bevy 0.17+.
///
/// ## Example:
/// ```ignore
/// use bevy::prelude::*;
/// use bevy_material_ui::prelude::*;
///
/// fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
///     commands.spawn((
///         material_button_bundle(&theme, "Click Me", ButtonVariant::Filled),
///         children![
///             (Text::new("Click Me"), TextColor(theme.on_primary)),
///         ],
///     ));
/// }
/// ```
pub fn material_button_bundle(
    theme: &MaterialTheme,
    label: impl Into<String>,
    variant: ButtonVariant,
) -> impl Bundle {
    MaterialButtonBuilder::new(label)
        .variant(variant)
        .build(theme)
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Marker component for button text label
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ButtonLabel;

/// Extension trait to spawn Material buttons as children
///
/// This trait provides a clean API for spawning buttons within UI hierarchies
/// using Bevy 0.17's ChildSpawnerCommands pattern.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_button(&theme, "Click Me", ButtonVariant::Filled);
///     children.spawn_filled_button(&theme, "Primary Action");
///     children.spawn_outlined_button(&theme, "Secondary");
///     children.spawn_text_button(&theme, "Learn More");
/// });
/// ```
pub trait SpawnButtonChild {
    /// Spawn a button with specified variant
    fn spawn_button(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        variant: ButtonVariant,
    );

    /// Spawn a filled button (primary action)
    fn spawn_filled_button(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn an outlined button (secondary action)
    fn spawn_outlined_button(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn a text button (tertiary action)
    fn spawn_text_button(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn a filled tonal button (medium emphasis)
    fn spawn_filled_tonal_button(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn an elevated button
    fn spawn_elevated_button(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn a button with full builder control
    fn spawn_button_with(&mut self, theme: &MaterialTheme, button: MaterialButton);
}

impl SpawnButtonChild for ChildSpawnerCommands<'_> {
    fn spawn_button(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        variant: ButtonVariant,
    ) {
        let label_str = label.into();
        let builder = MaterialButtonBuilder::new(label_str.clone()).variant(variant);
        let text_color = builder.button.text_color(theme);

        self.spawn(builder.build(theme)).with_children(|button| {
            button.spawn((
                ButtonLabel,
                Text::new(label_str),
                TextColor(text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
    }

    fn spawn_filled_button(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_button(theme, label, ButtonVariant::Filled);
    }

    fn spawn_outlined_button(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_button(theme, label, ButtonVariant::Outlined);
    }

    fn spawn_text_button(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_button(theme, label, ButtonVariant::Text);
    }

    fn spawn_filled_tonal_button(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_button(theme, label, ButtonVariant::FilledTonal);
    }

    fn spawn_elevated_button(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_button(theme, label, ButtonVariant::Elevated);
    }

    fn spawn_button_with(&mut self, theme: &MaterialTheme, button: MaterialButton) {
        let text_color = button.text_color(theme);
        let label_str = button.label.clone();
        let builder = MaterialButtonBuilder { button };

        self.spawn(builder.build(theme)).with_children(|btn| {
            btn.spawn((
                ButtonLabel,
                Text::new(label_str),
                TextColor(text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            ));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // ButtonVariant Tests
    // ============================================================================

    #[test]
    fn test_button_variant_default() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Filled);
    }

    #[test]
    fn test_button_variant_all_types() {
        // Just verify all variants exist and are distinct
        let variants = [
            ButtonVariant::Elevated,
            ButtonVariant::Filled,
            ButtonVariant::FilledTonal,
            ButtonVariant::Outlined,
            ButtonVariant::Text,
        ];

        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j]);
            }
        }
    }

    // ============================================================================
    // IconGravity Tests
    // ============================================================================

    #[test]
    fn test_icon_gravity_default() {
        assert_eq!(IconGravity::default(), IconGravity::Start);
    }

    #[test]
    fn test_icon_gravity_all_types() {
        let gravities = [
            IconGravity::Start,
            IconGravity::TextStart,
            IconGravity::End,
            IconGravity::TextEnd,
            IconGravity::Top,
            IconGravity::TextTop,
        ];

        for i in 0..gravities.len() {
            for j in (i + 1)..gravities.len() {
                assert_ne!(gravities[i], gravities[j]);
            }
        }
    }

    // ============================================================================
    // MaterialButton Tests
    // ============================================================================

    #[test]
    fn test_button_new_defaults() {
        let button = MaterialButton::new("Test");
        assert_eq!(button.label, "Test");
        assert_eq!(button.variant, ButtonVariant::Filled);
        assert!(!button.disabled);
        assert!(button.icon.is_none());
        assert!(button.trailing_icon.is_none());
        assert_eq!(button.icon_gravity, IconGravity::Start);
        assert_eq!(button.icon_padding, 8.0);
        assert_eq!(button.icon_size, 18.0);
        assert!(button.corner_radius.is_none());
        assert!(button.min_width.is_none());
        assert!(button.min_height.is_none());
        assert!(button.custom_background_color.is_none());
        assert!(button.custom_text_color.is_none());
        assert_eq!(button.stroke_width, 1.0);
        assert!(button.stroke_color.is_none());
        assert!(!button.checkable);
        assert!(!button.checked);
        assert!(!button.pressed);
        assert!(!button.hovered);
        assert!(!button.focused);
    }

    #[test]
    fn test_button_with_variant() {
        let button = MaterialButton::new("Test").with_variant(ButtonVariant::Outlined);
        assert_eq!(button.variant, ButtonVariant::Outlined);
    }

    #[test]
    fn test_button_disabled() {
        let button = MaterialButton::new("Test").disabled(true);
        assert!(button.disabled);

        let button = MaterialButton::new("Test").disabled(false);
        assert!(!button.disabled);
    }

    #[test]
    fn test_button_with_icon() {
        let button = MaterialButton::new("Test").with_icon("add");
        assert_eq!(button.icon, Some("add".to_string()));
    }

    #[test]
    fn test_button_with_trailing_icon() {
        let button = MaterialButton::new("Test").with_trailing_icon("arrow_forward");
        assert_eq!(button.trailing_icon, Some("arrow_forward".to_string()));
    }

    #[test]
    fn test_button_icon_gravity() {
        let button = MaterialButton::new("Test").icon_gravity(IconGravity::End);
        assert_eq!(button.icon_gravity, IconGravity::End);
    }

    #[test]
    fn test_button_icon_padding() {
        let button = MaterialButton::new("Test").icon_padding(16.0);
        assert_eq!(button.icon_padding, 16.0);
    }

    #[test]
    fn test_button_icon_size() {
        let button = MaterialButton::new("Test").icon_size(24.0);
        assert_eq!(button.icon_size, 24.0);
    }

    #[test]
    fn test_button_corner_radius() {
        let button = MaterialButton::new("Test").corner_radius(8.0);
        assert_eq!(button.corner_radius, Some(8.0));
    }

    #[test]
    fn test_button_min_width() {
        let button = MaterialButton::new("Test").min_width(100.0);
        assert_eq!(button.min_width, Some(100.0));
    }

    #[test]
    fn test_button_min_height() {
        let button = MaterialButton::new("Test").min_height(48.0);
        assert_eq!(button.min_height, Some(48.0));
    }

    #[test]
    fn test_button_custom_background_color() {
        let color = Color::srgb(1.0, 0.0, 0.0);
        let button = MaterialButton::new("Test").custom_background_color(color);
        assert_eq!(button.custom_background_color, Some(color));
    }

    #[test]
    fn test_button_custom_text_color() {
        let color = Color::srgb(0.0, 1.0, 0.0);
        let button = MaterialButton::new("Test").custom_text_color(color);
        assert_eq!(button.custom_text_color, Some(color));
    }

    #[test]
    fn test_button_stroke_width() {
        let button = MaterialButton::new("Test").stroke_width(2.0);
        assert_eq!(button.stroke_width, 2.0);
    }

    #[test]
    fn test_button_stroke_color() {
        let color = Color::srgb(0.0, 0.0, 1.0);
        let button = MaterialButton::new("Test").stroke_color(color);
        assert_eq!(button.stroke_color, Some(color));
    }

    #[test]
    fn test_button_checkable() {
        let button = MaterialButton::new("Test").checkable(true);
        assert!(button.checkable);
    }

    #[test]
    fn test_button_checked() {
        let button = MaterialButton::new("Test").checked(true);
        assert!(button.checked);
    }

    #[test]
    fn test_button_toggle_when_checkable() {
        let mut button = MaterialButton::new("Test").checkable(true);
        assert!(!button.checked);

        button.toggle();
        assert!(button.checked);

        button.toggle();
        assert!(!button.checked);
    }

    #[test]
    fn test_button_toggle_when_not_checkable() {
        let mut button = MaterialButton::new("Test").checkable(false);
        assert!(!button.checked);

        button.toggle();
        assert!(!button.checked); // Should not toggle
    }

    #[test]
    fn test_button_effective_corner_radius_default() {
        let button = MaterialButton::new("Test");
        assert_eq!(button.effective_corner_radius(), CornerRadius::FULL);
    }

    #[test]
    fn test_button_effective_corner_radius_custom() {
        let button = MaterialButton::new("Test").corner_radius(12.0);
        assert_eq!(button.effective_corner_radius(), 12.0);
    }

    #[test]
    fn test_button_builder_chain() {
        let button = MaterialButton::new("Submit")
            .with_variant(ButtonVariant::Outlined)
            .with_icon("send")
            .icon_gravity(IconGravity::End)
            .icon_padding(12.0)
            .icon_size(20.0)
            .corner_radius(8.0)
            .stroke_width(2.0)
            .disabled(false)
            .checkable(true)
            .checked(true);

        assert_eq!(button.label, "Submit");
        assert_eq!(button.variant, ButtonVariant::Outlined);
        assert_eq!(button.icon, Some("send".to_string()));
        assert_eq!(button.icon_gravity, IconGravity::End);
        assert_eq!(button.icon_padding, 12.0);
        assert_eq!(button.icon_size, 20.0);
        assert_eq!(button.corner_radius, Some(8.0));
        assert_eq!(button.stroke_width, 2.0);
        assert!(!button.disabled);
        assert!(button.checkable);
        assert!(button.checked);
    }

    // ============================================================================
    // MaterialButtonBuilder Tests
    // ============================================================================

    #[test]
    fn test_button_builder_new() {
        let builder = MaterialButtonBuilder::new("Test");
        assert_eq!(builder.button.label, "Test");
    }

    #[test]
    fn test_button_builder_variant() {
        let builder = MaterialButtonBuilder::new("Test").variant(ButtonVariant::Text);
        assert_eq!(builder.button.variant, ButtonVariant::Text);
    }

    #[test]
    fn test_button_builder_icon() {
        let builder = MaterialButtonBuilder::new("Test").icon("add");
        assert_eq!(builder.button.icon, Some("add".to_string()));
    }

    #[test]
    fn test_button_builder_disabled() {
        let builder = MaterialButtonBuilder::new("Test").disabled(true);
        assert!(builder.button.disabled);
    }
}
