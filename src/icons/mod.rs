//! Material Design Icons System
//!
//! This module provides support for Material Symbols icons in Bevy UI.
//! Icons can be rendered using an icon font or as custom images.
//!
//! # Icon Font Approach
//!
//! Material Symbols is a variable icon font that supports:
//! - **Fill**: 0 (outlined) to 1 (filled)
//! - **Weight**: 100 to 700 (thin to bold)
//! - **Grade**: -25 to 200 (emphasis adjustment)
//! - **Optical Size**: 20, 24, 40, 48 (optimized for size)
//!
//! # Usage
//!
//! ```rust,ignore
//! use bevy_material_ui::icons::{MaterialIcon, IconStyle, MaterialIconsPlugin};
//!
//! // Add the plugin to load the font
//! app.add_plugins(MaterialIconsPlugin);
//!
//! // Create an icon with the font
//! fn spawn_icon(mut commands: Commands, icon_font: Res<MaterialIconFont>) {
//!     commands.spawn((
//!         Text::new(MaterialIcon::home().as_str()),
//!         TextFont {
//!             font: icon_font.0.clone(),
//!             font_size: 24.0,
//!             ..default()
//!         },
//!     ));
//! }
//! ```
//!
//! # Available Icons
//!
//! This module includes commonly used icons from Material Symbols.
//! For a complete list, see <https://fonts.google.com/icons>

use bevy::prelude::*;

mod codepoints;
pub mod icon;
mod style;

pub use codepoints::*;
pub use icon::{IconBundle, MaterialIcon};
pub use style::{IconGrade, IconOpticalSize, IconStyle, IconWeight};

/// Embedded Material Symbols font data (compiled into the binary)
/// This eliminates file I/O and ensures the font is always available
pub const EMBEDDED_MATERIAL_SYMBOLS_FONT: &[u8] =
    include_bytes!("../../assets/fonts/MaterialSymbolsOutlined.ttf");

/// Path to the Material Symbols font file (for fallback/external loading)
pub const MATERIAL_SYMBOLS_FONT_PATH: &str = "fonts/MaterialSymbolsOutlined.ttf";

/// Resource holding the loaded Material Symbols font handle
#[derive(Resource, Clone)]
pub struct MaterialIconFont(pub Handle<Font>);

impl MaterialIconFont {
    /// Get the font handle
    pub fn handle(&self) -> Handle<Font> {
        self.0.clone()
    }
}

/// Plugin that loads the Material Symbols font
pub struct MaterialIconsPlugin;

impl Plugin for MaterialIconsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_material_icons_font);
    }
}

/// System to load the Material Symbols font at startup
/// Uses embedded font data for instant availability (no async loading)
fn load_material_icons_font(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    // Load from embedded bytes - this is synchronous and immediately available
    let font = Font::try_from_bytes(EMBEDDED_MATERIAL_SYMBOLS_FONT.to_vec())
        .expect("Failed to load embedded Material Symbols font");
    let font_handle = fonts.add(font);
    commands.insert_resource(MaterialIconFont(font_handle));
}
