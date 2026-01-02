//! Embedded icon system.
//!
//! By default, icons come from the `google-material-design-icons-bin` crate as ALPHA8.
//! For compatibility, icons are expanded to RGBA8 (white + alpha) when building UI images.
//! UI tinting is applied via `ImageNode.color`.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::ui::widget::ImageNode;
use std::collections::HashMap;

fn icon_pixels_rgba8(id: material_icons::IconId) -> Vec<u8> {
    let alpha = id.alpha();
    let mut rgba = Vec::with_capacity(alpha.len() * 4);
    for a in alpha.iter().copied() {
        rgba.extend_from_slice(&[255, 255, 255, a]);
    }
    rgba
}

/// Generated icon table + embedded alpha-only bytes.
///
/// This mirrors the source folder layout as Rust modules (platform/category/icon).
pub use google_material_design_icons_bin::material_icons;

/// Lookup an embedded icon by its folder name (case-insensitive).
///
/// This is a small compatibility shim for older call sites that used an
/// `icon_by_name` helper.
pub use material_icons::by_name as icon_by_name;

// ---------------------------------------------------------------------------
// Common icon name constants
// ---------------------------------------------------------------------------
// These are folder names from the upstream icon repository.

pub const ICON_CLOSE: &str = "close";
pub const ICON_CHECK: &str = "check";
pub const ICON_REMOVE: &str = "remove";
pub const ICON_DELETE: &str = "delete";
pub const ICON_SEARCH: &str = "search";
pub const ICON_MENU: &str = "menu";
pub const ICON_HOME: &str = "home";
pub const ICON_SETTINGS: &str = "settings";
pub const ICON_FAVORITE: &str = "favorite";
pub const ICON_ADD: &str = "add";
pub const ICON_EDIT: &str = "edit";
pub const ICON_STAR: &str = "star";
pub const ICON_EMAIL: &str = "email";
pub const ICON_MORE_VERT: &str = "more_vert";
pub const ICON_NOTIFICATIONS: &str = "notifications";
pub const ICON_ARROW_BACK: &str = "arrow_back";
pub const ICON_EXPAND_MORE: &str = "expand_more";
pub const ICON_EXPAND_LESS: &str = "expand_less";

/// Backwards-compatible icon name constants.
///
/// Some widgets historically referenced Android-style resource identifiers.
/// These now map to embedded icon folder names.
pub mod material_icon_names {
    pub const IC_KEYBOARD_BLACK_24DP: &str = "keyboard";
    pub const IC_CLOCK_BLACK_24DP: &str = "schedule";

    pub const MATERIAL_IC_EDIT_BLACK_24DP: &str = "edit";
    pub const MATERIAL_IC_CALENDAR_BLACK_24DP: &str = "calendar_today";
    pub const MATERIAL_IC_MENU_ARROW_UP_BLACK_24DP: &str = "expand_less";
    pub const MATERIAL_IC_MENU_ARROW_DOWN_BLACK_24DP: &str = "expand_more";
    pub const MATERIAL_IC_KEYBOARD_ARROW_PREVIOUS_BLACK_24DP: &str = "chevron_left";
    pub const MATERIAL_IC_KEYBOARD_ARROW_NEXT_BLACK_24DP: &str = "chevron_right";
}

// ---------------------------------------------------------------------------
// Compatibility shims (style + svg)
// ---------------------------------------------------------------------------

/// Simple icon style component.
///
/// Older widgets in this crate used an `IconStyle` component to drive size and
/// color updates separately from the icon itself. In the embedded icon system,
/// the source-of-truth is `MaterialIcon { size, color }`, but we keep this
/// lightweight shim to avoid rewriting every widget at once.
#[derive(Component, Clone, Copy, Debug)]
pub struct IconStyle {
    pub size: f32,
    pub color: Color,
}

impl Default for IconStyle {
    fn default() -> Self {
        Self {
            size: 24.0,
            color: Color::WHITE,
        }
    }
}

impl IconStyle {
    pub fn outlined() -> Self {
        Self::default()
    }

    pub fn filled() -> Self {
        Self::default()
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

/// Minimal stand-in for the old SVG icon component.
///
/// This does *not* load SVGs; it simply maps a name to an embedded bitmap icon.
pub mod svg {
    use super::{icon_by_name, MaterialIcon};
    use bevy::prelude::*;

    #[derive(Component, Clone, Debug)]
    pub struct SvgIcon {
        pub name: String,
        pub size: f32,
        pub color: Color,
    }

    impl SvgIcon {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                size: 24.0,
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

    pub(super) fn svg_icon_sync_system(
        mut commands: Commands,
        mut icons: Query<
            (
                Entity,
                &SvgIcon,
                Option<&mut MaterialIcon>,
                Option<&mut Visibility>,
            ),
            Or<(Added<SvgIcon>, Changed<SvgIcon>)>,
        >,
    ) {
        for (entity, svg, material_icon, visibility) in icons.iter_mut() {
            let Some(id) = icon_by_name(&svg.name) else {
                if let Some(mut visibility) = visibility {
                    *visibility = Visibility::Hidden;
                } else {
                    commands.entity(entity).insert(Visibility::Hidden);
                }
                continue;
            };

            if let Some(mut material_icon) = material_icon {
                material_icon.id = id;
                material_icon.size = svg.size;
                material_icon.color = svg.color;
            } else {
                commands.entity(entity).insert(
                    MaterialIcon::new(id)
                        .with_size(svg.size)
                        .with_color(svg.color),
                );
            }

            if let Some(mut visibility) = visibility {
                *visibility = Visibility::Inherited;
            } else {
                commands.entity(entity).insert(Visibility::Inherited);
            }
        }
    }
}

#[derive(Resource, Default)]
struct MaterialIconImageCache(HashMap<material_icons::IconId, Handle<Image>>);

/// Icon component for rendering an embedded icon via `ImageNode`.
#[derive(Component, Clone, Copy, Debug)]
pub struct MaterialIcon {
    pub id: material_icons::IconId,
    pub size: f32,
    pub color: Color,
}

impl MaterialIcon {
    pub fn new(id: material_icons::IconId) -> Self {
        Self {
            id,
            size: 20.0,
            color: Color::WHITE,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        material_icons::by_name(name).map(Self::new)
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

/// Plugin that enables `MaterialIcon` rendering.
pub struct MaterialIconsPlugin;

impl Plugin for MaterialIconsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaterialIconImageCache>();
        app.add_systems(
            Update,
            (
                material_icon_system,
                material_icon_repair_system,
                icon_style_sync_system,
                svg::svg_icon_sync_system,
            ),
        );
    }
}

fn icon_style_sync_system(
    mut icons: Query<(&mut MaterialIcon, &IconStyle), Or<(Added<IconStyle>, Changed<IconStyle>)>>,
) {
    for (mut icon, style) in icons.iter_mut() {
        icon.size = style.size;
        icon.color = style.color;
    }
}

fn material_icon_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut cache: ResMut<MaterialIconImageCache>,
    mut icons: Query<
        (
            Entity,
            &MaterialIcon,
            Option<&mut ImageNode>,
            Option<&mut Node>,
        ),
        Or<(Added<MaterialIcon>, Changed<MaterialIcon>)>,
    >,
) {
    for (entity, icon, image_node, node) in icons.iter_mut() {
        let handle = if let Some(handle) = cache.0.get(&icon.id) {
            handle.clone()
        } else {
            let extent = Extent3d {
                width: icon.id.width as u32,
                height: icon.id.height as u32,
                depth_or_array_layers: 1,
            };
            let image = Image::new(
                extent,
                TextureDimension::D2,
                icon_pixels_rgba8(icon.id),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );
            let handle = images.add(image);
            cache.0.insert(icon.id, handle.clone());
            handle
        };

        if let Some(mut image_node) = image_node {
            image_node.image = handle;
            image_node.color = icon.color;
        } else {
            commands
                .entity(entity)
                .insert(ImageNode::new(handle).with_color(icon.color));
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

fn material_icon_repair_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut cache: ResMut<MaterialIconImageCache>,
    mut icons: Query<
        (
            Entity,
            &MaterialIcon,
            Option<&mut ImageNode>,
            Option<&mut Node>,
        ),
        Or<(Without<ImageNode>, Without<Node>)>,
    >,
) {
    for (entity, icon, image_node, node) in icons.iter_mut() {
        let handle = if let Some(handle) = cache.0.get(&icon.id) {
            handle.clone()
        } else {
            let extent = Extent3d {
                width: icon.id.width as u32,
                height: icon.id.height as u32,
                depth_or_array_layers: 1,
            };
            let image = Image::new(
                extent,
                TextureDimension::D2,
                icon_pixels_rgba8(icon.id),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );
            let handle = images.add(image);
            cache.0.insert(icon.id, handle.clone());
            handle
        };

        if let Some(mut image_node) = image_node {
            image_node.image = handle;
            image_node.color = icon.color;
        } else {
            commands
                .entity(entity)
                .insert(ImageNode::new(handle).with_color(icon.color));
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
