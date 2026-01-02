# Demo Guide: Running with Full Logging and Telemetry

This guide explains how to run the bevy_material_ui demo examples with full logging and telemetry enabled for debugging, testing, and development purposes.

## Table of Contents
- [Environment Variables](#environment-variables)
- [Running Demos](#running-demos)
- [Available Demos](#available-demos)
- [Example Output](#example-output)
- [Troubleshooting](#troubleshooting)

## Environment Variables

### BEVY_TELEMETRY
Enables component telemetry for automated testing and debugging.

- **Purpose**: Adds stable TestId components to UI elements for test automation
- **Default**: Disabled (not set)
- **Enable**: Set to any value (e.g., `"1"`)

```powershell
# PowerShell (Windows)
$env:BEVY_TELEMETRY="1"

# Bash/Zsh (Linux/macOS)
export BEVY_TELEMETRY=1
```

### RUST_LOG
Controls Rust logging levels and filtering.

- **Purpose**: Configure log verbosity and filtering
- **Default**: Uses bevy_material_ui's default filter (info level)
- **Syntax**: `LEVEL[,MODULE=LEVEL...]`

#### Log Levels (least to most important):
- `trace` - Very detailed, noisy debugging
- `debug` - Helpful debugging information
- `info` - General informational messages (default)
- `warn` - Warnings about potential issues
- `error` - Errors that need attention

#### Common Configurations:

```powershell
# PowerShell Examples

# Full trace logging (VERY verbose)
$env:RUST_LOG="trace"

# Info level (recommended for normal use)
$env:RUST_LOG="info"

# Debug level for bevy_material_ui only, error for everything else
$env:RUST_LOG="error,bevy_material_ui=debug"

# Debug UI components, trace ECS systems, suppress wgpu noise
$env:RUST_LOG="info,bevy_material_ui=debug,bevy_ecs=trace,wgpu=error"

# Component-specific debugging
$env:RUST_LOG="info,bevy_material_ui::button=trace,bevy_material_ui::slider=debug"
```

```bash
# Bash/Zsh Examples

# Full trace logging
export RUST_LOG=trace

# Info level
export RUST_LOG=info

# Debug bevy_material_ui, error for wgpu
export RUST_LOG="error,bevy_material_ui=debug,wgpu=error"
```

## Running Demos

### Basic Run (No Special Logging)

```powershell
# PowerShell
cargo run --example button_demo

# Bash/Zsh
cargo run --example button_demo
```

### With Telemetry Only

```powershell
# PowerShell
$env:BEVY_TELEMETRY="1"; cargo run --example button_demo

# Bash/Zsh
BEVY_TELEMETRY=1 cargo run --example button_demo
```

### With Full Logging

```powershell
# PowerShell
$env:RUST_LOG="info"; cargo run --example button_demo

# Bash/Zsh
RUST_LOG=info cargo run --example button_demo
```

### With Telemetry AND Full Logging (Recommended for Development)

```powershell
# PowerShell
$env:BEVY_TELEMETRY="1"; $env:RUST_LOG="info,bevy_material_ui=debug,wgpu=error"; cargo run --example button_demo

# Bash/Zsh
BEVY_TELEMETRY=1 RUST_LOG="info,bevy_material_ui=debug,wgpu=error" cargo run --example button_demo
```

### Persistent Environment Variables

To avoid setting environment variables for each command:

```powershell
# PowerShell - Set for current session
$env:BEVY_TELEMETRY="1"
$env:RUST_LOG="info,bevy_material_ui=debug,wgpu=error"

# Now run any demo
cargo run --example button_demo
cargo run --example slider_demo
# etc...
```

```bash
# Bash/Zsh - Set for current session
export BEVY_TELEMETRY=1
export RUST_LOG="info,bevy_material_ui=debug,wgpu=error"

# Now run any demo
cargo run --example button_demo
cargo run --example slider_demo
# etc...
```

## Available Demos

All 27 component demos support telemetry and logging:

### Demos with Material Symbols Icons

The following demos properly demonstrate Material Symbols icons:
- `toolbar_demo` - Navigation and action icons in toolbars
- `search_demo` - Navigation icon in search bars  
- `icon_button_demo` - All icon button variants with Material Symbols
- `fab_demo` - FABs with icon integration
- `app_bar_demo` - App bars with navigation/action icons
- `all_icon_buttons` - Complete icon showcase (all embedded bitmaps)

## Scroll Container Usage

The `scroll_demo` and showcase examples demonstrate the simple scroll pattern. Creating a scrollable container requires **three components**:

### Required Components

```rust
.spawn((
    // 1. Configure scroll direction and behavior
    // Scrollbars spawn automatically when show_scrollbars=true (default)
    ScrollContainerBuilder::new()
        .vertical()           // or .horizontal() or .both()
        .with_scrollbars(true) // default, can omit
        .build(),
    
    // 2. Track scroll position
    ScrollPosition::default(),
    
    // 3. Define scrollable area with overflow and size
    Node {
        width: Val::Px(400.0),
        height: Val::Px(300.0),
        overflow: Overflow::scroll(), // Required! Both axes must be Scroll
        // ... other styling
    },
))
.with_children(|parent| {
    // Your scrollable content here
});
```

### What Happens Automatically

The `ScrollPlugin` automatically:
- ✅ Creates an internal `ScrollContent` wrapper node
- ✅ Spawns scrollbars (track + thumb) when `show_scrollbars=true`
- ✅ Syncs scroll position between components
- ✅ Handles mouse wheel scrolling
- ✅ Handles scrollbar thumb dragging

**No manual `spawn_scrollbars()` call needed!**

### Common Patterns

**Important:** Bevy's scroll system requires `overflow: Overflow::scroll()` (both axes) on the ScrollContent node, even when scrolling in only one direction. The `ScrollContainer.direction` field controls which direction actually scrolls.

**Vertical scroll (most common):**
```rust
ScrollContainerBuilder::new().vertical().build()
Node { overflow: Overflow::scroll(), ... }  // Not scroll_y()!
```

**Horizontal scroll:**
```rust
ScrollContainerBuilder::new().horizontal().build()
Node { overflow: Overflow::scroll(), ... }  // Not scroll_x()!
```

**Both directions:**
```rust
ScrollContainerBuilder::new().both().build()
Node { overflow: Overflow::scroll(), ... }
```

**Without visible scrollbars:**
```rust
ScrollContainerBuilder::new()
    .vertical()
    .with_scrollbars(false)
    .build()
```

### For Lists

Use `ListBuilder.build_scrollable()` which bundles the ScrollContainer:

```rust
.spawn(ListBuilder::new().build_scrollable())
.insert(Node {
    overflow: Overflow::scroll(),  // Both axes must be Scroll
    height: Val::Px(300.0),
    // ... other styling
})
.with_children(|list| {
    // List items here
});
```

See `scroll_demo.rs` and showcase examples for complete working code.

### Navigation & Layout
- `app_bar_demo` - Top and bottom app bars with navigation
- `toolbar_demo` - Toolbar with navigation icon and actions
- `list_demo` - Lists with items and dividers
- `card_demo` - Material cards with content
- `divider_demo` - Horizontal, inset, and vertical dividers
- `scroll_demo` - Scrollable content areas

### Buttons & Actions
- `button_demo` - All button variants (filled, outlined, text, etc.)
- `button_group_demo` - Grouped button collections
- `icon_button_demo` - Icon-only buttons (standard, filled, tonal, outlined)
- `fab_demo` - Floating Action Buttons (regular, small, extended)

### Input & Selection
- `textfield_demo` - Text input fields
- `switch_demo` - Toggle switches
- `checkbox_demo` - Checkboxes with states
- `radio_demo` - Radio button groups
- `slider_demo` - Value sliders
- `select_demo` - Dropdown selection
- `search_demo` - Search bars with optional navigation
- `datetime_picker_demo` - Date and time picker dialogs

### Chips & Badges
- `chip_demo` - All chip variants (assist, filter, input, suggestion) *Note: Uses unicode fallback chars*
- `badge_demo` - Dot, count, and text badges

### Feedback & Messaging
- `snackbar_demo` - Snackbar notifications with actions
- `tooltip_demo` - Tooltips on interactive elements
- `dialog_demo` - Modal dialogs
- `progress_demo` - Progress indicators (linear, circular)
- `loading_indicator_demo` - Loading states
- `menu_demo` - Context and dropdown menus

### Navigation
- `tabs_demo` - Tabbed navigation

## Example Output

### With Telemetry Enabled

When running with `BEVY_TELEMETRY=1`, you'll see:

```
2025-12-29T23:37:41.602070Z  INFO bevy_material_ui::telemetry: 📊 Telemetry enabled
```

This confirms that TestId components are being added to UI elements for test automation.

### With Full Logging

With `RUST_LOG=info,bevy_material_ui=debug`:

```
2025-12-29T23:37:40.239934Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA RTX A6000", ... }
2025-12-29T23:37:41.602070Z  INFO bevy_material_ui::telemetry: 📊 Telemetry enabled
2025-12-29T23:37:41.670988Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported
2025-12-29T23:37:41.685596Z  INFO bevy_winit::system: Creating new window button_demo (0v0)
```

### Component Event Logging

Some demos log component events when interacting with them:

```
# chip_demo.rs when clicking chips:
Chip clicked: Some("Assist Chip")
Chip deleted: Some("Deletable Input")

# datetime_picker_demo.rs when selecting dates:
Date/Time selected: 2025-01-15 13:30

# snackbar_demo.rs when clicking actions:
Snackbar action clicked!
```

## Troubleshooting

### Chip Icons Showing as Squares (chip_demo)

The chip component currently uses unicode fallback characters (★, ✓, ✕) instead of Material Symbols icons. These may not render correctly on all systems and appear as empty squares. This is a known limitation - chips will be updated to use Material Icons font in a future version.

**Workaround**: The chips are functional despite the visual issue. Other components (toolbar, search, icon buttons) use Material Symbols correctly.

### Too Much Output (wgpu/Vulkan Spam)

If you see excessive wgpu_hal or Vulkan layer messages:

```powershell
# Suppress wgpu noise while keeping material UI logs
$env:RUST_LOG="warn,bevy_material_ui=info,wgpu=error,wgpu_hal=error"
```

### Not Seeing Telemetry Message

Ensure both:
1. `BEVY_TELEMETRY` is set to any value
2. Log level allows INFO messages (`RUST_LOG` includes `info` or `debug`)

```powershell
# Verify environment variables
$env:BEVY_TELEMETRY
$env:RUST_LOG
```

### Debugging Specific Components

To debug a specific component module:

```powershell
# Trace logging for button module only
$env:RUST_LOG="info,bevy_material_ui::button=trace"
```

### Color Output Issues

If ANSI color codes appear garbled in your terminal:

```powershell
# Disable color output
$env:NO_COLOR="1"
```

## Advanced Usage

### Chrome Tracing (Feature-Gated)

If compiled with `tracing-chrome` feature:

```powershell
$env:TRACE_CHROME="trace.json"
cargo run --features tracing-chrome --example button_demo
```

This generates a `trace.json` file that can be viewed in Chrome's `chrome://tracing`.

### Running All Demos

To test all demos sequentially with full logging:

```powershell
# PowerShell
$env:BEVY_TELEMETRY="1"
$env:RUST_LOG="info,bevy_material_ui=debug,wgpu=error"

$demos = @(
    "app_bar_demo", "badge_demo", "button_demo", "button_group_demo",
    "card_demo", "checkbox_demo", "chip_demo", "datetime_picker_demo",
    "dialog_demo", "divider_demo", "fab_demo", "icon_button_demo",
    "list_demo", "loading_indicator_demo", "menu_demo", "progress_demo",
    "radio_demo", "scroll_demo", "search_demo", "select_demo",
    "slider_demo", "snackbar_demo", "switch_demo", "tabs_demo",
    "textfield_demo", "toolbar_demo", "tooltip_demo"
)

foreach ($demo in $demos) {
    Write-Host "`n=== Running $demo ===`n" -ForegroundColor Cyan
    cargo run --example $demo
}
```

```bash
# Bash/Zsh
export BEVY_TELEMETRY=1
export RUST_LOG="info,bevy_material_ui=debug,wgpu=error"

demos=(
    app_bar_demo badge_demo button_demo button_group_demo
    card_demo checkbox_demo chip_demo datetime_picker_demo
    dialog_demo divider_demo fab_demo icon_button_demo
    list_demo loading_indicator_demo menu_demo progress_demo
    radio_demo scroll_demo search_demo select_demo
    slider_demo snackbar_demo switch_demo tabs_demo
    textfield_demo toolbar_demo tooltip_demo
)

for demo in "${demos[@]}"; do
    echo -e "\n=== Running $demo ===\n"
    cargo run --example "$demo"
done
```

## Quick Reference

| Use Case | Command (PowerShell) | Command (Bash/Zsh) |
|----------|---------------------|---------------------|
| Basic run | `cargo run --example <demo>` | `cargo run --example <demo>` |
| With telemetry | `$env:BEVY_TELEMETRY="1"; cargo run --example <demo>` | `BEVY_TELEMETRY=1 cargo run --example <demo>` |
| With logging | `$env:RUST_LOG="info"; cargo run --example <demo>` | `RUST_LOG=info cargo run --example <demo>` |
| Full debug | `$env:BEVY_TELEMETRY="1"; $env:RUST_LOG="info,bevy_material_ui=debug,wgpu=error"; cargo run --example <demo>` | `BEVY_TELEMETRY=1 RUST_LOG="info,bevy_material_ui=debug,wgpu=error" cargo run --example <demo>` |

---

**Last Updated**: December 29, 2025  
**Version**: bevy_material_ui v0.2.3

