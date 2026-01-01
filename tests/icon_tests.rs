//! Tests for the embedded Material icon system.

use bevy_material_ui::icons::{icon_by_name, material_icons, MaterialIcon, ICON_HOME};

#[test]
fn test_icon_table_nonempty() {
    assert!(!material_icons::ALL.is_empty());
}

#[test]
fn test_icon_by_name_case_insensitive() {
    let id_lower = icon_by_name("home");
    let id_upper = icon_by_name("HOME");
    let id_mixed = icon_by_name("Home");

    assert!(id_lower.is_some());
    assert_eq!(id_lower, id_upper);
    assert_eq!(id_lower, id_mixed);
}

#[test]
fn test_material_icon_from_name_matches_lookup() {
    let id = icon_by_name(ICON_HOME).expect("home icon should exist");
    let icon = MaterialIcon::from_name(ICON_HOME).expect("MaterialIcon::from_name should work");
    assert_eq!(icon.id, id);
}

#[test]
fn test_icon_pixel_blob_has_expected_size() {
    let (name, id) = material_icons::ALL[0];
    let _ = name;

    let alpha = id.alpha();
    let expected_len = (id.width as usize) * (id.height as usize);
    assert_eq!(alpha.len(), expected_len);
}
