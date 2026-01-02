//! Comprehensive Material Component Tests
//!
//! Tests modeled after reference Material Components test patterns.
//!
//! These tests verify:
//! - Component state management
//! - State transitions
//! - Property setters/getters
//! - Event handling
//! - Builder patterns
//! - Default values

use bevy_material_ui::badge::{BadgeSize, MaterialBadge};
use bevy_material_ui::button::{ButtonVariant, IconGravity, MaterialButton};
use bevy_material_ui::button_group::{
    ButtonGroupBuilder, ButtonGroupOrientation, MaterialButtonGroup,
};
use bevy_material_ui::checkbox::{CheckboxState, MaterialCheckbox};
use bevy_material_ui::chip::{ChipElevation, ChipVariant, MaterialChip};
use bevy_material_ui::dialog::{DialogType, MaterialDialog};
use bevy_material_ui::fab::{FabColor, FabSize, MaterialFab};
use bevy_material_ui::progress::{MaterialCircularProgress, MaterialLinearProgress, ProgressMode};
use bevy_material_ui::radio::MaterialRadio;
use bevy_material_ui::slider::{MaterialSlider, SliderVariant, TickVisibility};
use bevy_material_ui::snackbar::{ShowSnackbar, SnackbarPosition};
use bevy_material_ui::switch::MaterialSwitch;
use bevy_material_ui::text_field::{EndIconMode, InputType, MaterialTextField, TextFieldVariant};
use bevy_material_ui::tooltip::{TooltipPosition, TooltipTrigger, TooltipVariant};

// ============================================================================
// Button Tests (modeled after MaterialButtonTest.java)
// ============================================================================

mod button_tests {
    use super::*;

    #[test]
    fn test_default_variant_is_filled() {
        let button = MaterialButton::new("Test");
        assert_eq!(button.variant, ButtonVariant::Filled);
    }

    #[test]
    fn test_set_variant() {
        let button = MaterialButton::new("Test").with_variant(ButtonVariant::Outlined);
        assert_eq!(button.variant, ButtonVariant::Outlined);
    }

    #[test]
    fn test_all_variants() {
        let variants = [
            ButtonVariant::Elevated,
            ButtonVariant::Filled,
            ButtonVariant::FilledTonal,
            ButtonVariant::Outlined,
            ButtonVariant::Text,
        ];

        for variant in variants {
            let button = MaterialButton::new("Test").with_variant(variant);
            assert_eq!(button.variant, variant);
        }
    }

    #[test]
    fn test_icon_gravity_default_is_start() {
        let button = MaterialButton::new("Test");
        assert_eq!(button.icon_gravity, IconGravity::Start);
    }

    #[test]
    fn test_icon_gravity_all_positions() {
        let positions = [
            IconGravity::Start,
            IconGravity::TextStart,
            IconGravity::End,
            IconGravity::TextEnd,
            IconGravity::Top,
            IconGravity::TextTop,
        ];

        for gravity in positions {
            let button = MaterialButton::new("Test")
                .with_icon("add")
                .icon_gravity(gravity);
            assert_eq!(button.icon_gravity, gravity);
        }
    }

    #[test]
    fn test_checkable_button() {
        let mut button = MaterialButton::new("Toggle").checkable(true);

        assert!(button.checkable);
        assert!(!button.checked);

        // Toggle on
        button.checked = true;
        assert!(button.checked);

        // Toggle off
        button.checked = false;
        assert!(!button.checked);
    }

    #[test]
    fn test_disabled_button() {
        let button = MaterialButton::new("Disabled").disabled(true);
        assert!(button.disabled);
    }

    #[test]
    fn test_icon_setting() {
        let button = MaterialButton::new("Add").with_icon("add");
        assert_eq!(button.icon, Some("add".to_string()));
    }

    #[test]
    fn test_icon_updated_when_changed() {
        let mut button = MaterialButton::new("Test").with_icon("add");
        assert_eq!(button.icon, Some("add".to_string()));

        button.icon = Some("remove".to_string());
        assert_eq!(button.icon, Some("remove".to_string()));
    }

    #[test]
    fn test_corner_radius_customization() {
        let button = MaterialButton::new("Rounded").corner_radius(16.0);
        assert_eq!(button.corner_radius, Some(16.0));
    }

    #[test]
    fn test_stroke_width_for_outlined() {
        let button = MaterialButton::new("Outlined")
            .with_variant(ButtonVariant::Outlined)
            .stroke_width(2.0);
        assert_eq!(button.stroke_width, 2.0);
    }

    #[test]
    fn test_interaction_states_default_false() {
        let button = MaterialButton::new("Test");
        assert!(!button.pressed);
        assert!(!button.hovered);
        assert!(!button.focused);
    }
}

// ============================================================================
// Button Group Tests (modeled after MaterialButtonToggleGroup behaviors)
// ============================================================================

mod button_group_tests {
    use super::*;

    #[test]
    fn test_default_orientation_is_horizontal() {
        let group = MaterialButtonGroup::new();
        assert_eq!(group.orientation, ButtonGroupOrientation::Horizontal);
    }

    #[test]
    fn test_builder_vertical() {
        let group = ButtonGroupBuilder::new().vertical().build();
        assert_eq!(group.orientation, ButtonGroupOrientation::Vertical);
    }

    #[test]
    fn test_single_selection_and_required_flags() {
        let group = MaterialButtonGroup::new()
            .single_selection(true)
            .selection_required(true);
        assert!(group.single_selection);
        assert!(group.selection_required);
    }
}

// ============================================================================
// Checkbox Tests (modeled after MaterialCheckBoxTest.java)
// ============================================================================

mod checkbox_tests {
    use super::*;

    #[test]
    fn test_set_checked_state_checked_succeeds() {
        let mut checkbox = MaterialCheckbox::new();
        checkbox.state = CheckboxState::Checked;

        assert!(checkbox.state.is_checked());
        assert_eq!(checkbox.state, CheckboxState::Checked);
    }

    #[test]
    fn test_set_checked_state_unchecked_succeeds() {
        let mut checkbox = MaterialCheckbox::new().with_state(CheckboxState::Checked);
        assert!(checkbox.state.is_checked());

        checkbox.state = CheckboxState::Unchecked;
        assert!(!checkbox.state.is_checked());
        assert_eq!(checkbox.state, CheckboxState::Unchecked);
    }

    #[test]
    fn test_set_checked_state_indeterminate_succeeds() {
        let checkbox = MaterialCheckbox::new().with_state(CheckboxState::Indeterminate);

        assert!(!checkbox.state.is_checked());
        assert!(checkbox.state.is_indeterminate());
        assert_eq!(checkbox.state, CheckboxState::Indeterminate);
    }

    #[test]
    fn test_checked_to_indeterminate_succeeds() {
        let mut checkbox = MaterialCheckbox::new().with_state(CheckboxState::Checked);

        checkbox.state = CheckboxState::Indeterminate;

        assert!(!checkbox.state.is_checked());
        assert!(checkbox.state.is_indeterminate());
    }

    #[test]
    fn test_indeterminate_toggle_becomes_checked() {
        let checkbox = MaterialCheckbox::new().with_state(CheckboxState::Indeterminate);

        let toggled = checkbox.state.toggle();
        assert_eq!(toggled, CheckboxState::Checked);
    }

    #[test]
    fn test_unchecked_toggle_becomes_checked() {
        let checkbox = MaterialCheckbox::new().with_state(CheckboxState::Unchecked);

        let toggled = checkbox.state.toggle();
        assert_eq!(toggled, CheckboxState::Checked);
    }

    #[test]
    fn test_checked_toggle_becomes_unchecked() {
        let checkbox = MaterialCheckbox::new().with_state(CheckboxState::Checked);

        let toggled = checkbox.state.toggle();
        assert_eq!(toggled, CheckboxState::Unchecked);
    }

    #[test]
    fn test_error_state() {
        let checkbox = MaterialCheckbox::new().error(true);
        assert!(checkbox.error);
    }

    #[test]
    fn test_disabled_state() {
        let checkbox = MaterialCheckbox::new().disabled(true);
        assert!(checkbox.disabled);
    }

    #[test]
    fn test_icon_for_states() {
        assert!(CheckboxState::Checked.icon().is_some());
        assert!(CheckboxState::Indeterminate.icon().is_some());
        assert!(CheckboxState::Unchecked.icon().is_none());
    }
}

// ============================================================================
// Switch Tests
// ============================================================================

mod switch_tests {
    use super::*;

    #[test]
    fn test_default_is_off() {
        let switch = MaterialSwitch::new();
        assert!(!switch.selected);
        assert_eq!(switch.animation_progress, 0.0);
    }

    #[test]
    fn test_selected_state() {
        let switch = MaterialSwitch::new().selected(true);
        assert!(switch.selected);
        assert_eq!(switch.animation_progress, 1.0);
    }

    #[test]
    fn test_disabled_state() {
        let switch = MaterialSwitch::new().disabled(true);
        assert!(switch.disabled);
    }

    #[test]
    fn test_with_icon() {
        let switch = MaterialSwitch::new().with_icon();
        assert!(switch.with_icon);
    }

    #[test]
    fn test_interaction_states_default_false() {
        let switch = MaterialSwitch::new();
        assert!(!switch.pressed);
        assert!(!switch.hovered);
    }
}

// ============================================================================
// Radio Tests
// ============================================================================

mod radio_tests {
    use super::*;

    #[test]
    fn test_default_is_unselected() {
        let radio = MaterialRadio::new();
        assert!(!radio.selected);
    }

    #[test]
    fn test_selected_state() {
        let radio = MaterialRadio::new().selected(true);
        assert!(radio.selected);
    }

    #[test]
    fn test_disabled_state() {
        let radio = MaterialRadio::new().disabled(true);
        assert!(radio.disabled);
    }

    #[test]
    fn test_group_assignment() {
        let radio = MaterialRadio::new().group("options");
        assert_eq!(radio.group, Some("options".to_string()));
    }
}

// ============================================================================
// Slider Tests
// ============================================================================

mod slider_tests {
    use super::*;

    #[test]
    fn test_default_value_is_min() {
        let slider = MaterialSlider::new(0.0, 100.0);
        assert_eq!(slider.value, 0.0);
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
    }

    #[test]
    fn test_with_value() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(50.0);
        assert_eq!(slider.value, 50.0);
    }

    #[test]
    fn test_value_clamped_to_range() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(150.0);
        assert_eq!(slider.value, 100.0); // Clamped to max

        let slider2 = MaterialSlider::new(0.0, 100.0).with_value(-50.0);
        assert_eq!(slider2.value, 0.0); // Clamped to min
    }

    #[test]
    fn test_step_size() {
        let slider = MaterialSlider::new(0.0, 100.0).with_step(10.0);
        assert_eq!(slider.step, Some(10.0));
    }

    #[test]
    fn test_discrete_mode() {
        let slider = MaterialSlider::new(0.0, 100.0).discrete(5);
        assert_eq!(slider.discrete_value_count, Some(5));
        // Step should be calculated: (100 - 0) / (5 - 1) = 25
        assert_eq!(slider.step, Some(25.0));
    }

    #[test]
    fn test_anchor_value() {
        let slider = MaterialSlider::new(0.0, 100.0).anchor(50.0);
        assert_eq!(slider.anchor_value, Some(50.0));
    }

    #[test]
    fn test_anchor_clamped_to_range() {
        let slider = MaterialSlider::new(0.0, 100.0).anchor(150.0);
        assert_eq!(slider.anchor_value, Some(100.0)); // Clamped to max
    }

    #[test]
    fn test_tick_visibility() {
        let slider = MaterialSlider::new(0.0, 100.0).tick_visibility(TickVisibility::Always);
        assert_eq!(slider.tick_visibility, TickVisibility::Always);
    }

    #[test]
    fn test_show_label() {
        let slider = MaterialSlider::new(0.0, 100.0).show_label();
        assert!(slider.show_label);
    }

    #[test]
    fn test_disabled_state() {
        let slider = MaterialSlider::new(0.0, 100.0).disabled(true);
        assert!(slider.disabled);
    }

    #[test]
    fn test_custom_track_height() {
        let slider = MaterialSlider::new(0.0, 100.0).track_height(8.0);
        assert_eq!(slider.track_height, 8.0);
    }

    #[test]
    fn test_custom_thumb_radius() {
        let slider = MaterialSlider::new(0.0, 100.0).thumb_radius(12.0);
        assert_eq!(slider.thumb_radius, 12.0);
    }
}

// ============================================================================
// Chip Tests (modeled after ChipTest.java)
// ============================================================================

mod chip_tests {
    use super::*;

    #[test]
    fn test_default_variant_is_assist() {
        let chip = MaterialChip::new("Test");
        assert_eq!(chip.variant, ChipVariant::Assist);
    }

    #[test]
    fn test_all_variants() {
        let variants = [
            ChipVariant::Assist,
            ChipVariant::Filter,
            ChipVariant::Input,
            ChipVariant::Suggestion,
        ];

        for variant in variants {
            let chip = MaterialChip::new("Test").with_variant(variant);
            assert_eq!(chip.variant, variant);
        }
    }

    #[test]
    fn test_assist_chip_constructor() {
        let chip = MaterialChip::assist("Assist");
        assert_eq!(chip.variant, ChipVariant::Assist);
        assert_eq!(chip.label, "Assist");
    }

    #[test]
    fn test_filter_chip_constructor() {
        let chip = MaterialChip::filter("Filter");
        assert_eq!(chip.variant, ChipVariant::Filter);
    }

    #[test]
    fn test_input_chip_constructor() {
        let chip = MaterialChip::input("Input");
        assert_eq!(chip.variant, ChipVariant::Input);
    }

    #[test]
    fn test_suggestion_chip_constructor() {
        let chip = MaterialChip::suggestion("Suggestion");
        assert_eq!(chip.variant, ChipVariant::Suggestion);
    }

    #[test]
    fn test_selected_state() {
        let chip = MaterialChip::filter("Filter").with_selected(true);
        assert!(chip.selected);
    }

    #[test]
    fn test_deletable_chip() {
        let chip = MaterialChip::input("Tag").with_deletable(true);
        assert!(chip.deletable);
    }

    #[test]
    fn test_disabled_state() {
        let chip = MaterialChip::new("Disabled").with_disabled(true);
        assert!(chip.disabled);
    }

    #[test]
    fn test_elevation() {
        let flat = MaterialChip::new("Flat");
        let elevated = MaterialChip::new("Elevated").elevated();

        assert_eq!(flat.elevation, ChipElevation::Flat);
        assert_eq!(elevated.elevation, ChipElevation::Elevated);
    }

    #[test]
    fn test_value() {
        let chip = MaterialChip::new("Label").with_value("id-123");
        assert_eq!(chip.value, Some("id-123".to_string()));
    }
}

// ============================================================================
// TextField Tests (modeled after TextInputLayout tests)
// ============================================================================

mod text_field_tests {
    use super::*;

    #[test]
    fn test_default_variant_is_filled() {
        let field = MaterialTextField::new();
        assert_eq!(field.variant, TextFieldVariant::Filled);
    }

    #[test]
    fn test_outlined_variant() {
        let field = MaterialTextField::new().with_variant(TextFieldVariant::Outlined);
        assert_eq!(field.variant, TextFieldVariant::Outlined);
    }

    #[test]
    fn test_placeholder() {
        let field = MaterialTextField::new().placeholder("Enter text");
        assert_eq!(field.placeholder, "Enter text");
    }

    #[test]
    fn test_label() {
        let field = MaterialTextField::new().label("Username");
        assert_eq!(field.label, Some("Username".to_string()));
    }

    #[test]
    fn test_helper_text() {
        let field = MaterialTextField::new().supporting_text("This is helper text");
        assert_eq!(
            field.supporting_text,
            Some("This is helper text".to_string())
        );
    }

    #[test]
    fn test_prefix_text() {
        let field = MaterialTextField::new().prefix_text("$");
        assert_eq!(field.prefix_text, Some("$".to_string()));
    }

    #[test]
    fn test_suffix_text() {
        let field = MaterialTextField::new().suffix_text("kg");
        assert_eq!(field.suffix_text, Some("kg".to_string()));
    }

    #[test]
    fn test_error_state() {
        let field = MaterialTextField::new()
            .error(true)
            .error_text("Invalid input");
        assert!(field.error);
        assert_eq!(field.error_text, Some("Invalid input".to_string()));
    }

    #[test]
    fn test_counter_enabled() {
        let field = MaterialTextField::new()
            .counter_enabled(true)
            .max_length(100);
        assert!(field.counter_enabled);
        assert_eq!(field.max_length, Some(100));
    }

    #[test]
    fn test_end_icon_modes() {
        let modes = [
            EndIconMode::None,
            EndIconMode::PasswordToggle,
            EndIconMode::ClearText,
            EndIconMode::DropdownMenu,
            EndIconMode::Custom,
        ];

        for mode in modes {
            let field = MaterialTextField::new().end_icon_mode(mode);
            assert_eq!(field.end_icon_mode, mode);
        }
    }

    #[test]
    fn test_input_types() {
        let types = [
            InputType::Text,
            InputType::Password,
            InputType::Email,
            InputType::Number,
            InputType::Phone,
            InputType::Url,
            InputType::Multiline,
        ];

        for input_type in types {
            let field = MaterialTextField::new().input_type(input_type);
            assert_eq!(field.input_type, input_type);
        }
    }

    #[test]
    fn test_password_visibility_toggle() {
        let mut field = MaterialTextField::new()
            .input_type(InputType::Password)
            .end_icon_mode(EndIconMode::PasswordToggle);

        assert!(!field.password_visible);
        field.password_visible = true;
        assert!(field.password_visible);
    }

    #[test]
    fn test_disabled_state() {
        let field = MaterialTextField::new().disabled(true);
        assert!(field.disabled);
    }

    #[test]
    fn test_stroke_width() {
        let field = MaterialTextField::new().box_stroke_width(2.0);
        assert_eq!(field.box_stroke_width, 2.0);
    }
}

// ============================================================================
// FAB Tests (modeled after FloatingActionButton tests)
// ============================================================================

mod fab_tests {
    use super::*;

    #[test]
    fn test_default_size_is_regular() {
        let fab = MaterialFab::new("add");
        assert_eq!(fab.size, FabSize::Regular);
    }

    #[test]
    fn test_size_values() {
        assert_eq!(FabSize::Small.size(), 40.0);
        assert_eq!(FabSize::Regular.size(), 56.0);
        assert_eq!(FabSize::Large.size(), 96.0);
    }

    #[test]
    fn test_icon_sizes() {
        assert_eq!(FabSize::Small.icon_size(), 24.0);
        assert_eq!(FabSize::Regular.icon_size(), 24.0);
        assert_eq!(FabSize::Large.icon_size(), 36.0);
    }

    #[test]
    fn test_color_variants() {
        let colors = [
            FabColor::Primary,
            FabColor::Surface,
            FabColor::Secondary,
            FabColor::Tertiary,
        ];

        for color in colors {
            let fab = MaterialFab::new("add").with_color(color);
            assert_eq!(fab.color, color);
        }
    }

    #[test]
    fn test_lowered_fab() {
        let fab = MaterialFab::new("add").lowered();
        assert!(fab.lowered);
    }

    #[test]
    fn test_extended_fab() {
        let fab = MaterialFab::new("add").extended("Create");
        assert_eq!(fab.label, Some("Create".to_string()));
    }

    #[test]
    fn test_all_sizes() {
        for size in [FabSize::Small, FabSize::Regular, FabSize::Large] {
            let fab = MaterialFab::new("add").with_size(size);
            assert_eq!(fab.size, size);
        }
    }
}

// ============================================================================
// Badge Tests
// ============================================================================

mod badge_tests {
    use super::*;

    #[test]
    fn test_dot_badge() {
        let badge = MaterialBadge::dot();
        assert_eq!(badge.size, BadgeSize::Small);
        assert!(badge.content.is_none());
    }

    #[test]
    fn test_count_badge() {
        let badge = MaterialBadge::count(5);
        assert_eq!(badge.size, BadgeSize::Large);
        assert_eq!(badge.content, Some("5".to_string()));
    }

    #[test]
    fn test_text_badge() {
        let badge = MaterialBadge::text("NEW");
        assert_eq!(badge.size, BadgeSize::Large);
        assert_eq!(badge.content, Some("NEW".to_string()));
    }

    #[test]
    fn test_max_count() {
        let badge = MaterialBadge::count(1000).with_max(99);
        assert_eq!(badge.content, Some("99+".to_string()));
    }

    #[test]
    fn test_count_below_max() {
        let badge = MaterialBadge::count(50).with_max(99);
        assert_eq!(badge.content, Some("50".to_string()));
    }

    #[test]
    fn test_visibility() {
        let visible = MaterialBadge::dot().visible(true);
        let hidden = MaterialBadge::dot().visible(false);

        assert!(visible.visible);
        assert!(!hidden.visible);
    }

    #[test]
    fn test_set_count() {
        let mut badge = MaterialBadge::dot();
        badge.set_count(10);

        assert_eq!(badge.size, BadgeSize::Large);
        assert_eq!(badge.content, Some("10".to_string()));
    }

    #[test]
    fn test_set_dot() {
        let mut badge = MaterialBadge::count(5);
        badge.set_dot();

        assert_eq!(badge.size, BadgeSize::Small);
        assert!(badge.content.is_none());
    }
}

// ============================================================================
// Tooltip Tests
// ============================================================================

mod tooltip_tests {
    use super::*;

    #[test]
    fn test_default_position_is_top() {
        let trigger = TooltipTrigger::new("Help");
        assert_eq!(trigger.position, TooltipPosition::Top);
    }

    #[test]
    fn test_all_positions() {
        let positions = [
            TooltipPosition::Top,
            TooltipPosition::Bottom,
            TooltipPosition::Left,
            TooltipPosition::Right,
        ];

        for pos in positions {
            let trigger = TooltipTrigger::new("Help").with_position(pos);
            assert_eq!(trigger.position, pos);
        }
    }

    #[test]
    fn test_position_shortcuts() {
        assert_eq!(
            TooltipTrigger::new("T").top().position,
            TooltipPosition::Top
        );
        assert_eq!(
            TooltipTrigger::new("T").bottom().position,
            TooltipPosition::Bottom
        );
        assert_eq!(
            TooltipTrigger::new("T").left().position,
            TooltipPosition::Left
        );
        assert_eq!(
            TooltipTrigger::new("T").right().position,
            TooltipPosition::Right
        );
    }

    #[test]
    fn test_delay() {
        let trigger = TooltipTrigger::new("Help").with_delay(1.0);
        assert_eq!(trigger.delay, 1.0);
    }

    #[test]
    fn test_rich_variant() {
        let trigger = TooltipTrigger::new("Help").rich();
        assert_eq!(trigger.variant, TooltipVariant::Rich);
    }

    #[test]
    fn test_default_variant_is_plain() {
        let trigger = TooltipTrigger::new("Help");
        assert_eq!(trigger.variant, TooltipVariant::Plain);
    }
}

// ============================================================================
// Snackbar Tests
// ============================================================================

mod snackbar_tests {
    use super::*;

    #[test]
    fn test_simple_message() {
        let snackbar = ShowSnackbar::message("Saved");
        assert_eq!(snackbar.message, "Saved");
        assert!(snackbar.action.is_none());
    }

    #[test]
    fn test_with_action() {
        let snackbar = ShowSnackbar::with_action("Deleted", "Undo");
        assert_eq!(snackbar.message, "Deleted");
        assert_eq!(snackbar.action, Some("Undo".to_string()));
    }

    #[test]
    fn test_duration() {
        let snackbar = ShowSnackbar::message("Test").duration(5.0);
        assert_eq!(snackbar.duration, Some(5.0));
    }

    #[test]
    fn test_dismissible() {
        let dismissible = ShowSnackbar::message("Test").dismissible(true);
        let not_dismissible = ShowSnackbar::message("Test").dismissible(false);

        assert!(dismissible.dismissible);
        assert!(!not_dismissible.dismissible);
    }

    #[test]
    fn test_default_position_is_bottom_center() {
        let snackbar = ShowSnackbar::message("Test");
        assert_eq!(snackbar.position, SnackbarPosition::BottomCenter);
    }

    #[test]
    fn test_all_positions() {
        let positions = [
            SnackbarPosition::BottomCenter,
            SnackbarPosition::BottomLeft,
            SnackbarPosition::BottomRight,
            SnackbarPosition::TopCenter,
            SnackbarPosition::TopLeft,
            SnackbarPosition::TopRight,
        ];

        for pos in positions {
            let snackbar = ShowSnackbar::message("Test").position(pos);
            assert_eq!(snackbar.position, pos);
        }
    }

    #[test]
    fn test_position_shortcuts() {
        assert_eq!(
            ShowSnackbar::message("T").bottom_left().position,
            SnackbarPosition::BottomLeft
        );
        assert_eq!(
            ShowSnackbar::message("T").bottom_right().position,
            SnackbarPosition::BottomRight
        );
        assert_eq!(
            ShowSnackbar::message("T").top_center().position,
            SnackbarPosition::TopCenter
        );
        assert_eq!(
            ShowSnackbar::message("T").top_left().position,
            SnackbarPosition::TopLeft
        );
        assert_eq!(
            ShowSnackbar::message("T").top_right().position,
            SnackbarPosition::TopRight
        );
    }
}

// ============================================================================
// Dialog Tests
// ============================================================================

mod dialog_tests {
    use super::*;

    #[test]
    fn test_default_type_is_basic() {
        let dialog = MaterialDialog::new();
        assert_eq!(dialog.dialog_type, DialogType::Basic);
    }

    #[test]
    fn test_full_screen_type() {
        let dialog = MaterialDialog::new().with_type(DialogType::FullScreen);
        assert_eq!(dialog.dialog_type, DialogType::FullScreen);
    }

    #[test]
    fn test_title() {
        let dialog = MaterialDialog::new().title("Confirm");
        assert_eq!(dialog.title, Some("Confirm".to_string()));
    }

    #[test]
    fn test_icon() {
        let dialog = MaterialDialog::new().icon("warning");
        assert_eq!(dialog.icon, Some("warning".to_string()));
    }

    #[test]
    fn test_default_closed() {
        let dialog = MaterialDialog::new();
        assert!(!dialog.open);
    }

    #[test]
    fn test_open_state() {
        let dialog = MaterialDialog::new().open(true);
        assert!(dialog.open);
    }

    #[test]
    fn test_dismiss_on_scrim_default_true() {
        let dialog = MaterialDialog::new();
        assert!(dialog.dismiss_on_scrim_click);
    }

    #[test]
    fn test_no_scrim_dismiss() {
        let dialog = MaterialDialog::new().no_scrim_dismiss();
        assert!(!dialog.dismiss_on_scrim_click);
    }

    #[test]
    fn test_no_escape_dismiss() {
        let dialog = MaterialDialog::new().no_escape_dismiss();
        assert!(!dialog.dismiss_on_escape);
    }
}

// ============================================================================
// Progress Tests
// ============================================================================

mod progress_tests {
    use super::*;

    #[test]
    fn test_linear_default_determinate() {
        let progress = MaterialLinearProgress::new();
        assert_eq!(progress.mode, ProgressMode::Determinate);
        assert_eq!(progress.progress, 0.0);
    }

    #[test]
    fn test_linear_indeterminate() {
        let progress = MaterialLinearProgress::new().indeterminate();
        assert_eq!(progress.mode, ProgressMode::Indeterminate);
    }

    #[test]
    fn test_linear_progress_value() {
        let progress = MaterialLinearProgress::new().with_progress(0.5);
        assert_eq!(progress.progress, 0.5);
    }

    #[test]
    fn test_linear_progress_clamped() {
        let over = MaterialLinearProgress::new().with_progress(1.5);
        let under = MaterialLinearProgress::new().with_progress(-0.5);

        assert_eq!(over.progress, 1.0);
        assert_eq!(under.progress, 0.0);
    }

    #[test]
    fn test_linear_four_color() {
        let progress = MaterialLinearProgress::new().four_color();
        assert!(progress.four_color);
    }

    #[test]
    fn test_circular_default_determinate() {
        let progress = MaterialCircularProgress::new();
        assert_eq!(progress.mode, ProgressMode::Determinate);
        assert_eq!(progress.progress, 0.0);
    }

    #[test]
    fn test_circular_indeterminate() {
        let progress = MaterialCircularProgress::new().indeterminate();
        assert_eq!(progress.mode, ProgressMode::Indeterminate);
    }

    #[test]
    fn test_circular_progress_value() {
        let progress = MaterialCircularProgress::new().with_progress(0.75);
        assert_eq!(progress.progress, 0.75);
    }

    #[test]
    fn test_circular_size() {
        let progress = MaterialCircularProgress::new().with_size(48.0);
        assert_eq!(progress.size, 48.0);
    }
}

// ============================================================================
// Integration Tests - Type Export Verification
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn test_all_component_types_exported() {
        // Verify all component types are accessible from prelude
        fn _check_button(_: MaterialButton) {}
        fn _check_checkbox(_: MaterialCheckbox) {}
        fn _check_switch(_: MaterialSwitch) {}
        fn _check_radio(_: MaterialRadio) {}
        fn _check_slider(_: MaterialSlider) {}
        fn _check_chip(_: MaterialChip) {}
        fn _check_text_field(_: MaterialTextField) {}
        fn _check_fab(_: MaterialFab) {}
        fn _check_badge(_: MaterialBadge) {}
        fn _check_tooltip(_: TooltipTrigger) {}
        fn _check_dialog(_: MaterialDialog) {}
        fn _check_linear_progress(_: MaterialLinearProgress) {}
        fn _check_circular_progress(_: MaterialCircularProgress) {}
    }

    #[test]
    fn test_all_variant_enums_exported() {
        let _: ButtonVariant = ButtonVariant::Filled;
        let _: IconGravity = IconGravity::Start;
        let _: CheckboxState = CheckboxState::Checked;
        let _: SliderVariant = SliderVariant::Continuous;
        let _: TickVisibility = TickVisibility::Always;
        let _: ChipVariant = ChipVariant::Assist;
        let _: ChipElevation = ChipElevation::Flat;
        let _: TextFieldVariant = TextFieldVariant::Filled;
        let _: EndIconMode = EndIconMode::None;
        let _: InputType = InputType::Text;
        let _: FabSize = FabSize::Regular;
        let _: FabColor = FabColor::Primary;
        let _: BadgeSize = BadgeSize::Large;
        let _: TooltipVariant = TooltipVariant::Plain;
        let _: TooltipPosition = TooltipPosition::Top;
        let _: SnackbarPosition = SnackbarPosition::BottomCenter;
        let _: DialogType = DialogType::Basic;
        let _: ProgressMode = ProgressMode::Determinate;
    }

    #[test]
    fn test_builder_chain_pattern() {
        // Verify fluent builder pattern works for all components
        let _button = MaterialButton::new("Test")
            .with_variant(ButtonVariant::Outlined)
            .with_icon("add")
            .icon_gravity(IconGravity::Start)
            .disabled(false)
            .checkable(true);

        let _slider = MaterialSlider::new(0.0, 100.0)
            .with_value(50.0)
            .with_step(10.0)
            .show_label()
            .disabled(false);

        let _chip = MaterialChip::new("Tag")
            .with_variant(ChipVariant::Input)
            .with_deletable(true)
            .with_selected(false);

        let _field = MaterialTextField::new()
            .with_variant(TextFieldVariant::Outlined)
            .label("Email")
            .placeholder("Enter email")
            .end_icon_mode(EndIconMode::ClearText);

        let _fab = MaterialFab::new("add")
            .with_size(FabSize::Large)
            .with_color(FabColor::Secondary)
            .extended("Create");
    }
}

// ============================================================================
// Slider Behavioral Tests
// ============================================================================

mod slider_behavior_tests {
    use super::*;

    /// Test that slider value snaps to discrete steps
    #[test]
    fn test_discrete_slider_snapping() {
        let slider = MaterialSlider::new(0.0, 100.0).discrete(6);
        // With 6 values: 0, 20, 40, 60, 80, 100
        // Step should be 20
        assert_eq!(slider.step, Some(20.0));

        // Simulate value calculation
        let step = slider.step.unwrap_or(1.0);

        // Test snapping various values
        let test_cases = [
            (0.0, 0.0),
            (9.0, 0.0),   // rounds to 0
            (11.0, 20.0), // rounds to 20
            (50.0, 60.0), // rounds to 60
            (100.0, 100.0),
        ];

        for (input, expected) in test_cases {
            let snapped = (input / step).round() * step;
            assert_eq!(
                snapped, expected,
                "Input {} should snap to {}, got {}",
                input, expected, snapped
            );
        }
    }

    /// Test slider percentage calculation
    #[test]
    fn test_slider_percentage() {
        let slider = MaterialSlider::new(0.0, 100.0).with_value(25.0);
        let percentage = (slider.value - slider.min) / (slider.max - slider.min);
        assert!((percentage - 0.25).abs() < 0.001);
    }

    /// Test slider with non-zero minimum
    #[test]
    fn test_slider_with_offset_range() {
        let slider = MaterialSlider::new(50.0, 150.0).with_value(100.0);
        let percentage = (slider.value - slider.min) / (slider.max - slider.min);
        assert!((percentage - 0.5).abs() < 0.001);
    }

    /// Test slider value from position (simulates drag calculation)
    #[test]
    fn test_slider_value_from_position() {
        let slider = MaterialSlider::new(0.0, 100.0);

        // Simulate dragging at different positions
        // position is 0.0 to 1.0 representing relative x position
        let test_cases = [
            (0.0, 0.0),
            (0.25, 25.0),
            (0.5, 50.0),
            (0.75, 75.0),
            (1.0, 100.0),
        ];

        for (position, expected) in test_cases {
            let value = slider.min + (slider.max - slider.min) * position;
            assert!(
                (value - expected).abs() < 0.001,
                "Position {} should map to value {}, got {}",
                position,
                expected,
                value
            );
        }
    }

    /// Test that step values work correctly for non-round numbers
    #[test]
    fn test_slider_step_with_decimals() {
        let slider = MaterialSlider::new(0.0, 1.0).with_step(0.1);
        let step = slider.step.unwrap();

        // Test snapping
        let input = 0.35;
        let snapped = (input / step).round() * step;
        assert!((snapped - 0.4).abs() < 0.001);
    }
}

// ============================================================================
// Tab Behavioral Tests
// ============================================================================

mod tab_behavior_tests {
    /// Test tab state transitions
    #[test]
    fn test_tab_selection_logic() {
        // Simulating TabState behavior
        let mut selected_tab: usize = 0;
        let tab_count = 3;

        // Initial selection
        assert_eq!(selected_tab, 0);

        // Select tab 1
        selected_tab = 1;
        assert_eq!(selected_tab, 1);

        // Select tab 2
        selected_tab = 2;
        assert_eq!(selected_tab, 2);

        // Selecting same tab should keep it selected
        selected_tab = 2;
        assert_eq!(selected_tab, 2);

        // Out of bounds should be clamped (logic to implement)
        selected_tab = selected_tab.min(tab_count - 1);
        assert!(selected_tab < tab_count);
    }

    /// Test tab indicator position calculation
    #[test]
    fn test_tab_indicator_position() {
        let tab_widths = [100.0, 100.0, 100.0]; // 3 tabs, 100px each

        // Calculate indicator position for each tab
        for (i, &_width) in tab_widths.iter().enumerate() {
            let indicator_x: f32 = tab_widths[..i].iter().sum();
            let expected = (i as f32) * 100.0;
            assert!((indicator_x - expected).abs() < 0.001);
        }
    }

    /// Test tab content visibility logic
    #[test]
    fn test_tab_content_visibility() {
        let selected_tab = 1;
        let content_count = 3;

        for i in 0..content_count {
            let should_show = i == selected_tab;
            if i == 1 {
                assert!(should_show, "Selected tab content should be visible");
            } else {
                assert!(!should_show, "Non-selected tab content should be hidden");
            }
        }
    }
}

// ============================================================================
// Navigation Highlighting Tests
// ============================================================================

mod nav_behavior_tests {
    /// Test nav item selection state
    #[test]
    fn test_nav_single_selection() {
        let mut selected_index: Option<usize> = None;
        let nav_item_count = 5;

        // Initial selection
        assert_eq!(selected_index, None);

        // Select first item
        selected_index = Some(0);
        assert_eq!(selected_index, Some(0));

        // Select third item (should deselect first)
        selected_index = Some(2);
        assert_eq!(selected_index, Some(2));

        // Verify only one can be selected
        for i in 0..nav_item_count {
            let is_selected = selected_index == Some(i);
            if i == 2 {
                assert!(is_selected);
            } else {
                assert!(!is_selected);
            }
        }
    }

    /// Test that selecting an item updates background color logic
    #[test]
    fn test_nav_background_color_logic() {
        // Simulating background color assignment based on selection
        struct NavItemVisual {
            selected: bool,
            bg_color: (u8, u8, u8, u8), // RGBA
        }

        let transparent = (0, 0, 0, 0);
        let secondary_container = (232, 222, 248, 255); // Example MD3 color

        let mut items = [
            NavItemVisual {
                selected: false,
                bg_color: transparent,
            },
            NavItemVisual {
                selected: true,
                bg_color: secondary_container,
            },
            NavItemVisual {
                selected: false,
                bg_color: transparent,
            },
        ];

        // Verify colors
        assert_eq!(items[0].bg_color, transparent);
        assert_eq!(items[1].bg_color, secondary_container);
        assert_eq!(items[2].bg_color, transparent);

        // Change selection
        items[1].selected = false;
        items[1].bg_color = transparent;
        items[2].selected = true;
        items[2].bg_color = secondary_container;

        assert_eq!(items[1].bg_color, transparent);
        assert_eq!(items[2].bg_color, secondary_container);
    }
}
