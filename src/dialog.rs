//! Material Design 3 Dialog component
//!
//! Dialogs inform users about a task and can contain critical information.
//! This module leverages native `BoxShadow` for elevation shadows.
//!
//! Reference: <https://m3.material.io/components/dialogs/overview>

use bevy::picking::Pickable;
use bevy::prelude::*;
use bevy::ui::BoxShadow;

use crate::{
    elevation::Elevation,
    theme::MaterialTheme,
    tokens::{CornerRadius, Spacing},
};

/// Plugin for the dialog component
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DialogOpenEvent>()
            .add_message::<DialogCloseEvent>()
            .add_message::<DialogConfirmEvent>()
            .add_systems(
                Update,
                (
                    dialog_visibility_system,
                    dialog_scrim_visibility_system,
                    dialog_scrim_pickable_system,
                    dialog_shadow_system,
                ),
            );
    }
}

/// Dialog types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DialogType {
    /// Basic dialog with title and content
    #[default]
    Basic,
    /// Full-screen dialog
    FullScreen,
}

/// Material dialog component
#[derive(Component)]
pub struct MaterialDialog {
    /// Dialog type
    pub dialog_type: DialogType,
    /// Whether the dialog is currently open
    pub open: bool,
    /// Dialog title
    pub title: Option<String>,
    /// Dialog icon
    pub icon: Option<String>,
    /// Whether clicking the scrim closes the dialog
    pub dismiss_on_scrim_click: bool,
    /// Whether pressing Escape closes the dialog
    pub dismiss_on_escape: bool,

    /// Whether the dialog should behave as a modal (block pointer interactions behind it).
    ///
    /// When `true`, the dialog scrim will be pickable and will block lower entities from receiving
    /// pointer interactions. When `false`, the scrim will be click-through.
    pub modal: bool,
}

impl MaterialDialog {
    /// Create a new dialog
    pub fn new() -> Self {
        Self {
            dialog_type: DialogType::default(),
            open: false,
            title: None,
            icon: None,
            dismiss_on_scrim_click: true,
            dismiss_on_escape: true,
            modal: true,
        }
    }

    /// Set the dialog type
    pub fn with_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set initial open state
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Disable scrim click dismissal
    pub fn no_scrim_dismiss(mut self) -> Self {
        self.dismiss_on_scrim_click = false;
        self
    }

    /// Disable escape key dismissal
    pub fn no_escape_dismiss(mut self) -> Self {
        self.dismiss_on_escape = false;
        self
    }

    /// Set whether the dialog is modal (blocks pointer interactions behind it).
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Get the surface color
    pub fn surface_color(&self, theme: &MaterialTheme) -> Color {
        theme.surface_container_high
    }

    /// Get the scrim color
    pub fn scrim_color(&self, theme: &MaterialTheme) -> Color {
        theme.scrim.with_alpha(0.32)
    }

    /// Get the title color
    pub fn title_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_surface
    }

    /// Get the content color
    pub fn content_color(&self, theme: &MaterialTheme) -> Color {
        theme.on_surface_variant
    }

    /// Get the icon color
    pub fn icon_color(&self, theme: &MaterialTheme) -> Color {
        theme.secondary
    }

    /// Get the elevation
    pub fn elevation(&self) -> Elevation {
        Elevation::Level3
    }
}

impl Default for MaterialDialog {
    fn default() -> Self {
        Self::new()
    }
}

/// Event to open a dialog
#[derive(Event, bevy::prelude::Message)]
pub struct DialogOpenEvent {
    pub entity: Entity,
}

/// Event when dialog is closed
#[derive(Event, bevy::prelude::Message)]
pub struct DialogCloseEvent {
    pub entity: Entity,
    /// Whether it was dismissed (scrim/escape) vs confirmed
    pub dismissed: bool,
}

/// Event when dialog action is confirmed
#[derive(Event, bevy::prelude::Message)]
pub struct DialogConfirmEvent {
    pub entity: Entity,
}

/// Dialog dimensions
pub const DIALOG_MIN_WIDTH: f32 = 280.0;
pub const DIALOG_MAX_WIDTH: f32 = 560.0;

/// System to handle dialog visibility
fn dialog_visibility_system(
    mut dialogs: Query<(&MaterialDialog, &mut Node), Changed<MaterialDialog>>,
) {
    for (dialog, mut node) in dialogs.iter_mut() {
        node.display = if dialog.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

/// System to update dialog shadows using native BoxShadow
fn dialog_shadow_system(
    mut dialogs: Query<(&MaterialDialog, &mut BoxShadow), Changed<MaterialDialog>>,
) {
    for (dialog, mut shadow) in dialogs.iter_mut() {
        // Only show shadow when dialog is open
        if dialog.open {
            *shadow = dialog.elevation().to_box_shadow();
        } else {
            *shadow = BoxShadow::default();
        }
    }
}

/// Keep dialog scrims in sync with their dialog's open state.
fn dialog_scrim_visibility_system(
    dialogs: Query<&MaterialDialog>,
    mut scrims: Query<(&DialogScrimFor, &mut Node), With<DialogScrim>>,
) {
    for (for_dialog, mut node) in scrims.iter_mut() {
        let open = dialogs.get(for_dialog.0).map(|d| d.open).unwrap_or(false);
        node.display = if open { Display::Flex } else { Display::None };
    }
}

/// Update scrim pickability when dialog modality changes.
fn dialog_scrim_pickable_system(
    changed_dialogs: Query<(Entity, &MaterialDialog), Changed<MaterialDialog>>,
    mut scrims: Query<(&DialogScrimFor, &mut Pickable), With<DialogScrim>>,
) {
    if changed_dialogs.is_empty() {
        return;
    }

    for (dialog_entity, dialog) in changed_dialogs.iter() {
        for (for_dialog, mut pickable) in scrims.iter_mut() {
            if for_dialog.0 != dialog_entity {
                continue;
            }

            *pickable = if dialog.modal {
                Pickable {
                    should_block_lower: true,
                    is_hoverable: false,
                }
            } else {
                Pickable::IGNORE
            };
        }
    }
}

/// Builder for dialogs
pub struct DialogBuilder {
    dialog: MaterialDialog,
}

impl DialogBuilder {
    /// Create a new dialog builder
    pub fn new() -> Self {
        Self {
            dialog: MaterialDialog::new(),
        }
    }

    /// Set dialog type
    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog.dialog_type = dialog_type;
        self
    }

    /// Make full-screen dialog
    pub fn full_screen(self) -> Self {
        self.dialog_type(DialogType::FullScreen)
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.dialog.title = Some(title.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.dialog.icon = Some(icon.into());
        self
    }

    /// Start open
    pub fn open(mut self) -> Self {
        self.dialog.open = true;
        self
    }

    /// Disable scrim dismissal
    pub fn no_scrim_dismiss(mut self) -> Self {
        self.dialog.dismiss_on_scrim_click = false;
        self
    }

    /// Disable escape dismissal
    pub fn no_escape_dismiss(mut self) -> Self {
        self.dialog.dismiss_on_escape = false;
        self
    }

    /// Set whether the dialog is modal (blocks pointer interactions behind it).
    pub fn modal(mut self, modal: bool) -> Self {
        self.dialog.modal = modal;
        self
    }

    /// Build the dialog bundle with native BoxShadow
    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let bg_color = self.dialog.surface_color(theme);
        let is_full_screen = self.dialog.dialog_type == DialogType::FullScreen;

        (
            self.dialog,
            Node {
                display: Display::None, // Hidden by default
                position_type: PositionType::Absolute,
                width: if is_full_screen {
                    Val::Percent(100.0)
                } else {
                    Val::Auto
                },
                height: if is_full_screen {
                    Val::Percent(100.0)
                } else {
                    Val::Auto
                },
                min_width: if is_full_screen {
                    Val::Auto
                } else {
                    Val::Px(DIALOG_MIN_WIDTH)
                },
                max_width: if is_full_screen {
                    Val::Auto
                } else {
                    Val::Px(DIALOG_MAX_WIDTH)
                },
                padding: UiRect::all(Val::Px(Spacing::EXTRA_LARGE)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderRadius::all(Val::Px(if is_full_screen {
                0.0
            } else {
                CornerRadius::EXTRA_LARGE
            })),
            // Native Bevy 0.17 shadow support (starts hidden since dialog is closed)
            BoxShadow::default(),
        )
    }
}

impl Default for DialogBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Marker for dialog scrim overlay
#[derive(Component)]
pub struct DialogScrim;

/// Associates a dialog scrim with a specific dialog entity.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogScrimFor(pub Entity);

/// Marker for dialog headline/title
#[derive(Component)]
pub struct DialogHeadline;

/// Marker for dialog content area
#[derive(Component)]
pub struct DialogContent;

/// Marker for dialog actions area
#[derive(Component)]
pub struct DialogActions;

/// Helper to create a dialog scrim
pub fn create_dialog_scrim(theme: &MaterialTheme) -> impl Bundle {
    (
        DialogScrim,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.scrim.with_alpha(0.32)),
        // Default scrim behavior is modal: block pointer interactions behind it.
        Pickable {
            should_block_lower: true,
            is_hoverable: false,
        },
    )
}

/// Helper to create a dialog scrim linked to a specific dialog entity.
///
/// The scrim starts hidden and is shown/hidden automatically based on the dialog's `open` state.
/// When `modal` is true, the scrim blocks pointer interactions behind it.
pub fn create_dialog_scrim_for(
    theme: &MaterialTheme,
    dialog_entity: Entity,
    modal: bool,
) -> impl Bundle {
    (
        DialogScrim,
        DialogScrimFor(dialog_entity),
        Node {
            display: Display::None, // Hidden by default; synced by system.
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.scrim.with_alpha(0.32)),
        if modal {
            Pickable {
                should_block_lower: true,
                is_hoverable: false,
            }
        } else {
            Pickable::IGNORE
        },
        GlobalZIndex(1000),
    )
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

/// Extension trait to spawn Material dialogs as children
///
/// This trait provides a clean API for spawning dialogs within UI hierarchies.
///
/// ## Example:
/// ```ignore
/// parent.spawn(Node::default()).with_children(|children| {
///     children.spawn_dialog(&theme, "Confirm", |dialog| {
///         dialog.spawn((Text::new("Are you sure?"), TextColor(theme.on_surface)));
///     });
/// });
/// ```
pub trait SpawnDialogChild {
    /// Spawn a dialog with headline and content builder
    fn spawn_dialog(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a dialog with full builder control
    fn spawn_dialog_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DialogBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    );

    /// Spawn a dialog scrim overlay
    fn spawn_dialog_scrim(&mut self, theme: &MaterialTheme);
}

impl SpawnDialogChild for ChildSpawnerCommands<'_> {
    fn spawn_dialog(
        &mut self,
        theme: &MaterialTheme,
        headline: impl Into<String>,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        self.spawn_dialog_with(theme, DialogBuilder::new().title(headline), with_content);
    }

    fn spawn_dialog_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DialogBuilder,
        with_content: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        let title_text: Option<String> = builder.dialog.title.clone();
        let headline_color = theme.on_surface;

        self.spawn(builder.build(theme)).with_children(|dialog| {
            // Headline/Title
            if let Some(ref title) = title_text {
                dialog.spawn((
                    DialogHeadline,
                    Text::new(title.as_str()),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(headline_color),
                    Node {
                        margin: UiRect::bottom(Val::Px(16.0)),
                        ..default()
                    },
                ));
            }

            // Content area
            dialog
                .spawn((
                    DialogContent,
                    Node {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        ..default()
                    },
                ))
                .with_children(with_content);
        });
    }

    fn spawn_dialog_scrim(&mut self, theme: &MaterialTheme) {
        self.spawn(create_dialog_scrim(theme));
    }
}
