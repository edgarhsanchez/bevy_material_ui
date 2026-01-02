//! Material Design 3 Radio Button component
//!
//! Radio buttons let users select one option from a set.
//! Reference: <https://m3.material.io/components/radio-button/overview>
//!
//! # Example
//! ```ignore
//! // Using the spawn extension trait (recommended)
//! commands.spawn_radio(&theme, false, "my_group", "Option 1");
//!
//! // Or using the builder for more control
//! parent.spawn_radio_in(&theme, RadioBuilder::new().selected(true).group("my_group"), "Label");
//! ```

use bevy::prelude::*;

use crate::{
    motion::StateLayer,
    ripple::RippleHost,
    telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId},
    theme::MaterialTheme,
    tokens::CornerRadius,
};

/// Marker component for the radio outer circle
#[derive(Component)]
pub struct RadioOuter;

/// Marker component for the radio inner dot
#[derive(Component)]
pub struct RadioInner;

/// Marker component for the radio state layer
#[derive(Component)]
pub struct RadioStateLayer;

/// Plugin for the radio button component
pub struct RadioPlugin;

impl Plugin for RadioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RadioChangeEvent>().add_systems(
            Update,
            (
                radio_interaction_system,
                radio_group_system,
                radio_style_system,
                radio_theme_refresh_system,
                radio_telemetry_system,
            ),
        );
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
    }
}

fn radio_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    radios: Query<(&TestId, &Children), With<MaterialRadio>>,
    children_query: Query<&Children>,
    radio_outer: Query<(), With<RadioOuter>>,
    radio_inner: Query<(), With<RadioInner>>,
    radio_state_layer: Query<(), With<RadioStateLayer>>,
) {
    let Some(telemetry) = telemetry else { return };
    if !telemetry.enabled {
        return;
    }

    for (test_id, children) in radios.iter() {
        let base = test_id.id();

        let mut found_outer = false;
        let mut found_inner = false;
        let mut found_state_layer = false;

        let mut stack: Vec<Entity> = children.iter().collect();
        while let Some(entity) = stack.pop() {
            if !found_state_layer && radio_state_layer.get(entity).is_ok() {
                found_state_layer = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/state_layer"),
                });
            }

            if !found_outer && radio_outer.get(entity).is_ok() {
                found_outer = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/outer"),
                });
            }

            if !found_inner && radio_inner.get(entity).is_ok() {
                found_inner = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/inner"),
                });
            }

            if found_outer && found_inner && found_state_layer {
                break;
            }

            if let Ok(children) = children_query.get(entity) {
                stack.extend(children.iter());
            }
        }
    }
}

/// Material radio button component
#[derive(Component)]
pub struct MaterialRadio {
    /// Whether this radio is selected
    pub selected: bool,
    /// Whether the radio is disabled
    pub disabled: bool,
    /// The group this radio belongs to
    pub group: Option<String>,
    /// Interaction states
    pub pressed: bool,
    pub hovered: bool,
}

impl MaterialRadio {
    /// Create a new radio button
    pub fn new() -> Self {
        Self {
            selected: false,
            disabled: false,
            group: None,
            pressed: false,
            hovered: false,
        }
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the radio group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Get the outer circle color
    pub fn outer_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.selected {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the inner dot color
    pub fn inner_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        theme.primary
    }

    /// Get the state layer color
    pub fn state_layer_color(&self, theme: &MaterialTheme) -> Color {
        if self.selected {
            theme.primary
        } else {
            theme.on_surface
        }
    }
}

impl Default for MaterialRadio {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to define a radio group
#[derive(Component)]
pub struct RadioGroup {
    /// Group identifier
    pub name: String,
    /// Currently selected value (entity ID)
    pub selected: Option<Entity>,
}

impl RadioGroup {
    /// Create a new radio group
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selected: None,
        }
    }
}

/// Event when radio selection changes
#[derive(Event, bevy::prelude::Message)]
pub struct RadioChangeEvent {
    pub entity: Entity,
    pub group: Option<String>,
    pub selected: bool,
}

/// Radio button size
pub const RADIO_SIZE: f32 = 20.0;
/// Radio inner dot size when selected
pub const RADIO_DOT_SIZE: f32 = 10.0;
/// Radio touch target size
pub const RADIO_TOUCH_TARGET: f32 = 48.0;

/// System to handle radio interactions
fn radio_interaction_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut MaterialRadio),
        (Changed<Interaction>, With<MaterialRadio>),
    >,
    mut change_events: MessageWriter<RadioChangeEvent>,
) {
    for (entity, interaction, mut radio) in interaction_query.iter_mut() {
        if radio.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                radio.pressed = true;
                radio.hovered = false;

                // Only fire event if not already selected
                if !radio.selected {
                    radio.selected = true;
                    change_events.write(RadioChangeEvent {
                        entity,
                        group: radio.group.clone(),
                        selected: true,
                    });
                }
            }
            Interaction::Hovered => {
                radio.pressed = false;
                radio.hovered = true;
            }
            Interaction::None => {
                radio.pressed = false;
                radio.hovered = false;
            }
        }
    }
}

/// System to handle radio group exclusivity
fn radio_group_system(
    mut change_events: MessageReader<RadioChangeEvent>,
    mut radios: Query<(Entity, &mut MaterialRadio)>,
) {
    for event in change_events.read() {
        if let Some(ref group_name) = event.group {
            // Deselect all other radios in the same group
            for (entity, mut radio) in radios.iter_mut() {
                if entity != event.entity {
                    if let Some(ref radio_group) = radio.group {
                        if radio_group == group_name && radio.selected {
                            radio.selected = false;
                        }
                    }
                }
            }
        }
    }
}

/// System to update radio visual styles when state changes
fn radio_style_system(
    theme: Option<Res<MaterialTheme>>,
    radios: Query<(&MaterialRadio, &Children), Changed<MaterialRadio>>,
    children_query: Query<&Children>,
    state_layer_query: Query<&Children, With<RadioStateLayer>>,
    mut outer_query: Query<(&mut BorderColor, &Children), With<RadioOuter>>,
    mut inner_query: Query<&mut BackgroundColor, With<RadioInner>>,
) {
    let Some(theme) = theme else { return };

    for (radio, radio_children) in radios.iter() {
        let outer_color = radio.outer_color(&theme);
        let inner_color = if radio.selected {
            radio.inner_color(&theme)
        } else {
            Color::NONE
        };

        // Navigate: Radio -> Touch target children -> StateLayer -> RadioOuter -> RadioInner
        for touch_target in radio_children.iter() {
            // Check if this child has a state layer
            if let Ok(state_layer_children) = state_layer_query.get(touch_target) {
                for state_layer_child in state_layer_children.iter() {
                    // Update RadioOuter
                    if let Ok((mut border, outer_children)) = outer_query.get_mut(state_layer_child)
                    {
                        *border = BorderColor::all(outer_color);

                        // Update RadioInner
                        for inner_entity in outer_children.iter() {
                            if let Ok(mut bg) = inner_query.get_mut(inner_entity) {
                                bg.0 = inner_color;
                            }
                        }
                    }
                }
            }

            // Also check children of touch target for state layer
            if let Ok(touch_children) = children_query.get(touch_target) {
                for touch_child in touch_children.iter() {
                    if let Ok(state_layer_children) = state_layer_query.get(touch_child) {
                        for state_layer_child in state_layer_children.iter() {
                            if let Ok((mut border, outer_children)) =
                                outer_query.get_mut(state_layer_child)
                            {
                                *border = BorderColor::all(outer_color);

                                for inner_entity in outer_children.iter() {
                                    if let Ok(mut bg) = inner_query.get_mut(inner_entity) {
                                        bg.0 = inner_color;
                                    }
                                }
                            }
                        }
                    }

                    // Direct RadioOuter under touch target
                    if let Ok((mut border, outer_children)) = outer_query.get_mut(touch_child) {
                        *border = BorderColor::all(outer_color);

                        for inner_entity in outer_children.iter() {
                            if let Ok(mut bg) = inner_query.get_mut(inner_entity) {
                                bg.0 = inner_color;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Refresh radio visuals when the theme changes.
fn radio_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    radios: Query<(&MaterialRadio, &Children)>,
    children_query: Query<&Children>,
    state_layer_query: Query<&Children, With<RadioStateLayer>>,
    mut outer_query: Query<(&mut BorderColor, &Children), With<RadioOuter>>,
    mut inner_query: Query<&mut BackgroundColor, With<RadioInner>>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (radio, radio_children) in radios.iter() {
        let outer_color = radio.outer_color(&theme);
        let inner_color = if radio.selected {
            radio.inner_color(&theme)
        } else {
            Color::NONE
        };

        for touch_target in radio_children.iter() {
            if let Ok(state_layer_children) = state_layer_query.get(touch_target) {
                for state_layer_child in state_layer_children.iter() {
                    if let Ok((mut border, outer_children)) = outer_query.get_mut(state_layer_child)
                    {
                        *border = BorderColor::all(outer_color);
                        for inner_entity in outer_children.iter() {
                            if let Ok(mut bg) = inner_query.get_mut(inner_entity) {
                                bg.0 = inner_color;
                            }
                        }
                    }
                }
            }

            if let Ok(touch_children) = children_query.get(touch_target) {
                for touch_child in touch_children.iter() {
                    if let Ok(state_layer_children) = state_layer_query.get(touch_child) {
                        for state_layer_child in state_layer_children.iter() {
                            if let Ok((mut border, outer_children)) =
                                outer_query.get_mut(state_layer_child)
                            {
                                *border = BorderColor::all(outer_color);
                                for inner_entity in outer_children.iter() {
                                    if let Ok(mut bg) = inner_query.get_mut(inner_entity) {
                                        bg.0 = inner_color;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Builder for radio buttons
pub struct RadioBuilder {
    radio: MaterialRadio,
}

impl RadioBuilder {
    /// Create a new radio builder
    pub fn new() -> Self {
        Self {
            radio: MaterialRadio::new(),
        }
    }

    /// Set initial selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.radio.selected = selected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.radio.disabled = disabled;
        self
    }

    /// Set the radio group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.radio.group = Some(group.into());
        self
    }

    /// Build the radio bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let _border_color = self.radio.outer_color(theme);

        (
            self.radio,
            Button,
            RippleHost::new(),
            Node {
                width: Val::Px(RADIO_TOUCH_TARGET),
                height: Val::Px(RADIO_TOUCH_TARGET),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderRadius::all(Val::Px(CornerRadius::FULL)),
        )
    }
}

impl Default for RadioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait to spawn radio buttons with full visual hierarchy
pub trait SpawnRadio {
    /// Spawn a radio button with a label
    fn spawn_radio(
        &mut self,
        theme: &MaterialTheme,
        selected: bool,
        group: impl Into<String>,
        label: &str,
    ) -> Entity;

    /// Spawn a radio button using a builder for more control
    fn spawn_radio_with(
        &mut self,
        theme: &MaterialTheme,
        builder: RadioBuilder,
        label: &str,
    ) -> Entity;
}

impl SpawnRadio for Commands<'_, '_> {
    fn spawn_radio(
        &mut self,
        theme: &MaterialTheme,
        selected: bool,
        group: impl Into<String>,
        label: &str,
    ) -> Entity {
        let builder = RadioBuilder::new().selected(selected).group(group);
        self.spawn_radio_with(theme, builder, label)
    }

    fn spawn_radio_with(
        &mut self,
        theme: &MaterialTheme,
        builder: RadioBuilder,
        label: &str,
    ) -> Entity {
        let label_color = theme.on_surface;
        let label_text = label.to_string();
        let is_selected = builder.radio.selected;
        let border_color = builder.radio.outer_color(theme);
        let inner_color = if is_selected {
            theme.primary
        } else {
            Color::NONE
        };
        let state_layer_color = builder.radio.state_layer_color(theme);

        self.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Radio touch target
            row.spawn((
                builder.radio,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(RADIO_TOUCH_TARGET),
                    height: Val::Px(RADIO_TOUCH_TARGET),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(CornerRadius::FULL)),
            ))
            .with_children(|touch| {
                // State layer
                touch
                    .spawn((
                        RadioStateLayer,
                        StateLayer::new(state_layer_color),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    ))
                    .with_children(|state_layer| {
                        // Outer circle
                        state_layer
                            .spawn((
                                RadioOuter,
                                Node {
                                    width: Val::Px(RADIO_SIZE),
                                    height: Val::Px(RADIO_SIZE),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderColor::all(border_color),
                                BorderRadius::all(Val::Px(RADIO_SIZE / 2.0)),
                            ))
                            .with_children(|outer| {
                                // Inner dot
                                outer.spawn((
                                    RadioInner,
                                    Node {
                                        width: Val::Px(RADIO_DOT_SIZE),
                                        height: Val::Px(RADIO_DOT_SIZE),
                                        ..default()
                                    },
                                    BackgroundColor(inner_color),
                                    BorderRadius::all(Val::Px(RADIO_DOT_SIZE / 2.0)),
                                ));
                            });
                    });
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

/// Extension trait to spawn radio buttons within a ChildSpawnerCommands context
pub trait SpawnRadioChild {
    /// Spawn a radio button with a label
    fn spawn_radio(
        &mut self,
        theme: &MaterialTheme,
        selected: bool,
        group: impl Into<String>,
        label: &str,
    );

    /// Spawn a radio button using a builder for more control
    fn spawn_radio_with(&mut self, theme: &MaterialTheme, builder: RadioBuilder, label: &str);
}

impl SpawnRadioChild for ChildSpawnerCommands<'_> {
    fn spawn_radio(
        &mut self,
        theme: &MaterialTheme,
        selected: bool,
        group: impl Into<String>,
        label: &str,
    ) {
        let builder = RadioBuilder::new().selected(selected).group(group);
        self.spawn_radio_with(theme, builder, label);
    }

    fn spawn_radio_with(&mut self, theme: &MaterialTheme, builder: RadioBuilder, label: &str) {
        let label_color = theme.on_surface;
        let label_text = label.to_string();
        let is_selected = builder.radio.selected;
        let border_color = builder.radio.outer_color(theme);
        let inner_color = if is_selected {
            theme.primary
        } else {
            Color::NONE
        };
        let state_layer_color = builder.radio.state_layer_color(theme);

        self.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Radio touch target
            row.spawn((
                builder.radio,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(RADIO_TOUCH_TARGET),
                    height: Val::Px(RADIO_TOUCH_TARGET),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(CornerRadius::FULL)),
            ))
            .with_children(|touch| {
                // State layer
                touch
                    .spawn((
                        RadioStateLayer,
                        StateLayer::new(state_layer_color),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    ))
                    .with_children(|state_layer| {
                        // Outer circle
                        state_layer
                            .spawn((
                                RadioOuter,
                                Node {
                                    width: Val::Px(RADIO_SIZE),
                                    height: Val::Px(RADIO_SIZE),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderColor::all(border_color),
                                BorderRadius::all(Val::Px(RADIO_SIZE / 2.0)),
                            ))
                            .with_children(|outer| {
                                // Inner dot
                                outer.spawn((
                                    RadioInner,
                                    Node {
                                        width: Val::Px(RADIO_DOT_SIZE),
                                        height: Val::Px(RADIO_DOT_SIZE),
                                        ..default()
                                    },
                                    BackgroundColor(inner_color),
                                    BorderRadius::all(Val::Px(RADIO_DOT_SIZE / 2.0)),
                                ));
                            });
                    });
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
