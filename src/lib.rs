//! # Bevy Material UI
//!
//! Material Design 3 UI components for the Bevy game engine.
//!
//! This library provides a comprehensive set of UI components following
//! [Material Design 3](https://m3.material.io/) guidelines, implemented
//! as Bevy ECS components and systems.
//!
//! ## Features
//!
//! - **Theme System**: Complete MD3 color scheme with light/dark mode support
//! - **Components**: Button, Card, Checkbox, Dialog, Divider, FAB, List, Menu,
//!   Progress, Radio, Ripple, Select, Slider, Switch, Tabs, TextField
//! - **Accessibility**: Built-in support for focus rings
//! - **Customization**: Token-based styling system for easy theming
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_material_ui::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(MaterialUiPlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
//!     commands.spawn(Camera2d);
//!     
//!     // Create a filled button
//!     commands.spawn(
//!         MaterialButtonBuilder::new("Click Me")
//!             .filled()
//!             .build(&theme)
//!     );
//! }
//! ```
//!
//! ## Architecture
//!
//! This library follows patterns from the official Material Design implementations:
//! - [material-web](https://github.com/material-components/material-web)
//! - [material-components-flutter](https://github.com/material-components/material-components-flutter)

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

// ============================================================================
// Core modules
// ============================================================================

/// Theme and color system based on Material Design 3
pub mod theme;

/// HCT color space and dynamic color generation
pub mod color;

/// Material Symbols icon system
pub mod icons;

/// Typography scale definitions
pub mod typography;

/// Spacing, corner radius, duration, and easing tokens
pub mod tokens;

/// Elevation and shadow utilities
pub mod elevation;

/// Focus ring component for accessibility
pub mod focus;

/// Ripple effect component for touch feedback
pub mod ripple;

/// Telemetry and test automation support
pub mod telemetry;

// ============================================================================
// Component modules
// ============================================================================

/// Button components (filled, outlined, text, elevated, tonal)
pub mod button;

/// Icon button component
pub mod icon_button;

/// Floating Action Button (FAB) component
pub mod fab;

/// Card components (elevated, filled, outlined)
pub mod card;

/// Checkbox component
pub mod checkbox;

/// Radio button component
pub mod radio;

/// Switch/toggle component
pub mod switch;

/// Slider component
pub mod slider;

/// Text field components (filled, outlined)
pub mod text_field;

/// Progress indicators (linear and circular)
pub mod progress;

/// Loading indicator (indeterminate activity indicator)
pub mod loading_indicator;

/// Dialog component
pub mod dialog;

/// Date & time picker component (dialog-based)
pub mod datetime_picker;

/// List and list item components
pub mod list;

/// Menu and menu item components
pub mod menu;

/// Tabs component
pub mod tabs;

/// Divider component
pub mod divider;

/// Select/dropdown component
pub mod select;

/// Adaptive layout utilities (window size classes)
pub mod adaptive;

/// Material layout components (e.g. Scaffold)
pub mod layout;

/// Motion and animation utilities
pub mod motion;

/// Snackbar component for brief messages
pub mod snackbar;

/// Chip components for filters, actions, and tags
pub mod chip;

/// App bar components (top and bottom)
pub mod app_bar;

/// Toolbar component (compact top row)
pub mod toolbar;

/// Badge component for notifications
pub mod badge;

/// Tooltip component for contextual help
pub mod tooltip;

/// Scroll container for scrollable content
pub mod scroll;

/// Search bar component
pub mod search;

/// Animation and transformation system
pub mod animation;

// ============================================================================
// Prelude
// ============================================================================

/// Prelude module for convenient imports
pub mod prelude {
    // Re-export Bevy UI types for convenience
    pub use bevy::ui::{BoxShadow, ShadowStyle, Outline};
    pub use bevy::ui::{BackgroundGradient, BorderGradient, Gradient, LinearGradient, RadialGradient, ConicGradient, ColorStop};

    // Core
    pub use crate::theme::{ColorScheme, MaterialTheme};
    pub use crate::typography::Typography;
    pub use crate::tokens::{CornerRadius, Duration, Easing, Spacing};
    pub use crate::elevation::{Elevation, ElevationShadow};
    pub use crate::focus::{FocusGained, FocusLost, Focusable, FocusPlugin, FocusRing, create_native_focus_outline};
    pub use crate::ripple::{Ripple, RippleHost, RipplePlugin, SpawnRipple};
    pub use crate::telemetry::{TelemetryConfig, TelemetryPlugin, TestId, ElementBounds, InsertTestId, test_id_if_enabled};

    // Color System
    pub use crate::color::{Hct, TonalPalette, MaterialColorScheme};

    // Icons
    pub use crate::icons::{
        MaterialIcon, IconBundle, IconStyle, IconWeight, IconGrade, IconOpticalSize,
        MaterialIconFont, MaterialIconsPlugin, MATERIAL_SYMBOLS_FONT_PATH,
        icon_by_name,
    };

    // Button
    pub use crate::button::{
        ButtonClickEvent, ButtonLabel, ButtonPlugin, ButtonVariant, MaterialButton, 
        MaterialButtonBuilder, SpawnButtonChild, spawn_material_button, material_button_bundle,
    };

    // Icon Button
    pub use crate::icon_button::{
        IconButtonBuilder, IconButtonClickEvent, IconButtonPlugin, IconButtonVariant,
        MaterialIconButton, SpawnIconButtonChild, ICON_BUTTON_SIZE, ICON_SIZE,
    };

    // Toolbar
    pub use crate::toolbar::{
        MaterialToolbar, SpawnToolbarChild, ToolbarAction, ToolbarActionEvent,
        ToolbarBuilder, ToolbarNavigationEvent, ToolbarPlugin,
        TOOLBAR_HEIGHT, TOOLBAR_ICON_SIZE,
    };

    // Loading Indicator
    pub use crate::loading_indicator::{
        LoadingIndicatorBuilder, LoadingIndicatorPlugin, MaterialLoadingIndicator,
        SpawnLoadingIndicatorChild, LOADING_INDICATOR_SIZE, LoadingShape,
    };

    // Search
    pub use crate::search::{
        MaterialSearchBar, SearchBarBuilder, SearchBarClickEvent, SearchPlugin,
        SearchQueryEvent, SpawnSearchBarChild, SEARCH_BAR_HEIGHT,
    };

    // Animation
    pub use crate::animation::{
        AnimatedValue, AnimationPlugin, FabTransformation, FabTransformState,
        MorphAnimation, SpringAnimation,
    };

    // FAB
    pub use crate::fab::{
        FabBuilder, FabClickEvent, FabColor, FabLabel, FabPlugin, FabSize, MaterialFab,
        SpawnFabChild,
    };

    // Card
    pub use crate::card::{
        CardBuilder, CardClickEvent, CardPlugin, CardVariant, MaterialCard, SpawnCardChild,
    };

    // Checkbox
    pub use crate::checkbox::{
        CheckboxBuilder, CheckboxChangeEvent, CheckboxPlugin, CheckboxState, MaterialCheckbox,
        CheckboxBox, CheckboxIcon, SpawnCheckbox, SpawnCheckboxChild,
        CHECKBOX_SIZE, CHECKBOX_TOUCH_TARGET,
    };

    // Radio
    pub use crate::radio::{
        RadioBuilder, RadioChangeEvent, RadioGroup, RadioPlugin, MaterialRadio,
        RadioOuter, RadioInner, RadioStateLayer, SpawnRadio, SpawnRadioChild,
        RADIO_DOT_SIZE, RADIO_SIZE, RADIO_TOUCH_TARGET,
    };

    // Switch
    pub use crate::switch::{
        SwitchBuilder, SwitchChangeEvent, SwitchHandle, SwitchStateLayer, SwitchPlugin, MaterialSwitch,
        SpawnSwitch, SpawnSwitchChild,
        SWITCH_HANDLE_SIZE_PRESSED, SWITCH_HANDLE_SIZE_SELECTED, SWITCH_HANDLE_SIZE_UNSELECTED,
        SWITCH_TRACK_HEIGHT, SWITCH_TRACK_WIDTH,
    };

    // Slider
    pub use crate::slider::{
        SliderActiveTrack, SliderBuilder, SliderChangeEvent, SliderHandle, SliderLabel,
        SliderPlugin, SliderTrack, MaterialSlider, SpawnSliderChild,
        SLIDER_HANDLE_SIZE, SLIDER_HANDLE_SIZE_PRESSED, SLIDER_LABEL_HEIGHT,
        SLIDER_TICK_SIZE, SLIDER_TRACK_HEIGHT, SLIDER_TRACK_HEIGHT_ACTIVE,
    };

    // Text Field
    pub use crate::text_field::{
        TextFieldBuilder, TextFieldChangeEvent, TextFieldInput, TextFieldLabel,
        TextFieldPlugin, TextFieldSubmitEvent, TextFieldSupportingText, TextFieldVariant,
        MaterialTextField, SpawnTextFieldChild, TEXT_FIELD_HEIGHT, TEXT_FIELD_MIN_WIDTH,
    };

    // Progress
    pub use crate::progress::{
        CircularProgressBuilder, LinearProgressBuilder, MaterialCircularProgress,
        MaterialLinearProgress, ProgressIndicator, ProgressMode, ProgressPlugin,
        ProgressTrack, ProgressVariant, SpawnProgressChild, CIRCULAR_PROGRESS_SIZE,
        CIRCULAR_PROGRESS_TRACK_WIDTH, LINEAR_PROGRESS_HEIGHT,
    };

    // Dialog
    pub use crate::dialog::{
        DialogActions, DialogBuilder, DialogCloseEvent, DialogConfirmEvent, DialogContent,
        DialogHeadline, DialogOpenEvent, DialogPlugin, DialogScrim, DialogType,
        MaterialDialog, SpawnDialogChild, create_dialog_scrim, DIALOG_MAX_WIDTH, DIALOG_MIN_WIDTH,
    };

    // DateTime Picker
    pub use crate::datetime_picker::{
        Date, DateTimePickerBuilder, DateTimePickerCancelEvent, DateTimePickerPlugin,
        DateTimePickerSubmitEvent, MaterialDateTimePicker, SpawnDateTimePickerChild,
        TimeFormat, Weekday,
    };

    // List
    pub use crate::list::{
        ListBuilder, ListDivider, ListItemBody, ListItemBuilder, ListItemClickEvent,
        ListItemHeadline, ListItemLeading, ListItemSupportingText, ListItemTrailing,
        ListItemVariant, ListPlugin, ListSelectionMode, MaterialList, MaterialListItem, ScrollableList,
        SpawnListChild, create_list_divider,
    };

    // Menu
    pub use crate::menu::{
        MenuAnchor, MenuBuilder, MenuCloseEvent, MenuDivider, MenuItemBuilder,
        MenuItemSelectEvent, MenuOpenEvent, MenuPlugin, MaterialMenu, MaterialMenuItem,
        SpawnMenuChild, create_menu_divider, MENU_ITEM_HEIGHT, MENU_MAX_WIDTH, MENU_MIN_WIDTH,
    };

    // Tabs
    pub use crate::tabs::{
        TabBuilder, TabChangeEvent, TabContent, TabIndicator, TabLabelText, TabVariant, TabsBuilder, TabsPlugin,
        MaterialTab, MaterialTabs, SpawnTabsChild, create_tab_indicator,
        TAB_HEIGHT_PRIMARY, TAB_HEIGHT_PRIMARY_ICON_ONLY, TAB_HEIGHT_SECONDARY,
        TAB_INDICATOR_HEIGHT,
    };

    // Divider
    pub use crate::divider::{
        DividerBuilder, DividerVariant, MaterialDivider, SpawnDividerChild,
        horizontal_divider, inset_divider, vertical_divider,
        DIVIDER_INSET, DIVIDER_THICKNESS,
    };

    // Select
    pub use crate::select::{
        SelectBuilder, SelectChangeEvent, SelectContainer, SelectDisplayText, SelectDropdown,
        SelectOption, SelectOptionItem, SelectPlugin, SelectTrigger, SelectVariant,
        MaterialSelect, SpawnSelectChild, SELECT_HEIGHT, SELECT_OPTION_HEIGHT,
    };

    // Adaptive Layout
    pub use crate::adaptive::{
        WindowWidthClass, WindowHeightClass, WindowSizeClass, WindowSizeClassPlugin,
        WindowSizeClassChanged,
    };

    // Layout
    pub use crate::layout::{
        AppBarOffsetConfig,
        AdaptiveNavigationScaffold,
        BottomNavigationScaffold,
        ListDetailScaffold,
        ModalDrawerScaffold,
        NavigationBarScaffold,
        NavigationRailScaffold,
        NavigationSuiteScaffold,
        PermanentDrawerScaffold,
        PaneEntities,
        PaneTestIds,
        ScaffoldEntities,
        ScaffoldTestIds,
        SupportingPanesScaffold,
        apply_app_bar_inset,
        spawn_adaptive_navigation_scaffold,
        spawn_bottom_navigation_scaffold,
        spawn_list_detail_scaffold,
        spawn_modal_drawer_scaffold,
        spawn_navigation_bar_scaffold,
        spawn_navigation_rail_scaffold,
        spawn_navigation_suite_scaffold,
        spawn_permanent_drawer_scaffold,
        spawn_supporting_panes_scaffold,
    };

    // Motion
    pub use crate::motion::{
        MotionPlugin, SpringConfig, StateLayer,
        ease_emphasized, ease_emphasized_accelerate, ease_emphasized_decelerate,
        ease_standard, ease_standard_accelerate, ease_standard_decelerate,
    };

    // Snackbar
    pub use crate::snackbar::{
        Snackbar, SnackbarAnimationState, SnackbarBuilder, SnackbarHostBuilder,
        SnackbarPlugin, SnackbarPosition, SnackbarQueue, SpawnSnackbarChild, spawn_snackbar, 
        ShowSnackbar, DismissSnackbar, SnackbarActionEvent, SNACKBAR_MAX_WIDTH,
    };

    // Chip
    pub use crate::chip::{
        ChipBuilder, ChipClickEvent, ChipDeleteButton, ChipDeleteEvent, ChipLabel, 
        ChipLeadingIcon, ChipPlugin, ChipVariant, MaterialChip, SpawnChipChild, CHIP_HEIGHT,
    };

    // App Bar
    pub use crate::app_bar::{
        AppBarPlugin, BottomAppBarBuilder, BottomAppBar, SpawnAppBarChild, TopAppBar,
        TopAppBarBuilder, TopAppBarVariant, TOP_APP_BAR_HEIGHT_LARGE,
        TOP_APP_BAR_HEIGHT_MEDIUM, TOP_APP_BAR_HEIGHT_SMALL, BOTTOM_APP_BAR_HEIGHT,
    };

    // Badge
    pub use crate::badge::{
        BadgeBuilder, BadgeContent, BadgePlugin, MaterialBadge, SpawnBadgeChild,
        BADGE_SIZE_LARGE, BADGE_SIZE_SMALL,
    };

    // Tooltip
    pub use crate::tooltip::{
        RichTooltip, Tooltip, TooltipAnimationState, TooltipPlugin, TooltipPosition,
        TooltipText, TooltipTrigger, TooltipTriggerBuilder, TooltipVariant, SpawnTooltipChild,
        spawn_rich_tooltip, spawn_tooltip, TOOLTIP_DELAY_DEFAULT, TOOLTIP_DELAY_SHORT,
        TOOLTIP_HEIGHT_PLAIN, TOOLTIP_MAX_WIDTH, TOOLTIP_OFFSET,
    };

    // Scroll Container
    pub use crate::scroll::{
        ScrollContainer, ScrollContainerBuilder, ScrollContent, ScrollDirection, ScrollPlugin,
        ScrollbarTrackVertical, ScrollbarThumbVertical, ScrollbarTrackHorizontal, ScrollbarThumbHorizontal,
        spawn_scrollbars,
    };

    // Main plugin
    pub use crate::MaterialUiPlugin;
}

// ============================================================================
// Main Plugin
// ============================================================================

/// Main plugin that adds all Material UI functionality to your Bevy app.
///
/// This plugin will:
/// - Initialize the Material theme resource
/// - Add component plugins for all components
/// - Set up the focus and ripple systems
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::MaterialUiPlugin;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(MaterialUiPlugin)
///     .run();
/// ```
pub struct MaterialUiPlugin;

impl Plugin for MaterialUiPlugin {
    fn build(&self, app: &mut App) {
        // Initialize theme resource
        app.init_resource::<theme::MaterialTheme>();

        // Core plugins
        app.add_plugins((
            focus::FocusPlugin,
            ripple::RipplePlugin,
            icons::icon::IconPlugin,
            icons::MaterialIconsPlugin,
        ));

        // Component plugins
        app.add_plugins((
            button::ButtonPlugin,
            icon_button::IconButtonPlugin,
            fab::FabPlugin,
            card::CardPlugin,
            checkbox::CheckboxPlugin,
            radio::RadioPlugin,
            switch::SwitchPlugin,
            slider::SliderPlugin,
            text_field::TextFieldPlugin,
            progress::ProgressPlugin,
            dialog::DialogPlugin,
            list::ListPlugin,
            menu::MenuPlugin,
            tabs::TabsPlugin,
            select::SelectPlugin,
        ));

        // New component plugins
        app.add_plugins((
            motion::MotionPlugin,
            snackbar::SnackbarPlugin,
            chip::ChipPlugin,
            app_bar::AppBarPlugin,
            toolbar::ToolbarPlugin,
            badge::BadgePlugin,
            tooltip::TooltipPlugin,
            scroll::ScrollPlugin,
            datetime_picker::DateTimePickerPlugin,
            loading_indicator::LoadingIndicatorPlugin,
            search::SearchPlugin,
            animation::AnimationPlugin,
        ));

        // Adaptive layout
        app.add_plugins(adaptive::WindowSizeClassPlugin);
    }
}

/// A plugin group that adds Material UI plugins in stages.
/// Use this if you want more control over which plugins are added.
pub struct MaterialUiPlugins;

impl PluginGroup for MaterialUiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(MaterialUiPlugin)
    }
}
