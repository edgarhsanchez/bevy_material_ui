# Internationalization (i18n) Guide

This guide explains the internationalization architecture used in the `bevy_material_ui` showcase application.

## Overview

The showcase demonstrates a complete i18n implementation using the library's built-in `LocalizedText` component and supporting infrastructure for multi-language and multi-script font rendering.

## Architecture

### Core Components

1. **LocalizedText Component**: Automatically updates text when language changes
   - Created with a translation key and optional default text
   - Responds to language switching events
   - Integrates with Bevy's ECS system

2. **MaterialI18n Resource**: Manages translation files and current language
   - Loads `.mui_lang` JSON files from `assets/i18n/`
   - Provides translation lookup by key
   - Triggers update events on language change

3. **Font Management**: Multi-script font system for international text
   - `ShowcaseFont` resource with handles for Latin, CJK, and Hebrew scripts
   - `NeedsInternationalFont` marker component
   - Automatic font switching based on language

### Translation Files

Translation files are stored in `assets/i18n/` using the `.mui_lang` JSON format:

```
assets/i18n/
├── en-US.mui_lang
├── es-ES.mui_lang
├── fr-FR.mui_lang
├── de-DE.mui_lang
├── ja-JP.mui_lang
├── zh-CN.mui_lang
└── he-IL.mui_lang
```

Each file contains key-value pairs:
```json
{
    "showcase.buttons.elevated": "Elevated",
    "showcase.buttons.filled": "Filled",
    "showcase.checkboxes.option_1": "Option 1"
}
```

### Translation Key Naming Convention

Keys follow a hierarchical pattern:
```
showcase.{view}.{element}
```

Examples:
- `showcase.buttons.elevated` → Button label
- `showcase.checkboxes.option_1` → Checkbox option
- `showcase.sliders.continuous` → Slider label

## Usage Patterns

### Basic Usage: Spawn Text with LocalizedText

```rust
use bevy_material_ui::prelude::*;

// Spawn localized text
parent.spawn((
    Text::new(""),  // Empty initial text
    LocalizedText::new("app.welcome")
        .with_default("Welcome"),
    TextFont { font_size: 16.0, ..default() },
    TextColor(theme.on_surface),
));
```

**Key points:**
- Always use `Text::new("")` with empty string
- Provide translation key to `LocalizedText::new()`
- Use `.with_default()` for fallback text
- Text updates automatically when language changes

### With International Font Support

For text that requires non-Latin scripts (Chinese, Japanese, Arabic, Hebrew, etc.):

```rust
parent.spawn((
    Text::new(""),
    LocalizedText::new("app.welcome")
        .with_default("Welcome"),
    TextFont { font_size: 16.0, ..default() },
    TextColor(theme.on_surface),
    NeedsInternationalFont,  // ← Add this marker
));
```

The `NeedsInternationalFont` marker triggers automatic font switching:
- Latin script → Noto Sans
- CJK (Chinese/Japanese/Korean) → Noto Sans CJK
- Hebrew/Arabic → Noto Sans Hebrew

### Custom Component Spawn Functions

For complex components like checkboxes, switches, and radios, use custom spawn functions from `i18n_helpers.rs`:

```rust
use crate::showcase::i18n_helpers::*;

// Spawn checkbox with i18n label
spawn_checkbox_i18n(
    parent,
    theme,
    CheckboxState::Unchecked,
    "settings.accept_terms",    // Translation key
    "Accept terms",              // Default text
);

// Spawn switch with i18n label
spawn_switch_i18n(
    parent,
    theme,
    false,
    "settings.wifi",
    "Wi-Fi",
);
```

**Why custom spawn functions?**
Material components have complex hierarchies. The spawn functions:
1. Replicate the library's component structure
2. Add `LocalizedText` to label entities
3. Add `NeedsInternationalFont` marker
4. Maintain proper component relationships

### Manual Component Construction

For maximum control, build components manually:

```rust
parent.spawn(Node::default()).with_children(|row| {
    // Spawn the component
    row.spawn((
        MaterialCheckbox::new().with_state(CheckboxState::Checked),
        Button,
        // ... other button components
    )).with_children(|checkbox| {
        // Add component internals (state layer, box, icon)
        // ...
    });
    
    // Add localized label
    row.spawn((
        Text::new(""),
        LocalizedText::new("settings.remember_me")
            .with_default("Remember me"),
        TextFont { font_size: 14.0, ..default() },
        TextColor(theme.on_surface),
        NeedsInternationalFont,
    ));
});
```

### When Labels Can't Use LocalizedText

Some spawn APIs only accept `&str` labels. For these cases, spawn the label separately:

```rust
col.spawn(Node {
    flex_direction: FlexDirection::Column,
    row_gap: Val::Px(4.0),
    ..default()
})
.with_children(|container| {
    // Add localized label first
    container.spawn((
        Text::new(""),
        LocalizedText::new("showcase.sliders.continuous")
            .with_default("Continuous"),
        TextFont { font_size: 12.0, ..default() },
        TextColor(theme.on_surface_variant),
        NeedsInternationalFont,
    ));
    
    // Spawn component without label
    container.spawn_slider_with(
        theme,
        MaterialSlider::new(0.0, 100.0).with_value(40.0),
        None,  // No label here
    );
});
```

## Font System Details

### ShowcaseFont Resource

Defined in [showcase.rs](../examples/showcase/showcase.rs):

```rust
#[derive(Resource)]
struct ShowcaseFont {
    latin: Handle<Font>,
    cjk: Handle<Font>,
    hebrew: Handle<Font>,
}
```

Fonts are loaded asynchronously from assets:
- Latin: `fonts/NotoSans-Medium.ttf`
- CJK: `fonts/NotoSansCJKsc-Medium.otf`
- Hebrew: `fonts/NotoSansHebrew-Medium.ttf`

### Font Switching System

Two systems handle font updates:

1. **apply_international_font_system**: Initial font assignment
   - Runs when fonts finish loading
   - Applies appropriate font based on current language

2. **update_font_on_language_change_system**: Language change response
   - Runs when language changes
   - Updates all text with `NeedsInternationalFont` marker

Language → Font mapping:
```rust
match language.tag.as_str() {
    "ja-JP" | "zh-CN" => ShowcaseFont.cjk,
    "he-IL" => ShowcaseFont.hebrew,
    _ => ShowcaseFont.latin,
}
```

## Adding New Translations

### Step 1: Add Translation Keys

Add to all 7 language files in `assets/i18n/`:

```json
// en-US.mui_lang
"showcase.new_view.title": "New Feature",
"showcase.new_view.description": "Feature description",

// es-ES.mui_lang
"showcase.new_view.title": "Nueva Característica",
"showcase.new_view.description": "Descripción de la característica",

// ... and so on for all languages
```

### Step 2: Use in Code

```rust
parent.spawn((
    Text::new(""),
    LocalizedText::new("showcase.new_view.title")
        .with_default("New Feature"),
    TextFont { font_size: 20.0, ..default() },
    TextColor(theme.on_surface),
    NeedsInternationalFont,
));
```

### Step 3: Test All Languages

Switch languages in the showcase app to verify all translations appear correctly.

## Best Practices

1. **Always provide default text**: Ensures something displays if translation is missing
   ```rust
   LocalizedText::new("key").with_default("Default Text")
   ```

2. **Use empty Text::new("")**: Let LocalizedText populate the content
   ```rust
   Text::new(""),  // Good
   Text::new("Default Text"),  // Bad - will be replaced anyway
   ```

3. **Add NeedsInternationalFont for all UI text**: Ensures correct rendering in all languages
   ```rust
   NeedsInternationalFont,  // Always include this
   ```

4. **Follow key naming convention**: Use hierarchical, descriptive keys
   ```rust
   "showcase.view_name.element_name"  // Good
   "label1", "text2"  // Bad
   ```

5. **Keep translations in sync**: When adding a key, add to ALL language files

6. **Use i18n_helpers for complex components**: Don't manually replicate component hierarchies
   ```rust
   spawn_checkbox_i18n(...)  // Good
   // Manual spawning with LocalizedText  // More error-prone
   ```

7. **Test with multiple languages**: Especially CJK and RTL languages to verify fonts work

## Common Patterns

### Section Headers
```rust
spawn_section_header(parent, theme, "showcase.view.title", "Default Title");
```

### Simple Labels
```rust
parent.spawn((
    Text::new(""),
    LocalizedText::new("key").with_default("Label"),
    TextFont { font_size: 14.0, ..default() },
    TextColor(theme.on_surface),
    NeedsInternationalFont,
));
```

### Component Labels (Checkbox, Switch, Radio)
```rust
spawn_checkbox_i18n(parent, theme, state, "key", "Default");
spawn_switch_i18n(parent, theme, selected, "key", "Default");
spawn_radio_i18n(parent, theme, state, "key", "Default");
```

### Chip Labels
```rust
spawn_chip_i18n(parent, theme, chip_type, "key", "Default");
```

### Button Labels
```rust
spawn_extended_fab_i18n(parent, theme, icon, "key", "Default");
```

## Architecture Benefits

1. **Automatic updates**: Text changes instantly when language switches
2. **Type safety**: Translation keys are strings, but structure is documented
3. **Font flexibility**: Supports any script with appropriate fonts
4. **ECS integration**: Uses Bevy's component system naturally
5. **Minimal boilerplate**: `LocalizedText` handles complexity
6. **Fallback safety**: Default text ensures UI always displays something
7. **Scalable**: Easy to add new languages or translations

## Limitations and Future Improvements

### Current Limitations
- Translation keys are not validated at compile time
- No pluralization support in `.mui_lang` format
- Font selection is language-based, not script-based
- RTL layout not automatically handled (only font switching)

### Potential Improvements
- Compile-time key validation using macros
- Plural form support in translation files
- Script detection for automatic font selection
- RTL layout helpers for Arabic/Hebrew
- Translation coverage reporting tool
- Hot-reload for translation files during development

## See Also

- [showcase.rs](../examples/showcase/showcase.rs) - Font loading and systems
- [common.rs](../examples/showcase/common.rs) - `NeedsInternationalFont` marker
- [i18n_helpers.rs](../examples/showcase/i18n_helpers.rs) - Custom spawn functions
- [views/](../examples/showcase/views/) - Usage examples in each view
- [assets/i18n/](../assets/i18n/) - Translation files

## Questions?

If you have questions about internationalization or encounter issues:
1. Check the showcase examples for working patterns
2. Verify translation keys exist in all language files
3. Ensure `NeedsInternationalFont` marker is present
4. Check that fonts are loaded (`ShowcaseFont` resource exists)
