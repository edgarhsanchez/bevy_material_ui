# Slider audit: bevy_material_ui vs Material Components Android

Goal: Align the public API + behavior of `MaterialSlider` with MDC Android’s slider where it makes sense, while implementing behavior using Bevy ECS + UI coordinate conventions.

Upstream references:
- MDC slider overview: `material-components-android/docs/components/Slider.md`
- MDC implementation: `material-components-android/lib/java/com/google/android/material/slider/BaseSlider.java`

## What MDC exposes (high-signal knobs)

From `BaseSlider.java`:

- **Orientation** is first-class: `setOrientation(... Slider_android_orientation ...)`.
- **Label behavior** is first-class: `setLabelBehavior(... Slider_labelBehavior ...)` and supports multiple modes (`LABEL_FLOATING`, `LABEL_WITHIN_BOUNDS`, `LABEL_GONE`, `LABEL_VISIBLE`).
- **Tick visibility** is first-class: `tickVisibilityMode` and legacy `tickVisible` conversion.
- **Minimum accessible touch target** is first-class: `minTouchTargetSize`.

These are not “styling details”; they change layout, hit testing, and interaction semantics.

## Current state in this repo

`MaterialSlider` (see `bevy_material_ui/src/slider.rs`) already has:

- `SliderOrientation::{Horizontal, Vertical}` ✅ (matches MDC conceptually)
- `SliderDirection::{StartToEnd, EndToStart}` ✅ (useful; can map to “direction”/RTL-style expectations)
- `step: Option<f32>` / discrete support ✅ (maps to MDC `stepSize`)
- Label is only `show_label: bool` ⚠️ (too coarse vs MDC labelBehavior)
- Ticks: `show_ticks: bool` and `TickVisibility::{Always, WhenDragging, Never}` ⚠️ (doesn’t match MDC tickVisibilityMode semantics)
- No explicit `min_touch_target_size` / touch target enforcement ⚠️
- Does not implement a two-thumb range slider (MDC `RangeSlider`) ❌ (future)

## Recommended alignment changes (MDC-first)

### 1) Replace `show_label: bool` with an enum like MDC

- Add `SliderLabelBehavior` enum mirroring MDC modes:
  - `Floating`, `WithinBounds`, `Hidden`, `Visible`
- Keep a convenience method for common behavior (e.g., `.label_hidden()` / `.label_floating()`).

ECS note: this is stored on `MaterialSlider` as data; systems implement label node visibility/positioning.

### 2) Rework tick configuration to resemble MDC `tickVisibilityMode`

MDC’s tick behavior is not only “show while dragging”; it has auto modes.

- Replace `TickVisibility` with something closer to MDC:
  - `Hidden`
  - `AutoHide`
  - `AutoLimit`
  - (Optional) `Always` as a non-MDC extension if we find it useful

If we keep a non-MDC extension, document it as such.

### 3) Add `min_touch_target_size`

- Add `min_touch_target_size: f32` (logical px) on `MaterialSlider` with a sensible default.
- Ensure hit testing uses the larger of (thumb size, min touch target) without changing visuals.

This is one of the main UX/accessibility differences between “looks right” and “feels right”.

### 4) Keep vertical orientation (MDC-supported)

- Keep `SliderOrientation` and ensure all coordinate math follows Bevy conventions:
  - Compute in physical space when using `ComputedNode` geometry
  - Convert to logical when writing `Node` styles

### 5) RangeSlider (future)

MDC has `RangeSlider` (two thumbs).

- Treat this as a separate component (`MaterialRangeSlider`) or a generalized multi-thumb slider.
- ECS approach: store `values: Vec<f32>` and emit change events with (thumb index, new value).

## Implementation shape (ECS-first)

- Components:
  - `MaterialSlider` holds config + state
  - marker components for parts: track, active track, thumb, tick nodes
- Events:
  - `SliderChangeEvent { entity, value }` already exists; extend later for multi-thumb
- Systems:
  - input/interaction system: pointer → value (handles orientation, direction, clamping)
  - geometry system: value → thumb/active-track placement
  - style/theme system: apply theme colors, elevation, disabled states

