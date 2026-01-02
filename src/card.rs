//! Material Design 3 Card component
//!
//! Cards contain content and actions about a single subject.
//! Reference: <https://m3.material.io/components/cards/overview>
//!
//! ## Bevy 0.17 Improvements
//!
//! This module now leverages:
//! - Native `BoxShadow` for elevation shadows
//! - Modern bundle patterns

use bevy::prelude::*;
use bevy::ui::BoxShadow;

use crate::{
    elevation::Elevation,
    theme::{blend_state_layer, MaterialTheme},
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the card component
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<CardClickEvent>().add_systems(
            Update,
            (
                card_interaction_system,
                card_style_system,
                card_theme_refresh_system,
                card_shadow_system,
            ),
        );
    }
}

/// Card variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CardVariant {
    /// Elevated card - Has shadow elevation
    #[default]
    Elevated,
    /// Filled card - Has filled background
    Filled,
    /// Outlined card - Has border outline
    Outlined,
}

/// Material card component
#[derive(Component)]
pub struct MaterialCard {
    /// Card variant
    pub variant: CardVariant,
    /// Whether the card is clickable/interactive
    pub clickable: bool,
    /// Whether the card is draggable
    pub draggable: bool,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialCard {
    /// Create a new card
    pub fn new() -> Self {
        Self {
            variant: CardVariant::default(),
            clickable: false,
            draggable: false,
            pressed: false,
            hovered: false,
        }
    }

    /// Set the card variant
    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make the card clickable
    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    /// Make the card draggable
    pub fn draggable(mut self) -> Self {
        self.draggable = true;
        self
    }

    /// Get the background color with state layer applied
    pub fn background_color(&self, theme: &MaterialTheme) -> Color {
        let base = match self.variant {
            CardVariant::Elevated => theme.surface_container_low,
            CardVariant::Filled => theme.surface_container_highest,
            CardVariant::Outlined => theme.surface,
        };

        // Apply state layer for clickable cards
        let state_opacity = self.state_layer_opacity();
        if state_opacity > 0.0 {
            blend_state_layer(base, theme.on_surface, state_opacity)
        } else {
            base
        }
    }

    /// Get the border color
    pub fn border_color(&self, theme: &MaterialTheme) -> Color {
        match self.variant {
            CardVariant::Outlined => theme.outline_variant,
            _ => Color::NONE,
        }
    }

    /// Get the elevation for this card
    pub fn elevation(&self) -> Elevation {
        match self.variant {
            CardVariant::Elevated => {
                if self.pressed || self.hovered {
                    Elevation::Level2
                } else {
                    Elevation::Level1
                }
            }
            CardVariant::Filled | CardVariant::Outlined => {
                if self.clickable && (self.pressed || self.hovered) {
                    Elevation::Level1
                } else {
                    Elevation::Level0
                }
            }
        }
    }

    /// Get state layer opacity
    pub fn state_layer_opacity(&self) -> f32 {
        if !self.clickable {
            return 0.0;
        }

        if self.pressed {
            0.12
        } else if self.hovered {
            0.08
        } else {
            0.0
        }
    }
}

impl Default for MaterialCard {
    fn default() -> Self {
        Self::new()
    }
}

/// Event when card is clicked
#[derive(Event, bevy::prelude::Message)]
pub struct CardClickEvent {
    pub entity: Entity,
}

/// System to handle card interactions
fn card_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialCard),
        (Changed<Interaction>, With<MaterialCard>),
    >,
    mut click_events: MessageWriter<CardClickEvent>,
) {
    for (entity, interaction, mut card) in interaction_query.iter_mut() {
        if !card.clickable {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                card.pressed = true;
                card.hovered = false;
                click_events.write(CardClickEvent { entity });
            }
            Interaction::Hovered => {
                card.pressed = false;
                card.hovered = true;
            }
            Interaction::None => {
                card.pressed = false;
                card.hovered = false;
            }
        }
    }
}

/// System to update card styles
fn card_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut cards: Query<
        (&MaterialCard, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialCard>,
    >,
) {
    let Some(theme) = theme else { return };

    for (card, mut bg_color, mut border_color) in cards.iter_mut() {
        *bg_color = BackgroundColor(card.background_color(&theme));
        *border_color = BorderColor::all(card.border_color(&theme));
    }
}

/// Refresh card colors when the theme changes.
fn card_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut cards: Query<(&MaterialCard, &mut BackgroundColor, &mut BorderColor)>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (card, mut bg_color, mut border_color) in cards.iter_mut() {
        *bg_color = BackgroundColor(card.background_color(&theme));
        *border_color = BorderColor::all(card.border_color(&theme));
    }
}

/// System to update card shadows using Bevy's native BoxShadow
fn card_shadow_system(mut cards: Query<(&MaterialCard, &mut BoxShadow), Changed<MaterialCard>>) {
    for (card, mut box_shadow) in cards.iter_mut() {
        let elevation = card.elevation();
        *box_shadow = elevation.to_box_shadow();
    }
}

/// Builder for cards
///
/// ## Example with Bevy 0.17's native shadows:
///
/// ```ignore
/// commands.spawn((
///     CardBuilder::new().elevated().build(&theme),
///     children![
///         (Text::new("Card Title"), TextFont { font_size: 20.0, ..default() }),
///     ],
/// ));
/// ```
pub struct CardBuilder {
    card: MaterialCard,
    width: Option<Val>,
    height: Option<Val>,
    padding: f32,
}

impl CardBuilder {
    /// Create a new card builder
    pub fn new() -> Self {
        Self {
            card: MaterialCard::new(),
            width: None,
            height: None,
            padding: Spacing::LARGE,
        }
    }

    /// Set the variant
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.card.variant = variant;
        self
    }

    /// Make elevated card
    pub fn elevated(self) -> Self {
        self.variant(CardVariant::Elevated)
    }

    /// Make filled card
    pub fn filled(self) -> Self {
        self.variant(CardVariant::Filled)
    }

    /// Make outlined card
    pub fn outlined(self) -> Self {
        self.variant(CardVariant::Outlined)
    }

    /// Make clickable
    pub fn clickable(mut self) -> Self {
        self.card.clickable = true;
        self
    }

    /// Make draggable
    pub fn draggable(mut self) -> Self {
        self.card.draggable = true;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Build the card bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.card.background_color(theme);
        let border_color = self.card.border_color(theme);
        let border_width = if self.card.variant == CardVariant::Outlined {
            1.0
        } else {
            0.0
        };
        let elevation = self.card.elevation();

        let mut node = Node {
            padding: UiRect::all(Val::Px(self.padding)),
            border: UiRect::all(Val::Px(border_width)),
            flex_direction: FlexDirection::Column,
            ..default()
        };

        if let Some(w) = self.width {
            node.width = w;
        }
        if let Some(h) = self.height {
            node.height = h;
        }

        (
            self.card,
            node,
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
            // Native Bevy 0.17 shadow support
            elevation.to_box_shadow(),
        )
    }

    /// Build the card bundle without shadow
    pub fn build_without_shadow(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.card.background_color(theme);
        let border_color = self.card.border_color(theme);
        let border_width = if self.card.variant == CardVariant::Outlined {
            1.0
        } else {
            0.0
        };

        let mut node = Node {
            padding: UiRect::all(Val::Px(self.padding)),
            border: UiRect::all(Val::Px(border_width)),
            flex_direction: FlexDirection::Column,
            ..default()
        };

        if let Some(w) = self.width {
            node.width = w;
        }
        if let Some(h) = self.height {
            node.height = h;
        }

        (
            self.card,
            node,
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
        )
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material cards as children
///
/// This trait provides a clean API for spawning cards within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_elevated_card(&theme, |card| {
///         card.spawn((Text::new("Card Content"), TextColor(theme.on_surface)));
///     });
/// });
/// ```
pub trait SpawnCardChild {
    /// Spawn an elevated card
    fn spawn_elevated_card(
        &mut self,
        theme: &MaterialTheme,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a filled card
    fn spawn_filled_card(
        &mut self,
        theme: &MaterialTheme,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn an outlined card
    fn spawn_outlined_card(
        &mut self,
        theme: &MaterialTheme,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a card with full builder control
    fn spawn_card_with(
        &mut self,
        theme: &MaterialTheme,
        builder: CardBuilder,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    );
}

impl SpawnCardChild for ChildSpawnerCommands<'_> {
    fn spawn_elevated_card(
        &mut self,
        theme: &MaterialTheme,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_card_with(theme, CardBuilder::new().elevated(), with_children);
    }

    fn spawn_filled_card(
        &mut self,
        theme: &MaterialTheme,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_card_with(theme, CardBuilder::new().filled(), with_children);
    }

    fn spawn_outlined_card(
        &mut self,
        theme: &MaterialTheme,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_card_with(theme, CardBuilder::new().outlined(), with_children);
    }

    fn spawn_card_with(
        &mut self,
        theme: &MaterialTheme,
        builder: CardBuilder,
        with_children: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn(builder.build(theme))
            .with_children(with_children);
    }
}
