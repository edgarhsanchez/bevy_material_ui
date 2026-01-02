# i18n Quick Reference

Quick patterns for adding internationalization to your `bevy_material_ui` application.

## Basic Text

```rust
parent.spawn((
    Text::new(""),
    LocalizedText::new("app.welcome").with_default("Welcome"),
    TextFont { font_size: 16.0, ..default() },
    TextColor(theme.on_surface),
    NeedsInternationalFont,  // Always include for multi-language support
));
```

## Components with Labels

### Checkbox
```rust
use crate::showcase::i18n_helpers::*;

spawn_checkbox_i18n(
    parent,
    theme,
    CheckboxState::Unchecked,
    "settings.accept_terms",  // Translation key
    "Accept terms",            // Default text
);
```

### Switch
```rust
spawn_switch_i18n(
    parent,
    theme,
    false,              // Initial state
    "settings.wifi",    // Translation key
    "Wi-Fi",            // Default text
);
```

### Radio Button
```rust
spawn_radio_i18n(
    parent,
    theme,
    RadioState::Selected,
    "options.choice_a",
    "Choice A",
);
```

### Chip
```rust
spawn_chip_i18n(
    parent,
    theme,
    MaterialChip::filter(""),
    "filters.active",
    "Active",
);
```

### Extended FAB
```rust
spawn_extended_fab_i18n(
    parent,
    theme,
    MaterialIcon::Add,
    "actions.create",
    "Create",
);
```

## Manual Construction

For maximum control:

```rust
parent.spawn(Node::default()).with_children(|row| {
    // 1. Spawn component
    row.spawn((MaterialSwitch::new().selected(true), Button, ...))
       .with_children(|switch| { /* internals */ });
    
    // 2. Add localized label
    row.spawn((
        Text::new(""),
        LocalizedText::new("settings.bluetooth")
            .with_default("Bluetooth"),
        TextFont { font_size: 14.0, ..default() },
        TextColor(theme.on_surface),
        NeedsInternationalFont,
    ));
});
```

## Labels for Components Without i18n Support

When spawn functions only accept `&str`:

```rust
col.spawn(Node {
    flex_direction: FlexDirection::Column,
    row_gap: Val::Px(4.0),
    ..default()
})
.with_children(|container| {
    // Localized label
    container.spawn((
        Text::new(""),
        LocalizedText::new("app.label").with_default("Label"),
        TextFont { font_size: 12.0, ..default() },
        TextColor(theme.on_surface_variant),
        NeedsInternationalFont,
    ));
    
    // Component without label
    container.spawn_component(theme, config, None);
});
```

## Translation Files

Add to all `.mui_lang` files in `assets/i18n/`:

```json
{
    "app.welcome": "Welcome",
    "app.settings": "Settings",
    "settings.accept_terms": "Accept terms",
    "settings.wifi": "Wi-Fi"
}
```

**Translation key pattern:** `app.{category}.{element}`

## Checklist

When adding new text:
- [ ] Use `Text::new("")` with empty string
- [ ] Add `LocalizedText` with key and default
- [ ] Add `NeedsInternationalFont` marker
- [ ] Add translation key to ALL 7 language files
- [ ] Test with CJK (ja-JP or zh-CN) and Hebrew (he-IL)

## Font Selection

Automatic based on language:
- `en-US`, `es-ES`, `fr-FR`, `de-DE` → Noto Sans (Latin)
- `ja-JP`, `zh-CN` → Noto Sans CJK
- `he-IL` → Noto Sans Hebrew

## Common Mistakes

❌ **Don't:**
```rust
Text::new("Hardcoded text")  // Won't translate
LocalizedText::new("key")    // Missing default
// Missing NeedsInternationalFont
```

✅ **Do:**
```rust
(
    Text::new(""),
    LocalizedText::new("key").with_default("Default"),
    NeedsInternationalFont,
)
```

## See Full Guide

For complete documentation, see [INTERNATIONALIZATION.md](INTERNATIONALIZATION.md)
