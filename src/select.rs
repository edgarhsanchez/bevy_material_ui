//! Material Design 3 Select (Dropdown) component
//!
//! Select menus display a list of choices on a temporary surface and allow users to select one.
//! Reference: <https://m3.material.io/components/menus/overview>

use bevy::prelude::*;

use crate::{
    i18n::{MaterialI18n, MaterialLanguage, MaterialLanguageOverride},
    icons::{icon_by_name, MaterialIcon, ICON_EXPAND_MORE},
    telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId},
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the select component
pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<SelectChangeEvent>().add_systems(
            Update,
            (
                select_interaction_system,
                select_style_system,
                select_content_style_system,
                select_theme_refresh_system,
                select_localization_system,
                select_dropdown_rebuild_options_system,
                select_dropdown_sync_system,
                select_option_interaction_system,
                select_telemetry_system,
            ),
        );
    }
}

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectLocalization {
    pub label_key: Option<String>,
    pub supporting_text_key: Option<String>,
    pub error_text_key: Option<String>,
}

impl SelectLocalization {
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.label_key = Some(key.into());
        self
    }

    pub fn supporting_text_key(mut self, key: impl Into<String>) -> Self {
        self.supporting_text_key = Some(key.into());
        self
    }

    pub fn error_text_key(mut self, key: impl Into<String>) -> Self {
        self.error_text_key = Some(key.into());
        self
    }

    fn is_enabled(&self) -> bool {
        self.label_key.is_some()
            || self.supporting_text_key.is_some()
            || self.error_text_key.is_some()
    }
}

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
struct SelectLocalizationState {
    last_revision: u64,
    last_language: String,
}

fn resolve_language_tag(
    mut entity: Entity,
    child_of: &Query<&ChildOf>,
    overrides: &Query<&MaterialLanguageOverride>,
    global: &MaterialLanguage,
) -> String {
    if let Ok(ov) = overrides.get(entity) {
        return ov.tag.clone();
    }

    while let Ok(parent) = child_of.get(entity) {
        entity = parent.parent();
        if let Ok(ov) = overrides.get(entity) {
            return ov.tag.clone();
        }
    }

    global.tag.clone()
}

fn select_localization_system(
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
    child_of: Query<&ChildOf>,
    overrides: Query<&MaterialLanguageOverride>,
    mut selects: Query<(
        Entity,
        &SelectLocalization,
        &mut MaterialSelect,
        Option<&mut SelectLocalizationState>,
    )>,
    mut commands: Commands,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let global_revision = i18n.revision();

    for (entity, loc, mut select, state) in selects.iter_mut() {
        if !loc.is_enabled() {
            continue;
        }

        let resolved_language = resolve_language_tag(entity, &child_of, &overrides, &language);

        let needs_update = match &state {
            Some(s) => s.last_revision != global_revision || s.last_language != resolved_language,
            None => true,
        };

        if !needs_update {
            continue;
        }

        if let Some(key) = loc.label_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if select.label.as_deref() != Some(next.as_str()) {
                    select.label = Some(next);
                }
            }
        }

        if let Some(key) = loc.supporting_text_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if select.supporting_text.as_deref() != Some(next.as_str()) {
                    select.supporting_text = Some(next);
                }
            }
        }

        if let Some(key) = loc.error_text_key.as_deref() {
            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if select.error_text.as_deref() != Some(next.as_str()) {
                    select.error_text = Some(next);
                }
            }
        }

        // Localize option labels.
        for option in select.options.iter_mut() {
            let Some(key) = option.label_key.as_deref() else {
                continue;
            };

            if let Some(v) = i18n.translate(&resolved_language, key) {
                let next = v.to_string();
                if option.label != next {
                    option.label = next;
                }
            }
        }

        if let Some(mut state) = state {
            state.last_revision = global_revision;
            state.last_language = resolved_language;
        } else {
            commands.entity(entity).insert(SelectLocalizationState {
                last_revision: global_revision,
                last_language: resolved_language,
            });
        }
    }
}

fn select_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    selects: Query<(&TestId, &Children), With<MaterialSelect>>,
    children_query: Query<&Children>,
    mut queries: ParamSet<(
        Query<(), With<SelectDisplayText>>,
        Query<(), With<SelectDropdownArrow>>,
        Query<(), With<SelectDropdown>>,
        Query<&SelectOptionItem>,
        Query<(), With<SelectOptionLabelText>>,
        Query<(), With<SelectOptionIcon>>,
    )>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    for (test_id, children) in selects.iter() {
        let base = test_id.id();

        let mut found_display = false;
        let mut found_arrow = false;
        let mut found_dropdown = false;

        let mut options: Vec<(Entity, usize)> = Vec::new();

        let mut stack: Vec<Entity> = children.iter().collect();
        while let Some(entity) = stack.pop() {
            if !found_display && queries.p0().get(entity).is_ok() {
                found_display = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/display_text"),
                });
            }

            if !found_arrow && queries.p1().get(entity).is_ok() {
                found_arrow = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/arrow"),
                });
            }

            if !found_dropdown && queries.p2().get(entity).is_ok() {
                found_dropdown = true;
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: format!("{base}/dropdown"),
                });
            }

            if let Ok(option) = queries.p3().get(entity) {
                let option_base = format!("{base}/option/{}", option.index);
                commands.queue(InsertTestIdIfExists {
                    entity,
                    id: option_base.clone(),
                });
                options.push((entity, option.index));
            }

            if let Ok(children) = children_query.get(entity) {
                stack.extend(children.iter());
            }
        }

        // Tag label/icon nodes under each option row with stable derived IDs.
        for (row_entity, index) in options {
            let Ok(children) = children_query.get(row_entity) else {
                continue;
            };

            let mut found_label = false;
            let mut found_icon = false;
            let mut stack: Vec<Entity> = children.iter().collect();
            while let Some(entity) = stack.pop() {
                if !found_icon && queries.p5().get(entity).is_ok() {
                    found_icon = true;
                    commands.queue(InsertTestIdIfExists {
                        entity,
                        id: format!("{base}/option/{index}/icon"),
                    });
                }

                if !found_label && queries.p4().get(entity).is_ok() {
                    found_label = true;
                    commands.queue(InsertTestIdIfExists {
                        entity,
                        id: format!("{base}/option/{index}/label"),
                    });
                }

                if found_label && found_icon {
                    break;
                }

                if let Ok(children) = children_query.get(entity) {
                    stack.extend(children.iter());
                }
            }
        }
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
    /// Optional i18n key for the label.
    pub label_key: Option<String>,
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
            label_key: None,
            value: None,
            icon: None,
            disabled: false,
        }
    }

    /// Set the label from an i18n key.
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.label = String::new();
        self.label_key = Some(key.into());
        self
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
        Query<(&ChildOf, &mut MaterialIcon), With<SelectDropdownArrow>>,
        Query<&mut TextColor, With<SelectOptionLabelText>>,
        Query<&mut MaterialIcon, With<SelectOptionIcon>>,
    )>,
    mut dropdowns: Query<
        (&ChildOf, &mut BackgroundColor),
        (
            With<SelectDropdown>,
            Without<SelectOptionItem>,
            Without<MaterialSelect>,
        ),
    >,
    mut option_rows: Query<
        (
            &SelectOwner,
            &SelectOptionItem,
            &mut BackgroundColor,
            &Children,
        ),
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
            color.color = select.label_color(&theme);
        }
    }

    for (parent, mut bg) in dropdowns.iter_mut() {
        if selects.get(parent.parent()).is_ok() {
            bg.0 = theme.surface_container;
        }
    }

    for (owner, option_item, mut row_bg, children) in option_rows.iter_mut() {
        let Ok(select) = selects.get(owner.0) else {
            continue;
        };

        let is_selected = select
            .selected_index
            .is_some_and(|i| i == option_item.index);
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
                c.color = text_color;
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
        Query<(&ChildOf, &mut MaterialIcon), With<SelectDropdownArrow>>,
        Query<&mut TextColor, With<SelectOptionLabelText>>,
        Query<&mut MaterialIcon, With<SelectOptionIcon>>,
    )>,
    mut dropdowns: Query<
        (&ChildOf, &mut BackgroundColor),
        (
            With<SelectDropdown>,
            Without<SelectOptionItem>,
            Without<MaterialSelect>,
        ),
    >,
    mut option_rows: Query<
        (
            &SelectOwner,
            &SelectOptionItem,
            &mut BackgroundColor,
            &Children,
        ),
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
            color.color = select.label_color(&theme);
        }
    }

    for (parent, mut bg) in dropdowns.iter_mut() {
        if selects.get(parent.parent()).is_ok() {
            bg.0 = theme.surface_container;
        }
    }

    for (owner, option_item, mut row_bg, children) in option_rows.iter_mut() {
        let Ok(select) = selects.get(owner.0) else {
            continue;
        };

        let is_selected = select
            .selected_index
            .is_some_and(|i| i == option_item.index);
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
                c.color = text_color;
            }
        }
    }
}

/// Builder for select components
pub struct SelectBuilder {
    select: MaterialSelect,
    width: Val,
    localization: SelectLocalization,
}

impl SelectBuilder {
    /// Create a new select builder
    pub fn new(options: Vec<SelectOption>) -> Self {
        Self {
            select: MaterialSelect::new(options),
            width: Val::Px(210.0),
            localization: SelectLocalization::default(),
        }
    }

    /// Localize the select's placeholder/label via a translation key.
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.localization = self.localization.label_key(key);
        self
    }

    /// Localize the select's supporting text via a translation key.
    pub fn supporting_text_key(mut self, key: impl Into<String>) -> Self {
        self.localization = self.localization.supporting_text_key(key);
        self
    }

    /// Localize the select's error text via a translation key.
    pub fn error_text_key(mut self, key: impl Into<String>) -> Self {
        self.localization = self.localization.error_text_key(key);
        self
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
            self.localization,
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

/// Internal marker for option icons rendered as embedded bitmaps.
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

/// Rebuild dropdown option rows when `MaterialSelect.options` changes.
///
/// The select component spawns option rows at build time. Some UIs (like the
/// showcase Translations view) populate options dynamically after spawn.
fn select_dropdown_rebuild_options_system(
    theme: Option<Res<MaterialTheme>>,
    selects: Query<(Entity, &MaterialSelect, &Children), Changed<MaterialSelect>>,
    dropdowns: Query<(), With<SelectDropdown>>,
    dropdown_children: Query<&Children, With<SelectDropdown>>,
    option_rows: Query<(), With<SelectOptionItem>>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {
    let Some(theme) = theme else { return };

    for (select_entity, select, children) in selects.iter() {
        // Find dropdown child.
        let Some(dropdown_entity) = children.iter().find(|e| dropdowns.get(*e).is_ok()) else {
            continue;
        };

        // Collect current option row entities.
        // Note: If the dropdown was spawned with zero children, it won't have a `Children`
        // component, so we treat a missing Children as "zero existing rows".
        let existing_rows: Vec<Entity> = dropdown_children
            .get(dropdown_entity)
            .map(|kids| {
                kids.iter()
                    .filter(|e| option_rows.get(*e).is_ok())
                    .collect()
            })
            .unwrap_or_default();

        if existing_rows.len() == select.options.len() {
            continue;
        }

        // Remove old rows.
        for row in existing_rows {
            let mut stack = vec![row];
            let mut to_despawn = Vec::new();
            while let Some(e) = stack.pop() {
                to_despawn.push(e);
                if let Ok(kids) = children_query.get(e) {
                    stack.extend(kids.iter());
                }
            }

            for e in to_despawn.into_iter().rev() {
                commands.entity(e).despawn();
            }
        }

        // Spawn new rows.
        let option_text_color = theme.on_surface;
        let selected_index = select.selected_index;
        let options = select.options.clone();

        commands.entity(dropdown_entity).with_children(|dropdown| {
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
                        if let Some(icon) = &option.icon {
                            if let Some(id) = icon_by_name(icon.as_str()) {
                                row.spawn((
                                    SelectOptionIcon,
                                    MaterialIcon::new(id).with_size(20.0).with_color(
                                        if is_disabled {
                                            option_text_color.with_alpha(0.38)
                                        } else {
                                            option_text_color
                                        },
                                    ),
                                ));
                            }
                        }

                        row.spawn((
                            SelectOptionLabelText,
                            Text::new(option.label.clone()),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(if is_disabled {
                                option_text_color.with_alpha(0.38)
                            } else {
                                option_text_color
                            }),
                        ));
                    });
            }
        });
    }
}

/// Keep dropdown visibility + displayed text in sync with `MaterialSelect`.
fn select_dropdown_sync_system(
    mut selects: Query<(Entity, &MaterialSelect, &Children), Changed<MaterialSelect>>,
    mut dropdowns: Query<&mut Visibility, With<SelectDropdown>>,
    mut display_texts: Query<&mut Text, (With<SelectDisplayText>, Without<SelectOptionLabelText>)>,
    mut option_rows: Query<(&SelectOwner, &mut SelectOptionItem, &Children)>,
    mut option_labels: Query<&mut Text, (With<SelectOptionLabelText>, Without<SelectDisplayText>)>,
) {
    for (select_entity, select, children) in selects.iter_mut() {
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
        let placeholder = select.label.as_deref().unwrap_or("");

        let display = select
            .selected_option()
            .map(|o| o.label.as_str())
            .unwrap_or(placeholder);

        for child in children.iter() {
            if let Ok(mut text) = display_texts.get_mut(child) {
                *text = Text::new(display);
            }
        }

        // Update option row labels in the dropdown.
        for (owner, mut option_item, row_children) in option_rows.iter_mut() {
            if owner.0 != select_entity {
                continue;
            }

            let Some(opt) = select.options.get(option_item.index) else {
                continue;
            };

            if option_item.label != opt.label {
                option_item.label = opt.label.clone();
            }

            for child in row_children.iter() {
                if let Ok(mut text) = option_labels.get_mut(child) {
                    *text = Text::new(opt.label.clone());
                }
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

// (no icon font system; icons are embedded bitmaps)

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
        self.spawn_select_with(theme, SelectBuilder::new(options).label(label).filled());
    }

    fn spawn_outlined_select(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        options: Vec<SelectOption>,
    ) {
        self.spawn_select_with(theme, SelectBuilder::new(options).label(label).outlined());
    }

    fn spawn_select_with(&mut self, theme: &MaterialTheme, builder: SelectBuilder) {
        let label_color = builder.select.label_color(theme);
        let text_color = builder.select.text_color(theme);
        let option_text_color = theme.on_surface;

        // Clone options for building the dropdown list
        let options = builder.select.options.clone();
        let selected_index = builder.select.selected_index;
        let placeholder = if builder.select.label.is_some() {
            builder.select.label.clone().unwrap_or_default()
        } else if builder.localization.label_key.is_some() {
            // Let `select_localization_system` resolve the placeholder without flashing a hard-coded string.
            String::new()
        } else {
            "Select".to_string()
        };

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
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(text_color),
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // Dropdown arrow
            select.spawn((
                SelectDropdownArrow,
                MaterialIcon::from_name(ICON_EXPAND_MORE)
                    .expect("embedded icon 'expand_more' not found")
                    .with_size(20.0)
                    .with_color(label_color),
            ));

            // Dropdown list (hidden by default)
            select
                .spawn((
                    SelectDropdown,
                    Visibility::Hidden,
                    // Ensure the dropdown renders above later siblings (e.g. code blocks).
                    // NOTE: Dialog scrims in this project use `GlobalZIndex(1000)`.
                    // If the dropdown is promoted to a root node by `GlobalZIndex`, it must
                    // be above modal overlays, otherwise it will render "behind" dialogs.
                    GlobalZIndex(1100),
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
                                    if let Some(id) = icon_by_name(icon.as_str()) {
                                        row.spawn((
                                            SelectOptionIcon,
                                            MaterialIcon::new(id).with_size(20.0).with_color(
                                                if is_disabled {
                                                    option_text_color.with_alpha(0.38)
                                                } else {
                                                    option_text_color
                                                },
                                            ),
                                        ));
                                    }
                                }

                                row.spawn((
                                    SelectOptionLabelText,
                                    Text::new(option.label.clone()),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
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
