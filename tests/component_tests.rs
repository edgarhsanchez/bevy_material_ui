//! Integration tests for new MD3 components
//!
//! Tests for Motion, Snackbar, Chip, App Bar, Badge, and Tooltip components.

use bevy_material_ui::prelude::*;
use bevy_material_ui::snackbar::{ShowSnackbar, SnackbarPosition};
use bevy_material_ui::chip::MaterialChip;
use bevy_material_ui::app_bar::TopAppBar;
use bevy_material_ui::badge::MaterialBadge;

// ============================================================================
// Motion Tests
// ============================================================================

mod motion_tests {
    use super::*;

    #[test]
    fn test_standard_easing_bounds() {
        // Standard easing should map [0, 1] to [0, 1]
        let start = ease_standard(0.0);
        let end = ease_standard(1.0);
        
        assert!((start - 0.0).abs() < 0.01);
        assert!((end - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_emphasized_easing_bounds() {
        let start = ease_emphasized(0.0);
        let end = ease_emphasized(1.0);
        
        assert!((start - 0.0).abs() < 0.01);
        assert!((end - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_decelerate_curves() {
        // Decelerate curves should start fast (high derivative at 0)
        // So at t=0.5, they should be > 0.5
        let standard_mid = ease_standard_decelerate(0.5);
        let emphasized_mid = ease_emphasized_decelerate(0.5);
        
        assert!(standard_mid > 0.5, "Standard decelerate at 0.5: {}", standard_mid);
        assert!(emphasized_mid > 0.5, "Emphasized decelerate at 0.5: {}", emphasized_mid);
    }

    #[test]
    fn test_accelerate_curves() {
        // Accelerate curves should start slow (low derivative at 0)
        // So at t=0.5, they should be < 0.5
        let standard_mid = ease_standard_accelerate(0.5);
        let emphasized_mid = ease_emphasized_accelerate(0.5);
        
        assert!(standard_mid < 0.5, "Standard accelerate at 0.5: {}", standard_mid);
        assert!(emphasized_mid < 0.5, "Emphasized accelerate at 0.5: {}", emphasized_mid);
    }

    #[test]
    fn test_easing_monotonicity() {
        // Easing functions should be monotonically increasing
        let mut prev = 0.0;
        for i in 0..=100 {
            let t = i as f32 / 100.0;
            let v = ease_standard(t);
            assert!(v >= prev - 0.001, "ease_standard not monotonic at t={}", t);
            prev = v;
        }
    }

    #[test]
    fn test_spring_config_default() {
        let config = SpringConfig::default();
        assert!(config.stiffness > 0.0);
        assert!(config.damping > 0.0);
        assert!(config.mass > 0.0);
    }

    #[test]
    fn test_spring_config_presets() {
        let gentle = SpringConfig::gentle();
        let bouncy = SpringConfig::bouncy();
        let stiff = SpringConfig::stiff();
        
        // Bouncy should have less damping than stiff
        assert!(bouncy.damping < stiff.damping);
        // Gentle should have less stiffness than stiff
        assert!(gentle.stiffness < stiff.stiffness);
    }
}

// ============================================================================
// Snackbar Tests
// ============================================================================

mod snackbar_tests {
    use super::*;

    #[test]
    fn test_show_snackbar_event() {
        let event = ShowSnackbar {
            message: "Test message".to_string(),
            action: Some("Undo".to_string()),
            duration: Some(5.0),
            dismissible: true,
            position: SnackbarPosition::BottomCenter,
        };
        
        assert_eq!(event.message, "Test message");
        assert_eq!(event.action, Some("Undo".to_string()));
        assert_eq!(event.duration, Some(5.0));
        assert_eq!(event.position, SnackbarPosition::BottomCenter);
    }

    #[test]
    fn test_show_snackbar_message() {
        let event = ShowSnackbar::message("Simple message");
        assert_eq!(event.message, "Simple message");
        assert!(event.action.is_none());
        assert!(event.dismissible);
    }

    #[test]
    fn test_snackbar_builder() {
        let _builder = SnackbarBuilder::new("Test message")
            .action("Undo")
            .duration(5.0);
    }

    #[test]
    fn test_snackbar_animation_states() {
        // Test the animation state enum values
        let entering = SnackbarAnimationState::Entering;
        let visible = SnackbarAnimationState::Visible;
        let exiting = SnackbarAnimationState::Exiting;
        
        assert_ne!(entering, visible);
        assert_ne!(visible, exiting);
    }

    #[test]
    fn test_snackbar_queue_default() {
        let queue = SnackbarQueue::default();
        assert!(queue.queue.is_empty());
        assert!(queue.active.is_none());
    }

    #[test]
    fn test_snackbar_max_width_constant() {
        // MD3 spec: max width for snackbar
        use std::hint::black_box;
        assert!(black_box(SNACKBAR_MAX_WIDTH) > black_box(200.0));
    }

    #[test]
    fn test_snackbar_close_button_marker() {
        // Verify the SnackbarCloseButton marker is a unit struct
        use bevy_material_ui::snackbar::SnackbarCloseButton;
        let _marker = SnackbarCloseButton;
        // If this compiles, the component exists
    }

    #[test]
    fn test_show_snackbar_builder_chain() {
        let event = ShowSnackbar::with_action("Network Error", "Retry")
            .duration(6.0)
            .position(SnackbarPosition::TopCenter)
            .dismissible(true);
        
        assert_eq!(event.message, "Network Error");
        assert_eq!(event.action, Some("Retry".to_string()));
        assert_eq!(event.duration, Some(6.0));
        assert_eq!(event.position, SnackbarPosition::TopCenter);
        assert!(event.dismissible);
    }

    #[test]
    fn test_snackbar_all_positions() {
        let positions = [
            SnackbarPosition::BottomCenter,
            SnackbarPosition::BottomLeft,
            SnackbarPosition::BottomRight,
            SnackbarPosition::TopCenter,
            SnackbarPosition::TopLeft,
            SnackbarPosition::TopRight,
        ];
        
        for pos in positions {
            let event = ShowSnackbar::message("Test").position(pos);
            assert_eq!(event.position, pos);
        }
    }
}

// ============================================================================
// Chip Tests
// ============================================================================

mod chip_tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let chip = MaterialChip::new("Test Chip");
        assert_eq!(chip.label, "Test Chip");
        assert_eq!(chip.variant, ChipVariant::Assist); // Default
        assert!(!chip.disabled);
        assert!(!chip.selected);
    }

    #[test]
    fn test_chip_variants() {
        let mut chip = MaterialChip::new("Test");
        
        chip.variant = ChipVariant::Assist;
        assert_eq!(chip.variant, ChipVariant::Assist);
        
        chip.variant = ChipVariant::Filter;
        assert_eq!(chip.variant, ChipVariant::Filter);
        
        chip.variant = ChipVariant::Input;
        assert_eq!(chip.variant, ChipVariant::Input);
        
        chip.variant = ChipVariant::Suggestion;
        assert_eq!(chip.variant, ChipVariant::Suggestion);
    }

    #[test]
    fn test_chip_selection() {
        let mut chip = MaterialChip::new("Toggle");
        assert!(!chip.selected);
        
        chip.selected = true;
        assert!(chip.selected);
        
        chip.selected = false;
        assert!(!chip.selected);
    }

    #[test]
    fn test_chip_builder() {
        let _builder = ChipBuilder::new("Test")
            .variant(ChipVariant::Filter)
            .selected(true);
    }

    #[test]
    fn test_chip_height_constant() {
        // MD3 spec: Chip height should be 32dp
        assert!((CHIP_HEIGHT - 32.0).abs() < 1.0);
    }
}

// ============================================================================
// App Bar Tests
// ============================================================================

mod app_bar_tests {
    use super::*;

    #[test]
    fn test_top_app_bar_creation() {
        let app_bar = TopAppBar::new("My App");
        assert_eq!(app_bar.title, "My App");
        assert_eq!(app_bar.variant, TopAppBarVariant::Small); // Default is Small
    }

    #[test]
    fn test_top_app_bar_variants() {
        let mut app_bar = TopAppBar::new("Test");
        
        app_bar.variant = TopAppBarVariant::CenterAligned;
        assert_eq!(app_bar.variant, TopAppBarVariant::CenterAligned);
        
        app_bar.variant = TopAppBarVariant::Small;
        assert_eq!(app_bar.variant, TopAppBarVariant::Small);
        
        app_bar.variant = TopAppBarVariant::Medium;
        assert_eq!(app_bar.variant, TopAppBarVariant::Medium);
        
        app_bar.variant = TopAppBarVariant::Large;
        assert_eq!(app_bar.variant, TopAppBarVariant::Large);
    }

    #[test]
    fn test_top_app_bar_height_constants() {
        // MD3 spec heights
        assert!((TOP_APP_BAR_HEIGHT_SMALL - 64.0).abs() < 1.0);
        assert!((TOP_APP_BAR_HEIGHT_MEDIUM - 112.0).abs() < 1.0);
        assert!((TOP_APP_BAR_HEIGHT_LARGE - 152.0).abs() < 1.0);
    }

    #[test]
    fn test_bottom_app_bar_height() {
        // MD3 spec: Bottom app bar should be 80dp
        assert!((BOTTOM_APP_BAR_HEIGHT - 80.0).abs() < 1.0);
    }

    #[test]
    fn test_top_app_bar_builder() {
        let _builder = TopAppBarBuilder::new("My App");
    }
}

// ============================================================================
// Badge Tests  
// ============================================================================

mod badge_tests {
    use super::*;

    #[test]
    fn test_badge_size_constants() {
        // MD3 spec: small (dot) is 6dp, large is 16dp
        assert!((BADGE_SIZE_SMALL - 6.0).abs() < 1.0);
        assert!((BADGE_SIZE_LARGE - 16.0).abs() < 1.0);
    }

    #[test]
    fn test_badge_creation_dot() {
        let badge = MaterialBadge::dot();
        assert!(badge.content.is_none());
        assert!(badge.visible);
    }

    #[test]
    fn test_badge_creation_count() {
        let badge = MaterialBadge::count(5);
        assert_eq!(badge.content, Some("5".to_string()));
        assert!(badge.visible);
    }

    #[test]
    fn test_badge_creation_text() {
        let badge = MaterialBadge::text("NEW");
        assert_eq!(badge.content, Some("NEW".to_string()));
        assert!(badge.visible);
    }

    #[test]
    fn test_badge_visibility() {
        let mut badge = MaterialBadge::dot();
        assert!(badge.visible);
        
        badge.visible = false;
        assert!(!badge.visible);
    }

    #[test]
    fn test_badge_update_count() {
        let mut badge = MaterialBadge::count(5);
        badge.set_count(10);
        assert_eq!(badge.content, Some("10".to_string()));
    }

    #[test]
    fn test_badge_count_max() {
        // Create a badge with max=99, so counts > 99 show as "99+"
        let mut badge = MaterialBadge::count(100);
        badge.max = 99;
        badge.set_count(150);
        assert_eq!(badge.content, Some("99+".to_string()));
    }

    #[test]
    fn test_badge_builder() {
        let _builder = BadgeBuilder::dot();
        let _builder2 = BadgeBuilder::count(5);
        let _builder3 = BadgeBuilder::text("New");
    }
}

// ============================================================================
// Tooltip Tests
// ============================================================================

mod tooltip_tests {
    use super::*;

    #[test]
    fn test_tooltip_trigger_creation() {
        let trigger = TooltipTrigger::new("Help text");
        assert_eq!(trigger.text, "Help text");
        assert_eq!(trigger.position, TooltipPosition::Top); // Default
        assert!((trigger.delay - TOOLTIP_DELAY_DEFAULT).abs() < 0.001);
    }

    #[test]
    fn test_tooltip_positions() {
        let top = TooltipTrigger::new("Test").top();
        let bottom = TooltipTrigger::new("Test").bottom();
        let left = TooltipTrigger::new("Test").left();
        let right = TooltipTrigger::new("Test").right();
        
        assert_eq!(top.position, TooltipPosition::Top);
        assert_eq!(bottom.position, TooltipPosition::Bottom);
        assert_eq!(left.position, TooltipPosition::Left);
        assert_eq!(right.position, TooltipPosition::Right);
    }

    #[test]
    fn test_tooltip_delay_constants() {
        use std::hint::black_box;
        assert!(black_box(TOOLTIP_DELAY_DEFAULT) > black_box(0.0));
        assert!(black_box(TOOLTIP_DELAY_SHORT) < black_box(TOOLTIP_DELAY_DEFAULT));
        assert!(black_box(TOOLTIP_DELAY_SHORT) > black_box(0.0));
    }

    #[test]
    fn test_tooltip_size_constants() {
        // MD3 spec: plain tooltip height 24dp
        assert!((TOOLTIP_HEIGHT_PLAIN - 24.0).abs() < 1.0);
        
        // Max width for plain tooltip
        use std::hint::black_box;
        let max_width = black_box(TOOLTIP_MAX_WIDTH);
        assert!(max_width > black_box(100.0) && max_width < black_box(300.0));
    }

    #[test]
    fn test_rich_tooltip() {
        let rich = RichTooltip::new("Description text")
            .with_title("Title")
            .with_action("Learn more");
        
        assert_eq!(rich.title, Some("Title".to_string()));
        assert_eq!(rich.supporting_text, "Description text");
        assert_eq!(rich.action, Some("Learn more".to_string()));
    }

    #[test]
    fn test_tooltip_variants() {
        // Plain is default
        let plain = TooltipTrigger::new("Plain");
        let rich = TooltipTrigger::new("Rich").rich();
        
        assert_eq!(plain.variant, TooltipVariant::Plain);
        assert_eq!(rich.variant, TooltipVariant::Rich);
    }

    #[test]
    fn test_tooltip_builder() {
        let builder = TooltipTriggerBuilder::new("Help")
            .position(TooltipPosition::Bottom)
            .delay(0.3);
        
        let trigger = builder.build();
        assert_eq!(trigger.text, "Help");
        assert_eq!(trigger.position, TooltipPosition::Bottom);
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn test_all_easing_functions_exported() {
        // This test verifies that all expected easing functions are exported
        let _: fn(f32) -> f32 = ease_standard;
        let _: fn(f32) -> f32 = ease_emphasized;
        let _: fn(f32) -> f32 = ease_standard_accelerate;
        let _: fn(f32) -> f32 = ease_standard_decelerate;
        let _: fn(f32) -> f32 = ease_emphasized_accelerate;
        let _: fn(f32) -> f32 = ease_emphasized_decelerate;
    }

    #[test]
    fn test_component_types_exported() {
        // Verify component types are accessible
        fn _check_chip(_: MaterialChip) {}
        fn _check_app_bar(_: TopAppBar) {}
        fn _check_badge(_: MaterialBadge) {}
        fn _check_tooltip(_: TooltipTrigger) {}
    }

    #[test]
    fn test_builder_types_exported() {
        // Verify builder types are accessible
        let _chip = ChipBuilder::new("test");
        let _app_bar = TopAppBarBuilder::new("test");
        let _badge = BadgeBuilder::dot();
        let _tooltip = TooltipTriggerBuilder::new("test");
        let _snackbar = SnackbarBuilder::new("test");
    }

    #[test]
    fn test_variant_types_exported() {
        // Verify variant enums are accessible
        let _: ChipVariant = ChipVariant::Assist;
        let _: TopAppBarVariant = TopAppBarVariant::Small;
        let _: TooltipVariant = TooltipVariant::Plain;
        let _: TooltipPosition = TooltipPosition::Top;
    }
}
