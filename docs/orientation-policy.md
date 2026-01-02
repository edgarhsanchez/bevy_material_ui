# Orientation policy (MDC-first, Bevy/ECS implementation)

This project follows the reference **Material Components (MDC)** implementation as the source of truth for:

- Which components have first-class **orientation** (vertical vs horizontal)
- What the public **API surface** should look like (properties, naming, defaults)
- Behavioral semantics (dragging, snapping, accessibility expectations, etc.)

We use **Bevy** as the source of truth for:

- Rendering, layout, and coordinate spaces (`ComputedNode`, `UiScale`, `UiGlobalTransform`)
- ECS-first architecture (data in Components/Resources, behavior in Systems)
- Input/picking integration and event handling

## Orientation matrix

| Component | Vertical supported? | Why / upstream signal | Notes for this repo |
|---|---:|---|---|
| Slider | Yes | MDC slider supports `orientation` and has an explicit `SliderOrientation` API | Keep `SliderOrientation` and `SliderDirection` (direction/reversed). Consider adding range slider later (two thumbs) to match MDC `RangeSlider`. |
| Scroll container / scrollbars | Yes | Bevy has first-class scroll + scrollbars with explicit horizontal/vertical | Keep `ScrollDirection::{Vertical, Horizontal, Both}` and scrollbar tracks/thumbs for both axes. |
| Divider | Yes | Material supports both horizontal and vertical dividers in practice; MDC docs explicitly discuss vertical/horizontal for divider decorations | Keep `MaterialDivider.vertical()` and ensure insets apply on the correct axis. |
| Button group / segmented buttons | Yes | MDC `MaterialButtonGroup` / `MaterialButtonToggleGroup` are `LinearLayout`-based and mention using `VERTICAL` | If/when this repo has a “button group / segmented” component, give it an explicit orientation API. |
| Tabs | No (by default) | MDC tabs are a horizontal strip (`TabLayout`); docs don’t present a vertical orientation | Keep tabs horizontal-only; do not add a vertical mode unless a concrete use-case demands it. |
| Linear progress indicator | No (by default) | MDC `LinearProgressIndicator` models *direction* (LTR/RTL/start/end), not vertical/horizontal | Prefer a direction/reversal API over vertical. If vertical exists, treat it as non-MDC extension and consider removing to stay aligned. |
| Circular progress indicator | N/A | Circular | No orientation; only direction/rotation is animation detail. |

## ECS implementation guidelines (important)

When implementing MDC-style widgets, keep the design ECS-native:

- **Components are data only.** Store configuration + state as components (e.g., `MaterialSlider`, `ScrollContainer`).
- **Resources are cross-cutting state.** Theme, tracing toggles, global settings, cached handles. Example: `MaterialTheme` resource.
- **Systems own behavior.** Input handling, animation, and layout updates run in systems and should be orderable.
- **Events/messages for interaction.** Emit events like `SliderChangeEvent`, `TabChangeEvent` rather than calling callbacks.
- **Avoid borrow conflicts by design.** Use two-phase updates, `ParamSet`, or collect pending changes (as in `tabs.rs`).
- **Coordinate correctness.** Use Bevy’s conventions:
  - Compute geometry in *physical* space when driven by `ComputedNode` / render target scale.
  - Convert back to logical pixels when writing `Node` style (`Val::Px`) using `ComputedNode.inverse_scale_factor` where appropriate.

## Immediate alignment targets

1. Slider: ensure the public surface maps cleanly to MDC expectations (orientation + valueFrom/valueTo + step/ticks/label behavior). Range slider is a future enhancement.
2. Scroll: keep Bevy-native scroll semantics, but expose MDC-like configuration knobs at the component level.
3. Progress: align linear progress with MDC (direction/reversal rather than vertical orientation) unless we explicitly choose an extension.
