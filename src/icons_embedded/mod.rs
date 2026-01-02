mod generated;

use bevy::prelude::*;
use std::collections::HashMap;

pub use generated::{by_name, IconId, ALL};

/// Caches converted embedded icons as Bevy `Image`s.
///
/// The embedded icon pixel data is stored as RGBA8 (white + alpha) so it can be tinted
/// by setting `ImageNode.color`.
#[derive(Resource, Default)]
pub struct EmbeddedIconCache {
    images: HashMap<IconId, Handle<Image>>,
}

impl EmbeddedIconCache {
    pub fn image_handle(
        &mut self,
        images: &mut Assets<Image>,
        icon: IconId,
    ) -> Handle<Image> {
        if let Some(handle) = self.images.get(&icon) {
            return handle.clone();
        }

        let mut image = Image::new_fill(
            Extent3d {
                width: icon.width as u32,
                height: icon.height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            icon.rgba(),
            TextureFormat::Rgba8UnormSrgb,
        );

        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT;

        let handle = images.add(image);
        self.images.insert(icon, handle.clone());
        handle
    }
}

pub struct EmbeddedIconsPlugin;

impl Plugin for EmbeddedIconsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmbeddedIconCache>();
    }
}
