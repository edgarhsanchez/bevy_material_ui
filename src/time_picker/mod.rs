//! Material Design 3 Time Picker Component
//!
//! A standalone time picker with clock face and keyboard input modes.

use bevy::prelude::*;
use bevy::ui::{ComputedNode, FocusPolicy, UiGlobalTransform};
use std::f32::consts::PI;

use crate::i18n::{MaterialI18n, MaterialLanguage, MaterialLanguageOverride};
use crate::icons::material_icon_names;
use crate::text_field::{
    spawn_text_field_control_with, InputType, MaterialTextField, TextFieldBuilder,
};
use crate::theme::MaterialTheme;
use crate::tokens::{CornerRadius, Spacing};
use bevy::ui::UiTransform;
use std::f32::consts::TAU;

mod clock;
mod format;

pub use clock::*;
pub use format::*;

// Touch targets: outer ring can be full 48px without overlap;
// inner ring (24h 00-11) must be smaller to avoid overlapping neighbors.
const CLOCK_NUMBER_OUTER_SIZE: f32 = 48.0;
const CLOCK_NUMBER_OUTER_HALF: f32 = CLOCK_NUMBER_OUTER_SIZE / 2.0;
const CLOCK_NUMBER_INNER_SIZE: f32 = 32.0;
const CLOCK_NUMBER_INNER_HALF: f32 = CLOCK_NUMBER_INNER_SIZE / 2.0;

/// Plugin for the Time Picker component.
pub struct TimePickerPlugin;

impl Plugin for TimePickerPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<TimePickerSubmitEvent>()
            .add_message::<TimePickerCancelEvent>()
            .add_systems(
                Update,
                (
                    time_picker_localization_system,
                    time_picker_visibility_system,
                    time_picker_keyboard_dismiss_system,
                    time_picker_mode_toggle_system,
                    time_picker_keyboard_input_system,
                    time_picker_view_visibility_system,
                    time_picker_period_toggle_system,
                    time_picker_selection_mode_system,
                    time_picker_clock_interaction_system,
                    time_picker_clock_number_button_system,
                    time_picker_action_system,
                    time_picker_render_system,
                    time_picker_theme_system,
                ),
            );
    }
}

// ============================================================================
// Localization
// ============================================================================

/// Optional i18n bindings for a `MaterialTimePicker`.
#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub struct TimePickerLocalization {
    pub title_key: Option<String>,
}

impl TimePickerLocalization {
    pub fn title_key(mut self, key: impl Into<String>) -> Self {
        self.title_key = Some(key.into());
        self
    }
}

#[derive(Component, Debug, Clone)]
struct TimePickerLocalizationState {
    last_revision: u64,
    last_language: String,
}

fn resolve_language_tag(
    mut entity: Entity,
    child_of: &Query<&ChildOf>,
    overrides: &Query<&MaterialLanguageOverride>,
    language: &MaterialLanguage,
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

    language.tag.clone()
}

fn time_picker_localization_system(
    mut commands: Commands,
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
    child_of: Query<&ChildOf>,
    overrides: Query<&MaterialLanguageOverride>,
    mut pickers: Query<
        (
            Entity,
            &TimePickerLocalization,
            &mut MaterialTimePicker,
            Option<&mut TimePickerLocalizationState>,
        ),
        With<MaterialTimePicker>,
    >,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let global_revision = i18n.revision();

    for (entity, loc, mut picker, state) in pickers.iter_mut() {
        if loc.title_key.is_none() {
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

        if let Some(key) = &loc.title_key {
            if let Some(value) = i18n.translate(&resolved_language, key) {
                picker.title = value.to_string();
            }
        }

        if let Some(mut state) = state {
            state.last_revision = global_revision;
            state.last_language = resolved_language;
        } else {
            commands.entity(entity).insert(TimePickerLocalizationState {
                last_revision: global_revision,
                last_language: resolved_language,
            });
        }
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Time picker input mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInputMode {
    /// Visual clock face
    Clock,
    /// Keyboard input (text fields)
    Keyboard,
}

/// Time format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    /// 24-hour format (0-23)
    H24,
    /// 12-hour format with AM/PM
    H12,
}

/// Time period for 12-hour format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    AM,
    PM,
}

/// Active selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSelectionMode {
    Hour,
    Minute,
}

/// Material Design 3 Time Picker Component
#[derive(Component, Debug, Clone)]
pub struct MaterialTimePicker {
    /// Whether the picker is open
    pub open: bool,
    /// Picker title
    pub title: String,
    /// Input mode (clock or keyboard)
    pub input_mode: TimeInputMode,
    /// Time format (12h or 24h)
    pub format: TimeFormat,
    /// Current hour (0-23)
    pub hour: u8,
    /// Current minute (0-59)
    pub minute: u8,
    /// Period for 12H format
    pub period: TimePeriod,
    /// Active selection mode
    pub selection_mode: TimeSelectionMode,
    /// Dismiss on scrim click
    pub dismiss_on_scrim_click: bool,
    /// Dismiss on escape key
    pub dismiss_on_escape: bool,
}

/// Builder for Material Time Picker
#[derive(Debug, Clone)]
pub struct TimePickerBuilder {
    title: String,
    localization: TimePickerLocalization,
    input_mode: TimeInputMode,
    format: TimeFormat,
    initial_hour: u8,
    initial_minute: u8,
    dismiss_on_scrim_click: bool,
    dismiss_on_escape: bool,
    width: Val,
}

impl Default for TimePickerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TimePickerBuilder {
    pub fn new() -> Self {
        Self {
            title: "Select time".to_string(),
            localization: TimePickerLocalization::default(),
            input_mode: TimeInputMode::Clock,
            format: TimeFormat::H24,
            initial_hour: 0,
            initial_minute: 0,
            dismiss_on_scrim_click: true,
            dismiss_on_escape: true,
            width: Val::Px(360.0),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set title from an i18n key.
    pub fn title_key(mut self, key: impl Into<String>) -> Self {
        self.title.clear();
        self.localization = self.localization.title_key(key);
        self
    }

    pub fn input_mode(mut self, mode: TimeInputMode) -> Self {
        self.input_mode = mode;
        self
    }

    pub fn format(mut self, format: TimeFormat) -> Self {
        self.format = format;
        self
    }

    pub fn initial_time(mut self, hour: u8, minute: u8) -> Self {
        self.initial_hour = hour % 24;
        self.initial_minute = minute % 60;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn dismiss_on_scrim_click(mut self, enabled: bool) -> Self {
        self.dismiss_on_scrim_click = enabled;
        self
    }

    pub fn dismiss_on_escape(mut self, enabled: bool) -> Self {
        self.dismiss_on_escape = enabled;
        self
    }

    fn build_picker(&self) -> MaterialTimePicker {
        let period = if self.initial_hour < 12 {
            TimePeriod::AM
        } else {
            TimePeriod::PM
        };

        MaterialTimePicker {
            open: false,
            title: self.title.clone(),
            input_mode: self.input_mode,
            format: self.format,
            hour: self.initial_hour,
            minute: self.initial_minute,
            period,
            selection_mode: TimeSelectionMode::Hour,
            dismiss_on_scrim_click: self.dismiss_on_scrim_click,
            dismiss_on_escape: self.dismiss_on_escape,
        }
    }
}

impl MaterialTimePicker {
    /// Get hour in 12H format (1-12)
    pub fn hour_12h(&self) -> u8 {
        let h = self.hour % 12;
        if h == 0 {
            12
        } else {
            h
        }
    }

    /// Set time from 12H format
    pub fn set_time_12h(&mut self, hour_12: u8, minute: u8, period: TimePeriod) {
        let hour_24 = match period {
            TimePeriod::AM => {
                if hour_12 == 12 {
                    0
                } else {
                    hour_12
                }
            }
            TimePeriod::PM => {
                if hour_12 == 12 {
                    12
                } else {
                    hour_12 + 12
                }
            }
        };

        self.hour = hour_24 % 24;
        self.minute = minute % 60;
        self.period = period;
    }

    /// Format time as string
    pub fn format_time(&self) -> String {
        match self.format {
            TimeFormat::H24 => format!("{:02}:{:02}", self.hour, self.minute),
            TimeFormat::H12 => {
                let period_str = match self.period {
                    TimePeriod::AM => "AM",
                    TimePeriod::PM => "PM",
                };
                format!("{:02}:{:02} {}", self.hour_12h(), self.minute, period_str)
            }
        }
    }
}

// ============================================================================
// Events
// ============================================================================

#[derive(Event, Message)]
pub struct TimePickerSubmitEvent {
    pub entity: Entity,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Event, Message)]
pub struct TimePickerCancelEvent {
    pub entity: Entity,
}

// ============================================================================
// Internal Components
// ============================================================================

#[derive(Component)]
struct TimePickerScrim {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerDialog;

#[derive(Component)]
struct TimePickerModeToggle {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerModeToggleLabel {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerPeriodToggle {
    picker: Entity,
    period: TimePeriod,
}

#[derive(Component)]
struct TimePickerSelectionChip {
    picker: Entity,
    mode: TimeSelectionMode,
}

#[derive(Component)]
struct TimePickerHourText {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerMinuteText {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerClockView {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerKeyboardView {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerHourField {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerMinuteField {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerClockFace {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerClockHand {
    picker: Entity,
}

#[derive(Component)]
struct TimePickerClockHandLine {
    picker: Entity,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TimePickerClockNumberKind {
    Hour,
    Minute,
}

#[derive(Component)]
struct TimePickerClockNumber {
    picker: Entity,
    kind: TimePickerClockNumberKind,
    value: u8,
    format: Option<TimeFormat>,
}

#[derive(Component)]
struct TimePickerAction {
    picker: Entity,
    is_confirm: bool,
}

// ============================================================================
// Systems
// ============================================================================

fn time_picker_visibility_system(
    pickers: Query<(Entity, &MaterialTimePicker), Changed<MaterialTimePicker>>,
    mut root_nodes: Query<&mut Node, Without<TimePickerDialog>>,
) {
    for (entity, picker) in pickers.iter() {
        if let Ok(mut node) = root_nodes.get_mut(entity) {
            node.display = if picker.open {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

fn time_picker_keyboard_dismiss_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut pickers: Query<(Entity, &mut MaterialTimePicker)>,
    mut cancel_events: MessageWriter<TimePickerCancelEvent>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    for (entity, mut picker) in pickers.iter_mut() {
        if picker.open && picker.dismiss_on_escape {
            picker.open = false;
            cancel_events.write(TimePickerCancelEvent { entity });
        }
    }
}

fn time_picker_mode_toggle_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    toggles: Query<(&Interaction, &TimePickerModeToggle), Changed<Interaction>>,
) {
    for (interaction, toggle) in toggles.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(toggle.picker) {
            if picker.open {
                picker.input_mode = match picker.input_mode {
                    TimeInputMode::Clock => TimeInputMode::Keyboard,
                    TimeInputMode::Keyboard => TimeInputMode::Clock,
                };
            }
        }
    }
}

fn time_picker_keyboard_input_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    mut changes: MessageReader<crate::text_field::TextFieldChangeEvent>,
    hour_fields: Query<&TimePickerHourField>,
    minute_fields: Query<&TimePickerMinuteField>,
) {
    for ev in changes.read() {
        if let Ok(field) = hour_fields.get(ev.entity) {
            let Ok(mut picker) = pickers.get_mut(field.picker) else {
                continue;
            };
            if !picker.open || picker.input_mode != TimeInputMode::Keyboard {
                continue;
            }

            let raw = ev.value.trim();
            if raw.is_empty() {
                continue;
            }

            let Ok(mut hour) = raw.parse::<u8>() else {
                continue;
            };

            picker.selection_mode = TimeSelectionMode::Hour;
            match picker.format {
                TimeFormat::H24 => {
                    hour = hour.min(23);
                    picker.hour = hour;
                }
                TimeFormat::H12 => {
                    hour = hour.clamp(1, 12);
                    let minute = picker.minute;
                    let period = picker.period;
                    picker.set_time_12h(hour, minute, period);
                }
            }
        }

        if let Ok(field) = minute_fields.get(ev.entity) {
            let Ok(mut picker) = pickers.get_mut(field.picker) else {
                continue;
            };
            if !picker.open || picker.input_mode != TimeInputMode::Keyboard {
                continue;
            }

            let raw = ev.value.trim();
            if raw.is_empty() {
                continue;
            }

            let Ok(minute) = raw.parse::<u8>() else {
                continue;
            };

            picker.selection_mode = TimeSelectionMode::Minute;
            picker.minute = minute.min(59);
        }
    }
}

fn time_picker_view_visibility_system(
    pickers: Query<(Entity, &MaterialTimePicker), Changed<MaterialTimePicker>>,
    mut views: ParamSet<(
        Query<(&TimePickerClockView, &mut Node)>,
        Query<(&TimePickerKeyboardView, &mut Node)>,
    )>,
) {
    for (picker_entity, picker) in pickers.iter() {
        if !picker.open {
            continue;
        }

        {
            let mut clock_views = views.p0();
            for (view, mut node) in clock_views.iter_mut() {
                if view.picker != picker_entity {
                    continue;
                }
                node.display = if picker.input_mode == TimeInputMode::Clock {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }

        {
            let mut keyboard_views = views.p1();
            for (view, mut node) in keyboard_views.iter_mut() {
                if view.picker != picker_entity {
                    continue;
                }
                node.display = if picker.input_mode == TimeInputMode::Keyboard {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

fn time_picker_period_toggle_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    toggles: Query<(&Interaction, &TimePickerPeriodToggle), Changed<Interaction>>,
) {
    for (interaction, toggle) in toggles.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(toggle.picker) {
            if picker.open && picker.format == TimeFormat::H12 {
                picker.period = toggle.period;

                // Update hour to match new period
                let hour_12 = picker.hour_12h();
                let minute = picker.minute;
                let period = picker.period;
                picker.set_time_12h(hour_12, minute, period);
            }
        }
    }
}

fn time_picker_selection_mode_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    chips: Query<(&Interaction, &TimePickerSelectionChip), Changed<Interaction>>,
) {
    for (interaction, chip) in chips.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(chip.picker) {
            if picker.open {
                picker.selection_mode = chip.mode;
            }
        }
    }
}

fn time_picker_clock_interaction_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    clock_faces: Query<(&ComputedNode, &UiGlobalTransform, &TimePickerClockFace)>,
    clock_numbers: Query<(&TimePickerClockNumber, &Interaction, &Node)>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let is_dragging = mouse.pressed(MouseButton::Left);
    let is_release = mouse.just_released(MouseButton::Left);
    if !is_dragging && !is_release {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // `Window::cursor_position()` is in logical pixels; UI layout/transforms are in physical.
    let cursor_physical = cursor_pos * window.scale_factor();

    for (computed, transform, clock) in clock_faces.iter() {
        let Ok(mut picker) = pickers.get_mut(clock.picker) else {
            continue;
        };

        if !picker.open || picker.input_mode != TimeInputMode::Clock {
            continue;
        }

        // If the user is interacting with a visible clock number button, do not also run
        // the clock-face angle selection logic. Otherwise, slight misses around the glyph can
        // fall through and select the next tick (e.g. clicking 05 selecting 10).
        let over_number_button = clock_numbers.iter().any(|(n, interaction, number_node)| {
            n.picker == clock.picker
                && number_node.display != Display::None
                && matches!(*interaction, Interaction::Hovered | Interaction::Pressed)
        });
        if over_number_button {
            continue;
        }

        // UI transform translation is the node center in physical pixels.
        let clock_center = transform.translation;
        // Use computed size; assume square.
        let clock_radius = (computed.size().x.min(computed.size().y)) / 2.0;

        let relative = cursor_physical - clock_center;
        let distance = relative.length();

        // Ignore interactions outside of the clock face.
        // NOTE: the number buttons extend beyond the face radius (40x40 centered near the edge),
        // so we allow a small margin to make clicking the numbers reliably register.
        let hit_radius = clock_radius + 24.0;
        if distance > hit_radius {
            continue;
        }

        // Calculate angle (0 = top, clockwise)
        //
        // NOTE: In Bevy UI / window coordinates, +Y points down. This matches the typical
        // `atan2(dY, dX) + 90°` convention for a clock dial: 0 at 12 o'clock, increasing
        // clockwise. Do NOT negate Y here, otherwise the dial direction is inverted.
        let angle = (relative.y).atan2(relative.x) + PI / 2.0;
        let angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };

        match picker.selection_mode {
            TimeSelectionMode::Hour => {
                // Determine value based on angle
                let value = ((angle / (2.0 * PI) * 12.0).round() as u8) % 12;

                match picker.format {
                    TimeFormat::H12 => {
                        // Simple 12-hour clock
                        let hour_12 = if value == 0 { 12 } else { value };
                        let minute = picker.minute;
                        let period = picker.period;
                        picker.set_time_12h(hour_12, minute, period);
                    }
                    TimeFormat::H24 => {
                        // Dual-level clock: inner ring 0-11, outer ring 12-23
                        let is_inner = distance < clock_radius * 0.72;
                        let hour = if is_inner { value } else { value + 12 };
                        picker.hour = hour % 24;
                    }
                }

                if is_release {
                    // Match Material behavior: selecting an hour advances to minutes.
                    picker.selection_mode = TimeSelectionMode::Minute;
                }
            }
            TimeSelectionMode::Minute => {
                // Minutes: 0-59, snap to 5-minute increments by default
                let value = ((angle / (2.0 * PI) * 60.0).round() as u8) % 60;
                picker.minute = value;
            }
        }
    }
}

fn time_picker_clock_number_button_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    numbers: Query<(&Interaction, &TimePickerClockNumber), Changed<Interaction>>,
) {
    for (interaction, number) in numbers.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(mut picker) = pickers.get_mut(number.picker) else {
            continue;
        };

        if !picker.open || picker.input_mode != TimeInputMode::Clock {
            continue;
        }

        match number.kind {
            TimePickerClockNumberKind::Hour => {
                picker.selection_mode = TimeSelectionMode::Hour;
                match picker.format {
                    TimeFormat::H12 => {
                        let hour_12 = number.value.clamp(1, 12);
                        let minute = picker.minute;
                        let period = picker.period;
                        picker.set_time_12h(hour_12, minute, period);
                    }
                    TimeFormat::H24 => {
                        picker.hour = number.value.min(23);
                    }
                }

                // Match Material behavior: selecting an hour advances to minutes.
                picker.selection_mode = TimeSelectionMode::Minute;
            }
            TimePickerClockNumberKind::Minute => {
                picker.selection_mode = TimeSelectionMode::Minute;
                picker.minute = number.value.min(59);
            }
        }
    }
}

fn time_picker_action_system(
    mut pickers: Query<&mut MaterialTimePicker>,
    actions: Query<(&Interaction, &TimePickerAction), Changed<Interaction>>,
    scrim: Query<(&Interaction, &TimePickerScrim), Changed<Interaction>>,
    mut submit_events: MessageWriter<TimePickerSubmitEvent>,
    mut cancel_events: MessageWriter<TimePickerCancelEvent>,
) {
    // Handle scrim clicks
    for (interaction, scrim) in scrim.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(scrim.picker) {
            if picker.open && picker.dismiss_on_scrim_click {
                picker.open = false;
                cancel_events.write(TimePickerCancelEvent {
                    entity: scrim.picker,
                });
            }
        }
    }

    // Handle action buttons
    for (interaction, action) in actions.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(action.picker) {
            if !picker.open {
                continue;
            }

            if action.is_confirm {
                picker.open = false;
                submit_events.write(TimePickerSubmitEvent {
                    entity: action.picker,
                    hour: picker.hour,
                    minute: picker.minute,
                });
            } else {
                picker.open = false;
                cancel_events.write(TimePickerCancelEvent {
                    entity: action.picker,
                });
            }
        }
    }
}

fn time_picker_render_system(
    pickers: Query<(Entity, &MaterialTimePicker), Changed<MaterialTimePicker>>,
    mut text_nodes: Query<(
        &mut Text,
        Option<&TimePickerHourText>,
        Option<&TimePickerMinuteText>,
    )>,
    mut mode_toggle_icons: Query<(&mut crate::icons::svg::SvgIcon, &TimePickerModeToggleLabel)>,
    mut styled_nodes: ParamSet<(
        Query<(
            &mut BackgroundColor,
            &Children,
            &mut Node,
            Option<&TimePickerSelectionChip>,
            Option<&TimePickerPeriodToggle>,
            Option<&TimePickerClockNumber>,
        )>,
        Query<(&TimePickerClockHandLine, &mut Node)>,
    )>,
    mut clock_hand: Query<(&TimePickerClockHand, &mut UiTransform)>,
    mut text_colors: Query<&mut TextColor>,
    mut keyboard_fields: Query<(
        &mut MaterialTextField,
        Option<&TimePickerHourField>,
        Option<&TimePickerMinuteField>,
    )>,
    theme: Res<MaterialTheme>,
) {
    for (picker_entity, picker) in pickers.iter() {
        if !picker.open {
            continue;
        }

        // Update the hour/minute display text and the mode-toggle label.
        let (display_hour, display_period) = match picker.format {
            TimeFormat::H24 => (picker.hour, None),
            TimeFormat::H12 => (picker.hour_12h(), Some(picker.period)),
        };
        let hour_text = format!("{:02}", display_hour);
        let minute_text = format!("{:02}", picker.minute);

        for (mut text, hour_marker, minute_marker) in text_nodes.iter_mut() {
            if let Some(m) = hour_marker {
                if m.picker == picker_entity {
                    text.0 = hour_text.clone();
                }
            }
            if let Some(m) = minute_marker {
                if m.picker == picker_entity {
                    text.0 = minute_text.clone();
                }
            }
        }

        // Update mode-toggle icon (keyboard <-> clock).
        // When in clock mode, show the KEYBOARD icon; when in keyboard mode, show the CLOCK icon.
        let mode_toggle_icon_name = match picker.input_mode {
            TimeInputMode::Clock => material_icon_names::IC_KEYBOARD_BLACK_24DP,
            TimeInputMode::Keyboard => material_icon_names::IC_CLOCK_BLACK_24DP,
        };
        for (mut icon, marker) in mode_toggle_icons.iter_mut() {
            if marker.picker != picker_entity {
                continue;
            }
            icon.name = mode_toggle_icon_name.to_string();
            icon.color = theme.on_surface;
        }

        // Style chips, toggles, and clock numbers.
        {
            let mut styled_nodes = styled_nodes.p0();
            for (mut bg, children, mut node, chip, toggle, number) in styled_nodes.iter_mut() {
                if let Some(chip) = chip {
                    if chip.picker != picker_entity {
                        continue;
                    }

                    let is_active = chip.mode == picker.selection_mode;
                    let (bg_color, text_color) = if is_active {
                        (theme.primary_container, theme.on_primary_container)
                    } else {
                        (Color::NONE, theme.on_surface)
                    };

                    *bg = BackgroundColor(bg_color);
                    for child in children.iter() {
                        if let Ok(mut tc) = text_colors.get_mut(child) {
                            *tc = TextColor(text_color);
                        }
                    }

                    continue;
                }

                if let Some(toggle) = toggle {
                    if toggle.picker != picker_entity {
                        continue;
                    }

                    let is_active =
                        picker.format == TimeFormat::H12 && picker.period == toggle.period;
                    let (bg_color, text_color) = if is_active {
                        (theme.primary, theme.on_primary)
                    } else {
                        (Color::NONE, theme.on_surface)
                    };

                    *bg = BackgroundColor(bg_color);
                    for child in children.iter() {
                        if let Ok(mut tc) = text_colors.get_mut(child) {
                            *tc = TextColor(text_color);
                        }
                    }

                    continue;
                }

                if let Some(number) = number {
                    if number.picker != picker_entity {
                        continue;
                    }

                    let should_show = match picker.selection_mode {
                        TimeSelectionMode::Hour => {
                            number.kind == TimePickerClockNumberKind::Hour
                                && number.format.map(|f| f == picker.format).unwrap_or(true)
                        }
                        TimeSelectionMode::Minute => {
                            number.kind == TimePickerClockNumberKind::Minute
                        }
                    };
                    node.display = if should_show {
                        Display::Flex
                    } else {
                        Display::None
                    };

                    if !should_show {
                        continue;
                    }

                    let is_selected = match picker.selection_mode {
                        TimeSelectionMode::Hour => {
                            if picker.format == TimeFormat::H12 {
                                number.value == picker.hour_12h()
                            } else {
                                number.value == picker.hour
                            }
                        }
                        TimeSelectionMode::Minute => {
                            let snapped = (((picker.minute as u16 + 2) / 5) * 5) % 60;
                            number.value == snapped as u8
                        }
                    };

                    let (bg_color, text_color) = if is_selected {
                        (theme.primary, theme.on_primary)
                    } else {
                        (Color::NONE, theme.on_surface)
                    };

                    *bg = BackgroundColor(bg_color);
                    for child in children.iter() {
                        if let Ok(mut tc) = text_colors.get_mut(child) {
                            *tc = TextColor(text_color);
                        }
                    }
                }
            }
        }

        // Update clock hand rotation.
        let layout = crate::time_picker::clock::ClockFaceLayout::new(120.0);
        let mut desired_hand_length: Option<f32> = None;
        for (hand, mut transform) in clock_hand.iter_mut() {
            if hand.picker != picker_entity {
                continue;
            }

            // Bevy UI rotation is driven by UiTransform, not Transform.
            // UiTransform rotation is clockwise; our hand line points "up" at rest.
            let (angle, length) = match picker.selection_mode {
                TimeSelectionMode::Hour => {
                    if picker.format == TimeFormat::H24 {
                        let is_inner = picker.hour < 12;
                        let hour_value = picker.hour % 12;
                        let (_, length) = layout.hand_transform(hour_value, 12, is_inner);
                        let angle = (hour_value as f32 / 12.0) * TAU;
                        (angle, length)
                    } else {
                        // Map 12 -> 0 so the hand points to 12 o'clock.
                        let hour_value = picker.hour_12h() % 12;
                        let (_, length) = layout.hand_transform(hour_value, 12, false);
                        let angle = (hour_value as f32 / 12.0) * TAU;
                        (angle, length)
                    }
                }
                TimeSelectionMode::Minute => {
                    let (_, length) = layout.hand_transform(picker.minute, 60, false);
                    let angle = (picker.minute as f32 / 60.0) * TAU;
                    (angle, length)
                }
            };

            transform.rotation = bevy::math::Rot2::radians(angle);
            desired_hand_length = Some(length);
        }

        // Update hand line length (kept simple but synchronized).
        if let Some(length) = desired_hand_length {
            let mut hand_lines = styled_nodes.p1();
            for (line, mut node) in hand_lines.iter_mut() {
                if line.picker != picker_entity {
                    continue;
                }

                node.height = Val::Px(length);
                node.top = Val::Px(-length);
            }
        }

        // Keep keyboard fields synced when not focused.
        for (mut text_field, hour_field, minute_field) in keyboard_fields.iter_mut() {
            if let Some(field) = hour_field {
                if field.picker != picker_entity {
                    continue;
                }
                if !text_field.focused {
                    let desired = match picker.format {
                        TimeFormat::H24 => format!("{:02}", picker.hour),
                        TimeFormat::H12 => format!("{:02}", picker.hour_12h()),
                    };
                    if text_field.value != desired {
                        text_field.value = desired;
                        text_field.has_content = !text_field.value.is_empty();
                    }
                }
                continue;
            }

            if let Some(field) = minute_field {
                if field.picker != picker_entity {
                    continue;
                }
                if !text_field.focused {
                    let desired = format!("{:02}", picker.minute);
                    if text_field.value != desired {
                        text_field.value = desired;
                        text_field.has_content = !text_field.value.is_empty();
                    }
                }
            }
        }

        // (Unused today but kept so clippy doesn't complain about it)
        let _ = display_period;
    }
}

fn time_picker_theme_system(
    theme: Res<MaterialTheme>,
    mut backgrounds: Query<(
        &mut BackgroundColor,
        Option<&TimePickerScrim>,
        Option<&TimePickerDialog>,
    )>,
) {
    if !theme.is_changed() {
        return;
    }

    for (mut bg, scrim, dialog) in backgrounds.iter_mut() {
        if scrim.is_some() {
            *bg = BackgroundColor(theme.scrim.with_alpha(0.32));
        } else if dialog.is_some() {
            *bg = BackgroundColor(theme.surface_container_high);
        }
    }
}

// ============================================================================
// Spawn Trait
// ============================================================================

pub trait SpawnTimePicker {
    fn spawn_time_picker(&mut self, theme: &MaterialTheme, builder: TimePickerBuilder) -> Entity;
}

impl SpawnTimePicker for ChildSpawnerCommands<'_> {
    fn spawn_time_picker(&mut self, theme: &MaterialTheme, builder: TimePickerBuilder) -> Entity {
        let picker = builder.build_picker();
        let width = builder.width;
        let bg_color = theme.surface_container_high;
        let on_surface = theme.on_surface;
        let primary = theme.primary;

        let initial_input_mode = picker.input_mode;
        let initial_format = picker.format;
        let initial_hour = picker.hour;
        let initial_minute = picker.minute;
        let initial_period = picker.period;

        // Spawn root container
        let mut root = self.spawn((
            picker,
            builder.localization,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            Transform::default(),
            GlobalTransform::default(),
            GlobalZIndex(9999),
        ));
        let entity = root.id();

        root.with_children(|root| {
            // Scrim overlay
            root.spawn((
                TimePickerScrim { picker: entity },
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.scrim.with_alpha(0.32)),
                ZIndex(0),
            ));

            // Dialog container
            root.spawn((
                TimePickerDialog,
                FocusPolicy::Block,
                Interaction::None,
                Node {
                    width,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(Spacing::LARGE)),
                    row_gap: Val::Px(Spacing::MEDIUM),
                    ..default()
                },
                Transform::default(),
                GlobalTransform::default(),
                BackgroundColor(bg_color),
                BorderRadius::all(Val::Px(CornerRadius::EXTRA_LARGE)),
                BoxShadow::default(),
                ZIndex(1),
            ))
            .with_children(|dialog| {
                // Title row + mode toggle
                dialog
                    .spawn(Node {
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Select Time"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(on_surface),
                        ));

                        row.spawn((
                            Button,
                            TimePickerModeToggle { picker: entity },
                            Interaction::None,
                            Node {
                                width: Val::Px(40.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                TimePickerModeToggleLabel { picker: entity },
                                crate::icons::svg::SvgIcon::new(match initial_input_mode {
                                    TimeInputMode::Clock => {
                                        material_icon_names::IC_KEYBOARD_BLACK_24DP
                                    }
                                    TimeInputMode::Keyboard => {
                                        material_icon_names::IC_CLOCK_BLACK_24DP
                                    }
                                })
                                .with_size(18.0)
                                .with_color(on_surface),
                            ));
                        });
                    });

                // Time display (hour/minute as selectable chips)
                dialog
                    .spawn(Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                        ..default()
                    })
                    .with_children(|time_display| {
                        let (display_hour, _) = match initial_format {
                            TimeFormat::H24 => (initial_hour, None),
                            TimeFormat::H12 => {
                                let h = initial_hour % 12;
                                (if h == 0 { 12 } else { h }, Some(initial_period))
                            }
                        };

                        time_display
                            .spawn((
                                Button,
                                TimePickerSelectionChip {
                                    picker: entity,
                                    mode: TimeSelectionMode::Hour,
                                },
                                Interaction::None,
                                Node {
                                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(theme.primary_container),
                                BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
                            ))
                            .with_children(|chip| {
                                chip.spawn((
                                    TimePickerHourText { picker: entity },
                                    Text::new(format!("{:02}", display_hour)),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_primary_container),
                                ));
                            });

                        time_display.spawn((
                            Text::new(":"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(primary),
                        ));

                        time_display
                            .spawn((
                                Button,
                                TimePickerSelectionChip {
                                    picker: entity,
                                    mode: TimeSelectionMode::Minute,
                                },
                                Interaction::None,
                                Node {
                                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
                            ))
                            .with_children(|chip| {
                                chip.spawn((
                                    TimePickerMinuteText { picker: entity },
                                    Text::new(format!("{:02}", initial_minute)),
                                    TextFont {
                                        font_size: 32.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                            });
                    });

                // AM/PM toggles for 12H format
                let period_display = if initial_format == TimeFormat::H12 {
                    Display::Flex
                } else {
                    Display::None
                };
                dialog
                    .spawn(Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        display: period_display,
                        ..default()
                    })
                    .with_children(|row| {
                        for (label, period) in [("AM", TimePeriod::AM), ("PM", TimePeriod::PM)] {
                            let is_active = initial_period == period;
                            let (bg, fg) = if is_active {
                                (theme.primary, theme.on_primary)
                            } else {
                                (Color::NONE, theme.on_surface)
                            };

                            row.spawn((
                                Button,
                                TimePickerPeriodToggle {
                                    picker: entity,
                                    period,
                                },
                                Interaction::None,
                                Node {
                                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(bg),
                                BorderRadius::all(Val::Px(CornerRadius::MEDIUM)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new(label),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(fg),
                                ));
                            });
                        }
                    });

                // Views: Clock + Keyboard (always spawned, display toggled)
                let clock_display = if initial_input_mode == TimeInputMode::Clock {
                    Display::Flex
                } else {
                    Display::None
                };
                let keyboard_display = if initial_input_mode == TimeInputMode::Keyboard {
                    Display::Flex
                } else {
                    Display::None
                };

                // Clock view (contains clock face)
                dialog
                    .spawn((
                        TimePickerClockView { picker: entity },
                        Node {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: clock_display,
                            ..default()
                        },
                        Transform::default(),
                        GlobalTransform::default(),
                    ))
                    .with_children(|clock_view| {
                        clock_view
                            .spawn((
                                TimePickerClockFace { picker: entity },
                                FocusPolicy::Block,
                                Interaction::None,
                                Node {
                                    width: Val::Px(240.0),
                                    height: Val::Px(240.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::all(Val::Px(Spacing::MEDIUM)),
                                    ..default()
                                },
                                Transform::default(),
                                GlobalTransform::default(),
                                BackgroundColor(theme.surface_container),
                                BorderRadius::all(Val::Px(CornerRadius::FULL)),
                            ))
                            .with_children(|clock| {
                                let layout = crate::time_picker::clock::ClockFaceLayout::new(120.0);

                                // Hour numbers (1-12)
                                for hour in 1..=12 {
                                    let pos = layout.number_position(hour, 12, false);
                                    let is_selected = initial_format == TimeFormat::H12
                                        && hour == ((initial_hour % 12).max(1));
                                    let (bg, fg) = if is_selected {
                                        (theme.primary, theme.on_primary)
                                    } else {
                                        (Color::NONE, on_surface)
                                    };

                                    let display = if initial_format == TimeFormat::H12 {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    };

                                    clock
                                        .spawn((
                                            Button,
                                            FocusPolicy::Block,
                                            TimePickerClockNumber {
                                                picker: entity,
                                                kind: TimePickerClockNumberKind::Hour,
                                                value: hour,
                                                format: Some(TimeFormat::H12),
                                            },
                                            Interaction::None,
                                            Node {
                                                position_type: PositionType::Absolute,
                                                display,
                                                left: Val::Px(
                                                    120.0 + pos.x - CLOCK_NUMBER_OUTER_HALF,
                                                ),
                                                top: Val::Px(
                                                    120.0 + pos.y - CLOCK_NUMBER_OUTER_HALF,
                                                ),
                                                width: Val::Px(CLOCK_NUMBER_OUTER_SIZE),
                                                height: Val::Px(CLOCK_NUMBER_OUTER_SIZE),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(bg),
                                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                        ))
                                        .with_children(|n| {
                                            n.spawn((
                                                Text::new(hour.to_string()),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(fg),
                                            ));
                                        });
                                }

                                // Hour numbers (24H): inner 0-11 and outer 12-23
                                for idx in 0..12u8 {
                                    let pos_outer = layout.number_position(idx, 12, false);
                                    let pos_inner = layout.number_position(idx, 12, true);

                                    let inner_hour = idx;
                                    let outer_hour = idx + 12;

                                    let display = if initial_format == TimeFormat::H24 {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    };

                                    // Inner ring
                                    clock
                                        .spawn((
                                            Button,
                                            FocusPolicy::Block,
                                            TimePickerClockNumber {
                                                picker: entity,
                                                kind: TimePickerClockNumberKind::Hour,
                                                value: inner_hour,
                                                format: Some(TimeFormat::H24),
                                            },
                                            Interaction::None,
                                            Node {
                                                position_type: PositionType::Absolute,
                                                display,
                                                left: Val::Px(
                                                    120.0 + pos_inner.x - CLOCK_NUMBER_INNER_HALF,
                                                ),
                                                top: Val::Px(
                                                    120.0 + pos_inner.y - CLOCK_NUMBER_INNER_HALF,
                                                ),
                                                width: Val::Px(CLOCK_NUMBER_INNER_SIZE),
                                                height: Val::Px(CLOCK_NUMBER_INNER_SIZE),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::NONE),
                                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                        ))
                                        .with_children(|n| {
                                            n.spawn((
                                                Text::new(format!("{:02}", inner_hour)),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(on_surface),
                                            ));
                                        });

                                    // Outer ring
                                    clock
                                        .spawn((
                                            Button,
                                            FocusPolicy::Block,
                                            TimePickerClockNumber {
                                                picker: entity,
                                                kind: TimePickerClockNumberKind::Hour,
                                                value: outer_hour,
                                                format: Some(TimeFormat::H24),
                                            },
                                            Interaction::None,
                                            Node {
                                                position_type: PositionType::Absolute,
                                                display,
                                                left: Val::Px(
                                                    120.0 + pos_outer.x - CLOCK_NUMBER_OUTER_HALF,
                                                ),
                                                top: Val::Px(
                                                    120.0 + pos_outer.y - CLOCK_NUMBER_OUTER_HALF,
                                                ),
                                                width: Val::Px(CLOCK_NUMBER_OUTER_SIZE),
                                                height: Val::Px(CLOCK_NUMBER_OUTER_SIZE),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::NONE),
                                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                        ))
                                        .with_children(|n| {
                                            n.spawn((
                                                Text::new(format!("{:02}", outer_hour)),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(on_surface),
                                            ));
                                        });
                                }

                                // Minute numbers: 0,5,...55 (shown only in minute mode)
                                for idx in 0..12u8 {
                                    let minute = (idx * 5) % 60;
                                    let pos = layout.number_position(idx, 12, false);

                                    clock
                                        .spawn((
                                            Button,
                                            FocusPolicy::Block,
                                            TimePickerClockNumber {
                                                picker: entity,
                                                kind: TimePickerClockNumberKind::Minute,
                                                value: minute,
                                                format: None,
                                            },
                                            Interaction::None,
                                            Node {
                                                position_type: PositionType::Absolute,
                                                display: Display::None,
                                                left: Val::Px(
                                                    120.0 + pos.x - CLOCK_NUMBER_OUTER_HALF,
                                                ),
                                                top: Val::Px(
                                                    120.0 + pos.y - CLOCK_NUMBER_OUTER_HALF,
                                                ),
                                                width: Val::Px(CLOCK_NUMBER_OUTER_SIZE),
                                                height: Val::Px(CLOCK_NUMBER_OUTER_SIZE),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::NONE),
                                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                        ))
                                        .with_children(|n| {
                                            n.spawn((
                                                Text::new(format!("{:02}", minute)),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(on_surface),
                                            ));
                                        });
                                }

                                // Clock hand pivot at center (rotated in render system)
                                clock
                                    .spawn((
                                        TimePickerClockHand { picker: entity },
                                        Node {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(120.0),
                                            top: Val::Px(120.0),
                                            width: Val::Px(0.0),
                                            height: Val::Px(0.0),
                                            ..default()
                                        },
                                        UiTransform::default(),
                                    ))
                                    .with_children(|pivot| {
                                        pivot.spawn((
                                            TimePickerClockHandLine { picker: entity },
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Px(-2.0),
                                                top: Val::Px(-80.0),
                                                width: Val::Px(4.0),
                                                height: Val::Px(80.0),
                                                ..default()
                                            },
                                            BackgroundColor(primary),
                                        ));

                                        pivot.spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Px(-6.0),
                                                top: Val::Px(-6.0),
                                                width: Val::Px(12.0),
                                                height: Val::Px(12.0),
                                                ..default()
                                            },
                                            BackgroundColor(primary),
                                            BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                        ));
                                    });
                            });
                    });

                // Keyboard view: two numeric text fields (hour/minute)
                dialog
                    .spawn((
                        TimePickerKeyboardView { picker: entity },
                        Node {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(16.0),
                            padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                            display: keyboard_display,
                            ..default()
                        },
                        Transform::default(),
                        GlobalTransform::default(),
                    ))
                    .with_children(|keyboard| {
                        let hour_value = match initial_format {
                            TimeFormat::H24 => format!("{:02}", initial_hour),
                            TimeFormat::H12 => {
                                let h = initial_hour % 12;
                                format!("{:02}", if h == 0 { 12 } else { h })
                            }
                        };

                        let minute_value = format!("{:02}", initial_minute);

                        spawn_text_field_control_with(
                            keyboard,
                            theme,
                            TextFieldBuilder::new()
                                .label("Hour")
                                .value(hour_value)
                                .outlined()
                                .input_type(InputType::Number)
                                .width(Val::Px(96.0)),
                            TimePickerHourField { picker: entity },
                        );

                        spawn_text_field_control_with(
                            keyboard,
                            theme,
                            TextFieldBuilder::new()
                                .label("Minute")
                                .value(minute_value)
                                .outlined()
                                .input_type(InputType::Number)
                                .width(Val::Px(96.0)),
                            TimePickerMinuteField { picker: entity },
                        );
                    });

                // Action buttons
                dialog
                    .spawn(Node {
                        justify_content: JustifyContent::End,
                        column_gap: Val::Px(Spacing::SMALL),
                        margin: UiRect::top(Val::Px(Spacing::MEDIUM)),
                        ..default()
                    })
                    .with_children(|actions| {
                        // Cancel button
                        actions.spawn((
                            TimePickerAction {
                                picker: entity,
                                is_confirm: false,
                            },
                            Interaction::None,
                            Text::new("Cancel"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(primary),
                            Node {
                                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                ..default()
                            },
                        ));

                        // OK button
                        actions.spawn((
                            TimePickerAction {
                                picker: entity,
                                is_confirm: true,
                            },
                            Interaction::None,
                            Text::new("OK"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(primary),
                            Node {
                                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                                ..default()
                            },
                        ));
                    });
            });
        });

        entity
    }
}
