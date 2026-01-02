# Migration Guide: v0.2.3 → v0.2.4

This guide helps you migrate your code from bevy_material_ui v0.2.3 to v0.2.4.

## Table of Contents

- [Breaking Changes](#breaking-changes)
  - [Text Field Spawn API](#text-field-spawn-api)
- [New Features](#new-features)
  - [CurrentDate Resource](#currentdate-resource)
  - [Internationalization](#internationalization)
- [Code Quality Improvements](#code-quality-improvements)

## Breaking Changes

### Text Field Spawn API

**What Changed**: The trait method `SpawnTextFieldChild::spawn_text_field_with` has been removed from the public API.

**Why**: This was an internal implementation detail that leaked into the public API. The standalone function provides cleaner, more flexible usage.

### Color Palette Generation

**What Changed**: Primary color palette generation now preserves highly chromatic seed colors instead of clamping all seeds to fixed chroma.

**Technical Details**:
- **Before**: `primary: TonalPalette::new(hue, 48.0)` - always used 48.0 chroma
- **After**: `primary: TonalPalette::new(hue, chroma.max(48.0))` - uses seed chroma if > 48.0

**Impact**: 
- Seeds with chroma ≤ 48: **No change** (still use 48.0)
- Seeds with chroma > 48: **More vibrant** primary colors that better match the seed

**Example**:
```rust
// A highly saturated seed color (chroma = 90)
let vivid_red = Color::srgb(1.0, 0.0, 0.0);
let palette = CorePalette::from_bevy_color(vivid_red);

// v0.2.3: Primary palette had chroma = 48.0 (desaturated from seed)
// v0.2.4: Primary palette has chroma = 90.0 (preserves seed vibrancy)
```

**Migration**: If your app relies on specific color values, test with your seed colors. Most apps won't notice the difference, but highly saturated seeds will produce more vibrant themes.

#### Migration Steps

**Before (v0.2.3):**
```rust
use bevy::prelude::*;
use bevy_material_ui::text_field::{SpawnTextFieldChild, TextFieldBuilder};
use bevy_material_ui::theme::MaterialTheme;

fn my_system(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Node::default()).with_children(|parent| {
        // Using trait method
        parent.spawn_text_field_with(
            &theme,
            TextFieldBuilder::default()
                .label("Username")
                .placeholder("Enter username")
        );
    });
}
```

**After (v0.2.4):**
```rust
use bevy::prelude::*;
use bevy_material_ui::text_field::{spawn_text_field_control_with, TextFieldBuilder};
use bevy_material_ui::theme::MaterialTheme;

fn my_system(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Node::default()).with_children(|parent| {
        // Using standalone function
        spawn_text_field_control_with(
            parent,
            &theme,
            TextFieldBuilder::default()
                .label("Username")
                .placeholder("Enter username")
        );
    });
}
```

**Key Differences:**
1. Import changes from `SpawnTextFieldChild` trait to `spawn_text_field_control_with` function
2. Call site changes from `parent.spawn_text_field_with(&theme, builder)` to `spawn_text_field_control_with(parent, &theme, builder)`
3. First parameter is now the parent commands/childbuilder

#### Quick Find & Replace

For simple migrations, you can use these patterns:

**Find:**
```rust
parent.spawn_text_field_with(
    &theme,
```

**Replace:**
```rust
spawn_text_field_control_with(
    parent,
    &theme,
```

**Don't forget to update imports:**
- Remove: `use bevy_material_ui::text_field::SpawnTextFieldChild;`
- Add: `use bevy_material_ui::text_field::spawn_text_field_control_with;`

## New Features

### CurrentDate Resource

**Added**: Production-ready date handling for date pickers

The date picker now supports a `CurrentDate` resource for providing the actual current date instead of the hardcoded placeholder.

**Usage:**

```rust
use bevy::prelude::*;
use bevy_material_ui::date_picker::{Date, CurrentDate};

fn setup_current_date(mut commands: Commands) {
    // Set the current date (e.g., from system time)
    commands.insert_resource(CurrentDate(Date::new(2026, 1, 2)));
}

// Optional: Update periodically
fn update_current_date(mut current: ResMut<CurrentDate>) {
    // Integrate with your date/time crate
    // Example with chrono:
    // let now = chrono::Local::now().naive_local().date();
    // current.0 = Date::new(now.year(), now.month() as u8, now.day() as u8);
}
```

**Benefits:**
- Date picker "today" highlighting uses actual current date
- Clear separation between placeholder and production date handling
- Easy integration with `chrono`, `time`, or other date crates

### Internationalization

**Added**: Comprehensive i18n support across all components

v0.2.4 adds full internationalization support with 7 languages and automatic font switching for different scripts.

**Quick Start:**

```rust
use bevy::prelude::*;
use bevy_material_ui::i18n::{MaterialI18n, MaterialLanguage};
use bevy_material_ui::text_field::LocalizedText;

fn setup_i18n(mut commands: Commands) {
    // Load translation files
    let i18n = MaterialI18n::load_from_dir("assets/translations")
        .expect("Failed to load translations");
    
    commands.insert_resource(i18n);
    commands.insert_resource(MaterialLanguage::new("en-US"));
}

fn spawn_localized_ui(mut commands: Commands) {
    commands.spawn(LocalizedText {
        key: "app.welcome".to_string(),
        fallback: Some("Welcome".to_string()),
    });
}
```

**Documentation:**
- Full guide: [docs/INTERNATIONALIZATION.md](INTERNATIONALIZATION.md)
- Quick reference: [docs/I18N_QUICK_REFERENCE.md](I18N_QUICK_REFERENCE.md)
- Implementation review: [docs/I18N_IMPLEMENTATION_REVIEW.md](I18N_IMPLEMENTATION_REVIEW.md)

## Code Quality Improvements

### Removed Internal Implementation Details

The following internal code has been removed with no impact on public API:

- **Legacy HCT implementation** (hct.rs, math.rs) - Now using external `hct-cam16` crate
- **Unused date validation** - `DateInputPattern::is_valid_complete_basic()` removed; use `Date::is_valid()`
- **Dead code cleanup** - Removed unused `CalendarPresenter` and `DatePickerDialog.picker`

### Named Constants

Magic numbers have been replaced with named constants:

- `MAX_ANCESTOR_DEPTH` constant (32) for entity traversal depth limits in scroll, list, and app bar components

**No action required** - these are internal improvements.

## Getting Help

If you encounter issues during migration:

1. Check the [Developer Guide](DEVELOPER_GUIDE.md)
2. Review component documentation in [docs/components/](components/)
3. Run the showcase example: `cargo run --example showcase`
4. Open an issue on [GitHub](https://github.com/edgarhsanchez/bevy_material_ui/issues)

## Changelog

See [CHANGELOG.md](../CHANGELOG.md) for the complete list of changes in v0.2.4.
