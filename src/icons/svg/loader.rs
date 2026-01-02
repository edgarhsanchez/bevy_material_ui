use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::tasks::ConditionalSendFuture;

use resvg::{tiny_skia, usvg};

#[derive(Default)]
pub struct SvgImageLoader;

impl AssetLoader for SvgImageLoader {
    type Asset = Image;
    type Settings = ();
    type Error = anyhow::Error;

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let svg_str = std::str::from_utf8(&bytes)?;

            let mut options = usvg::Options::default();
            // We don't need any external resources for these icons.
            options.resources_dir = None;

            let tree = usvg::Tree::from_str(svg_str, &options)?;
            let size = tree.size();

            // Rasterize at a high resolution then scale down in UI for crisp results.
            // These icons are typically authored at 24x24.
            let target_width: u32 = 256;
            let scale = target_width as f32 / size.width();
            let target_height: u32 = (size.height() * scale).round().max(1.0) as u32;

            let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height)
                .ok_or_else(|| anyhow::anyhow!("failed to allocate SVG pixmap"))?;

            let transform = usvg::Transform::from_scale(scale, scale);
            resvg::render(&tree, transform, &mut pixmap.as_mut());

            let size = Extent3d {
                width: target_width,
                height: target_height,
                depth_or_array_layers: 1,
            };

            // Pixmap data is RGBA8 premultiplied. Bevy's UI path is fine with this.
            let image = Image::new_fill(
                size,
                TextureDimension::D2,
                pixmap.data(),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::default(),
            );

            Ok(image)
        })
    }
}
