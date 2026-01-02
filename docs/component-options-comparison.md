# Material Design Component Options Comparison

This document compares the configurable options from the reference Material Components implementation (1.13.0) with our bevy_material_ui implementation.

Legend:
- ✅ = Implemented
- 🔄 = Partially implemented
- ❌ = Not implemented
- ➖ = Not applicable (platform-specific or N/A for Bevy)

---

## Button (`MaterialButton`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Variants** | - | `variant: ButtonVariant` | ✅ Filled, FilledTonal, Outlined, Elevated, Text |
| **Disabled** | `enabled` | `disabled: bool` | ✅ |
| **Label** | `text` | `label: String` | ✅ |
| **Icon** | `icon` | `icon: Option<String>` | ✅ |
| **Trailing Icon** | - | `trailing_icon: Option<String>` | ✅ |
| **Icon Gravity** | `iconGravity` (START, TEXT_START, END, TEXT_END, TOP, TEXT_TOP) | `icon_gravity: IconGravity` | ✅ |
| **Icon Padding** | `iconPadding` | `icon_padding: f32` | ✅ |
| **Icon Size** | `iconSize` | `icon_size: f32` | ✅ |
| **Icon Tint** | `iconTint` | `custom_text_color` (shared) | 🔄 Needs separate field |
| **Corner Radius** | `cornerRadius`, `shapeAppearance` | `corner_radius: Option<f32>` | ✅ |
| **Min Width** | `minWidth` | `min_width: Option<f32>` | ✅ |
| **Min Height** | `minHeight` | `min_height: Option<f32>` | ✅ |
| **Background Tint** | `backgroundTint` | `custom_background_color: Option<Color>` | ✅ |
| **Text Color** | `textColor` | `custom_text_color: Option<Color>` | ✅ |
| **Stroke Width** | `strokeWidth` | `stroke_width: f32` | ✅ |
| **Stroke Color** | `strokeColor` | `stroke_color: Option<Color>` | ✅ |
| **Checkable** | `checkable` | `checkable: bool` | ✅ |
| **Checked** | `checked` | `checked: bool` | ✅ |
| **Ripple Color** | `rippleColor` | - | ❌ Add `ripple_color: Option<Color>` |
| **Elevation** | `elevation` | - (uses variant default) | ❌ Add `elevation: Option<Elevation>` |
| **Insets** | `insetTop/Bottom/Left/Right` | - | ❌ Add margin/inset options |

### Missing Button Options to Add:
```rust
pub icon_tint: Option<Color>,       // Separate from text color
pub ripple_color: Option<Color>,    // Custom ripple color
pub elevation: Option<Elevation>,   // Custom elevation override
pub inset: Option<UiRect>,          // Insets/margins
```

---

## Slider (`MaterialSlider`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Value** | `value` | `value: f32` | ✅ |
| **Value From** | `valueFrom` | `min: f32` | ✅ |
| **Value To** | `valueTo` | `max: f32` | ✅ |
| **Step Size** | `stepSize` | `step: Option<f32>` | ✅ |
| **Disabled** | `enabled` | `disabled: bool` | ✅ |
| **Discrete Mode** | - | `discrete_value_count: Option<usize>` | ✅ |
| **Show Ticks** | `tickVisible`, `tickVisibilityMode` | `show_ticks: bool`, `tick_visibility: TickVisibility` | ✅ |
| **Show Label** | `labelBehavior` | `show_label: bool` | ✅ |
| **Anchor Value** | - | `anchor_value: Option<f32>` | ✅ |
| **Track Height** | `trackHeight` | `track_height: f32` | ✅ |
| **Thumb Radius** | `thumbRadius` | `thumb_radius: f32` | ✅ |
| **Thumb Elevation** | `thumbElevation` | `thumb_elevation: f32` | ✅ |
| **Thumb Width** | `thumbWidth` | - | ❌ |
| **Thumb Height** | `thumbHeight` | - | ❌ |
| **Thumb Color** | `thumbColor` | - | ❌ |
| **Thumb Stroke Color** | `thumbStrokeColor` | - | ❌ |
| **Thumb Stroke Width** | `thumbStrokeWidth` | - | ❌ |
| **Thumb-Track Gap** | `thumbTrackGapSize` | - | ❌ |
| **Halo Color** | `haloColor` | - | ❌ |
| **Halo Radius** | `haloRadius` | - | ❌ |
| **Track Color** | `trackColor` | - | ❌ |
| **Track Color Active** | `trackColorActive` | - | ❌ |
| **Track Color Inactive** | `trackColorInactive` | - | ❌ |
| **Track Corner Size** | `trackCornerSize` | - | ❌ |
| **Tick Color** | `tickColor` | - | ❌ |
| **Tick Color Active** | `tickColorActive` | - | ❌ |
| **Tick Color Inactive** | `tickColorInactive` | - | ❌ |
| **Tick Radius Active** | `tickRadiusActive` | - | ❌ |
| **Tick Radius Inactive** | `tickRadiusInactive` | - | ❌ |
| **Label Style** | `labelStyle` | - | ❌ |
| **Min Touch Target** | `minTouchTargetSize` | - | ❌ |

### Missing Slider Options to Add:
```rust
pub thumb_width: Option<f32>,
pub thumb_height: Option<f32>,
pub thumb_color: Option<Color>,
pub thumb_stroke_color: Option<Color>,
pub thumb_stroke_width: Option<f32>,
pub thumb_track_gap: f32,
pub halo_color: Option<Color>,
pub halo_radius: f32,
pub track_color: Option<Color>,
pub track_color_active: Option<Color>,
pub track_color_inactive: Option<Color>,
pub track_corner_radius: f32,
pub tick_color: Option<Color>,
pub tick_color_active: Option<Color>,
pub tick_color_inactive: Option<Color>,
pub tick_radius_active: f32,
pub tick_radius_inactive: f32,
```

---

## TextField (`MaterialTextField`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Variant** | `boxBackgroundMode` | `variant: TextFieldVariant` | ✅ Filled, Outlined |
| **Value** | - | `value: String` | ✅ |
| **Placeholder** | `placeholderText` | `placeholder: String` | ✅ |
| **Label/Hint** | `hint` | `label: Option<String>` | ✅ |
| **Hint Enabled** | `hintEnabled` | - (always enabled if label set) | 🔄 |
| **Hint Animation** | `hintAnimationEnabled` | `hint_animation_enabled: bool` | ✅ |
| **Helper Text** | `helperText` | `supporting_text: Option<String>` | ✅ |
| **Helper Text Enabled** | `helperTextEnabled` | - | ➖ |
| **Prefix Text** | `prefixText` | `prefix_text: Option<String>` | ✅ |
| **Suffix Text** | `suffixText` | `suffix_text: Option<String>` | ✅ |
| **Start Icon** | `startIconDrawable` | `leading_icon: Option<String>` | ✅ |
| **End Icon Mode** | `endIconMode` | `end_icon_mode: EndIconMode` | ✅ None, PasswordToggle, ClearText, DropdownMenu, Custom |
| **End Icon** | `endIconDrawable` | `trailing_icon: Option<String>` | ✅ |
| **Disabled** | `enabled` | `disabled: bool` | ✅ |
| **Error State** | `errorEnabled` | `error: bool` | ✅ |
| **Error Text** | `errorTextAppearance` | `error_text: Option<String>` | ✅ |
| **Counter Enabled** | `counterEnabled` | `counter_enabled: bool` | ✅ |
| **Counter Max Length** | `counterMaxLength` | `max_length: Option<usize>` | ✅ |
| **Box Stroke Width** | `boxStrokeWidth` | `box_stroke_width: f32` | ✅ |
| **Box Stroke Focused** | `boxStrokeWidthFocused` | `box_stroke_width_focused: f32` | ✅ |
| **Box Corner Radius** | `boxCornerRadiusTopStart/End/BottomStart/End` | `box_corner_radius: Option<f32>` | 🔄 Single value vs 4 corners |
| **Input Type** | `inputType` | `input_type: InputType` | ✅ |
| **Password Visible** | - | `password_visible: bool` | ✅ |
| **Hint Text Color** | `textColorHint` | - | ❌ |
| **Helper Text Color** | `helperTextTextColor` | - | ❌ |
| **Error Text Color** | `errorTextColor` | - | ❌ |
| **Prefix Text Color** | `prefixTextColor` | - | ❌ |
| **Suffix Text Color** | `suffixTextColor` | - | ❌ |
| **Box Stroke Color** | `boxStrokeColor` | - | ❌ |
| **Box Stroke Error Color** | `boxStrokeErrorColor` | - | ❌ |
| **Box Background Color** | `boxBackgroundColor` | - | ❌ |
| **Cursor Color** | `cursorColor` | - | ❌ |
| **Error Icon** | `errorIconDrawable` | - | ❌ |
| **Start Icon Tint** | `startIconTint` | - | ❌ |
| **End Icon Tint** | `endIconTint` | - | ❌ |
| **Start Icon Checkable** | `startIconCheckable` | - | ❌ |
| **End Icon Checkable** | `endIconCheckable` | - | ❌ |

### Missing TextField Options to Add:
```rust
pub hint_text_color: Option<Color>,
pub helper_text_color: Option<Color>,
pub error_text_color: Option<Color>,
pub prefix_text_color: Option<Color>,
pub suffix_text_color: Option<Color>,
pub box_stroke_color: Option<Color>,
pub box_stroke_error_color: Option<Color>,
pub box_background_color: Option<Color>,
pub cursor_color: Option<Color>,
pub error_icon: Option<String>,
pub start_icon_tint: Option<Color>,
pub end_icon_tint: Option<Color>,
pub box_corner_radius_top_start: Option<f32>,
pub box_corner_radius_top_end: Option<f32>,
pub box_corner_radius_bottom_start: Option<f32>,
pub box_corner_radius_bottom_end: Option<f32>,
```

---

## Chip (`MaterialChip`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Variant** | - | `variant: ChipVariant` | ✅ Assist, Filter, Input, Suggestion |
| **Label** | `text` | `label: String` | ✅ |
| **Value** | - | `value: Option<String>` | ✅ |
| **Selected** | - | `selected: bool` | ✅ |
| **Disabled** | - | `disabled: bool` | ✅ |
| **Deletable** | `closeIconVisible` | `deletable: bool` | ✅ |
| **Has Leading Icon** | `chipIconVisible` | `has_leading_icon: bool` | ✅ |
| **Elevation** | - | `elevation: ChipElevation` | ✅ |
| **Chip Background Color** | `chipBackgroundColor` | - | ❌ |
| **Chip Stroke Color** | `chipStrokeColor` | - | ❌ |
| **Chip Stroke Width** | `chipStrokeWidth` | - | ❌ |
| **Chip Corner Radius** | `chipCornerRadius` | - | ❌ |
| **Chip Min Height** | `chipMinHeight` | - | ❌ |
| **Ripple Color** | `rippleColor` | - | ❌ |
| **Chip Icon** | `chipIcon` | - | ❌ (just bool) |
| **Chip Icon Tint** | `chipIconTint` | - | ❌ |
| **Chip Icon Size** | `chipIconSize` | - | ❌ |
| **Close Icon** | `closeIcon` | - | ❌ |
| **Close Icon Tint** | `closeIconTint` | - | ❌ |
| **Close Icon Size** | `closeIconSize` | - | ❌ |
| **Checked Icon** | `checkedIcon` | - | ❌ |
| **Checked Icon Visible** | `checkedIconVisible` | - | ❌ |
| **Checkable** | `checkable` | - | ❌ |
| **Text Color** | `textColor` | - | ❌ |
| **Text Size** | `textSize` | - | ❌ |
| **Chip Start Padding** | `chipStartPadding` | - | ❌ |
| **Chip End Padding** | `chipEndPadding` | - | ❌ |
| **Icon Start Padding** | `iconStartPadding` | - | ❌ |
| **Icon End Padding** | `iconEndPadding` | - | ❌ |
| **Text Start Padding** | `textStartPadding` | - | ❌ |
| **Text End Padding** | `textEndPadding` | - | ❌ |
| **Close Icon Start Padding** | `closeIconStartPadding` | - | ❌ |
| **Close Icon End Padding** | `closeIconEndPadding` | - | ❌ |

### Missing Chip Options to Add:
```rust
pub chip_background_color: Option<Color>,
pub chip_stroke_color: Option<Color>,
pub chip_stroke_width: f32,
pub chip_corner_radius: Option<f32>,
pub chip_min_height: Option<f32>,
pub ripple_color: Option<Color>,
pub chip_icon: Option<String>,
pub chip_icon_tint: Option<Color>,
pub chip_icon_size: Option<f32>,
pub close_icon: Option<String>,
pub close_icon_tint: Option<Color>,
pub close_icon_size: Option<f32>,
pub checked_icon: Option<String>,
pub checked_icon_visible: bool,
pub checkable: bool,
pub text_color: Option<Color>,
pub text_size: Option<f32>,
pub chip_start_padding: f32,
pub chip_end_padding: f32,
pub icon_start_padding: f32,
pub icon_end_padding: f32,
pub text_start_padding: f32,
pub text_end_padding: f32,
pub close_icon_start_padding: f32,
pub close_icon_end_padding: f32,
```

---

## Snackbar (`ShowSnackbar`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Message** | `setText` | `message: String` | ✅ |
| **Action** | `setAction` | `action: Option<String>` | ✅ |
| **Duration** | `setDuration` (LENGTH_SHORT, LENGTH_LONG, LENGTH_INDEFINITE, custom) | `duration: Option<f32>` | ✅ |
| **Dismissible** | swipe behavior | `dismissible: bool` | ✅ |
| **Position** | anchor view | `position: SnackbarPosition` | ✅ BottomCenter, BottomLeft, BottomRight, TopCenter, TopLeft, TopRight |
| **Animation Mode** | `animationMode` (slide, fade) | - | ❌ |
| **Background Tint** | `backgroundTint` | - | ❌ |
| **Text Color** | `setTextColor` | - | ❌ |
| **Action Text Color** | `setActionTextColor` | - | ❌ |
| **Text Max Lines** | `setTextMaxLines` | - | ❌ |
| **Max Inline Action Width** | `maxActionInlineWidth` | - | ❌ |
| **Shape Appearance** | `shapeAppearance` | - | ❌ |
| **Elevation** | `elevation` | - (fixed) | ❌ |
| **Max Width** | `maxWidth` | - | ❌ |

### Missing Snackbar Options to Add:
```rust
pub animation_mode: SnackbarAnimationMode,  // Slide or Fade
pub background_tint: Option<Color>,
pub text_color: Option<Color>,
pub action_text_color: Option<Color>,
pub text_max_lines: u32,
pub max_inline_action_width: Option<f32>,
pub corner_radius: Option<f32>,
pub elevation: Option<Elevation>,
pub max_width: Option<f32>,
```

---

## Tooltip (`TooltipTrigger`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Text** | `text` | `text: String` | ✅ |
| **Variant** | - | `variant: TooltipVariant` | ✅ Plain, Rich |
| **Position** | - | `position: TooltipPosition` | ✅ Top, Bottom, Left, Right |
| **Delay** | - | `delay: f32` | ✅ |
| **Text Color** | `textColor` | - | ❌ |
| **Background Tint** | `backgroundTint` | - | ❌ |
| **Min Width** | `minWidth` | - | ❌ |
| **Min Height** | `minHeight` | - | ❌ |
| **Padding** | `padding` | - | ❌ |
| **Show Marker/Arrow** | `showMarker` | - | ❌ |

### Missing Tooltip Options to Add:
```rust
pub text_color: Option<Color>,
pub background_tint: Option<Color>,
pub min_width: Option<f32>,
pub min_height: Option<f32>,
pub padding: Option<f32>,
pub show_arrow: bool,
pub duration: Option<f32>,  // How long to show
```

---

## Checkbox (`MaterialCheckbox`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **State** | `checkedState` (unchecked, checked, indeterminate) | `state: CheckboxState` | ✅ |
| **Disabled** | - | `disabled: bool` | ✅ |
| **Error** | `errorShown` | `error: bool` | ✅ |
| **Button Tint** | `buttonTint` | - | ❌ |
| **Button Icon** | `buttonIcon` | - | ❌ |
| **Button Icon Tint** | `buttonIconTint` | - | ❌ |
| **Center If No Text** | `centerIfNoTextEnabled` | - | ❌ |
| **Error Accessibility Label** | `errorAccessibilityLabel` | - | ➖ |
| **Use Material Theme Colors** | `useMaterialThemeColors` | - | ➖ |

### Missing Checkbox Options to Add:
```rust
pub button_tint: Option<Color>,
pub checked_icon: Option<String>,
pub unchecked_icon: Option<String>,
pub indeterminate_icon: Option<String>,
pub icon_tint: Option<Color>,
pub size: f32,
```

---

## Switch (`MaterialSwitch`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Selected** | `checked` | `selected: bool` | ✅ |
| **Disabled** | - | `disabled: bool` | ✅ |
| **With Icon** | - | `with_icon: bool` | ✅ |
| **Thumb Icon** | `thumbIcon` | - | ❌ |
| **Thumb Icon Tint** | `thumbIconTint` | - | ❌ |
| **Thumb Icon Size** | `thumbIconSize` | - | ❌ |
| **Track Decoration** | `trackDecoration` | - | ❌ |
| **Track Decoration Tint** | `trackDecorationTint` | - | ❌ |
| **Thumb Color** | - | - | ❌ |
| **Track Color** | - | - | ❌ |

### Missing Switch Options to Add:
```rust
pub thumb_icon: Option<String>,
pub thumb_icon_tint: Option<Color>,
pub thumb_icon_size: Option<f32>,
pub thumb_color: Option<Color>,
pub track_color: Option<Color>,
pub track_decoration: Option<String>,
pub track_decoration_tint: Option<Color>,
```

---

## Radio (`MaterialRadio`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Selected** | `checked` | `selected: bool` | ✅ |
| **Disabled** | - | `disabled: bool` | ✅ |
| **Group** | - | `group: Option<String>` | ✅ |
| **Button Tint** | `buttonTint` | - | ❌ |
| **Use Material Theme Colors** | `useMaterialThemeColors` | - | ➖ |

### Missing Radio Options to Add:
```rust
pub button_tint: Option<Color>,
pub size: f32,
```

---

## FAB (`MaterialFab`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Size** | `fabSize` (auto, normal, mini), `fabCustomSize` | `size: FabSize` | ✅ Small, Regular, Large |
| **Color** | - | `color: FabColor` | ✅ Primary, Surface, Secondary, Tertiary |
| **Lowered** | - | `lowered: bool` | ✅ |
| **Icon** | - | `icon: String` | ✅ |
| **Extended Label** | - | `label: Option<String>` | ✅ |
| **Background Tint** | `backgroundTint` | - | ❌ |
| **Ripple Color** | `rippleColor` | - | ❌ |
| **Elevation** | `elevation` | - (uses default) | ❌ |
| **Border Width** | `borderWidth` | - | ❌ |
| **Use Compat Padding** | `useCompatPadding` | - | ➖ |
| **Max Image Size** | `maxImageSize` | - | ❌ |
| **Shape Appearance** | `shapeAppearance` | - | ❌ |
| **Hovered/Focused Translation Z** | `hoveredFocusedTranslationZ` | - | ❌ |
| **Pressed Translation Z** | `pressedTranslationZ` | - | ❌ |
| **Extend Strategy** | `extendStrategy` | - | ❌ |

### Missing FAB Options to Add:
```rust
pub background_tint: Option<Color>,
pub ripple_color: Option<Color>,
pub elevation: Option<Elevation>,
pub icon_tint: Option<Color>,
pub icon_size: Option<f32>,
pub custom_size: Option<f32>,
pub corner_radius: Option<f32>,
```

---

## Badge (`MaterialBadge`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Size** | - | `size: BadgeSize` | ✅ Small (dot), Large |
| **Content** | `number`, `badgeText` | `content: Option<String>` | ✅ |
| **Max** | `maxNumber` | `max: u32` | ✅ |
| **Visible** | - | `visible: bool` | ✅ |
| **Background Color** | `backgroundColor` | - | ❌ |
| **Text Color** | `badgeTextColor` | - | ❌ |
| **Badge Width** | `badgeWidth` | - | ❌ |
| **Badge Height** | `badgeHeight` | - | ❌ |
| **Badge With Text Width** | `badgeWithTextWidth` | - | ❌ |
| **Badge With Text Height** | `badgeWithTextHeight` | - | ❌ |
| **Badge Gravity** | `badgeGravity` (TOP_END, TOP_START, BOTTOM_END, BOTTOM_START) | - | ❌ |
| **Horizontal Offset** | `horizontalOffset` | - | ❌ |
| **Vertical Offset** | `verticalOffset` | - | ❌ |
| **Wide Padding** | `badgeWidePadding` | - | ❌ |
| **Vertical Padding** | `badgeVerticalPadding` | - | ❌ |
| **Shape Appearance** | `badgeShapeAppearance` | - | ❌ |

### Missing Badge Options to Add:
```rust
pub background_color: Option<Color>,
pub text_color: Option<Color>,
pub badge_width: Option<f32>,
pub badge_height: Option<f32>,
pub badge_gravity: BadgeGravity,
pub horizontal_offset: f32,
pub vertical_offset: f32,
pub corner_radius: Option<f32>,
```

---

## Dialog (`MaterialDialog`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Type** | - | `dialog_type: DialogType` | ✅ Basic, FullScreen |
| **Open** | - | `open: bool` | ✅ |
| **Title** | - | `title: Option<String>` | ✅ |
| **Icon** | - | `icon: Option<String>` | ✅ |
| **Dismiss on Scrim** | - | `dismiss_on_scrim_click: bool` | ✅ |
| **Dismiss on Escape** | - | `dismiss_on_escape: bool` | ✅ |
| **Background Tint** | `backgroundTint` | - | ❌ |
| **Background Insets** | `backgroundInsetStart/Top/End/Bottom` | - | ❌ |
| **Title Text Style** | `materialAlertDialogTitleTextStyle` | - | ❌ |
| **Body Text Style** | `materialAlertDialogBodyTextStyle` | - | ❌ |
| **Title Icon Style** | `materialAlertDialogTitleIconStyle` | - | ❌ |

### Missing Dialog Options to Add:
```rust
pub background_tint: Option<Color>,
pub background_insets: Option<UiRect>,
pub title_color: Option<Color>,
pub body_color: Option<Color>,
pub icon_tint: Option<Color>,
pub corner_radius: Option<f32>,
pub max_width: Option<f32>,
pub min_width: Option<f32>,
```

---

## Progress Indicator (`MaterialLinearProgress`, `MaterialCircularProgress`)

| Option | Reference Attribute | bevy_material_ui | Status |
|--------|-------------------|------------------|--------|
| **Progress** | - | `progress: f32` | ✅ |
| **Mode** | `indeterminate` | `mode: ProgressMode` | ✅ Determinate, Indeterminate |
| **Four Color** | - | `four_color: bool` | ✅ |
| **Size** (circular) | `indicatorSize` | `size: f32` | ✅ |
| **Track Thickness** | `trackThickness` | - | ❌ |
| **Track Corner Radius** | `trackCornerRadius` | - | ❌ |
| **Indicator Color** | `indicatorColor` | - | ❌ |
| **Track Color** | `trackColor` | - | ❌ |
| **Show Animation** | `showAnimationBehavior` | - | ❌ |
| **Hide Animation** | `hideAnimationBehavior` | - | ❌ |
| **Show Delay** | `showDelay` | - | ❌ |
| **Min Hide Delay** | `minHideDelay` | - | ❌ |
| **Indicator-Track Gap** | `indicatorTrackGapSize` | - | ❌ |
| **Indicator Direction (Linear)** | `indicatorDirectionLinear` | - | ❌ |
| **Indicator Direction (Circular)** | `indicatorDirectionCircular` | - | ❌ |
| **Track Stop Indicator Size** | `trackStopIndicatorSize` | - | ❌ |
| **Indeterminate Animation Type** | `indeterminateAnimationType` | - | ❌ |
| **Indeterminate Track Visible** | `indeterminateTrackVisible` | - | ❌ |

### Missing Progress Options to Add:
```rust
// LinearProgress
pub track_thickness: f32,
pub track_corner_radius: Option<f32>,
pub indicator_color: Option<Color>,
pub track_color: Option<Color>,
pub indicator_direction: LinearIndicatorDirection,
pub track_stop_indicator_size: Option<f32>,

// CircularProgress
pub track_thickness: f32,
pub indicator_color: Option<Color>,
pub track_color: Option<Color>,
pub indicator_direction: CircularIndicatorDirection,
pub indeterminate_track_visible: bool,
pub indicator_inset: f32,
```

---

## Summary

### Implementation Coverage by Component

| Component | Reference Options | Implemented | Coverage |
|-----------|-----------------|-------------|----------|
| Button | ~25 | 15 | 60% |
| Slider | ~35 | 12 | 34% |
| TextField | ~50 | 22 | 44% |
| Chip | ~35 | 8 | 23% |
| Snackbar | ~15 | 5 | 33% |
| Tooltip | ~10 | 4 | 40% |
| Checkbox | ~10 | 3 | 30% |
| Switch | ~10 | 3 | 30% |
| Radio | ~5 | 3 | 60% |
| FAB | ~20 | 5 | 25% |
| Badge | ~20 | 4 | 20% |
| Dialog | ~15 | 6 | 40% |
| Progress | ~25 | 4 | 16% |

### Priority for Enhancement

1. **High Priority** (core styling):
   - Color customization for all components (background, text, icon tints)
   - Ripple color customization
   - Corner radius per component
   - Stroke/border options

2. **Medium Priority** (advanced styling):
   - Size and dimension customization
   - Padding/margin options
   - Icon customization
   - Animation options

3. **Lower Priority** (specialized):
   - Platform-specific behaviors
   - Accessibility options
   - Advanced shape appearance
