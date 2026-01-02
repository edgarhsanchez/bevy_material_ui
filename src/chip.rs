//! Material Design 3 Chips component
//!
//! Chips help people enter information, make selections, filter content, or trigger actions.
//! They can show multiple interactive elements together in the same area.
//! This module leverages native `BoxShadow` for elevated chip shadows.
//!
//! Reference: <https://m3.material.io/components/chips/overview>

use bevy::prelude::*;
use bevy::ui::BoxShadow;

use crate::{
    elevation::Elevation,
    ripple::RippleHost,
    theme::{blend_state_layer, MaterialTheme},
    tokens::Spacing,
};

/// Plugin for the chip component
pub struct ChipPlugin;

impl Plugin for ChipPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<ChipClickEvent>()
            .add_message::<ChipDeleteEvent>()
            .add_systems(
                Update,
                (
                    chip_interaction_system,
                    chip_style_system,
                    chip_content_style_system,
                    chip_theme_refresh_system,
                    chip_shadow_system,
                ),
            );
    }
}

// ============================================================================
// Events
// ============================================================================

/// Event fired when a chip is clicked
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct ChipClickEvent {
    /// The chip entity
    pub entity: Entity,
    /// The chip value (if any)
    pub value: Option<String>,
}

/// Event fired when a chip's delete button is clicked
#[derive(Event, Clone, bevy::prelude::Message)]
pub struct ChipDeleteEvent {
    /// The chip entity
    pub entity: Entity,
    /// The chip value (if any)
    pub value: Option<String>,
}

// ============================================================================
// Types
// ============================================================================

/// Chip variants following Material Design 3
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ChipVariant {
    /// Assist chips - Help complete a task or workflow
    #[default]
    Assist,
    /// Filter chips - Select options to filter content
    Filter,
    /// Input chips - Represent user input (tags, contacts)
    Input,
    /// Suggestion chips - Dynamically generated suggestions
    Suggestion,
}

/// Chip elevation state
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ChipElevation {
    /// Flat chip (no elevation)
    #[default]
    Flat,
    /// Elevated chip (with shadow)
    Elevated,
}

impl ChipElevation {
    /// Convert chip elevation to Material Elevation for BoxShadow
    pub fn to_elevation(&self) -> Elevation {
        match self {
            ChipElevation::Flat => Elevation::Level0,
            ChipElevation::Elevated => Elevation::Level1,
        }
    }

    /// Convert to native BoxShadow
    pub fn to_box_shadow(&self) -> BoxShadow {
        self.to_elevation().to_box_shadow()
    }
}

// ============================================================================
// Components
// ============================================================================

/// Material chip component
#[derive(Component)]
pub struct MaterialChip {
    /// Chip variant
    pub variant: ChipVariant,
    /// Label text
    pub label: String,
    /// Optional value for identification
    pub value: Option<String>,
    /// Whether the chip is selected (for filter chips)
    pub selected: bool,
    /// Whether the chip is disabled
    pub disabled: bool,
    /// Whether the chip has a delete button
    pub deletable: bool,
    /// Whether the chip has a leading icon
    pub has_leading_icon: bool,
    /// Elevation style
    pub elevation: ChipElevation,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialChip {
    /// Create a new chip
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            variant: ChipVariant::default(),
            label: label.into(),
            value: None,
            selected: false,
            disabled: false,
            deletable: false,
            has_leading_icon: false,
            elevation: ChipElevation::default(),
            pressed: false,
            hovered: false,
        }
    }

    /// Set the chip variant
    pub fn with_variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Create an assist chip
    pub fn assist(label: impl Into<String>) -> Self {
        Self::new(label).with_variant(ChipVariant::Assist)
    }

    /// Create a filter chip
    pub fn filter(label: impl Into<String>) -> Self {
        Self::new(label).with_variant(ChipVariant::Filter)
    }

    /// Create an input chip
    pub fn input(label: impl Into<String>) -> Self {
        Self::new(label)
            .with_variant(ChipVariant::Input)
            .with_deletable(true)
    }

    /// Create a suggestion chip
    pub fn suggestion(label: impl Into<String>) -> Self {
        Self::new(label).with_variant(ChipVariant::Suggestion)
    }

    /// Set the value
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set selected state
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set disabled state
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set deletable
    pub fn with_deletable(mut self, deletable: bool) -> Self {
        self.deletable = deletable;
        self
    }

    /// Set elevated
    pub fn elevated(mut self) -> Self {
        self.elevation = ChipElevation::Elevated;
        self
    }

    /// Set leading icon
    pub fn with_leading_icon(mut self) -> Self {
        self.has_leading_icon = true;
        self
    }

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        let base = match (self.variant, self.selected, self.elevation) {
            // Selected filter chips have secondary container background
            (ChipVariant::Filter, true, _) => theme.secondary_container,
            // Elevated chips have surface container low
            (_, _, ChipElevation::Elevated) => theme.surface_container_low,
            // Flat chips are transparent
            _ => Color::NONE,
        };

        // Apply state layer
        let state_opacity = self.state_layer_opacity();
        let state_color = self.state_layer_color(theme);
        if state_opacity > 0.0 {
            if base == Color::NONE {
                // For transparent backgrounds, just show the state layer
                state_color.with_alpha(state_opacity)
            } else {
                blend_state_layer(base, state_color, state_opacity)
            }
        } else {
            base
        }
    }

    /// Get the outline color
    pub fn outline_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.12);
        }

        match (self.variant, self.selected, self.elevation) {
            // Selected filter chips have no outline
            (ChipVariant::Filter, true, _) => Color::NONE,
            // Elevated chips have no outline
            (_, _, ChipElevation::Elevated) => Color::NONE,
            // Normal chips have outline
            _ => theme.outline,
        }
    }

    /// Get the label color
    pub fn label_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        match (self.variant, self.selected) {
            (ChipVariant::Filter, true) => theme.on_secondary_container,
            _ => theme.on_surface_variant,
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        match (self.variant, self.selected) {
            (ChipVariant::Filter, true) => theme.on_secondary_container,
            _ => theme.primary,
        }
    }

    /// Get the state layer color
    pub fn state_layer_color(&self, theme: &MaterialTheme) -> Color {
        match (self.variant, self.selected) {
            (ChipVariant::Filter, true) => theme.on_secondary_container,
            _ => theme.on_surface_variant,
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
}

impl Default for MaterialChip {
    fn default() -> Self {
        Self::new("")
    }
}

/// Marker for chip delete button
#[derive(Component)]
pub struct ChipDeleteButton;

/// Marker for chip leading icon
#[derive(Component)]
pub struct ChipLeadingIcon;

/// Marker for chip label
#[derive(Component)]
pub struct ChipLabel;

/// Marker for chip delete icon text ("✕")
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ChipDeleteIcon;

// ============================================================================
// Dimensions
// ============================================================================

/// Standard chip height
pub const CHIP_HEIGHT: f32 = 32.0;
/// Chip horizontal padding
pub const CHIP_PADDING_HORIZONTAL: f32 = 16.0;
/// Chip icon size
pub const CHIP_ICON_SIZE: f32 = 18.0;
/// Chip with icon padding (left side)
pub const CHIP_PADDING_WITH_ICON: f32 = 8.0;

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating chips with proper styling
pub struct ChipBuilder {
    chip: MaterialChip,
    leading_icon: Option<String>,
}

impl ChipBuilder {
    /// Create a new chip builder
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            chip: MaterialChip::new(label),
            leading_icon: None,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: ChipVariant) -> Self {
        self.chip.variant = variant;
        self
    }

    /// Create an assist chip
    pub fn assist(label: impl Into<String>) -> Self {
        Self::new(label).variant(ChipVariant::Assist)
    }

    /// Create a filter chip
    pub fn filter(label: impl Into<String>) -> Self {
        Self::new(label).variant(ChipVariant::Filter)
    }

    /// Create an input chip
    pub fn input(label: impl Into<String>) -> Self {
        Self::new(label).variant(ChipVariant::Input).deletable(true)
    }

    /// Create a suggestion chip
    pub fn suggestion(label: impl Into<String>) -> Self {
        Self::new(label).variant(ChipVariant::Suggestion)
    }

    /// Set the value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.chip.value = Some(value.into());
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.chip.selected = selected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.chip.disabled = disabled;
        self
    }

    /// Set deletable
    pub fn deletable(mut self, deletable: bool) -> Self {
        self.chip.deletable = deletable;
        self
    }

    /// Set elevated
    pub fn elevated(mut self) -> Self {
        self.chip.elevation = ChipElevation::Elevated;
        self
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self.chip.has_leading_icon = true;
        self
    }

    /// Build the chip bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.chip.background_color(theme);
        let outline_color = self.chip.outline_color(theme);
        let has_outline = outline_color != Color::NONE;
        let elevation = self.chip.elevation;

        let padding_left = if self.chip.has_leading_icon {
            CHIP_PADDING_WITH_ICON
        } else {
            CHIP_PADDING_HORIZONTAL
        };

        let padding_right = if self.chip.deletable {
            CHIP_PADDING_WITH_ICON
        } else {
            CHIP_PADDING_HORIZONTAL
        };

        (
            self.chip,
            Button,
            RippleHost::new(),
            Node {
                height: Val::Px(CHIP_HEIGHT),
                padding: UiRect {
                    left: Val::Px(padding_left),
                    right: Val::Px(padding_right),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                border: UiRect::all(Val::Px(if has_outline { 1.0 } else { 0.0 })),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(Spacing::SMALL),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(outline_color),
            BorderRadius::all(Val::Px(CHIP_HEIGHT / 2.0)), // Pill shape
            // Native Bevy 0.17 shadow support
            elevation.to_box_shadow(),
        )
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a chip with its children
pub fn spawn_chip(commands: &mut Commands, theme: &MaterialTheme, builder: ChipBuilder) -> Entity {
    let label = builder.chip.label.clone();
    let label_color = builder.chip.label_color(theme);
    let icon_color = builder.chip.icon_color(theme);
    let deletable = builder.chip.deletable;
    let has_leading = builder.chip.has_leading_icon;
    let selected = builder.chip.selected;
    let variant = builder.chip.variant;
    let leading_icon = builder.leading_icon.clone();

    commands
        .spawn(builder.build(theme))
        .with_children(|parent| {
            // Leading icon (or checkmark for selected filter chips)
            if variant == ChipVariant::Filter && selected {
                parent.spawn((
                    ChipLeadingIcon,
                    Text::new("✓"),
                    TextFont {
                        font_size: CHIP_ICON_SIZE,
                        ..default()
                    },
                    TextColor(icon_color),
                ));
            } else if has_leading {
                parent.spawn((
                    ChipLeadingIcon,
                    Text::new(leading_icon.as_deref().unwrap_or("★")),
                    TextFont {
                        font_size: CHIP_ICON_SIZE,
                        ..default()
                    },
                    TextColor(icon_color),
                ));
            }

            // Label
            parent.spawn((
                ChipLabel,
                Text::new(&label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(label_color),
            ));

            // Delete button
            if deletable {
                parent
                    .spawn((
                        ChipDeleteButton,
                        Button,
                        Node {
                            width: Val::Px(CHIP_ICON_SIZE),
                            height: Val::Px(CHIP_ICON_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ChipDeleteIcon,
                            Text::new("✕"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(icon_color),
                        ));
                    });
            }
        })
        .id()
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material chips as children
///
/// This trait provides a clean API for spawning chips within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_chip(&theme, ChipBuilder::assist("Help"));
///     children.spawn_filter_chip(&theme, "Category", false);
///     children.spawn_input_chip(&theme, "tag@example.com", true);
/// });
/// ```
pub trait SpawnChipChild {
    /// Spawn a chip using a builder
    fn spawn_chip_with(&mut self, theme: &MaterialTheme, builder: ChipBuilder);

    /// Spawn an assist chip
    fn spawn_assist_chip(&mut self, theme: &MaterialTheme, label: impl Into<String>);

    /// Spawn a filter chip
    fn spawn_filter_chip(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        selected: bool,
    );

    /// Spawn an input chip (deletable)
    fn spawn_input_chip(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        deletable: bool,
    );

    /// Spawn a suggestion chip
    fn spawn_suggestion_chip(&mut self, theme: &MaterialTheme, label: impl Into<String>);
}

impl SpawnChipChild for ChildSpawnerCommands<'_> {
    fn spawn_chip_with(&mut self, theme: &MaterialTheme, builder: ChipBuilder) {
        let label = builder.chip.label.clone();
        let label_color = builder.chip.label_color(theme);
        let icon_color = builder.chip.icon_color(theme);
        let deletable = builder.chip.deletable;
        let has_leading = builder.chip.has_leading_icon;
        let selected = builder.chip.selected;
        let variant = builder.chip.variant;
        let leading_icon = builder.leading_icon.clone();

        self.spawn(builder.build(theme)).with_children(|parent| {
            // Leading icon (or checkmark for selected filter chips)
            if variant == ChipVariant::Filter && selected {
                parent.spawn((
                    ChipLeadingIcon,
                    Text::new("✓"),
                    TextFont {
                        font_size: CHIP_ICON_SIZE,
                        ..default()
                    },
                    TextColor(icon_color),
                ));
            } else if has_leading {
                parent.spawn((
                    ChipLeadingIcon,
                    Text::new(leading_icon.as_deref().unwrap_or("★")),
                    TextFont {
                        font_size: CHIP_ICON_SIZE,
                        ..default()
                    },
                    TextColor(icon_color),
                ));
            }

            // Label
            parent.spawn((
                ChipLabel,
                Text::new(&label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(label_color),
            ));

            // Delete button
            if deletable {
                parent
                    .spawn((
                        ChipDeleteButton,
                        Button,
                        Node {
                            width: Val::Px(CHIP_ICON_SIZE),
                            height: Val::Px(CHIP_ICON_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ChipDeleteIcon,
                            Text::new("✕"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(icon_color),
                        ));
                    });
            }
        });
    }

    fn spawn_assist_chip(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_chip_with(theme, ChipBuilder::assist(label));
    }

    fn spawn_filter_chip(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        selected: bool,
    ) {
        self.spawn_chip_with(theme, ChipBuilder::filter(label).selected(selected));
    }

    fn spawn_input_chip(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        deletable: bool,
    ) {
        let mut builder = ChipBuilder::input(label);
        if deletable {
            builder = builder.deletable(true);
        }
        self.spawn_chip_with(theme, builder);
    }

    fn spawn_suggestion_chip(&mut self, theme: &MaterialTheme, label: impl Into<String>) {
        self.spawn_chip_with(theme, ChipBuilder::suggestion(label));
    }
}

// ============================================================================
// Systems
// ============================================================================

/// System to handle chip interactions
fn chip_interaction_system(
    mut interaction_query: Query<(Entity, &Interaction, &mut MaterialChip), Changed<Interaction>>,
    delete_buttons: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<ChipDeleteButton>)>,
    mut click_events: MessageWriter<ChipClickEvent>,
    mut delete_events: MessageWriter<ChipDeleteEvent>,
) {
    // Handle chip clicks
    for (entity, interaction, mut chip) in interaction_query.iter_mut() {
        if chip.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                chip.pressed = true;
                chip.hovered = false;

                // Toggle selection for filter chips
                if chip.variant == ChipVariant::Filter {
                    chip.selected = !chip.selected;
                }

                click_events.write(ChipClickEvent {
                    entity,
                    value: chip.value.clone(),
                });
            }
            Interaction::Hovered => {
                chip.pressed = false;
                chip.hovered = true;
            }
            Interaction::None => {
                chip.pressed = false;
                chip.hovered = false;
            }
        }
    }

    // Handle delete button clicks
    for (interaction, parent) in delete_buttons.iter() {
        if *interaction == Interaction::Pressed {
            // For delete buttons, we emit an event with the parent chip entity
            delete_events.write(ChipDeleteEvent {
                entity: parent.parent(),
                value: None, // Value will be looked up by event handler if needed
            });
        }
    }
}

/// System to update chip styles
fn chip_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut chips: Query<
        (&MaterialChip, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialChip>,
    >,
) {
    let Some(theme) = theme else { return };

    for (chip, mut bg_color, mut border_color) in chips.iter_mut() {
        *bg_color = BackgroundColor(chip.background_color(&theme));
        *border_color = BorderColor::all(chip.outline_color(&theme));
    }
}

/// System to update chip label/icon colors when chip state changes.
///
/// The base chip styling system updates background + outline based on `MaterialChip`.
/// This system ensures the textual contents (label, leading icon, delete icon)
/// also follow the chip's computed colors.
fn chip_content_style_system(
    theme: Option<Res<MaterialTheme>>,
    chips: Query<(Entity, &MaterialChip), Changed<MaterialChip>>,
    children_q: Query<&Children>,
    mut colors: ParamSet<(
        Query<&mut TextColor, With<ChipLabel>>,
        Query<&mut TextColor, With<ChipLeadingIcon>>,
        Query<&mut TextColor, With<ChipDeleteIcon>>,
    )>,
) {
    let Some(theme) = theme else { return };

    for (chip_entity, chip) in chips.iter() {
        let Ok(children) = children_q.get(chip_entity) else {
            continue;
        };

        let label_color = chip.label_color(&theme);
        let icon_color = chip.icon_color(&theme);

        for child in children.iter() {
            if let Ok(mut color) = colors.p0().get_mut(child) {
                color.0 = label_color;
            }
            if let Ok(mut color) = colors.p1().get_mut(child) {
                color.0 = icon_color;
            }

            // Delete icon is a grandchild under ChipDeleteButton.
            if let Ok(grandchildren) = children_q.get(child) {
                for grandchild in grandchildren.iter() {
                    if let Ok(mut color) = colors.p2().get_mut(grandchild) {
                        color.0 = icon_color;
                    }
                }
            }
        }
    }
}

/// System to update chip shadows using native BoxShadow
fn chip_shadow_system(mut chips: Query<(&MaterialChip, &mut BoxShadow), Changed<MaterialChip>>) {
    for (chip, mut shadow) in chips.iter_mut() {
        *shadow = chip.elevation.to_box_shadow();
    }
}

/// Refresh chip visuals when the theme changes.
fn chip_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut chips: Query<(
        Entity,
        &MaterialChip,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    children_q: Query<&Children>,
    mut colors: ParamSet<(
        Query<&mut TextColor, With<ChipLabel>>,
        Query<&mut TextColor, With<ChipLeadingIcon>>,
        Query<&mut TextColor, With<ChipDeleteIcon>>,
    )>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (chip_entity, chip, mut bg_color, mut border_color) in chips.iter_mut() {
        *bg_color = BackgroundColor(chip.background_color(&theme));
        *border_color = BorderColor::all(chip.outline_color(&theme));

        let Ok(children) = children_q.get(chip_entity) else {
            continue;
        };
        let label_color = chip.label_color(&theme);
        let icon_color = chip.icon_color(&theme);

        for child in children.iter() {
            if let Ok(mut color) = colors.p0().get_mut(child) {
                color.0 = label_color;
            }
            if let Ok(mut color) = colors.p1().get_mut(child) {
                color.0 = icon_color;
            }

            // Delete icon is a grandchild under ChipDeleteButton.
            if let Ok(grandchildren) = children_q.get(child) {
                for grandchild in grandchildren.iter() {
                    if let Ok(mut color) = colors.p2().get_mut(grandchild) {
                        color.0 = icon_color;
                    }
                }
            }
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
    fn test_chip_creation() {
        let chip = MaterialChip::new("Test");
        assert_eq!(chip.label, "Test");
        assert_eq!(chip.variant, ChipVariant::Assist);
        assert!(!chip.selected);
        assert!(!chip.disabled);
    }

    #[test]
    fn test_filter_chip() {
        let chip = MaterialChip::filter("Category").with_selected(true);
        assert_eq!(chip.variant, ChipVariant::Filter);
        assert!(chip.selected);
    }

    #[test]
    fn test_input_chip() {
        let chip = MaterialChip::input("Tag");
        assert_eq!(chip.variant, ChipVariant::Input);
        assert!(chip.deletable);
    }

    #[test]
    fn test_chip_with_value() {
        let chip = MaterialChip::new("Label").with_value("value-123");
        assert_eq!(chip.value, Some("value-123".to_string()));
    }

    #[test]
    fn test_chip_builder() {
        let builder = ChipBuilder::filter("Size")
            .value("size-large")
            .selected(true);

        assert_eq!(builder.chip.label, "Size");
        assert_eq!(builder.chip.value, Some("size-large".to_string()));
        assert!(builder.chip.selected);
    }
}
