# Toolbar

Toolbars provide a compact top row for navigation, title, and actions.

This crate includes a pragmatic MD3-style toolbar component (`MaterialToolbar`) that’s useful for game/desktop UIs where you want an always-on header row.

## Usage

```rust
use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::icons::{ICON_MENU, ICON_SEARCH};

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Camera2d);

    commands.spawn(Node::default()).with_children(|ui| {
        ui.spawn_toolbar_with(
            &theme,
            ToolbarBuilder::new("Inventory")
                .navigation_icon(MaterialIcon::new(ICON_MENU))
                .action(MaterialIcon::new(ICON_SEARCH), "search"),
        );
    });
}

fn handle_toolbar(
    mut nav: MessageReader<ToolbarNavigationEvent>,
    mut actions: MessageReader<ToolbarActionEvent>,
) {
    for _ev in nav.read() {
        // open menu
    }

    for ev in actions.read() {
        if ev.action == "search" {
            // do search
        }
    }
}
```

## Notes

- Icons are rendered as embedded bitmaps from the `google-material-design-icons-bin` crate included by `MaterialUiPlugin`.
- If you need scroll/collapse behavior, use the app bar components (`TopAppBar`/`BottomAppBar`) instead.
