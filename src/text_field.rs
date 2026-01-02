//! Material Design 3 Text Field component
//!
//! Text fields let users enter and edit text.
//! Reference: <https://m3.material.io/components/text-fields/overview>

use bevy::prelude::*;

use crate::{
    i18n::{MaterialI18n, MaterialLanguage, MaterialLanguageOverride},
    icons::{icon_by_name, IconStyle, MaterialIcon, ICON_CLOSE},
    locale::{DateFieldOrder, DateInputPattern},
    ripple::RippleHost,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub struct TextFieldLocalization {
    pub label_key: Option<String>,
    pub placeholder_key: Option<String>,
    pub supporting_text_key: Option<String>,
    pub error_text_key: Option<String>,
}

impl TextFieldLocalization {
    fn is_empty(&self) -> bool {
        self.label_key.is_none()
            && self.placeholder_key.is_none()
            && self.supporting_text_key.is_none()
            && self.error_text_key.is_none()
    }

    fn needs_supporting_entity(&self) -> bool {
        self.supporting_text_key.is_some() || self.error_text_key.is_some()
    }
}

fn resolve_icon_id(icon: &str) -> Option<crate::icons::material_icons::IconId> {
    let icon = icon.trim();
    if icon.is_empty() {
        return None;
    }

    icon_by_name(icon)
}

/// Plugin for the text field component
pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        app.add_message::<TextFieldChangeEvent>()
            .add_message::<TextFieldSubmitEvent>()
            .init_resource::<ActiveTextField>()
            .init_resource::<TextFieldClipboard>()
            .init_resource::<TextFieldCaretBlink>()
            // These systems have ordering dependencies (input/focus must run before
            // placeholder/display rendering updates in the same frame).
            .add_systems(
                Update,
                (
                    text_field_focus_system,
                    text_field_end_icon_click_system,
                    text_field_input_system,
                    text_field_formatter_system,
                    text_field_localization_system,
                    text_field_caret_blink_system,
                    text_field_label_system,
                    text_field_placeholder_system,
                    text_field_display_system,
                    text_field_supporting_text_system,
                    text_field_icon_system,
                    text_field_style_system,
                )
                    .chain(),
            );
    }
}

fn resolve_language_tag_for_entity(
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

fn text_field_localization_system(
    i18n: Option<Res<MaterialI18n>>,
    language: Option<Res<MaterialLanguage>>,
    child_of: Query<&ChildOf>,
    overrides: Query<&MaterialLanguageOverride>,
    mut fields: ParamSet<(
        Query<(Entity, &mut MaterialTextField, &TextFieldLocalization)>,
        Query<
            (Entity, &mut MaterialTextField, &TextFieldLocalization),
            Or<(Added<TextFieldLocalization>, Changed<TextFieldLocalization>)>,
        >,
    )>,
) {
    let (Some(i18n), Some(language)) = (i18n, language) else {
        return;
    };

    let global_changed = i18n.is_changed() || language.is_changed();

    let apply = |entity: Entity, mut field: Mut<MaterialTextField>, loc: &TextFieldLocalization| {
        if loc.is_empty() {
            return;
        }

        let lang = resolve_language_tag_for_entity(entity, &child_of, &overrides, &language);

        if let Some(key) = &loc.label_key {
            let resolved = i18n.translate(&lang, key).unwrap_or(key.as_str());
            if field.label.as_deref() != Some(resolved) {
                field.label = Some(resolved.to_string());
            }
        }

        if let Some(key) = &loc.placeholder_key {
            let resolved = i18n.translate(&lang, key).unwrap_or(key.as_str());
            if field.placeholder.as_str() != resolved {
                field.placeholder = resolved.to_string();
            }
        }

        if let Some(key) = &loc.supporting_text_key {
            let resolved = i18n.translate(&lang, key).unwrap_or(key.as_str());
            if field.supporting_text.as_deref() != Some(resolved) {
                field.supporting_text = Some(resolved.to_string());
            }
        }

        if let Some(key) = &loc.error_text_key {
            let resolved = i18n.translate(&lang, key).unwrap_or(key.as_str());
            if field.error_text.as_deref() != Some(resolved) {
                field.error_text = Some(resolved.to_string());
            }
        }
    };

    if global_changed {
        for (entity, field, loc) in fields.p0().iter_mut() {
            apply(entity, field, loc);
        }
    } else {
        for (entity, field, loc) in fields.p1().iter_mut() {
            apply(entity, field, loc);
        }
    }
}

/// Tracks which text field should currently receive keyboard input.
#[derive(Resource, Default, Debug, Clone, Copy)]
struct ActiveTextField(pub Option<Entity>);

/// Clipboard helper for text fields.
///
/// When the `clipboard` feature is enabled, this uses `arboard` to integrate with
/// the host OS clipboard. Without the feature, methods are no-ops.
#[derive(Resource, Default)]
struct TextFieldClipboard {
    #[cfg(feature = "clipboard")]
    clipboard: Option<arboard::Clipboard>,
}

impl TextFieldClipboard {
    fn get_text(&mut self) -> Option<String> {
        #[cfg(feature = "clipboard")]
        {
            if self.clipboard.is_none() {
                self.clipboard = arboard::Clipboard::new().ok();
            }
            self.clipboard.as_mut().and_then(|c| c.get_text().ok())
        }

        #[cfg(not(feature = "clipboard"))]
        {
            let _ = self;
            None
        }
    }

    fn set_text(&mut self, text: String) {
        #[cfg(feature = "clipboard")]
        {
            if self.clipboard.is_none() {
                self.clipboard = arboard::Clipboard::new().ok();
            }
            if let Some(c) = self.clipboard.as_mut() {
                let _ = c.set_text(text);
            }
        }

        #[cfg(not(feature = "clipboard"))]
        {
            let _ = (self, text);
        }
    }
}

/// Shared caret blink state for all text fields.
#[derive(Resource)]
struct TextFieldCaretBlink {
    timer: Timer,
    visible: bool,
}

impl Default for TextFieldCaretBlink {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            visible: true,
        }
    }
}

/// Text field variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextFieldVariant {
    /// Filled text field - Has background fill
    #[default]
    Filled,
    /// Outlined text field - Has border outline
    Outlined,
}

/// End icon mode - determines the trailing icon behavior
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EndIconMode {
    /// No end icon
    #[default]
    None,
    /// Password visibility toggle (eye icon)
    PasswordToggle,
    /// Clear text button (X icon) - visible when field has content
    ClearText,
    /// Dropdown menu indicator (arrow down)
    DropdownMenu,
    /// Custom icon with custom behavior
    Custom,
}

/// Input type for the text field
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum InputType {
    /// Regular text input
    #[default]
    Text,
    /// Password input (obscured by default)
    Password,
    /// Email address input
    Email,
    /// Numeric input
    Number,
    /// Phone number input
    Phone,
    /// URL input
    Url,
    /// Multi-line text input
    Multiline,
}

/// Material text field component
///
/// Matches properties from the reference `TextInputLayout`:
/// - Box background mode (filled/outlined)
/// - Box stroke width and colors
/// - Box corner radii
/// - Hint/label with animation
/// - Prefix/suffix text
/// - Helper text and error text
/// - Counter with max length
/// - Start/end icons with modes
/// - Placeholder text
#[derive(Component)]
pub struct MaterialTextField {
    /// Text field variant
    pub variant: TextFieldVariant,
    /// Current text value
    pub value: String,
    /// Placeholder/hint text (shown when empty and unfocused)
    pub placeholder: String,
    /// Label text (floats above when focused/has content)
    pub label: Option<String>,
    /// Supporting text below the field (helper text)
    pub supporting_text: Option<String>,
    /// Prefix text (displayed before input, e.g., "$")
    pub prefix_text: Option<String>,
    /// Suffix text (displayed after input, e.g., "kg")
    pub suffix_text: Option<String>,
    /// Leading icon
    pub leading_icon: Option<String>,
    /// Trailing icon
    pub trailing_icon: Option<String>,
    /// End icon mode (determines trailing icon behavior)
    pub end_icon_mode: EndIconMode,
    /// Whether the field is disabled
    pub disabled: bool,
    /// Whether the field has an error
    pub error: bool,
    /// Error message
    pub error_text: Option<String>,
    /// Maximum character count (None = unlimited)
    pub max_length: Option<usize>,
    /// Whether to show character counter
    pub counter_enabled: bool,
    /// Whether the field is focused
    pub focused: bool,
    /// If true, this field will automatically take focus when the user starts typing
    /// and no other text field is currently focused.
    pub auto_focus: bool,
    /// Whether the field has content
    pub has_content: bool,
    /// Whether hint animation is enabled
    pub hint_animation_enabled: bool,
    /// Password visibility (for password toggle mode)
    pub password_visible: bool,
    /// Box stroke width (default, in px)
    pub box_stroke_width: f32,
    /// Box stroke width when focused (in px)
    pub box_stroke_width_focused: f32,
    /// Custom box corner radius (if None, uses theme default)
    pub box_corner_radius: Option<f32>,
    /// Input type (affects keyboard and visibility)
    pub input_type: InputType,
}

impl MaterialTextField {
    /// Create a new text field
    pub fn new() -> Self {
        Self {
            variant: TextFieldVariant::default(),
            value: String::new(),
            placeholder: String::new(),
            label: None,
            supporting_text: None,
            prefix_text: None,
            suffix_text: None,
            leading_icon: None,
            trailing_icon: None,
            end_icon_mode: EndIconMode::default(),
            disabled: false,
            error: false,
            error_text: None,
            max_length: None,
            counter_enabled: false,
            focused: false,
            auto_focus: false,
            has_content: false,
            hint_animation_enabled: true,
            password_visible: false,
            box_stroke_width: 1.0,
            box_stroke_width_focused: 2.0,
            box_corner_radius: None,
            input_type: InputType::default(),
        }
    }

    /// Set the variant
    pub fn with_variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set initial value
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.has_content = !self.value.is_empty();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    /// Set end icon mode (PASSWORD_TOGGLE, CLEAR_TEXT, DROPDOWN_MENU, etc.)
    pub fn end_icon_mode(mut self, mode: EndIconMode) -> Self {
        self.end_icon_mode = mode;
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

    /// Set max length
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Enable character counter
    pub fn counter_enabled(mut self, enabled: bool) -> Self {
        self.counter_enabled = enabled;
        self
    }

    /// Set prefix text (displayed before input, e.g., "$")
    pub fn prefix_text(mut self, text: impl Into<String>) -> Self {
        self.prefix_text = Some(text.into());
        self
    }

    /// Set suffix text (displayed after input, e.g., "kg")
    pub fn suffix_text(mut self, text: impl Into<String>) -> Self {
        self.suffix_text = Some(text.into());
        self
    }

    /// Set input type
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        // Auto-enable password toggle for password fields
        if matches!(input_type, InputType::Password)
            && matches!(self.end_icon_mode, EndIconMode::None)
        {
            self.end_icon_mode = EndIconMode::PasswordToggle;
        }
        self
    }

    /// Set box stroke width
    pub fn box_stroke_width(mut self, width: f32) -> Self {
        self.box_stroke_width = width;
        self
    }

    /// Set box stroke width when focused
    pub fn box_stroke_width_focused(mut self, width: f32) -> Self {
        self.box_stroke_width_focused = width;
        self
    }

    /// Set custom box corner radius
    pub fn box_corner_radius(mut self, radius: f32) -> Self {
        self.box_corner_radius = Some(radius);
        self
    }

    /// Set hint animation enabled
    pub fn hint_animation_enabled(mut self, enabled: bool) -> Self {
        self.hint_animation_enabled = enabled;
        self
    }

    /// Get current character count for counter display
    pub fn character_count(&self) -> usize {
        self.value.chars().count()
    }

    /// Get counter text (e.g., "5 / 100")
    pub fn counter_text(&self) -> String {
        if let Some(max) = self.max_length {
            format!("{} / {}", self.character_count(), max)
        } else {
            format!("{}", self.character_count())
        }
    }

    /// Check if character limit is exceeded
    pub fn is_counter_overflow(&self) -> bool {
        if let Some(max) = self.max_length {
            self.character_count() > max
        } else {
            false
        }
    }

    /// Get effective stroke width based on focus state
    pub fn effective_stroke_width(&self) -> f32 {
        if self.focused {
            self.box_stroke_width_focused
        } else {
            self.box_stroke_width
        }
    }

    /// Toggle password visibility (for password toggle mode)
    pub fn toggle_password_visibility(&mut self) {
        self.password_visible = !self.password_visible;
    }

    /// Check if input should be obscured (password field with visibility off)
    pub fn should_obscure_input(&self) -> bool {
        matches!(self.input_type, InputType::Password) && !self.password_visible
    }

    /// Get the effective trailing icon based on end icon mode
    pub fn effective_trailing_icon(&self) -> Option<&str> {
        match self.end_icon_mode {
            EndIconMode::None => self.trailing_icon.as_deref(),
            EndIconMode::PasswordToggle => Some(if self.password_visible {
                "visibility"
            } else {
                "visibility_off"
            }),
            EndIconMode::ClearText => {
                if self.has_content {
                    Some(ICON_CLOSE)
                } else {
                    None
                }
            }
            EndIconMode::DropdownMenu => Some("arrow_drop_down"),
            EndIconMode::Custom => self.trailing_icon.as_deref(),
        }
    }

    /// Get the container color (for filled variant)
    pub fn container_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.04);
        }

        match self.variant {
            TextFieldVariant::Filled => theme.surface_container_highest,
            TextFieldVariant::Outlined => Color::NONE,
        }
    }

    /// Get the active indicator / outline color
    pub fn indicator_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            return theme.error;
        }

        if self.focused {
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

        if self.focused {
            theme.primary
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the input text color
    pub fn input_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface
        }
    }

    /// Get the placeholder text color
    pub fn placeholder_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the supporting text color
    pub fn supporting_text_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            return theme.on_surface.with_alpha(0.38);
        }

        if self.error {
            theme.error
        } else {
            theme.on_surface_variant
        }
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        if self.disabled {
            theme.on_surface.with_alpha(0.38)
        } else if self.error {
            theme.error
        } else {
            theme.on_surface_variant
        }
    }

    /// Check if label should be floating (raised above input)
    pub fn is_label_floating(&self) -> bool {
        self.focused || self.has_content
    }
}

impl Default for MaterialTextField {
    fn default() -> Self {
        Self::new()
    }
}

/// Event when text field value changes
#[derive(Event, bevy::prelude::Message)]
pub struct TextFieldChangeEvent {
    pub entity: Entity,
    pub value: String,
}

/// Event when text field is submitted (Enter pressed)
#[derive(Event, bevy::prelude::Message)]
pub struct TextFieldSubmitEvent {
    pub entity: Entity,
    pub value: String,
}

// ============================================================================
// Formatters (Android-style "TextWatcher" idea)
// ============================================================================

/// Optional text formatting + validation behavior attached to a `MaterialTextField`.
///
/// This is modeled after Android's `TextWatcher` approach: as the user types,
/// a formatter can normalize the value (e.g. auto-insert delimiters) and
/// optionally surface format errors.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextFieldFormatter {
    /// No formatting/validation.
    #[default]
    None,
    /// Date format: `MM/DD/YYYY`.
    DateMmDdYyyy,
    /// Date format driven by a `DateInputPattern`.
    DatePattern(DateInputPattern),
}

/// Tracks whether the current error state was set by the formatter.
///
/// This prevents the formatter system from clearing errors that were set by
/// higher-level widgets (e.g. date picker out-of-range or invalid-range errors).
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextFieldFormatState {
    pub(crate) format_error: bool,
}

fn normalize_date_by_pattern(input: &str, pattern: DateInputPattern) -> String {
    pattern.normalize_digits(input)
}

fn is_valid_complete_date_by_pattern(input: &str, pattern: DateInputPattern) -> bool {
    // Keep this intentionally "basic" (format-level validation).
    // Higher-level widgets (like date pickers) can do constraint validation.
    use crate::date_picker::Date;

    let Some((year, month, day)) = pattern.try_parse_complete(input) else {
        return false;
    };

    Date::new(year, month, day).is_valid()
}

/// Apply formatter normalization/format validation for fields that opt-in.
///
/// Runs immediately after `text_field_input_system` in the same chained stage.
fn text_field_formatter_system(
    mut fields: Query<(
        &TextFieldFormatter,
        &mut MaterialTextField,
        &mut TextFieldFormatState,
    )>,
    mut change_events: MessageReader<TextFieldChangeEvent>,
) {
    for ev in change_events.read() {
        let Ok((formatter, mut field, mut state)) = fields.get_mut(ev.entity) else {
            continue;
        };

        match *formatter {
            TextFieldFormatter::None => {}
            TextFieldFormatter::DateMmDdYyyy => {
                let pattern = DateInputPattern::new(DateFieldOrder::Mdy, '/');

                // Normalize (auto-insert delimiters).
                let normalized = normalize_date_by_pattern(&field.value, pattern);
                if field.value != normalized {
                    field.value = normalized;
                }
                field.has_content = !field.value.is_empty();

                // Match Android: don't validate until complete.
                if field.value.is_empty() || field.value.len() < pattern.formatted_len() {
                    if state.format_error {
                        field.error = false;
                        field.error_text = None;
                        state.format_error = false;
                    }
                    continue;
                }

                if !is_valid_complete_date_by_pattern(&field.value, pattern) {
                    field.error = true;
                    field.error_text = Some(format!("Invalid format. Use {}", pattern.hint()));
                    state.format_error = true;
                } else if state.format_error {
                    // Only clear if we set the error.
                    field.error = false;
                    field.error_text = None;
                    state.format_error = false;
                }
            }
            TextFieldFormatter::DatePattern(pattern) => {
                // Normalize (auto-insert delimiters).
                let normalized = normalize_date_by_pattern(&field.value, pattern);
                if field.value != normalized {
                    field.value = normalized;
                }
                field.has_content = !field.value.is_empty();

                // Match Android: don't validate until complete.
                if field.value.is_empty() || field.value.len() < pattern.formatted_len() {
                    if state.format_error {
                        field.error = false;
                        field.error_text = None;
                        state.format_error = false;
                    }
                    continue;
                }

                if !is_valid_complete_date_by_pattern(&field.value, pattern) {
                    field.error = true;
                    field.error_text = Some(format!("Invalid format. Use {}", pattern.hint()));
                    state.format_error = true;
                } else if state.format_error {
                    field.error = false;
                    field.error_text = None;
                    state.format_error = false;
                }
            }
        }
    }
}

/// Text field dimensions
pub const TEXT_FIELD_HEIGHT: f32 = 56.0;
pub const TEXT_FIELD_MIN_WIDTH: f32 = 210.0;

/// System to handle text field focus
fn text_field_focus_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut keyboard_inputs: MessageReader<bevy::input::keyboard::KeyboardInput>,
    mut active: ResMut<ActiveTextField>,
    mut fields: ParamSet<(
        Query<(Entity, &Interaction), (Changed<Interaction>, With<MaterialTextField>)>,
        Query<(Entity, &mut MaterialTextField), With<MaterialTextField>>,
    )>,
) {
    let mut activated_this_frame = false;

    // Determine which field was pressed this frame.
    for (entity, interaction) in fields.p0().iter_mut() {
        if *interaction == Interaction::Pressed
            // Some frames (notably during window resize) can miss the transient
            // `Pressed` state. When the mouse button is released, interaction will
            // typically be `Hovered`, so treat that as activation as well.
            || (*interaction == Interaction::Hovered && mouse.just_released(MouseButton::Left))
        {
            active.0 = Some(entity);
            activated_this_frame = true;
        }
    }

    // Clicking outside any text field should blur focus.
    if mouse.just_pressed(MouseButton::Left) && !activated_this_frame {
        active.0 = None;
    }

    // If nothing is focused, allow an auto-focus field to take focus when the user
    // starts typing (no need to click first).
    if active.0.is_none() {
        let mut saw_text_input = false;

        for ev in keyboard_inputs.read() {
            if ev.state != bevy::input::ButtonState::Pressed {
                continue;
            }

            let text: Option<&str> = ev.text.as_deref().or_else(|| match &ev.logical_key {
                bevy::input::keyboard::Key::Character(s) => Some(s.as_str()),
                _ => None,
            });

            let Some(text) = text else {
                continue;
            };

            if text.chars().any(|ch| !ch.is_control()) {
                saw_text_input = true;
                break;
            }
        }

        if saw_text_input {
            for (entity, field) in fields.p1().iter_mut() {
                if field.disabled {
                    continue;
                }
                if !field.auto_focus {
                    continue;
                }
                active.0 = Some(entity);
                break;
            }
        }
    } else {
        // Avoid replaying old keyboard events later for this system.
        keyboard_inputs.clear();
    }

    // Keep `focused` consistent: exactly one focused at a time.
    let active_entity = active.0;
    for (entity, mut field) in fields.p1().iter_mut() {
        field.focused = active_entity.is_some_and(|active| active == entity);
    }
}

/// Handle keyboard input for the currently focused text field.
fn text_field_input_system(
    active: Res<ActiveTextField>,
    mut keyboard_inputs: MessageReader<bevy::input::keyboard::KeyboardInput>,
    keys: Res<ButtonInput<KeyCode>>,
    mut clipboard: ResMut<TextFieldClipboard>,
    mut fields: Query<(Entity, &mut MaterialTextField)>,
    mut change_events: MessageWriter<TextFieldChangeEvent>,
    mut submit_events: MessageWriter<TextFieldSubmitEvent>,
) {
    let Some(active_entity) = active.0 else {
        keyboard_inputs.clear();
        return;
    };

    let Ok((entity, mut field)) = fields.get_mut(active_entity) else {
        keyboard_inputs.clear();
        return;
    };

    if field.disabled {
        keyboard_inputs.clear();
        return;
    }

    let mut changed = false;

    // Clipboard shortcuts (desktop): Ctrl/Cmd + C/X/V.
    // Current text field input is append-only (caret at end), so paste appends.
    let modifier_down = keys.pressed(KeyCode::ControlLeft)
        || keys.pressed(KeyCode::ControlRight)
        || keys.pressed(KeyCode::SuperLeft)
        || keys.pressed(KeyCode::SuperRight);

    if modifier_down {
        // Copy
        if keys.just_pressed(KeyCode::KeyC) {
            clipboard.set_text(field.value.clone());
        }

        // Cut
        if keys.just_pressed(KeyCode::KeyX) {
            clipboard.set_text(field.value.clone());
            if !field.value.is_empty() {
                field.value.clear();
                changed = true;
            }
        }

        // Paste
        if keys.just_pressed(KeyCode::KeyV) {
            if let Some(text) = clipboard.get_text() {
                for mut ch in text.chars() {
                    // Normalize newlines for single-line inputs.
                    if ch == '\n' || ch == '\r' {
                        if field.input_type == InputType::Multiline {
                            // keep
                        } else {
                            ch = ' ';
                        }
                    }

                    if ch.is_control() {
                        continue;
                    }

                    if !is_allowed_input_char(&field, ch) {
                        continue;
                    }

                    if let Some(max) = field.max_length {
                        if field.value.chars().count() >= max {
                            break;
                        }
                    }

                    field.value.push(ch);
                    changed = true;
                }
            }
        }
    }

    // Backspace
    if keys.just_pressed(KeyCode::Backspace) && !field.value.is_empty() {
        field.value.pop();
        changed = true;
    }

    // Text entry
    // Primary: `KeyboardInput.text`
    // Fallback: if `text` is None, use `logical_key == Key::Character(_)`.
    for ev in keyboard_inputs.read() {
        if ev.state != bevy::input::ButtonState::Pressed {
            continue;
        }

        let text: Option<&str> = ev.text.as_deref().or_else(|| match &ev.logical_key {
            bevy::input::keyboard::Key::Character(s) => Some(s.as_str()),
            _ => None,
        });

        let Some(text) = text else {
            continue;
        };

        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }

            if !is_allowed_input_char(&field, ch) {
                continue;
            }

            if let Some(max) = field.max_length {
                if field.value.chars().count() >= max {
                    break;
                }
            }

            field.value.push(ch);
            changed = true;
        }
    }

    field.has_content = !field.value.is_empty();

    if changed {
        change_events.write(TextFieldChangeEvent {
            entity,
            value: field.value.clone(),
        });
    }

    // Submit / newline
    if keys.just_pressed(KeyCode::Enter) {
        if field.input_type == InputType::Multiline {
            if field
                .max_length
                .is_none_or(|max| field.value.chars().count() < max)
            {
                field.value.push('\n');
                field.has_content = !field.value.is_empty();
                change_events.write(TextFieldChangeEvent {
                    entity,
                    value: field.value.clone(),
                });
            }
        } else {
            submit_events.write(TextFieldSubmitEvent {
                entity,
                value: field.value.clone(),
            });
        }
    }
}

fn is_allowed_input_char(field: &MaterialTextField, ch: char) -> bool {
    match field.input_type {
        InputType::Number => {
            if ch.is_ascii_digit() {
                return true;
            }

            // Allow a leading sign character.
            if (ch == '-' || ch == '+') && field.value.is_empty() {
                return true;
            }

            false
        }
        InputType::Phone => {
            // Keep this permissive for typical phone number formats.
            ch.is_ascii_digit() || matches!(ch, ' ' | '+' | '-' | '(' | ')')
        }
        // For other input types we currently accept any non-control characters.
        _ => true,
    }
}

/// Blink the caret for focused text fields.
fn text_field_caret_blink_system(
    time: Res<Time>,
    mut blink: ResMut<TextFieldCaretBlink>,
    mut fields: Query<&mut MaterialTextField>,
) {
    blink.timer.tick(time.delta());
    if blink.timer.just_finished() {
        blink.visible = !blink.visible;
        // Force the display system to re-run for focused fields so the inline caret updates.
        for mut field in fields.iter_mut() {
            if !field.disabled && field.focused {
                field.set_changed();
            }
        }
    }
}

/// Update the displayed input text when the text field state changes.
fn text_field_display_system(
    theme: Option<Res<MaterialTheme>>,
    blink: Res<TextFieldCaretBlink>,
    changed_fields: Query<(Entity, &MaterialTextField), Changed<MaterialTextField>>,
    mut input_text: Query<(&TextFieldInputFor, &mut Text, &mut TextColor), With<TextFieldInput>>,
) {
    let Some(theme) = theme else { return };

    // Keep a stable line height even when the caret is "hidden".
    // If we used an empty string, Bevy's text node can collapse, causing the
    // floating label to move up/down as the caret blinks.
    const ZERO_WIDTH_SPACE: &str = "\u{200B}";

    let caret = if blink.visible { "|" } else { ZERO_WIDTH_SPACE };

    for (field_entity, field) in changed_fields.iter() {
        let has_label = field.label.is_some();
        // Expanded hint (inside field) is the label if present, otherwise the placeholder.
        let expanded_hint = if has_label {
            field.label.as_deref().unwrap_or("")
        } else {
            field.placeholder.as_str()
        };

        // Inline caret: render it as part of the input text so it appears right after the
        // last glyph instead of being pushed to the far right by flex layout.
        let (display, color) = if field.value.is_empty() {
            if field.is_label_floating() {
                // Label is floating (focused or has content). If empty, show just the caret.
                if field.focused {
                    (caret.to_string(), field.input_color(&theme))
                } else {
                    (ZERO_WIDTH_SPACE.to_string(), field.input_color(&theme))
                }
            } else {
                // Expanded hint inside the field.
                let hint_color = if has_label {
                    field.label_color(&theme)
                } else {
                    field.placeholder_color(&theme)
                };
                (expanded_hint.to_string(), hint_color)
            }
        } else {
            // Actual value (obscure for password types if needed)
            let shown_value = if field.should_obscure_input() {
                "•".repeat(field.value.chars().count())
            } else {
                field.value.clone()
            };

            if field.focused {
                (
                    format!("{}{}", shown_value, caret),
                    field.input_color(&theme),
                )
            } else {
                (shown_value, field.input_color(&theme))
            }
        };

        for (owner, mut text, mut text_color) in input_text.iter_mut() {
            if owner.0 == field_entity {
                *text = Text::new(display.clone());
                *text_color = TextColor(color);
            }
        }
    }
}

fn text_field_placeholder_system(
    theme: Option<Res<MaterialTheme>>,
    changed_fields: Query<(Entity, &MaterialTextField), Changed<MaterialTextField>>,
    mut placeholders: Query<
        (
            &TextFieldPlaceholderFor,
            &mut Text,
            &mut TextColor,
            &mut Node,
            &mut Visibility,
        ),
        With<TextFieldPlaceholder>,
    >,
) {
    let Some(theme) = theme else { return };

    for (field_entity, field) in changed_fields.iter() {
        // Reference M3 placeholder behavior:
        // - Placeholder is a separate layer.
        // - Shown only when the label is floating (hint collapsed) and the field is empty.
        // We only enable this when a label exists; otherwise placeholder acts as the expanded hint.
        let show_placeholder = field.label.is_some()
            && !field.placeholder.is_empty()
            && field.value.is_empty()
            && field.is_label_floating();

        let display = if show_placeholder {
            Display::Flex
        } else {
            Display::None
        };
        let visibility = if show_placeholder {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        for (owner, mut text, mut color, mut node, mut vis) in placeholders.iter_mut() {
            if owner.0 == field_entity {
                *text = Text::new(field.placeholder.as_str());
                *color = TextColor(field.placeholder_color(&theme));
                node.display = display;
                *vis = visibility;
            }
        }
    }
}

fn text_field_label_system(
    theme: Option<Res<MaterialTheme>>,
    changed_fields: Query<(Entity, &MaterialTextField), Changed<MaterialTextField>>,
    mut labels: Query<(&TextFieldLabelFor, &mut TextColor, &mut Node), With<TextFieldLabel>>,
) {
    let Some(theme) = theme else { return };

    for (field_entity, field) in changed_fields.iter() {
        let color = field.label_color(&theme);
        let show_label = field.is_label_floating() && field.label.is_some();
        let display = if show_label {
            Display::Flex
        } else {
            Display::None
        };

        for (owner, mut text_color, mut node) in labels.iter_mut() {
            if owner.0 == field_entity {
                *text_color = TextColor(color);
                node.display = display;
            }
        }
    }
}

fn text_field_supporting_text_system(
    theme: Option<Res<MaterialTheme>>,
    fields: Query<&MaterialTextField>,
    mut supporting: Query<
        (&TextFieldSupportingFor, &mut Text, &mut TextColor),
        With<TextFieldSupportingText>,
    >,
) {
    let Some(theme) = theme else { return };

    for (owner, mut text, mut color) in supporting.iter_mut() {
        let Ok(field) = fields.get(owner.0) else {
            continue;
        };

        let (message, message_color) = if field.error {
            (field.error_text.as_deref().unwrap_or(""), theme.error)
        } else {
            (
                if field.value.is_empty() {
                    field.supporting_text.as_deref().unwrap_or("")
                } else {
                    ""
                },
                theme.on_surface_variant,
            )
        };

        *text = Text::new(message);
        *color = TextColor(message_color);
    }
}

/// System to update text field styles
fn text_field_style_system(
    theme: Option<Res<MaterialTheme>>,
    mut text_fields: Query<
        (&MaterialTextField, &mut BackgroundColor, &mut BorderColor),
        Changed<MaterialTextField>,
    >,
) {
    let Some(theme) = theme else { return };

    for (text_field, mut bg_color, mut border_color) in text_fields.iter_mut() {
        *bg_color = BackgroundColor(text_field.container_color(&theme));
        *border_color = BorderColor::all(text_field.indicator_color(&theme));
    }
}

/// Builder for text fields
pub struct TextFieldBuilder {
    text_field: MaterialTextField,
    width: Val,
    formatter: TextFieldFormatter,
    localization: TextFieldLocalization,
}

impl TextFieldBuilder {
    /// Create a new text field builder
    pub fn new() -> Self {
        Self {
            text_field: MaterialTextField::new(),
            width: Val::Px(TEXT_FIELD_MIN_WIDTH),
            formatter: TextFieldFormatter::None,
            localization: TextFieldLocalization::default(),
        }
    }

    /// Attach a formatter to this text field.
    pub fn formatter(mut self, formatter: TextFieldFormatter) -> Self {
        self.formatter = formatter;
        self
    }

    /// Convenience: set up a date field in `MM/DD/YYYY` format.
    pub fn date_mm_dd_yyyy(self) -> Self {
        self.date_pattern(DateInputPattern::new(DateFieldOrder::Mdy, '/'))
    }

    /// Convenience: set up a date field with a specific input pattern.
    pub fn date_pattern(self, pattern: DateInputPattern) -> Self {
        self.formatter(TextFieldFormatter::DatePattern(pattern))
            .placeholder(pattern.hint())
            .input_type(InputType::Number)
            .max_length(pattern.formatted_len())
    }

    /// Set variant
    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.text_field.variant = variant;
        self
    }

    /// Make filled variant
    pub fn filled(self) -> Self {
        self.variant(TextFieldVariant::Filled)
    }

    /// Make outlined variant
    pub fn outlined(self) -> Self {
        self.variant(TextFieldVariant::Outlined)
    }

    /// Set initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.text_field.value = value.into();
        self.text_field.has_content = !self.text_field.value.is_empty();
        self
    }

    /// Set placeholder
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.text_field.placeholder = placeholder.into();
        self
    }

    /// Set placeholder from i18n key.
    pub fn placeholder_key(mut self, key: impl Into<String>) -> Self {
        self.localization.placeholder_key = Some(key.into());
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.text_field.label = Some(label.into());
        self
    }

    /// Set label from i18n key.
    pub fn label_key(mut self, key: impl Into<String>) -> Self {
        self.text_field.label = Some(String::new());
        self.localization.label_key = Some(key.into());
        self
    }

    /// Set supporting text
    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.text_field.supporting_text = Some(text.into());
        self
    }

    /// Set supporting text from i18n key.
    pub fn supporting_text_key(mut self, key: impl Into<String>) -> Self {
        self.text_field.supporting_text = Some(String::new());
        self.localization.supporting_text_key = Some(key.into());
        self
    }

    /// Set leading icon
    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.text_field.leading_icon = Some(icon.into());
        self
    }

    /// Set trailing icon
    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.text_field.trailing_icon = Some(icon.into());
        self
    }

    /// Set input type
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.text_field.input_type = input_type;
        // Auto-enable password toggle for password fields
        if matches!(input_type, InputType::Password)
            && matches!(self.text_field.end_icon_mode, EndIconMode::None)
        {
            self.text_field.end_icon_mode = EndIconMode::PasswordToggle;
        }
        self
    }

    /// Enable/disable auto-focus behavior.
    ///
    /// When enabled, this field becomes focused as soon as the user types any
    /// text character (and no other field is currently focused).
    pub fn auto_focus(mut self, enabled: bool) -> Self {
        self.text_field.auto_focus = enabled;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.text_field.disabled = disabled;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.text_field.error = error;
        self
    }

    /// Set error text
    pub fn error_text(mut self, text: impl Into<String>) -> Self {
        self.text_field.error_text = Some(text.into());
        self.text_field.error = true;
        self
    }

    /// Set error text from i18n key.
    pub fn error_text_key(mut self, key: impl Into<String>) -> Self {
        self.text_field.error_text = Some(String::new());
        self.localization.error_text_key = Some(key.into());
        self.text_field.error = true;
        self
    }

    /// Set max length
    pub fn max_length(mut self, max: usize) -> Self {
        self.text_field.max_length = Some(max);
        self
    }

    /// Set width
    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    /// Build the text field bundle
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.text_field.container_color(theme);
        let border_color = self.text_field.indicator_color(theme);
        let is_outlined = self.text_field.variant == TextFieldVariant::Outlined;

        (
            self.text_field,
            self.formatter,
            TextFieldFormatState::default(),
            self.localization,
            Button,
            Interaction::None,
            Node {
                width: self.width,
                height: Val::Px(TEXT_FIELD_HEIGHT),
                padding: UiRect::axes(Val::Px(Spacing::LARGE), Val::Px(Spacing::MEDIUM)),
                border: if is_outlined {
                    UiRect::all(Val::Px(1.0))
                } else {
                    UiRect::bottom(Val::Px(1.0))
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            BorderRadius::top(Val::Px(CornerRadius::EXTRA_SMALL)),
        )
    }
}

impl Default for TextFieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker for the label element
#[derive(Component)]
pub struct TextFieldLabel;

/// Links a label entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldLabelFor(pub Entity);

/// Marker for the input element
#[derive(Component)]
pub struct TextFieldInput;

/// Links an input text entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldInputFor(pub Entity);

/// Marker for the placeholder element.
#[derive(Component)]
pub struct TextFieldPlaceholder;

/// Links a placeholder entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldPlaceholderFor(pub Entity);

/// Marker for the leading icon button (start icon).
#[derive(Component)]
pub struct TextFieldLeadingIconButton;

/// Links a leading icon button entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldLeadingIconButtonFor(pub Entity);

/// Marker for the leading icon glyph entity.
#[derive(Component)]
pub struct TextFieldLeadingIcon;

/// Links a leading icon glyph entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldLeadingIconFor(pub Entity);

/// Marker for the end icon button (trailing icon).
#[derive(Component)]
pub struct TextFieldEndIconButton;

/// Links an end icon button entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldEndIconButtonFor(pub Entity);

/// Marker for the end icon glyph entity.
#[derive(Component)]
pub struct TextFieldEndIcon;

/// Links an end icon glyph entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldEndIconFor(pub Entity);

/// Marker for the supporting text element
#[derive(Component)]
pub struct TextFieldSupportingText;

/// Links a supporting-text entity to its owning text field entity.
#[derive(Component)]
pub struct TextFieldSupportingFor(pub Entity);

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material text fields as children
///
/// This trait provides a clean API for spawning text fields within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_filled_text_field(&theme, "Email", "user@example.com");
///     children.spawn_outlined_text_field(&theme, "Password", "");
/// });
/// ```
pub trait SpawnTextFieldChild {
    /// Spawn a filled text field
    fn spawn_filled_text_field(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        value: impl Into<String>,
    );

    /// Spawn an outlined text field
    fn spawn_outlined_text_field(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        value: impl Into<String>,
    );

    /// Spawn a text field with full builder control
    fn spawn_text_field_with(&mut self, theme: &MaterialTheme, builder: TextFieldBuilder);
}

impl SpawnTextFieldChild for ChildSpawnerCommands<'_> {
    fn spawn_filled_text_field(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.spawn_text_field_with(
            theme,
            TextFieldBuilder::new().label(label).value(value).filled(),
        );
    }

    fn spawn_outlined_text_field(
        &mut self,
        theme: &MaterialTheme,
        label: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.spawn_text_field_with(
            theme,
            TextFieldBuilder::new().label(label).value(value).outlined(),
        );
    }

    fn spawn_text_field_with(&mut self, theme: &MaterialTheme, builder: TextFieldBuilder) {
        let label_text = builder.text_field.label.clone();
        let value_text = builder.text_field.value.clone();
        let placeholder_text = builder.text_field.placeholder.clone();
        let label_color = builder.text_field.label_color(theme);
        let input_color = builder.text_field.input_color(theme);
        let placeholder_color = builder.text_field.placeholder_color(theme);
        let icon_color = builder.text_field.icon_color(theme);
        let leading_icon_text = builder.text_field.leading_icon.clone();
        let end_icon_text = builder
            .text_field
            .effective_trailing_icon()
            .map(|s| s.to_string());
        let initial_is_label_floating = builder.text_field.is_label_floating();

        let supporting_text = builder.text_field.supporting_text.clone();
        let error = builder.text_field.error;
        let error_text = builder.text_field.error_text.clone();

        let (supporting_display, supporting_color) = if error {
            (error_text.as_deref().unwrap_or(""), theme.error)
        } else {
            (
                supporting_text.as_deref().unwrap_or(""),
                theme.on_surface_variant,
            )
        };

        let should_spawn_supporting =
            !supporting_display.is_empty() || builder.localization.needs_supporting_entity();

        // Wrapper so supporting/error text can appear below the 56px field.
        self.spawn(Node {
            width: builder.width,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|wrapper| {
            let mut field_commands = wrapper.spawn(builder.build(theme));
            let field_entity = field_commands.id();

            field_commands.with_children(|container| {
                // Leading icon (start icon)
                let leading_icon_visible = leading_icon_text
                    .as_deref()
                    .and_then(resolve_icon_id)
                    .is_some();
                container
                    .spawn((
                        TextFieldLeadingIconButton,
                        TextFieldLeadingIconButtonFor(field_entity),
                        Button,
                        RippleHost::new(),
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: if leading_icon_visible {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    ))
                    .with_children(|btn| {
                        let icon_id = leading_icon_text
                            .as_deref()
                            .and_then(resolve_icon_id)
                            .or_else(|| icon_by_name(ICON_CLOSE))
                            .expect("embedded icon 'close' not found");
                        btn.spawn((
                            TextFieldLeadingIcon,
                            TextFieldLeadingIconFor(field_entity),
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });

                // Content column so the label can float above the input.
                container
                    .spawn(Node {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|content| {
                        // Floating label (hidden when expanded)
                        if let Some(ref label) = label_text {
                            content.spawn((
                                TextFieldLabel,
                                TextFieldLabelFor(field_entity),
                                Text::new(label.as_str()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(label_color),
                                Node {
                                    display: if initial_is_label_floating {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    ..default()
                                },
                            ));
                        }

                        // Input text node (renders value/caret or in-field hint)
                        let expanded_hint = if label_text.is_some() {
                            label_text.as_deref().unwrap_or("")
                        } else {
                            placeholder_text.as_str()
                        };

                        let initial_display = if value_text.is_empty() {
                            if initial_is_label_floating {
                                "\u{200B}"
                            } else {
                                expanded_hint
                            }
                        } else {
                            value_text.as_str()
                        };

                        let initial_color = if value_text.is_empty() {
                            if initial_is_label_floating {
                                input_color
                            } else if label_text.is_some() {
                                // Expanded label/hint uses label color.
                                label_color
                            } else {
                                placeholder_color
                            }
                        } else {
                            input_color
                        };

                        // Input line: placeholder (overlay) + actual input text.
                        content
                            .spawn(Node {
                                position_type: PositionType::Relative,
                                ..default()
                            })
                            .with_children(|input_line| {
                                // Separate placeholder (shown only when label is floating and value is empty)
                                // Spawn it even if it's initially hidden so systems can toggle it.
                                input_line.spawn((
                                    TextFieldPlaceholder,
                                    TextFieldPlaceholderFor(field_entity),
                                    Text::new(placeholder_text.as_str()),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(placeholder_color),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        display: Display::None,
                                        ..default()
                                    },
                                    Visibility::Hidden,
                                ));

                                input_line.spawn((
                                    TextFieldInput,
                                    TextFieldInputFor(field_entity),
                                    Text::new(initial_display),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(initial_color),
                                ));
                            });
                    });

                // End icon (trailing icon)
                let end_icon_visible = end_icon_text.as_deref().and_then(resolve_icon_id).is_some();
                container
                    .spawn((
                        TextFieldEndIconButton,
                        TextFieldEndIconButtonFor(field_entity),
                        Button,
                        RippleHost::new(),
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: if end_icon_visible {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    ))
                    .with_children(|btn| {
                        let icon_id = end_icon_text
                            .as_deref()
                            .and_then(resolve_icon_id)
                            .or_else(|| icon_by_name(ICON_CLOSE))
                            .expect("embedded icon 'close' not found");
                        btn.spawn((
                            TextFieldEndIcon,
                            TextFieldEndIconFor(field_entity),
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });
            });

            if should_spawn_supporting {
                wrapper.spawn((
                    TextFieldSupportingText,
                    TextFieldSupportingFor(field_entity),
                    Text::new(supporting_display),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(supporting_color),
                    Node {
                        margin: UiRect::left(Val::Px(Spacing::LARGE)),
                        ..default()
                    },
                ));
            }
        });
    }
}

// ============================================================================
// Standalone spawn helpers
// ============================================================================

/// Spawn a Material text field (including its internal children) and return the field entity.
///
/// This is useful when you need to later query the specific field entity (e.g. for routing events).
pub fn spawn_text_field_control(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    builder: TextFieldBuilder,
) -> Entity {
    // Duplicate the SpawnTextFieldChild implementation, but return the field entity.
    let label_text: Option<String> = builder.text_field.label.clone();
    let value_text = builder.text_field.value.clone();
    let placeholder_text = builder.text_field.placeholder.clone();
    let label_color = builder.text_field.label_color(theme);
    let input_color = builder.text_field.input_color(theme);
    let placeholder_color = builder.text_field.placeholder_color(theme);
    let icon_color = builder.text_field.icon_color(theme);
    let leading_icon_text = builder.text_field.leading_icon.clone();
    let end_icon_text = builder
        .text_field
        .effective_trailing_icon()
        .map(|s| s.to_string());
    let initial_is_label_floating = builder.text_field.is_label_floating();

    let supporting_text = builder.text_field.supporting_text.clone();
    let error = builder.text_field.error;
    let error_text = builder.text_field.error_text.clone();

    let (supporting_display, supporting_color) = if error {
        (error_text.as_deref().unwrap_or(""), theme.error)
    } else {
        (
            supporting_text.as_deref().unwrap_or(""),
            theme.on_surface_variant,
        )
    };

    let should_spawn_supporting =
        !supporting_display.is_empty() || builder.localization.needs_supporting_entity();

    let mut spawned_field: Option<Entity> = None;
    parent
        .spawn(Node {
            width: builder.width,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|wrapper| {
            let mut field_commands = wrapper.spawn(builder.build(theme));
            let field_entity = field_commands.id();
            spawned_field = Some(field_entity);

            field_commands.with_children(|container| {
                // Leading icon (start icon)
                let leading_icon_visible = leading_icon_text
                    .as_deref()
                    .and_then(resolve_icon_id)
                    .is_some();
                container
                    .spawn((
                        TextFieldLeadingIconButton,
                        TextFieldLeadingIconButtonFor(field_entity),
                        Button,
                        RippleHost::new(),
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: if leading_icon_visible {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    ))
                    .with_children(|btn| {
                        let icon_id = leading_icon_text
                            .as_deref()
                            .and_then(resolve_icon_id)
                            .or_else(|| icon_by_name(ICON_CLOSE))
                            .expect("embedded icon 'close' not found");
                        btn.spawn((
                            TextFieldLeadingIcon,
                            TextFieldLeadingIconFor(field_entity),
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });

                // Content column so the label can float above the input.
                container
                    .spawn(Node {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|content| {
                        // Floating label (hidden when expanded)
                        if let Some(ref label) = label_text {
                            content.spawn((
                                TextFieldLabel,
                                TextFieldLabelFor(field_entity),
                                Text::new(label.as_str()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(label_color),
                                Node {
                                    display: if initial_is_label_floating {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    ..default()
                                },
                            ));
                        }

                        // Input text node (renders value/caret or in-field hint)
                        let expanded_hint = if label_text.is_some() {
                            label_text.as_deref().unwrap_or("")
                        } else {
                            placeholder_text.as_str()
                        };

                        let initial_display = if value_text.is_empty() {
                            if initial_is_label_floating {
                                "\u{200B}"
                            } else {
                                expanded_hint
                            }
                        } else {
                            value_text.as_str()
                        };

                        let initial_color = if value_text.is_empty() {
                            if initial_is_label_floating {
                                input_color
                            } else if label_text.is_some() {
                                // Expanded label/hint uses label color.
                                label_color
                            } else {
                                placeholder_color
                            }
                        } else {
                            input_color
                        };

                        // Input line: placeholder (overlay) + actual input text.
                        content
                            .spawn(Node {
                                position_type: PositionType::Relative,
                                ..default()
                            })
                            .with_children(|input_line| {
                                // Separate placeholder (shown only when label is floating and value is empty)
                                // Spawn it even if it's initially hidden so systems can toggle it.
                                input_line.spawn((
                                    TextFieldPlaceholder,
                                    TextFieldPlaceholderFor(field_entity),
                                    Text::new(placeholder_text.as_str()),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(placeholder_color),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        display: Display::None,
                                        ..default()
                                    },
                                    Visibility::Hidden,
                                ));

                                input_line.spawn((
                                    TextFieldInput,
                                    TextFieldInputFor(field_entity),
                                    Text::new(initial_display),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(initial_color),
                                ));
                            });
                    });

                // End icon (trailing icon)
                let end_icon_visible = end_icon_text.as_deref().and_then(resolve_icon_id).is_some();
                container
                    .spawn((
                        TextFieldEndIconButton,
                        TextFieldEndIconButtonFor(field_entity),
                        Button,
                        RippleHost::new(),
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: if end_icon_visible {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    ))
                    .with_children(|btn| {
                        let icon_id = end_icon_text
                            .as_deref()
                            .and_then(resolve_icon_id)
                            .or_else(|| icon_by_name(ICON_CLOSE))
                            .expect("embedded icon 'close' not found");
                        btn.spawn((
                            TextFieldEndIcon,
                            TextFieldEndIconFor(field_entity),
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });
            });

            if should_spawn_supporting {
                wrapper.spawn((
                    TextFieldSupportingText,
                    TextFieldSupportingFor(field_entity),
                    Text::new(supporting_display),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(supporting_color),
                    Node {
                        margin: UiRect::left(Val::Px(Spacing::LARGE)),
                        ..default()
                    },
                ));
            }
        });

    spawned_field.expect("spawn_text_field_control must spawn a field")
}

/// Spawn a Material text field (including its internal children), attach `marker` to the field
/// entity, and return the field entity.
pub fn spawn_text_field_control_with<M: Component>(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    builder: TextFieldBuilder,
    marker: M,
) -> Entity {
    // Copy the control spawn logic so we can insert the marker at spawn-time.
    let label_text: Option<String> = builder.text_field.label.clone();
    let value_text = builder.text_field.value.clone();
    let placeholder_text = builder.text_field.placeholder.clone();
    let label_color = builder.text_field.label_color(theme);
    let input_color = builder.text_field.input_color(theme);
    let placeholder_color = builder.text_field.placeholder_color(theme);
    let icon_color = builder.text_field.icon_color(theme);
    let leading_icon_text = builder.text_field.leading_icon.clone();
    let end_icon_text = builder
        .text_field
        .effective_trailing_icon()
        .map(|s| s.to_string());
    let initial_is_label_floating = builder.text_field.is_label_floating();

    let supporting_text = builder.text_field.supporting_text.clone();
    let error = builder.text_field.error;
    let error_text = builder.text_field.error_text.clone();

    let (supporting_display, supporting_color) = if error {
        (error_text.as_deref().unwrap_or(""), theme.error)
    } else {
        (
            supporting_text.as_deref().unwrap_or(""),
            theme.on_surface_variant,
        )
    };

    let should_spawn_supporting =
        !supporting_display.is_empty() || builder.localization.needs_supporting_entity();

    let mut spawned_field: Option<Entity> = None;
    parent
        .spawn(Node {
            width: builder.width,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|wrapper| {
            let mut field_commands = wrapper.spawn(builder.build(theme));
            field_commands.insert(marker);
            let field_entity = field_commands.id();
            spawned_field = Some(field_entity);

            field_commands.with_children(|container| {
                // Leading icon (start icon)
                let leading_icon_visible = leading_icon_text
                    .as_deref()
                    .and_then(resolve_icon_id)
                    .is_some();
                container
                    .spawn((
                        TextFieldLeadingIconButton,
                        TextFieldLeadingIconButtonFor(field_entity),
                        Button,
                        RippleHost::new(),
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: if leading_icon_visible {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    ))
                    .with_children(|btn| {
                        let icon_id = leading_icon_text
                            .as_deref()
                            .and_then(resolve_icon_id)
                            .or_else(|| icon_by_name(ICON_CLOSE))
                            .expect("embedded icon 'close' not found");
                        btn.spawn((
                            TextFieldLeadingIcon,
                            TextFieldLeadingIconFor(field_entity),
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });

                // Content column so the label can float above the input.
                container
                    .spawn(Node {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|content| {
                        // Floating label (hidden when expanded)
                        if let Some(ref label) = label_text {
                            content.spawn((
                                TextFieldLabel,
                                TextFieldLabelFor(field_entity),
                                Text::new(label.as_str()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(label_color),
                                Node {
                                    display: if initial_is_label_floating {
                                        Display::Flex
                                    } else {
                                        Display::None
                                    },
                                    ..default()
                                },
                            ));
                        }

                        // Input text node (renders value/caret or in-field hint)
                        let expanded_hint = if label_text.is_some() {
                            label_text.as_deref().unwrap_or("")
                        } else {
                            placeholder_text.as_str()
                        };

                        let initial_display = if value_text.is_empty() {
                            if initial_is_label_floating {
                                "\u{200B}"
                            } else {
                                expanded_hint
                            }
                        } else {
                            value_text.as_str()
                        };

                        let initial_color = if value_text.is_empty() {
                            if initial_is_label_floating {
                                input_color
                            } else if label_text.is_some() {
                                // Expanded label/hint uses label color.
                                label_color
                            } else {
                                placeholder_color
                            }
                        } else {
                            input_color
                        };

                        // Input line: placeholder (overlay) + actual input text.
                        content
                            .spawn(Node {
                                position_type: PositionType::Relative,
                                ..default()
                            })
                            .with_children(|input_line| {
                                // Separate placeholder (shown only when label is floating and value is empty)
                                // Spawn it even if it's initially hidden so systems can toggle it.
                                input_line.spawn((
                                    TextFieldPlaceholder,
                                    TextFieldPlaceholderFor(field_entity),
                                    Text::new(placeholder_text.as_str()),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(placeholder_color),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        right: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        display: Display::None,
                                        ..default()
                                    },
                                    Visibility::Hidden,
                                ));

                                input_line.spawn((
                                    TextFieldInput,
                                    TextFieldInputFor(field_entity),
                                    Text::new(initial_display),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(initial_color),
                                ));
                            });
                    });

                // End icon (trailing icon)
                let end_icon_visible = end_icon_text.as_deref().and_then(resolve_icon_id).is_some();
                container
                    .spawn((
                        TextFieldEndIconButton,
                        TextFieldEndIconButtonFor(field_entity),
                        Button,
                        RippleHost::new(),
                        Interaction::None,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            display: if end_icon_visible {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(CornerRadius::FULL)),
                    ))
                    .with_children(|btn| {
                        let icon_id = end_icon_text
                            .as_deref()
                            .and_then(resolve_icon_id)
                            .or_else(|| icon_by_name(ICON_CLOSE))
                            .expect("embedded icon 'close' not found");
                        btn.spawn((
                            TextFieldEndIcon,
                            TextFieldEndIconFor(field_entity),
                            MaterialIcon::new(icon_id),
                            IconStyle::outlined().with_color(icon_color).with_size(24.0),
                        ));
                    });
            });

            if should_spawn_supporting {
                wrapper.spawn((
                    TextFieldSupportingText,
                    TextFieldSupportingFor(field_entity),
                    Text::new(supporting_display),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(supporting_color),
                    Node {
                        margin: UiRect::left(Val::Px(Spacing::LARGE)),
                        ..default()
                    },
                ));
            }
        });

    spawned_field.expect("spawn_text_field_control_with must spawn a field")
}

fn text_field_end_icon_click_system(
    mut click_events: MessageWriter<TextFieldChangeEvent>,
    mut fields: Query<&mut MaterialTextField>,
    interactions: Query<
        (&Interaction, &TextFieldEndIconButtonFor),
        (Changed<Interaction>, With<TextFieldEndIconButton>),
    >,
) {
    for (interaction, TextFieldEndIconButtonFor(field_entity)) in interactions.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(mut field) = fields.get_mut(*field_entity) else {
            continue;
        };

        match field.end_icon_mode {
            EndIconMode::PasswordToggle => {
                field.toggle_password_visibility();
            }
            EndIconMode::ClearText => {
                if !field.value.is_empty() {
                    field.value.clear();
                    field.has_content = false;
                    click_events.write(TextFieldChangeEvent {
                        entity: *field_entity,
                        value: field.value.clone(),
                    });
                }
            }
            _ => {}
        }
    }
}

fn text_field_icon_system(
    theme: Option<Res<MaterialTheme>>,
    changed_fields: Query<(Entity, &MaterialTextField), Changed<MaterialTextField>>,
    mut leading_buttons: Query<
        (&TextFieldLeadingIconButtonFor, &mut Node),
        (
            With<TextFieldLeadingIconButton>,
            Without<TextFieldEndIconButton>,
        ),
    >,
    mut leading_icons: Query<
        (&TextFieldLeadingIconFor, &mut MaterialIcon, &mut IconStyle),
        (With<TextFieldLeadingIcon>, Without<TextFieldEndIcon>),
    >,
    mut end_buttons: Query<
        (&TextFieldEndIconButtonFor, &mut Node),
        (
            With<TextFieldEndIconButton>,
            Without<TextFieldLeadingIconButton>,
        ),
    >,
    mut end_icons: Query<
        (&TextFieldEndIconFor, &mut MaterialIcon, &mut IconStyle),
        (With<TextFieldEndIcon>, Without<TextFieldLeadingIcon>),
    >,
) {
    let Some(theme) = theme else { return };

    for (field_entity, field) in changed_fields.iter() {
        let icon_color = field.icon_color(&theme);

        // Leading
        let leading_icon_id = field.leading_icon.as_deref().and_then(resolve_icon_id);
        for (owner, mut icon, mut style) in leading_icons.iter_mut() {
            if owner.0 != field_entity {
                continue;
            }
            if let Some(id) = leading_icon_id {
                icon.id = id;
                style.color = icon_color;
                style.size = 24.0;
            }
        }
        for (owner, mut node) in leading_buttons.iter_mut() {
            if owner.0 != field_entity {
                continue;
            }
            node.display = if leading_icon_id.is_some() {
                Display::Flex
            } else {
                Display::None
            };
        }

        // End icon (trailing)
        let end_icon_id = field.effective_trailing_icon().and_then(resolve_icon_id);
        for (owner, mut icon, mut style) in end_icons.iter_mut() {
            if owner.0 != field_entity {
                continue;
            }
            if let Some(id) = end_icon_id {
                icon.id = id;
                style.color = icon_color;
                style.size = 24.0;
            }
        }
        for (owner, mut node) in end_buttons.iter_mut() {
            if owner.0 != field_entity {
                continue;
            }
            node.display = if end_icon_id.is_some() {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}
