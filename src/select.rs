//! Material Design 3 Select (Dropdown) component
//!
//! Select menus display a list of choices on a temporary surface and allow users to select one.
//! Reference: <https://m3.material.io/components/menus/overview>

use bevy::prelude::*;

use crate::{
    icons::MaterialIcon,
    icons::MaterialIconFont,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the select component
pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SelectChangeEvent>()
            .add_systems(
                Update,
                (
                    select_interaction_system,
                    select_style_system,
                    select_content_style_system,
                    select_theme_refresh_system,
                    select_dropdown_sync_system,
                    select_option_interaction_system,
                    select_option_icon_font_system,
                ),
            );
    }
}

/// Select variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectVariant {
    /// Filled select field
    #[default]
    Filled,
    /// Outlined select field
    Outlined,
}

/// Material select component
#[derive(Component)]
pub struct MaterialSelect {
    /// Select variant
    pub variant: SelectVariant,
    /// Currently selected option index
    pub selected_index: Option<usize>,
    /// Options list
    pub options: Vec<SelectOption>,
    /// Label text
    pub label: Option<String>,
    /// Supporting text
    pub supporting_text: Option<String>,
    /// Whether the select is disabled
    pub disabled: bool,
    /// Whether there's an error
    pub error: bool,
    /// Error message
    pub error_text: Option<String>,
    /// Whether the dropdown is open
    pub open: bool,
    /// Interaction states
    pub focused: bool,
    pub hovered: bool,
}

impl MaterialSelect {
    /// Create a new select
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            variant: SelectVariant::default(),
            selected_index: None,
            options,
            label: None,
            supporting_text: None,
            disabled: false,
            error: false,
            error_text: None,
            open: false,
            focused: false,
            hovered: false,
        }
    }

    /// Set variant
    pub fn with_variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set initially selected option
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected_index = Some(index);
        }
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set error text
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.error_text = Some(text.into());
        self.error = true;
        self
    }

    /// Get the selected option
    pub fn selected_option(&self) -> Option<&SelectOption> {
        self.selected_index.and_then(|i| self.options.get(i))
    }

    /// Get the display text for the current selection
    pub fn display_text(&self) -> String {
        self.selected_option()
            .map(|o| o.label.clone())
            .unwrap_or_default()
    }

    /// Get the container color
    pub fn container_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.04);
        }

        match self.variant {
            SelectVariant::Filled => theme.surface_container_highest,
            SelectVariant::Outlined => Color::NONE,
        }
    }

    /// Get the indicator/outline color
    pub fn indicator_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        if self.focused || self.open {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the label color
    pub fn label_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        if self.focused || self.open {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the text color
    pub fn text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the trailing icon color
    pub fn trailing_icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else if self.error {
            theme.error
        } else {
            theme.on_surface_variant
        }
    }
}

/// A select option
#[derive(Debug, Clone)]
pub struct SelectOption {
    /// Display label
    pub label: String,
    /// Optional value (can be used for form submission)
    pub value: Option<String>,
    /// Optional leading icon
    pub icon: Option<String>,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl SelectOption {
    /// Create a new option
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            icon: None,
            disabled: false,
        }
    }

    /// Set the value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Event when selection changes
#[derive(Event, bevy::prelude::Message)]
pub struct SelectChangeEvent {
    pub entity: Entity,
    pub index: usize,
    pub option: SelectOption,
}

/// Select dimensions
pub const SELECT_HEIGHT: f32 = 56.0;
pub const SELECT_OPTION_HEIGHT: f32 = 48.0;

/// System to handle select interactions
fn select_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut MaterialSelect),
        (Changed<Interaction>, With<MaterialSelect>),
    >,
) {
    for (interaction, mut select) in interaction_query.iter_mut() {
        if select.disabled {
            continue;
        }

        match *interaction {
            Interaction::Pressed => {
                select.open = !select.open;
                select.focused = true;
            }
            Interaction::Hovered => {
                select.hovered = true;
            }
            Interaction::None => {
                select.hovered = false;
            }
        }
    }
}

/// System to update select styles
fn select_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut selects: Query<
        (&MaterialSelect, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialSelect>,
    >,
) {
    let Some(theme) = theme else { return };

    for (select, mut bg_color, mut border_color) in selects.iter_mut() {
        *bg_color = BackgroundColor(select.container_color(&theme));
        *border_color = BorderColor::all(select.indicator_color(&theme));
    }
}

/// Update select child visuals (text colors, dropdown surface, option selection highlight)
/// whenever select state changes.
fn select_content_style_system(
    theme: Option<Res<MaterialTheme>>,
    changed_selects: Query<Entity, Changed<MaterialSelect>>,
    selects: Query<&MaterialSelect>,
    mut text_colors: ParamSet<(
        Query<(&ChildOf, &mut TextColor), With<SelectDisplayText>>,
        Query<(&ChildOf, &mut TextColor), With<SelectDropdownArrow>>,
        Query<&mut TextColor, With<SelectOptionLabelText>>,
        Query<&mut TextColor, With<SelectOptionIcon>>,
    )>,
    mut dropdowns: Query<
        (&ChildOf, &mut BackgroundColor),
        (With<SelectDropdown>, Without<SelectOptionItem>, Without<MaterialSelect>),
    >,
    mut option_rows: Query<
        (&SelectOwner, &SelectOptionItem, &mut BackgroundColor, &Children),
        (Without<SelectDropdown>, Without<MaterialSelect>),
    >,
) {
    let Some(theme) = theme else { return };
    if changed_selects.iter().next().is_none() {
        return;
    }

    for (parent, mut color) in text_colors.p0().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.0 = select.text_color(&theme);
        }
    }

    for (parent, mut color) in text_colors.p1().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.0 = select.label_color(&theme);
        }
    }

    for (parent, mut bg) in dropdowns.iter_mut() {
        if selects.get(parent.parent()).is_ok() {
            bg.0 = theme.surface_container;
        }
    }

    for (owner, option_item, mut row_bg, children) in option_rows.iter_mut() {
        let Ok(select) = selects.get(owner.0) else { continue };

        let is_selected = select.selected_index.is_some_and(|i| i == option_item.index);
        row_bg.0 = if is_selected {
            theme.secondary_container
        } else {
            Color::NONE
        };

        let base = theme.on_surface;
        let is_disabled = select
            .options
            .get(option_item.index)
            .is_some_and(|o| o.disabled);
        let text_color = if is_disabled {
            base.with_alpha(0.38)
        } else {
            base
        };

        for child in children.iter() {
            if let Ok(mut c) = text_colors.p2().get_mut(child) {
                c.0 = text_color;
            }
            if let Ok(mut c) = text_colors.p3().get_mut(child) {
                c.0 = text_color;
            }
        }
    }
}

/// Refresh select visuals when the theme changes.
fn select_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    selects: Query<&MaterialSelect>,
    mut triggers: Query<
        (&MaterialSelect, &mut BackgroundColor, &mut BorderColor),
        (Without<SelectDropdown>, Without<SelectOptionItem>),
    >,
    mut text_colors: ParamSet<(
        Query<(&ChildOf, &mut TextColor), With<SelectDisplayText>>,
        Query<(&ChildOf, &mut TextColor), With<SelectDropdownArrow>>,
        Query<&mut TextColor, With<SelectOptionLabelText>>,
        Query<&mut TextColor, With<SelectOptionIcon>>,
    )>,
    mut dropdowns: Query<
        (&ChildOf, &mut BackgroundColor),
        (With<SelectDropdown>, Without<SelectOptionItem>, Without<MaterialSelect>),
    >,
    mut option_rows: Query<
        (&SelectOwner, &SelectOptionItem, &mut BackgroundColor, &Children),
        (Without<SelectDropdown>, Without<MaterialSelect>),
    >,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (select, mut bg, mut border) in triggers.iter_mut() {
        bg.0 = select.container_color(&theme);
        *border = BorderColor::all(select.indicator_color(&theme));
    }

    for (parent, mut color) in text_colors.p0().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.0 = select.text_color(&theme);
        }
    }

    for (parent, mut color) in text_colors.p1().iter_mut() {
        if let Ok(select) = selects.get(parent.parent()) {
            color.0 = select.label_color(&theme);
        }
    }

    for (parent, mut bg) in dropdowns.iter_mut() {
        if selects.get(parent.parent()).is_ok() {
            bg.0 = theme.surface_container;
        }
    }

    for (owner, option_item, mut row_bg, children) in option_rows.iter_mut() {
        let Ok(select) = selects.get(owner.0) else { continue };

        let is_selected = select.selected_index.is_some_and(|i| i == option_item.index);
        row_bg.0 = if is_selected {
            theme.secondary_container
        } else {
            Color::NONE
        };

        let base = theme.on_surface;
        let is_disabled = select
            .options
            .get(option_item.index)
            .is_some_and(|o| o.disabled);
        let text_color = if is_disabled {
            base.with_alpha(0.38)
        } else {
            base
        };

        for child in children.iter() {
            if let Ok(mut c) = text_colors.p2().get_mut(child) {
                c.0 = text_color;
            }
            if let Ok(mut c) = text_colors.p3().get_mut(child) {
                c.0 = text_color;
            }
        }
    }
}

/// Builder for select components
pub struct SelectBuilder {
    select: MaterialSelect,
    width: Val,
}

impl SelectBuilder {
    /// Create a new select builder
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            select: MaterialSelect::new(options),
            width: Val::Px(210.0),
        }
    }

    /// Set variant
    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.select.variant = variant;
        self
    }

    /// Make filled
    pub fn filled(self) -> Self {
        self.variant(SelectVariant::Filled)
    }

    /// Make outlined
    pub fn outlined(self) -> Self {
        self.variant(SelectVariant::Outlined)
    }

    /// Set initially selected option
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.select.options.len() {
            self.select.selected_index = Some(index);
        }
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.select.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.select.supporting_text = Some(text.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.select.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.select.error = error;
        self
    }

    /// Set error text
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.select.error_text = Some(text.into());
        self.select.error = true;
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the select bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.select.container_color(theme);
        let border_color = self.select.indicator_color(theme);
        let is_outlined = self.select.variant == SelectVariant::Outlined;

        (
            self.select,
            Button,
            Node {
                width: self.width,
                height: Val::Px(SELECT_HEIGHT),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                border: if is_outlined {
                    UiRect::all(Val::Px(1.0))
                } else {
                    UiRect::bottom(Val::Px(1.0))
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::top(Val::Px(CornerRadius::EXTRA_SMALL)),
        )
    }
}

/// Marker for select dropdown
#[derive(Component)]
pub struct SelectDropdown;

/// Internal marker for option icons so we can apply the Material Symbols font.
#[derive(Component)]
struct SelectOptionIcon;

/// Internal marker used to route option clicks back to the owning select.
#[derive(Component, Clone, Copy)]
struct SelectOwner(Entity);

/// Marker for select option item (component attached to each option in the dropdown)
#[derive(Component)]
pub struct SelectOptionItem {
    /// Index of this option in the options list
    pub index: usize,
    /// Display label for this option
    pub label: String,
}

/// Marker for select container (parent of trigger and dropdown)
#[derive(Component)]
pub struct SelectContainer;

/// Marker for select trigger button
#[derive(Component)]
pub struct SelectTrigger {
    /// Available options
    #[allow(dead_code)]
    pub options: Vec<String>,
    /// Currently selected index
    pub selected_index: usize,
}

/// Marker for select's displayed text
#[derive(Component)]
pub struct SelectDisplayText;

/// Marker for the dropdown arrow text node.
#[derive(Component)]
pub struct SelectDropdownArrow;

/// Marker for select option label text nodes.
#[derive(Component)]
pub struct SelectOptionLabelText;

/// Keep dropdown visibility + displayed text in sync with `MaterialSelect`.
fn select_dropdown_sync_system(
    mut selects: Query<(&MaterialSelect, &Children), Changed<MaterialSelect>>,
    mut dropdowns: Query<&mut Visibility, With<SelectDropdown>>,
    mut display_texts: Query<&mut Text, With<SelectDisplayText>>,
) {
    for (select, children) in selects.iter_mut() {
        // Update dropdown visibility
        for child in children.iter() {
            if let Ok(mut vis) = dropdowns.get_mut(child) {
                *vis = if select.open {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }

        // Update displayed text
        let placeholder = select
            .label
            .as_deref()
            .unwrap_or("Select");

        let display = select
            .selected_option()
            .map(|o| o.label.as_str())
            .unwrap_or(placeholder);

        for child in children.iter() {
            if let Ok(mut text) = display_texts.get_mut(child) {
                *text = Text::new(display);
            }
        }
    }
}

/// Handle clicks on option items.
fn select_option_interaction_system(
    mut interactions: Query<(&Interaction, &SelectOptionItem, &SelectOwner), Changed<Interaction>>,
    mut selects: Query<(Entity, &mut MaterialSelect)>,
    mut events: MessageWriter<SelectChangeEvent>,
) {
    for (interaction, option_item, owner) in interactions.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok((select_entity, mut select)) = selects.get_mut(owner.0) else {
            continue;
        };

        // Ignore disabled options
        let Some(option) = select.options.get(option_item.index).cloned() else {
            continue;
        };
        if option.disabled {
            continue;
        }

        select.selected_index = Some(option_item.index);
        select.open = false;
        select.focused = true;

        events.write(SelectChangeEvent {
            entity: select_entity,
            index: option_item.index,
            option,
        });
    }
}

/// Apply the Material Symbols font to select option icon text nodes.
fn select_option_icon_font_system(
    icon_font: Option<Res<MaterialIconFont>>,
    mut icons: Query<&mut TextFont, Or<(With<SelectOptionIcon>, With<SelectDropdownArrow>)>>,
) {
    let Some(icon_font) = icon_font else { return };
    for mut text_font in icons.iter_mut() {
        text_font.font = icon_font.handle();
    }
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material selects as children
pub trait SpawnSelectChild {
    /// Spawn a filled select
    fn spawn_filled_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    );
    
    /// Spawn an outlined select
    fn spawn_outlined_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    );
    
    /// Spawn a select with full builder control
    fn spawn_select_with(&mut self, theme: &MaterialTheme, builder: SelectBuilder);
}

impl SpawnSelectChild for ChildSpawnerCommands<'_> {
    fn spawn_filled_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    ) {
        self.spawn_select_with(
            theme,
            SelectBuilder::new(options).label(label).filled(),
        );
    }
    
    fn spawn_outlined_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    ) {
        self.spawn_select_with(
            theme,
            SelectBuilder::new(options).label(label).outlined(),
        );
    }
    
    fn spawn_select_with(&mut self, theme: &MaterialTheme, builder: SelectBuilder) {
        let label_color = builder.select.label_color(theme);
        let text_color = builder.select.text_color(theme);
        let option_text_color = theme.on_surface;

        // Clone options for building the dropdown list
        let options = builder.select.options.clone();
        let selected_index = builder.select.selected_index;
        let placeholder = builder
            .select
            .label
            .clone()
            .unwrap_or_else(|| "Select".to_string());

        let mut select_entity_commands = self.spawn(builder.build(theme));
        let select_entity = select_entity_commands.id();

        select_entity_commands.with_children(|select| {
                // Display text
                let display_label = selected_index
                    .and_then(|idx| options.get(idx))
                    .map(|o| o.label.as_str())
                    .unwrap_or(placeholder.as_str());

                select.spawn((
                    SelectDisplayText,
                    Text::new(display_label),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(text_color),
                    Node { flex_grow: 1.0, ..default() },
                ));

                // Dropdown arrow
                select.spawn((
                    SelectDropdownArrow,
                    Text::new(MaterialIcon::expand_more().as_str()),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(label_color),
                ));

                // Dropdown list (hidden by default)
                select
                    .spawn((
                        SelectDropdown,
                        Visibility::Hidden,
                        // Ensure the dropdown renders above later siblings (e.g. code blocks).
                        GlobalZIndex(100),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(SELECT_HEIGHT + 4.0),
                            left: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::vertical(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container),
                        BorderRadius::all(Val::Px(8.0)),
                    ))
                    .with_children(|dropdown| {
                        for (index, option) in options.iter().enumerate() {
                            let is_disabled = option.disabled;
                            let is_selected = selected_index.is_some_and(|i| i == index);
                            let row_bg = if is_selected {
                                theme.secondary_container
                            } else {
                                Color::NONE
                            };

                            dropdown
                                .spawn((
                                    SelectOwner(select_entity),
                                    SelectOptionItem {
                                        index,
                                        label: option.label.clone(),
                                    },
                                    Button,
                                    Interaction::None,
                                    Node {
                                        height: Val::Px(SELECT_OPTION_HEIGHT),
                                        padding: UiRect::horizontal(Val::Px(Spacing::LARGE)),
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(Spacing::MEDIUM),
                                        ..default()
                                    },
                                    BackgroundColor(row_bg),
                                ))
                                .with_children(|row| {
                                    // Optional leading icon
                                    if let Some(icon) = &option.icon {
                                        let icon_text = MaterialIcon::from_name(icon.as_str())
                                            .map(|i| i.as_str())
                                            .unwrap_or_else(|| icon.clone());

                                        row.spawn((
                                            SelectOptionIcon,
                                            Text::new(icon_text),
                                            TextFont { font_size: 20.0, ..default() },
                                            TextColor(if is_disabled {
                                                option_text_color.with_alpha(0.38)
                                            } else {
                                                option_text_color
                                            }),
                                        ));
                                    }

                                    row.spawn((
                                        SelectOptionLabelText,
                                        Text::new(option.label.clone()),
                                        TextFont { font_size: 14.0, ..default() },
                                        TextColor(if is_disabled {
                                            option_text_color.with_alpha(0.38)
                                        } else {
                                            option_text_color
                                        }),
                                    ));
                                });
                        }
                    });
            });
    }
}
