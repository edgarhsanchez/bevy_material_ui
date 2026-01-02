# Divider audit: bevy_material_ui vs MDC Divider

Goal: Keep divider behavior aligned with MDC’s divider expectations: thin rule, optional insets, and usable in both vertical and horizontal list/layout contexts.

Upstream reference:
- MDC docs: `material-components-android/docs/components/Divider.md`

## What MDC emphasizes

- A divider is a simple decorative separator.
- Inset behavior is expressed as **start/end insets** (`dividerInsetStart`, `dividerInsetEnd`).
- MDC also provides `MaterialDividerItemDecoration` which supports both `VERTICAL` and `HORIZONTAL` list orientations.

## Current state in this repo (`bevy_material_ui/src/divider.rs`)

- ✅ Supports **horizontal and vertical** dividers (`MaterialDivider::new()` and `MaterialDivider::vertical()`).
- ✅ Supports inset variants:
  - `FullWidth`
  - `Inset`
  - `MiddleInset`
- ✅ Insets are axis-aware:
  - Horizontal divider uses left/horizontal margins
  - Vertical divider uses top/vertical margins

## Recommended alignment tweaks

1) Prefer explicit orientation type over `vertical: bool`
- Replace the `vertical: bool` field with an enum, e.g. `DividerOrientation::{Horizontal, Vertical}`.
- This mirrors how we treat orientation elsewhere (slider/scroll).

2) Model insets as “start/end” rather than “left/top”
- Current mapping is reasonable (vertical: start→top).
- Document this mapping in code/docs so it’s unambiguous.

3) Consider allowing thickness override
- MDC treats thickness as a configurable attribute.
- We already have `DIVIDER_THICKNESS`; consider adding `with_thickness(f32)` to the builder.

ECS note:
- Divider is purely presentational; it should remain a simple bundle with no systems.

