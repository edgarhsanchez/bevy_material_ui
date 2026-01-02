//! Material button group / toggle group (segmented buttons)
//!
//! This is an ECS-native analog to the reference MDC `MaterialButtonToggleGroup`.
//! The group is responsible for:
//! - Orientation (horizontal / vertical)
//! - Optional single-selection behavior
//! - Optional selection-required behavior
//! - Applying connected corner radii to child buttons

use bevy::prelude::*;

use crate::button::MaterialButton;
use crate::telemetry::{InsertTestIdIfExists, TelemetryConfig, TestId};
use crate::tokens::CornerRadius;

/// Plugin for button groups.
pub struct ButtonGroupPlugin;

impl Plugin for ButtonGroupPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }

        app.add_systems(
            Update,
            (
                button_group_layout_system,
                button_group_toggle_system,
                button_group_corner_radius_system,
                button_group_telemetry_system,
            ),
        );
    }
}

fn button_group_telemetry_system(
    mut commands: Commands,
    telemetry: Option<Res<TelemetryConfig>>,
    groups: Query<(&TestId, &Children), With<MaterialButtonGroup>>,
    buttons: Query<(), With<MaterialButton>>,
) {
    let Some(telemetry) = telemetry else {
        return;
    };
    if !telemetry.enabled {
        return;
    }

    for (group_id, children) in groups.iter() {
        let group_id = group_id.id();
        let mut index = 0usize;

        for child in children.iter() {
            if buttons.get(child).is_err() {
                continue;
            }

            commands.queue(InsertTestIdIfExists {
                entity: child,
                id: format!("{group_id}/button/{index}"),
            });
            index += 1;
        }
    }
}

/// Orientation for a button group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    /// Lay out buttons left-to-right.
    #[default]
    Horizontal,
    /// Lay out buttons top-to-bottom.
    Vertical,
}

/// Material button group (segmented buttons).
///
/// Intended to be used as a container entity whose direct children are `MaterialButton` entities.
#[derive(Component, Debug, Clone)]
pub struct MaterialButtonGroup {
    pub orientation: ButtonGroupOrientation,

    /// If true, only one button in the group can be checked at a time.
    pub single_selection: bool,

    /// If true, prevents the group from having zero checked buttons.
    pub selection_required: bool,

    /// Gap between buttons in pixels.
    ///
    /// Note: a typical segmented control uses `0.0` spacing.
    pub spacing: f32,
}

impl Default for MaterialButtonGroup {
    fn default() -> Self {
        Self {
            orientation: ButtonGroupOrientation::Horizontal,
            single_selection: false,
            selection_required: false,
            spacing: 0.0,
        }
    }
}

impl MaterialButtonGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = ButtonGroupOrientation::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.orientation = ButtonGroupOrientation::Horizontal;
        self
    }

    pub fn single_selection(mut self, enabled: bool) -> Self {
        self.single_selection = enabled;
        self
    }

    pub fn selection_required(mut self, required: bool) -> Self {
        self.selection_required = required;
        self
    }

    pub fn spacing(mut self, px: f32) -> Self {
        self.spacing = px;
        self
    }
}

/// Builder for creating button groups.
pub struct ButtonGroupBuilder {
    group: MaterialButtonGroup,
}

impl Default for ButtonGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonGroupBuilder {
    pub fn new() -> Self {
        Self {
            group: MaterialButtonGroup::default(),
        }
    }

    pub fn horizontal(mut self) -> Self {
        self.group.orientation = ButtonGroupOrientation::Horizontal;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.group.orientation = ButtonGroupOrientation::Vertical;
        self
    }

    pub fn single_selection(mut self, enabled: bool) -> Self {
        self.group.single_selection = enabled;
        self
    }

    pub fn selection_required(mut self, required: bool) -> Self {
        self.group.selection_required = required;
        self
    }

    pub fn spacing(mut self, px: f32) -> Self {
        self.group.spacing = px;
        self
    }

    pub fn build(self) -> MaterialButtonGroup {
        self.group
    }
}

fn button_group_layout_system(
    mut groups: Query<
        (&MaterialButtonGroup, &mut Node),
        Or<(Added<MaterialButtonGroup>, Changed<MaterialButtonGroup>)>,
    >,
) {
    for (group, mut node) in groups.iter_mut() {
        node.flex_direction = match group.orientation {
            ButtonGroupOrientation::Horizontal => FlexDirection::Row,
            ButtonGroupOrientation::Vertical => FlexDirection::Column,
        };

        // Apply spacing on the major axis.
        match group.orientation {
            ButtonGroupOrientation::Horizontal => {
                node.column_gap = Val::Px(group.spacing);
                node.row_gap = Val::Px(0.0);
            }
            ButtonGroupOrientation::Vertical => {
                node.row_gap = Val::Px(group.spacing);
                node.column_gap = Val::Px(0.0);
            }
        }
    }
}

fn button_group_toggle_system(
    groups: Query<&MaterialButtonGroup>,
    mut buttons: ParamSet<(
        Query<
            (Entity, &Interaction, &ChildOf),
            (Changed<Interaction>, With<Button>, With<MaterialButton>),
        >,
        Query<(Entity, &ChildOf, &MaterialButton), (With<Button>, With<MaterialButton>)>,
        Query<(Entity, &ChildOf, &mut MaterialButton), (With<Button>, With<MaterialButton>)>,
    )>,
) {
    // Collect plans first (no mutation), then apply in one pass.
    #[derive(Clone, Copy)]
    struct TogglePlan {
        group: Entity,
        clicked: Entity,
        clicked_set_checked: Option<bool>,
        uncheck_others: bool,
    }

    let mut plans: Vec<TogglePlan> = Vec::new();

    // First, collect pressed interactions without borrowing multiple ParamSet views at once.
    let pressed: Vec<(Entity, Entity)> = {
        let changed = buttons.p0();
        changed
            .iter()
            .filter_map(|(entity, interaction, parent)| {
                (*interaction == Interaction::Pressed).then_some((entity, parent.parent()))
            })
            .collect()
    };

    if pressed.is_empty() {
        return;
    }

    {
        let read_buttons = buttons.p1();
        for (clicked_entity, group_entity) in pressed.iter().copied() {
            let Ok(group) = groups.get(group_entity) else {
                continue;
            };

            let Ok((_, _, clicked_button)) = read_buttons.get(clicked_entity) else {
                continue;
            };

            if clicked_button.disabled || !clicked_button.checkable {
                continue;
            }

            // Count checked buttons in this group.
            let mut checked_count = 0usize;
            for (_, p, b) in read_buttons.iter() {
                if p.parent() == group_entity && b.checkable && b.checked {
                    checked_count += 1;
                }
            }

            let clicked_is_checked = clicked_button.checked;

            if group.single_selection {
                if clicked_is_checked {
                    // Clicking an already-checked button toggles it off only if selection isn't required.
                    if !group.selection_required {
                        plans.push(TogglePlan {
                            group: group_entity,
                            clicked: clicked_entity,
                            clicked_set_checked: Some(false),
                            uncheck_others: false,
                        });
                    }
                } else {
                    // Select clicked and unselect everyone else.
                    plans.push(TogglePlan {
                        group: group_entity,
                        clicked: clicked_entity,
                        clicked_set_checked: Some(true),
                        uncheck_others: true,
                    });
                }
            } else {
                // Multi-select.
                if clicked_is_checked {
                    // Prevent unselecting the last checked item when selection_required.
                    if !(group.selection_required && checked_count <= 1) {
                        plans.push(TogglePlan {
                            group: group_entity,
                            clicked: clicked_entity,
                            clicked_set_checked: Some(false),
                            uncheck_others: false,
                        });
                    }
                } else {
                    plans.push(TogglePlan {
                        group: group_entity,
                        clicked: clicked_entity,
                        clicked_set_checked: Some(true),
                        uncheck_others: false,
                    });
                }
            }
        }
    }

    if plans.is_empty() {
        return;
    }

    // Apply plans.
    let mut write_buttons = buttons.p2();
    for plan in plans {
        if plan.uncheck_others {
            for (entity, parent, mut button) in write_buttons.iter_mut() {
                if parent.parent() != plan.group {
                    continue;
                }
                if entity != plan.clicked && button.checkable {
                    button.checked = false;
                }
            }
        }

        if let Some(set_checked) = plan.clicked_set_checked {
            if let Ok((_, _, mut button)) = write_buttons.get_mut(plan.clicked) {
                button.checked = set_checked;
            }
        }
    }
}

fn button_group_corner_radius_system(
    mut commands: Commands,
    groups: Query<
        (&MaterialButtonGroup, &Children),
        Or<(
            Added<MaterialButtonGroup>,
            Changed<MaterialButtonGroup>,
            Changed<Children>,
        )>,
    >,
    buttons: Query<&MaterialButton>,
    mut radii: Query<&mut BorderRadius>,
) {
    for (group, children) in groups.iter() {
        let mut button_children: Vec<(Entity, f32)> = Vec::new();
        for child in children.iter() {
            if let Ok(button) = buttons.get(child) {
                if button.checkable {
                    let radius = button.corner_radius.unwrap_or(CornerRadius::FULL);
                    button_children.push((child, radius));
                }
            }
        }

        let count = button_children.len();
        if count == 0 {
            continue;
        }

        for (index, (entity, radius)) in button_children.iter().enumerate() {
            let border_radius = segment_border_radius(group.orientation, index, count, *radius);
            match radii.get_mut(*entity) {
                Ok(mut existing) => {
                    *existing = border_radius;
                }
                Err(_) => {
                    commands.entity(*entity).insert(border_radius);
                }
            }
        }
    }
}

fn segment_border_radius(
    orientation: ButtonGroupOrientation,
    index: usize,
    count: usize,
    radius_px: f32,
) -> BorderRadius {
    let r = Val::Px(radius_px);
    let z = Val::Px(0.0);

    if count <= 1 {
        return BorderRadius::all(r);
    }

    match orientation {
        ButtonGroupOrientation::Horizontal => {
            if index == 0 {
                BorderRadius {
                    top_left: r,
                    bottom_left: r,
                    top_right: z,
                    bottom_right: z,
                }
            } else if index + 1 == count {
                BorderRadius {
                    top_left: z,
                    bottom_left: z,
                    top_right: r,
                    bottom_right: r,
                }
            } else {
                BorderRadius::all(z)
            }
        }
        ButtonGroupOrientation::Vertical => {
            if index == 0 {
                BorderRadius {
                    top_left: r,
                    top_right: r,
                    bottom_left: z,
                    bottom_right: z,
                }
            } else if index + 1 == count {
                BorderRadius {
                    top_left: z,
                    top_right: z,
                    bottom_left: r,
                    bottom_right: r,
                }
            } else {
                BorderRadius::all(z)
            }
        }
    }
}
