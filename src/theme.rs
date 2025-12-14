//! Material Design 3 Theme System
//!
//! Provides a complete color scheme and theming system based on MD3 guidelines.
//! Reference: <https://m3.material.io/styles/color/overview>

use bevy::prelude::*;

use crate::color::MaterialColorScheme;

/// Theme mode (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// Light theme
    Light,
    /// Dark theme (default for game applications)
    #[default]
    Dark,
}

/// Color scheme variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    /// Default Material You purple/violet scheme
    #[default]
    Default,
    /// Custom scheme (use with `MaterialTheme::from_seed`)
    Custom,
}

/// Material Design 3 Theme Resource
///
/// Contains all color tokens for the Material Design 3 color system.
/// Use this resource to style your UI components consistently.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::theme::MaterialTheme;
///
/// fn setup_ui(theme: Res<MaterialTheme>, mut commands: Commands) {
///     commands.spawn((
///         Node {
///             width: Val::Percent(100.0),
///             height: Val::Percent(100.0),
///             ..default()
///         },
///         BackgroundColor(theme.surface),
///     ));
/// }
/// ```
#[derive(Resource, Debug, Clone)]
pub struct MaterialTheme {
    /// Current theme mode
    pub mode: ThemeMode,
    
    // Primary colors
    /// Primary brand color
    pub primary: Color,
    /// Color for content on primary
    pub on_primary: Color,
    /// Primary container color
    pub primary_container: Color,
    /// Color for content on primary container
    pub on_primary_container: Color,

    // Secondary colors
    /// Secondary brand color
    pub secondary: Color,
    /// Color for content on secondary
    pub on_secondary: Color,
    /// Secondary container color
    pub secondary_container: Color,
    /// Color for content on secondary container
    pub on_secondary_container: Color,

    // Tertiary colors
    /// Tertiary accent color
    pub tertiary: Color,
    /// Color for content on tertiary
    pub on_tertiary: Color,
    /// Tertiary container color
    pub tertiary_container: Color,
    /// Color for content on tertiary container
    pub on_tertiary_container: Color,

    // Error colors
    /// Error state color
    pub error: Color,
    /// Color for content on error
    pub on_error: Color,
    /// Error container color
    pub error_container: Color,
    /// Color for content on error container
    pub on_error_container: Color,

    // Surface colors
    /// Base surface color
    pub surface: Color,
    /// Color for content on surface
    pub on_surface: Color,
    /// Variant of on_surface for less emphasis
    pub on_surface_variant: Color,
    /// Lowest surface container
    pub surface_container_lowest: Color,
    /// Low surface container
    pub surface_container_low: Color,
    /// Default surface container
    pub surface_container: Color,
    /// High surface container
    pub surface_container_high: Color,
    /// Highest surface container
    pub surface_container_highest: Color,

    // Other colors
    /// Outline color for borders
    pub outline: Color,
    /// Variant outline for subtle borders
    pub outline_variant: Color,
    /// Inverse surface for contrast
    pub inverse_surface: Color,
    /// Content on inverse surface
    pub inverse_on_surface: Color,
    /// Inverse primary for contrast
    pub inverse_primary: Color,
    /// Scrim overlay color
    pub scrim: Color,
    /// Shadow color
    pub shadow: Color,

    // Custom game-specific colors
    /// Color for selected/active states
    pub selected: Color,
    /// Color for unselected/inactive states
    pub unselected: Color,
}

impl Default for MaterialTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl MaterialTheme {
    /// Create a theme from a seed color using MD3 scheme generation.
    pub fn from_seed(seed: Color, mode: ThemeMode) -> Self {
        let scheme = match mode {
            ThemeMode::Dark => MaterialColorScheme::dark_from_bevy_color(seed),
            ThemeMode::Light => MaterialColorScheme::light_from_bevy_color(seed),
        };

        Self {
            mode,

            primary: scheme.primary,
            on_primary: scheme.on_primary,
            primary_container: scheme.primary_container,
            on_primary_container: scheme.on_primary_container,

            secondary: scheme.secondary,
            on_secondary: scheme.on_secondary,
            secondary_container: scheme.secondary_container,
            on_secondary_container: scheme.on_secondary_container,

            tertiary: scheme.tertiary,
            on_tertiary: scheme.on_tertiary,
            tertiary_container: scheme.tertiary_container,
            on_tertiary_container: scheme.on_tertiary_container,

            error: scheme.error,
            on_error: scheme.on_error,
            error_container: scheme.error_container,
            on_error_container: scheme.on_error_container,

            surface: scheme.surface,
            on_surface: scheme.on_surface,
            on_surface_variant: scheme.on_surface_variant,
            surface_container_lowest: scheme.surface_container_lowest,
            surface_container_low: scheme.surface_container_low,
            surface_container: scheme.surface_container,
            surface_container_high: scheme.surface_container_high,
            surface_container_highest: scheme.surface_container_highest,

            outline: scheme.outline,
            outline_variant: scheme.outline_variant,
            inverse_surface: scheme.inverse_surface,
            inverse_on_surface: scheme.inverse_on_surface,
            inverse_primary: scheme.inverse_primary,
            scrim: scheme.scrim,
            shadow: scheme.shadow,

            selected: scheme.primary,
            unselected: scheme.outline,
        }
    }

    /// Create a dark theme (recommended for games)
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            
            // Primary - Purple/Violet
            primary: Color::srgb(0.82, 0.71, 1.0),           // #D0B4FF
            on_primary: Color::srgb(0.25, 0.09, 0.46),       // #402076
            primary_container: Color::srgb(0.38, 0.23, 0.58), // #61398E
            on_primary_container: Color::srgb(0.92, 0.85, 1.0), // #EBDAFF

            // Secondary
            secondary: Color::srgb(0.80, 0.78, 0.90),        // #CCC6E0
            on_secondary: Color::srgb(0.21, 0.19, 0.31),     // #343046
            secondary_container: Color::srgb(0.32, 0.30, 0.43), // #4B465E
            on_secondary_container: Color::srgb(0.92, 0.90, 1.0), // #E9E1FC

            // Tertiary
            tertiary: Color::srgb(0.94, 0.73, 0.78),         // #F0BAC7
            on_tertiary: Color::srgb(0.29, 0.14, 0.20),      // #4A2532
            tertiary_container: Color::srgb(0.42, 0.26, 0.34), // #633B49
            on_tertiary_container: Color::srgb(1.0, 0.85, 0.89), // #FFD9E3

            // Error
            error: Color::srgb(1.0, 0.71, 0.68),             // #FFB4AB
            on_error: Color::srgb(0.41, 0.0, 0.04),          // #690006
            error_container: Color::srgb(0.58, 0.0, 0.07),   // #93000A
            on_error_container: Color::srgb(1.0, 0.85, 0.82), // #FFD9D4

            // Surface - Dark theme
            surface: Color::srgb(0.08, 0.07, 0.09),          // #141316
            on_surface: Color::srgb(0.90, 0.87, 0.92),       // #E6E1E9
            on_surface_variant: Color::srgb(0.78, 0.74, 0.82), // #C9C4D0
            surface_container_lowest: Color::srgb(0.05, 0.04, 0.06), // #0D0C0F
            surface_container_low: Color::srgb(0.11, 0.10, 0.12),   // #1C1B1E
            surface_container: Color::srgb(0.13, 0.12, 0.14),       // #211F23
            surface_container_high: Color::srgb(0.17, 0.16, 0.18),  // #2B292D
            surface_container_highest: Color::srgb(0.21, 0.20, 0.23), // #363438

            // Other
            outline: Color::srgb(0.58, 0.55, 0.62),          // #938E9A
            outline_variant: Color::srgb(0.29, 0.27, 0.32),  // #48454F
            inverse_surface: Color::srgb(0.90, 0.87, 0.92),  // #E6E1E9
            inverse_on_surface: Color::srgb(0.19, 0.18, 0.20), // #302E32
            inverse_primary: Color::srgb(0.50, 0.35, 0.71),  // #7F58B5
            scrim: Color::srgb(0.0, 0.0, 0.0),               // #000000
            shadow: Color::srgb(0.0, 0.0, 0.0),              // #000000

            // Game-specific
            selected: Color::srgb(0.82, 0.71, 1.0),          // Same as primary
            unselected: Color::srgb(0.58, 0.55, 0.62),       // Same as outline
        }
    }

    /// Create a light theme
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            
            // Primary - Purple/Violet
            primary: Color::srgb(0.50, 0.35, 0.71),          // #7F58B5
            on_primary: Color::srgb(1.0, 1.0, 1.0),          // #FFFFFF
            primary_container: Color::srgb(0.92, 0.85, 1.0), // #EBDAFF
            on_primary_container: Color::srgb(0.15, 0.0, 0.34), // #260052

            // Secondary
            secondary: Color::srgb(0.38, 0.36, 0.50),        // #605D75
            on_secondary: Color::srgb(1.0, 1.0, 1.0),        // #FFFFFF
            secondary_container: Color::srgb(0.92, 0.90, 1.0), // #E9E1FD
            on_secondary_container: Color::srgb(0.11, 0.09, 0.20), // #1C1930

            // Tertiary
            tertiary: Color::srgb(0.52, 0.33, 0.41),         // #7D5260
            on_tertiary: Color::srgb(1.0, 1.0, 1.0),         // #FFFFFF
            tertiary_container: Color::srgb(1.0, 0.85, 0.89), // #FFD9E3
            on_tertiary_container: Color::srgb(0.19, 0.05, 0.13), // #31101D

            // Error
            error: Color::srgb(0.73, 0.11, 0.15),            // #BA1A24
            on_error: Color::srgb(1.0, 1.0, 1.0),            // #FFFFFF
            error_container: Color::srgb(1.0, 0.85, 0.82),   // #FFD9D4
            on_error_container: Color::srgb(0.26, 0.0, 0.02), // #410003

            // Surface - Light theme
            surface: Color::srgb(0.99, 0.97, 1.0),           // #FDF8FF
            on_surface: Color::srgb(0.11, 0.10, 0.12),       // #1C1B1E
            on_surface_variant: Color::srgb(0.29, 0.27, 0.32), // #48454F
            surface_container_lowest: Color::srgb(1.0, 1.0, 1.0),   // #FFFFFF
            surface_container_low: Color::srgb(0.97, 0.95, 0.98),   // #F7F2FA
            surface_container: Color::srgb(0.95, 0.93, 0.96),       // #F1ECF4
            surface_container_high: Color::srgb(0.92, 0.90, 0.93),  // #EBE6EE
            surface_container_highest: Color::srgb(0.90, 0.87, 0.91), // #E5E1E9

            // Other
            outline: Color::srgb(0.47, 0.44, 0.51),          // #79757F
            outline_variant: Color::srgb(0.78, 0.75, 0.82),  // #C9C4D0
            inverse_surface: Color::srgb(0.19, 0.18, 0.20),  // #302E32
            inverse_on_surface: Color::srgb(0.96, 0.94, 0.97), // #F4EFF7
            inverse_primary: Color::srgb(0.82, 0.71, 1.0),   // #D0B4FF
            scrim: Color::srgb(0.0, 0.0, 0.0),               // #000000
            shadow: Color::srgb(0.0, 0.0, 0.0),              // #000000

            // Game-specific
            selected: Color::srgb(0.50, 0.35, 0.71),         // Same as primary
            unselected: Color::srgb(0.47, 0.44, 0.51),       // Same as outline
        }
    }

    /// Toggle between light and dark mode
    pub fn toggle_mode(&mut self) {
        *self = match self.mode {
            ThemeMode::Light => Self::dark(),
            ThemeMode::Dark => Self::light(),
        };
    }

    /// Get the appropriate state layer opacity for a given interaction state
    pub fn state_layer_opacity(state: StateLayer) -> f32 {
        match state {
            StateLayer::None => 0.0,
            StateLayer::Hover => 0.08,
            StateLayer::Focus => 0.12,
            StateLayer::Pressed => 0.12,
            StateLayer::Dragged => 0.16,
        }
    }

    /// Apply a state layer color on top of a base color
    pub fn with_state_layer(&self, base: Color, state: StateLayer, content_color: Color) -> Color {
        let opacity = Self::state_layer_opacity(state);
        if opacity == 0.0 {
            return base;
        }
        
        // Blend the content color over the base with the state layer opacity
        let base_linear = base.to_linear();
        let content_linear = content_color.to_linear();
        
        Color::linear_rgba(
            base_linear.red + (content_linear.red - base_linear.red) * opacity,
            base_linear.green + (content_linear.green - base_linear.green) * opacity,
            base_linear.blue + (content_linear.blue - base_linear.blue) * opacity,
            base_linear.alpha,
        )
    }
}

/// State layer for interaction feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StateLayer {
    /// No state layer
    #[default]
    None,
    /// Hover state (8% opacity)
    Hover,
    /// Focus state (12% opacity)
    Focus,
    /// Pressed state (12% opacity)
    Pressed,
    /// Dragged state (16% opacity)
    Dragged,
}

/// Blend a state layer color over a base color with given opacity
/// 
/// This is a standalone helper function for applying MD3 state layers.
/// The state layer is a semi-transparent overlay of the content/state color.
/// 
/// # Arguments
/// * `base` - The base background color
/// * `state_layer_color` - The color of the state layer (usually "on" color like on_primary)
/// * `opacity` - The opacity of the state layer (0.08 for hover, 0.12 for pressed)
pub fn blend_state_layer(base: Color, state_layer_color: Color, opacity: f32) -> Color {
    if opacity <= 0.0 {
        return base;
    }
    
    let base_linear = base.to_linear();
    let layer_linear = state_layer_color.to_linear();
    
    // Alpha blending: result = base * (1 - opacity) + layer * opacity
    Color::linear_rgba(
        base_linear.red * (1.0 - opacity) + layer_linear.red * opacity,
        base_linear.green * (1.0 - opacity) + layer_linear.green * opacity,
        base_linear.blue * (1.0 - opacity) + layer_linear.blue * opacity,
        base_linear.alpha,
    )
}
