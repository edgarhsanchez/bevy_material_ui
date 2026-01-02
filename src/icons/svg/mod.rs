mod svg_icon;
mod loader;

use bevy::asset::AssetApp;
use bevy::prelude::*;

pub use svg_icon::{svg_icon_path, svg_icon_system, SvgIcon};
pub use loader::SvgImageLoader;

/// Enables loading `.svg` files as `Image` assets and rendering them in UI via `SvgIcon`.
pub struct SvgIconAssetsPlugin;

impl Plugin for SvgIconAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<SvgImageLoader>();
        app.add_systems(Update, svg_icon_system);
    }
}
