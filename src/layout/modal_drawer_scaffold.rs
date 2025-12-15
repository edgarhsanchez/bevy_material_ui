use bevy::prelude::*;

use crate::theme::MaterialTheme;

use super::{ScaffoldEntities, ScaffoldTestIds};

/// Configuration for a modal/standard drawer scaffold (drawer + content + scrim).
#[derive(Debug, Clone)]
pub struct ModalDrawerScaffold {
    pub drawer_width_px: f32,
    pub root_padding_px: f32,
    pub content_padding_px: f32,
    pub scrim_color: Color,
    pub test_ids: ScaffoldTestIds,
}

impl Default for ModalDrawerScaffold {
    fn default() -> Self {
        Self {
            drawer_width_px: 280.0,
            root_padding_px: 0.0,
            content_padding_px: 0.0,
            scrim_color: Color::srgba(0.0, 0.0, 0.0, 0.32),
            test_ids: ScaffoldTestIds::default(),
        }
    }
}

/// Spawn a modal drawer scaffold. The drawer is always shown in this helper; show/hide
/// behavior can be layered by toggling `Visibility` on the returned entities.
pub fn spawn_modal_drawer_scaffold(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    config: &ModalDrawerScaffold,
    drawer_children: impl FnOnce(&mut ChildSpawnerCommands),
    content_children: impl FnOnce(&mut ChildSpawnerCommands),
) -> ScaffoldEntities {
    let mut navigation = Entity::PLACEHOLDER;
    let mut content = Entity::PLACEHOLDER;

    let root = parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(config.root_padding_px)),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
            config.test_ids.root.clone(),
        ))
        .with_children(|root| {
            // Scrim overlay (full size); by default visible. Consumers can toggle visibility.
            root.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(config.scrim_color),
                // No TestId for scrim; automation does not need it.
            ));

            navigation = root
                .spawn((
                    Node {
                        width: Val::Px(config.drawer_width_px),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(16.0)),
                        row_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    config.test_ids.navigation.clone(),
                ))
                .with_children(drawer_children)
                .id();

            content = root
                .spawn((
                    Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(config.content_padding_px)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                    config.test_ids.content.clone(),
                ))
                .with_children(content_children)
                .id();
        })
        .id();

    ScaffoldEntities {
        root,
        navigation,
        content,
    }
}
