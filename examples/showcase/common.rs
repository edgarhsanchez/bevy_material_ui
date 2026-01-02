//! Common types, resources, and helper functions shared across the showcase application.

use bevy::prelude::*;
use bevy::text::{Justify, LineBreak, TextLayout};
use bevy_material_ui::prelude::*;
use bevy_material_ui::theme::ThemeMode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// TELEMETRY SYSTEM - Reports component state for automated testing
// ============================================================================

/// Global telemetry state that can be written to file for test tooling
#[derive(Resource, Default)]
pub struct ComponentTelemetry {
    /// Component states as key-value pairs
    pub states: HashMap<String, String>,
    /// Recent events log
    pub events: Vec<String>,
    /// Element bounds for test automation (test_id -> bounds)
    pub elements: HashMap<String, ElementBounds>,
    /// Whether to write telemetry to file
    pub enabled: bool,
}

impl ComponentTelemetry {
    pub fn log_event(&mut self, event: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        self.events.push(format!("[{}] {}", timestamp, event));
        // Keep only last 100 events
        if self.events.len() > 100 {
            self.events.remove(0);
        }
    }

    pub fn write_to_file(&self) {
        if !self.enabled {
            return;
        }

        // Convert elements to serializable format
        let elements_json: Vec<_> = self
            .elements
            .values()
            .map(|e| {
                serde_json::json!({
                    "test_id": e.test_id,
                    "x": e.x,
                    "y": e.y,
                    "width": e.width,
                    "height": e.height,
                    "parent": e.parent,
                })
            })
            .collect();

        let json = serde_json::json!({
            "states": self.states,
            "events": self.events,
            "elements": elements_json,
        });
        if let Ok(mut file) = File::create("telemetry.json") {
            let _ = file.write_all(json.to_string().as_bytes());
        }
    }
}

// ElementBounds is now provided by the library's telemetry module
// Re-exported via bevy_material_ui::prelude::ElementBounds

// ============================================================================
// Component Sections
// ============================================================================

/// Enum representing all available component sections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ComponentSection {
    #[default]
    Buttons,
    Checkboxes,
    Switches,
    RadioButtons,
    Chips,
    Fab,
    Badges,
    Progress,
    Cards,
    Dividers,
    Lists,
    Icons,
    IconButtons,
    Sliders,
    TextFields,
    Dialogs,
    DatePicker,
    TimePicker,
    Menus,
    Tabs,
    Select,
    Snackbar,
    Tooltips,
    AppBar,
    Toolbar,
    Layouts,
    LoadingIndicator,
    Search,
    ThemeColors,
    Translations,
}

impl ComponentSection {
    /// Localization key for the section label.
    pub fn i18n_key(&self) -> &'static str {
        match self {
            Self::Buttons => "showcase.nav.buttons",
            Self::Checkboxes => "showcase.nav.checkboxes",
            Self::Switches => "showcase.nav.switches",
            Self::RadioButtons => "showcase.nav.radio_buttons",
            Self::Chips => "showcase.nav.chips",
            Self::Fab => "showcase.nav.fab",
            Self::Badges => "showcase.nav.badges",
            Self::Progress => "showcase.nav.progress",
            Self::Cards => "showcase.nav.cards",
            Self::Dividers => "showcase.nav.dividers",
            Self::Lists => "showcase.nav.lists",
            Self::Icons => "showcase.nav.icons",
            Self::IconButtons => "showcase.nav.icon_buttons",
            Self::Sliders => "showcase.nav.sliders",
            Self::TextFields => "showcase.nav.text_fields",
            Self::Dialogs => "showcase.nav.dialogs",
            Self::DatePicker => "showcase.nav.date_picker",
            Self::TimePicker => "showcase.nav.time_picker",
            Self::Menus => "showcase.nav.menus",
            Self::Tabs => "showcase.nav.tabs",
            Self::Select => "showcase.nav.select",
            Self::Snackbar => "showcase.nav.snackbar",
            Self::Tooltips => "showcase.nav.tooltips",
            Self::AppBar => "showcase.nav.app_bar",
            Self::Toolbar => "showcase.nav.toolbar",
            Self::Layouts => "showcase.nav.layouts",
            Self::LoadingIndicator => "showcase.nav.loading_indicator",
            Self::Search => "showcase.nav.search",
            Self::ThemeColors => "showcase.nav.theme_colors",
            Self::Translations => "showcase.nav.translations",
        }
    }

    /// Get display name for the component
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Checkboxes => "Checkboxes",
            Self::Switches => "Switches",
            Self::RadioButtons => "Radio Buttons",
            Self::Chips => "Chips",
            Self::Fab => "FAB",
            Self::Badges => "Badges",
            Self::Progress => "Progress",
            Self::Cards => "Cards",
            Self::Dividers => "Dividers",
            Self::Lists => "Lists",
            Self::Icons => "Icons",
            Self::IconButtons => "Icon Buttons",
            Self::Sliders => "Sliders",
            Self::TextFields => "Text Fields",
            Self::Dialogs => "Dialogs",
            Self::DatePicker => "Date Picker",
            Self::TimePicker => "Time Picker",
            Self::Menus => "Menus",
            Self::Tabs => "Tabs",
            Self::Select => "Select",
            Self::Snackbar => "Snackbar",
            Self::Tooltips => "Tooltips",
            Self::AppBar => "App Bar",
            Self::Toolbar => "Toolbar",
            Self::Layouts => "Layouts",
            Self::LoadingIndicator => "Loading Indicator",
            Self::Search => "Search",
            Self::ThemeColors => "Theme Colors",
            Self::Translations => "Translations",
        }
    }

    /// Telemetry-friendly identifier name used by the UI automation tooling.
    ///
    /// This intentionally matches `tests/ui_tests/quick_test.py` expectations.
    pub fn telemetry_name(&self) -> &'static str {
        match self {
            Self::Buttons => "Buttons",
            Self::Checkboxes => "Checkboxes",
            Self::Switches => "Switches",
            Self::RadioButtons => "RadioButtons",
            Self::Chips => "Chips",
            Self::Fab => "FAB",
            Self::Badges => "Badges",
            Self::Progress => "Progress",
            Self::Cards => "Cards",
            Self::Dividers => "Dividers",
            Self::Lists => "Lists",
            Self::Icons => "Icons",
            Self::IconButtons => "IconButtons",
            Self::Sliders => "Sliders",
            Self::TextFields => "TextFields",
            Self::Dialogs => "Dialogs",
            Self::DatePicker => "DatePicker",
            Self::TimePicker => "TimePicker",
            Self::Menus => "Menus",
            Self::Tabs => "Tabs",
            Self::Select => "Select",
            Self::Snackbar => "Snackbar",
            Self::Tooltips => "Tooltips",
            Self::AppBar => "AppBar",
            Self::Toolbar => "Toolbar",
            Self::Layouts => "Layouts",
            Self::LoadingIndicator => "LoadingIndicator",
            Self::Search => "Search",
            Self::ThemeColors => "ThemeColors",
            Self::Translations => "Translations",
        }
    }

    /// Get all component sections in order
    pub fn all() -> &'static [ComponentSection] {
        &[
            Self::Buttons,
            Self::Checkboxes,
            Self::Switches,
            Self::RadioButtons,
            Self::Chips,
            Self::Fab,
            Self::Badges,
            Self::Progress,
            Self::Cards,
            Self::Dividers,
            Self::Lists,
            Self::Icons,
            Self::IconButtons,
            Self::Sliders,
            Self::TextFields,
            Self::Dialogs,
            Self::DatePicker,
            Self::TimePicker,
            Self::Menus,
            Self::Tabs,
            Self::Select,
            Self::Snackbar,
            Self::Tooltips,
            Self::AppBar,
            Self::Toolbar,
            Self::Layouts,
            Self::LoadingIndicator,
            Self::Search,
            Self::ThemeColors,
            Self::Translations,
        ]
    }
}

/// Resource tracking the currently selected component section
#[derive(Resource, Default)]
pub struct SelectedSection {
    pub current: ComponentSection,
}

// ============================================================================
// Re-export TestId from the library
// ============================================================================

// TestId is now provided by the library's telemetry module
// Import it from bevy_material_ui::prelude::TestId

// ============================================================================
// Marker Components
// ============================================================================

/// Marker for the detail content area
#[derive(Component)]
pub struct DetailContent;

/// Dialog positioning options
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum DialogPosition {
    #[default]
    CenterWindow,
    CenterParent,
    BelowTrigger,
}

// Use the library's list selection mode type directly.

/// Tooltip demo options
#[derive(Resource)]
pub struct TooltipDemoOptions {
    pub position: TooltipPosition,
    pub delay: f32,
}

impl Default for TooltipDemoOptions {
    fn default() -> Self {
        Self {
            position: TooltipPosition::Bottom,
            delay: 0.5,
        }
    }
}

/// Snackbar demo options
#[derive(Resource)]
pub struct SnackbarDemoOptions {
    pub duration: f32,
    pub has_action: bool,
}

impl Default for SnackbarDemoOptions {
    fn default() -> Self {
        Self {
            duration: 4.0,
            has_action: false,
        }
    }
}

// ============================================================================
// Interactive Marker Components
// ============================================================================

// NOTE: Slider components (SliderHandle, SliderLabel, SliderTrack, SliderActiveTrack)
// are now imported from bevy_material_ui::prelude

/// Marker for selectable list items
#[derive(Component)]
pub struct SelectableListItem;

/// Marker for the demo list root (to apply selection mode changes)
#[derive(Component)]
pub struct ListDemoRoot;

/// Marker for dialog container
#[derive(Component)]
pub struct DialogContainer;

/// Marker for dialog show button
#[derive(Component)]
pub struct ShowDialogButton;

/// Marker for dialog close button
#[derive(Component)]
pub struct DialogCloseButton;

/// Marker for dialog confirm button
#[derive(Component)]
pub struct DialogConfirmButton;

/// Marker for dialog result display
#[derive(Component)]
pub struct DialogResultDisplay;

/// Marker for dialog modal option chips (true = modal, false = click-through).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogModalOption(pub bool);

/// Marker for date picker demo open button
#[derive(Component)]
pub struct DatePickerOpenButton(pub Entity);

/// Marker for date picker demo result display
#[derive(Component)]
pub struct DatePickerResultDisplay(pub Entity);

/// Marker for time picker demo open button
#[derive(Component)]
pub struct TimePickerOpenButton(pub Entity);

/// Marker for time picker demo result display
#[derive(Component)]
pub struct TimePickerResultDisplay(pub Entity);

/// Marker for menu trigger button
#[derive(Component)]
pub struct MenuTrigger;

/// Marker for menu dropdown
#[derive(Component)]
pub struct MenuDropdown;

/// Marker for menu item with its label
#[derive(Component)]
pub struct MenuItemMarker(pub String);

/// Marker for the text that shows the selected menu item
#[derive(Component)]
pub struct MenuSelectedText;

/// Marker for snackbar trigger button
#[derive(Component)]
pub struct SnackbarTrigger;

/// Marker for interactive icon buttons
#[derive(Component)]
pub struct IconButtonMarker;

/// Marker for tooltip demo button (updates when options change)
#[derive(Component)]
pub struct TooltipDemoButton;

/// Marker for tooltip position option buttons
#[derive(Component)]
pub struct TooltipPositionOption(pub TooltipPosition);

/// Marker for tooltip delay option buttons  
#[derive(Component)]
pub struct TooltipDelayOption(pub f32);

/// Marker for snackbar duration option buttons
#[derive(Component)]
pub struct SnackbarDurationOption(pub f32);

/// Marker for snackbar action toggle
#[derive(Component)]
pub struct SnackbarActionToggle;

/// Marker for theme mode option buttons
#[derive(Component)]
pub struct ThemeModeOption(pub ThemeMode);

/// Resource tracking the currently selected theme seed (ARGB).
#[derive(Resource, Debug, Clone, Copy)]
pub struct ShowcaseThemeSelection {
    pub seed_argb: u32,
}

impl Default for ShowcaseThemeSelection {
    fn default() -> Self {
        Self {
            // Default Material You purple
            seed_argb: 0xFF6750A4,
        }
    }
}

/// Marker for theme seed option buttons (ARGB).
#[derive(Component)]
pub struct ThemeSeedOption(pub u32);

/// Slot wrapper used to locate the theme seed text field after it is spawned.
/// (The spawn helper builds an internal hierarchy, so we attach markers in a follow-up system.)
#[derive(Component)]
pub struct ThemeSeedTextFieldSlot;

/// Marker on the actual `MaterialTextField` entity used for pasting/typing a theme seed.
#[derive(Component)]
pub struct ThemeSeedTextField;

/// Marker for dialog position option buttons
#[derive(Component)]
pub struct DialogPositionOption(pub DialogPosition);

/// Marker for list selection mode option buttons
#[derive(Component)]
pub struct ListSelectionModeOption(pub bevy_material_ui::list::ListSelectionMode);

// NOTE: RadioOuter, RadioInner, and SwitchHandle are exported by the library
// Use bevy_material_ui::prelude::{RadioOuter, RadioInner, SwitchHandle}

// NOTE: Select components (SelectContainer, SelectTrigger, SelectDropdown,
// SelectOptionItem, SelectDisplayText) are now imported from bevy_material_ui::prelude

// ============================================================================
// Helper Functions
// ============================================================================

/// Spawn a code block with syntax highlighting style
pub fn spawn_code_block(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, code: &str) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(theme.surface_container.with_alpha(0.8)),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|block| {
            block.spawn((
                Text::new(code),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface.with_alpha(0.87)),
                // Prevent long lines from forcing the whole page to be wider than the viewport.
                // This keeps the *page* horizontal scrollbar from appearing unless some other
                // component truly overflows horizontally.
                TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
            ));
        });
}

/// Spawn a section header with title and description
pub fn spawn_section_header(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    title_key: &str,
    title_default: &str,
    description_key: &str,
    description_default: &str,
) {
    parent.spawn((
        Text::new(""),
        LocalizedText::new(title_key).with_default(title_default),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(theme.primary),
        NeedsInternationalFont, // Marker for font system to apply
    ));

    if !description_default.is_empty() {
        parent.spawn((
            Text::new(""),
            LocalizedText::new(description_key).with_default(description_default),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(theme.on_surface_variant),
            Node {
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
            NeedsInternationalFont, // Marker for font system to apply
        ));
    }
}

/// Marker component indicating that a text node should use the international font when available.
/// This is applied to all localized text to ensure international characters display correctly.
#[derive(Component)]
pub struct NeedsInternationalFont;
