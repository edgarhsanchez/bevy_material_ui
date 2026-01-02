# Scroll/scrollbars audit: bevy_material_ui vs Bevy UI patterns

Goal: Keep Bevy-native scrolling semantics (Overflow + ScrollPosition) while providing MDC-ish convenience and styling. Use Bevy for coordinate correctness and ECS architecture.

Upstream Bevy references:
- Bevy scroll example: `bevy/examples/ui/scroll.rs`
- Bevy scrollbars example: `bevy/examples/ui/scrollbars.rs`
- Bevy headless scrollbar widget: `bevy/crates/bevy_ui_widgets/src/scrollbar.rs`

## What Bevy does (high-signal patterns)

- Scrolling is driven by `ScrollPosition` on the scrollable node, with `Overflow::scroll_x()` / `Overflow::scroll_y()`.
- Bevy scrollbars are a **headless widget** (`Scrollbar`) with explicit `ControlOrientation::{Horizontal, Vertical}` and a `min_thumb_length`.
- Bevy’s widget uses picking pointer events (`Pointer<Press>`, `Pointer<DragStart>`, `Pointer<Drag>`, ...) rather than relying on `Interaction` state.
- All sizing math uses `ComputedNode.size()` / `content_size()` and converts via `inverse_scale_factor`.

## Current state in this repo (`bevy_material_ui/src/scroll.rs`)

Strengths:

- ✅ `ScrollDirection::{Vertical, Horizontal, Both}` is explicit and maps cleanly to Bevy overflow concepts.
- ✅ Mouse wheel scrolling walks up the parent chain to find a `ScrollContainer` ancestor (ECS-friendly; works when hovering children).
- ✅ Scrollbars are spawned automatically based on container settings and can be shown for vertical/horizontal/both.
- ✅ Uses `ComputedNode.inverse_scale_factor()` in several places to avoid logical/physical mixing.

Potential misalignments / issues to address:

1) **Duplicated state** (`ScrollContainer.offset/target_offset/max_offset/...` vs authoritative `ScrollPosition`)
- This is OK if we want smooth scrolling and additional metrics, but we should be explicit:
  - `ScrollPosition` is the authoritative scroll offset used by Bevy layout.
  - `ScrollContainer` caches derived values (content/container sizes, max offset) and optional smoothing state.

2) **Thumb sizing defaults are hard-coded**
- The min thumb size is currently `30.0` in multiple places.
- Bevy’s widget makes this a field (`min_thumb_length`).

Recommendation:
- Add `min_thumb_length: f32` to `ScrollContainer` (or a dedicated scrollbar config component) and use it consistently.

3) **Pointer/drag handling uses `Interaction` + mouse button**
- There is a comment noting that `Interaction::Pressed` can lag a frame.

Recommendation (Bevy-aligned):
- Switch scrollbar thumb dragging to picking events (like Bevy `bevy_ui_widgets`), or provide an internal option:
  - `On<Pointer<DragStart>>` to capture drag origin reliably
  - `On<Pointer<Drag>>` to apply scroll deltas
  - `On<Pointer<DragEnd | Cancel>>` to stop dragging

This makes the scrollbar behavior more deterministic and more “ECS-native” (events in, state updated).

4) **Scale-factor source during dragging should be per-node**

In `scrollbar_thumb_drag_system`, we often use `computed.inverse_scale_factor()` where `computed` comes from the scroll container, while `track_node.size()` and `thumb_node.size()` come from different entities.

Recommendation:
- When converting a node’s `ComputedNode.size()` to logical, use that node’s `inverse_scale_factor` (or a shared scale factor derived from render target info), to avoid subtle mismatches under unusual UI scaling setups.

5) **Debug logging is always-on**

The `SBAR Drag*` logs are unconditional.

Recommendation:
- Gate under a `ScrollTraceSettings` resource (like the slider trace settings), and throttle per-entity.

## ECS implementation guidance

- Components:
  - `ScrollContainer` remains the high-level config/state.
  - `ScrollContent` identifies the inner scroll node carrying `ScrollPosition`.
  - Track/thumb markers remain styling hooks.
- Systems:
  - Keep derived metrics updated from `ComputedNode` (content size, max offset) every frame.
  - Use event-based drag systems to avoid `Interaction` timing issues.
- Avoid deep mutable aliasing:
  - Prefer two-phase updates: compute new offsets, then apply to `ScrollPosition`.

