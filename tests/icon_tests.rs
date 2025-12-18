//! Tests for the Material Symbols Icon System

use bevy_material_ui::icons::{
    icon_by_name, IconGrade, IconOpticalSize, IconStyle, IconWeight, MaterialIcon, ICON_DELETE,
    ICON_HOME, ICON_SEARCH, ICON_SETTINGS,
};

/// Test creating icons from constructors
#[test]
fn test_icon_constructors() {
    let home = MaterialIcon::home();
    assert_eq!(home.codepoint, ICON_HOME);

    let settings = MaterialIcon::settings();
    assert_eq!(settings.codepoint, ICON_SETTINGS);

    let search = MaterialIcon::search();
    assert_eq!(search.codepoint, ICON_SEARCH);

    let delete = MaterialIcon::delete();
    assert_eq!(delete.codepoint, ICON_DELETE);
}

/// Test icon as_str returns valid character
#[test]
fn test_icon_as_str() {
    let icon = MaterialIcon::home();
    let s = icon.as_str();
    assert!(!s.is_empty());
    assert_eq!(s.chars().count(), 1);
}

/// Test icon from_name with various names
#[test]
fn test_icon_from_name() {
    // Standard names
    assert!(MaterialIcon::from_name("home").is_some());
    assert!(MaterialIcon::from_name("settings").is_some());
    assert!(MaterialIcon::from_name("search").is_some());

    // Alternative names
    assert!(MaterialIcon::from_name("gear").is_some()); // alias for settings
    assert!(MaterialIcon::from_name("trash").is_some()); // alias for delete
    assert!(MaterialIcon::from_name("plus").is_some()); // alias for add

    // Case insensitive
    assert!(MaterialIcon::from_name("HOME").is_some());
    assert!(MaterialIcon::from_name("Home").is_some());

    // Non-existent
    assert!(MaterialIcon::from_name("nonexistent_icon").is_none());
}

/// Test icon_by_name function
#[test]
fn test_icon_by_name() {
    assert_eq!(icon_by_name("home"), Some(ICON_HOME));
    assert_eq!(icon_by_name("settings"), Some(ICON_SETTINGS));
    assert_eq!(icon_by_name("nonexistent"), None);
}

/// Test icon style defaults
#[test]
fn test_icon_style_defaults() {
    let style = IconStyle::default();

    assert!(!style.filled, "Default should be outlined");
    assert_eq!(style.weight, IconWeight::Regular);
    assert_eq!(style.grade, IconGrade::Normal);
    assert_eq!(style.optical_size, IconOpticalSize::Default);
    assert!(style.color.is_none());
    assert!(style.size.is_none());
}

/// Test icon style presets
#[test]
fn test_icon_style_presets() {
    let filled = IconStyle::filled();
    assert!(filled.filled);

    let outlined = IconStyle::outlined();
    assert!(!outlined.filled);

    let small = IconStyle::small();
    assert_eq!(small.optical_size, IconOpticalSize::Small);

    let large = IconStyle::large();
    assert_eq!(large.optical_size, IconOpticalSize::Large);

    let bold = IconStyle::bold();
    assert_eq!(bold.weight, IconWeight::Bold);

    let light = IconStyle::light();
    assert_eq!(light.weight, IconWeight::Light);
}

/// Test icon style builder pattern
#[test]
fn test_icon_style_builder() {
    let style = IconStyle::outlined()
        .with_fill(true)
        .with_weight(IconWeight::Bold)
        .with_grade(IconGrade::High)
        .with_optical_size(IconOpticalSize::Large)
        .with_size(32.0);

    assert!(style.filled);
    assert_eq!(style.weight, IconWeight::Bold);
    assert_eq!(style.grade, IconGrade::High);
    assert_eq!(style.optical_size, IconOpticalSize::Large);
    assert_eq!(style.size, Some(32.0));
    assert_eq!(style.effective_size(), 32.0);
}

/// Test icon weight values
#[test]
fn test_icon_weight_values() {
    assert_eq!(IconWeight::Thin.value(), 100);
    assert_eq!(IconWeight::ExtraLight.value(), 200);
    assert_eq!(IconWeight::Light.value(), 300);
    assert_eq!(IconWeight::Regular.value(), 400);
    assert_eq!(IconWeight::Medium.value(), 500);
    assert_eq!(IconWeight::SemiBold.value(), 600);
    assert_eq!(IconWeight::Bold.value(), 700);
}

/// Test icon grade values
#[test]
fn test_icon_grade_values() {
    assert_eq!(IconGrade::Low.value(), -25);
    assert_eq!(IconGrade::Normal.value(), 0);
    assert_eq!(IconGrade::High.value(), 200);
}

/// Test icon optical size values
#[test]
fn test_icon_optical_size_values() {
    assert_eq!(IconOpticalSize::Small.value(), 20);
    assert_eq!(IconOpticalSize::Default.value(), 24);
    assert_eq!(IconOpticalSize::Large.value(), 40);
    assert_eq!(IconOpticalSize::ExtraLarge.value(), 48);

    assert_eq!(IconOpticalSize::Small.size_px(), 20.0);
    assert_eq!(IconOpticalSize::Default.size_px(), 24.0);
}

/// Test effective size calculation
#[test]
fn test_effective_size() {
    // Without custom size, uses optical size
    let style = IconStyle::default();
    assert_eq!(style.effective_size(), 24.0);

    let style = IconStyle::small();
    assert_eq!(style.effective_size(), 20.0);

    // With custom size, uses that
    let style = IconStyle::default().with_size(48.0);
    assert_eq!(style.effective_size(), 48.0);
}

/// Test fill value for font variation
#[test]
fn test_fill_value() {
    let outlined = IconStyle::outlined();
    assert_eq!(outlined.fill_value(), 0.0);

    let filled = IconStyle::filled();
    assert_eq!(filled.fill_value(), 1.0);
}

/// Test all navigation icons exist
#[test]
fn test_navigation_icons() {
    let icons = [
        MaterialIcon::home(),
        MaterialIcon::menu(),
        MaterialIcon::more_vert(),
        MaterialIcon::more_horiz(),
        MaterialIcon::arrow_back(),
        MaterialIcon::arrow_forward(),
        MaterialIcon::arrow_upward(),
        MaterialIcon::arrow_downward(),
        MaterialIcon::close(),
        MaterialIcon::check(),
        MaterialIcon::expand_more(),
        MaterialIcon::expand_less(),
        MaterialIcon::chevron_left(),
        MaterialIcon::chevron_right(),
    ];

    for icon in icons {
        assert!(icon.codepoint as u32 > 0, "Icon codepoint should be valid");
        assert!(!icon.as_str().is_empty(), "Icon string should not be empty");
    }
}

/// Test all action icons exist
#[test]
fn test_action_icons() {
    let icons = [
        MaterialIcon::add(),
        MaterialIcon::remove(),
        MaterialIcon::delete(),
        MaterialIcon::edit(),
        MaterialIcon::save(),
        MaterialIcon::search(),
        MaterialIcon::refresh(),
        MaterialIcon::settings(),
        MaterialIcon::help(),
        MaterialIcon::info(),
        MaterialIcon::share(),
        MaterialIcon::download(),
        MaterialIcon::upload(),
    ];

    for icon in icons {
        assert!(icon.codepoint as u32 > 0);
    }
}

/// Test game/D&D related icons
#[test]
fn test_game_icons() {
    let icons = [
        MaterialIcon::dice(),
        MaterialIcon::shield(),
        MaterialIcon::combat(),
        MaterialIcon::magic(),
        MaterialIcon::inventory(),
        MaterialIcon::book(),
        MaterialIcon::health(),
        MaterialIcon::strength(),
        MaterialIcon::speed(),
        MaterialIcon::mind(),
        MaterialIcon::lightbulb(),
    ];

    for icon in icons {
        assert!(
            icon.codepoint as u32 > 0,
            "Game icon should have valid codepoint"
        );
    }
}
