use bevy::prelude::*;
use bevy::ui::widget::ImageNode;

/// Renders a named SVG icon asset in UI.
///
/// The SVGs live in `assets/material_icons/` and are named `<icon_name>.svg`.
#[derive(Component, Clone, Copy, Debug)]
pub struct SvgIcon {
    pub name: &'static str,
    pub size: f32,
    pub color: Color,
}

impl SvgIcon {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            size: 20.0,
            color: Color::WHITE,
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub fn svg_icon_path(name: &str) -> String {
    format!("material_icons/{name}.svg")
}

pub fn svg_icon_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut icons: Query<
        (Entity, &SvgIcon, Option<&mut ImageNode>, Option<&mut Node>),
        Or<(Added<SvgIcon>, Changed<SvgIcon>)>,
    >,
) {
    for (entity, icon, image_node, node) in icons.iter_mut() {
        let image: Handle<Image> = asset_server.load(svg_icon_path(icon.name));

        if let Some(mut image_node) = image_node {
            image_node.image = image;
            image_node.color = icon.color;
        } else {
            commands
                .entity(entity)
                .insert(ImageNode::new(image).with_color(icon.color));
        }

        if let Some(mut node) = node {
            node.width = Val::Px(icon.size);
            node.height = Val::Px(icon.size);
        } else {
            commands.entity(entity).insert(Node {
                width: Val::Px(icon.size),
                height: Val::Px(icon.size),
                ..default()
            });
        }
    }
}
