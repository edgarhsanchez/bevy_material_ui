//! Material Design 3 Loading Indicator
//!
//! The MD3 LoadingIndicator is an indeterminate activity indicator with morphing shapes.
//! Reference: <https://m3.material.io/components/loading-indicator/overview>

use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

use crate::theme::MaterialTheme;

pub const SHAPE_MORPH_SHADER_HANDLE: Handle<Shader> =
    bevy::asset::uuid_handle!("5a0d5e7c-4d3d-4a0e-a2b9-8b26e5f31b2d");

/// Custom UI material for shape morphing using SDF shaders
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ShapeMorphMaterial {
    #[uniform(0)]
    pub shape_from: u32,
    #[uniform(0)]
    pub shape_to: u32,
    #[uniform(0)]
    pub morph_t: f32,
    #[uniform(0)]
    pub rotation: f32,
    #[uniform(0)]
    pub color: LinearRgba,
}

impl UiMaterial for ShapeMorphMaterial {
    fn fragment_shader() -> ShaderRef {
        SHAPE_MORPH_SHADER_HANDLE.clone().into()
    }
}

impl Default for ShapeMorphMaterial {
    fn default() -> Self {
        Self {
            shape_from: 0,
            shape_to: 1,
            morph_t: 0.0,
            rotation: 0.0,
            color: LinearRgba::WHITE,
        }
    }
}

/// Plugin for loading indicator components
pub struct LoadingIndicatorPlugin;

impl Plugin for LoadingIndicatorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<crate::MaterialUiCorePlugin>() {
            app.add_plugins(crate::MaterialUiCorePlugin);
        }
        // Embed the shader so downstream apps don't need to copy it into their `assets/` folder.
        // Path is relative to this source file (bevy_asset::load_internal_asset! uses include_str!).
        bevy::asset::load_internal_asset!(
            app,
            SHAPE_MORPH_SHADER_HANDLE,
            "../assets/shaders/shape_morph.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(UiMaterialPlugin::<ShapeMorphMaterial>::default())
            .add_systems(
                Update,
                (
                    loading_indicator_morph_animation_system,
                    loading_indicator_rotation_animation_system,
                    loading_indicator_material_update_system,
                    loading_indicator_theme_refresh_system,
                )
                    .chain(),
            );
    }
}

/// Material loading indicator with morphing shapes
#[derive(Component, Debug, Clone, PartialEq)]
pub struct MaterialLoadingIndicator {
    /// Overall size (width/height)
    pub size: f32,
    /// Whether to fill parent container instead of using fixed size
    pub fill_parent: bool,
    /// Whether the indicator is shown on a contained surface
    pub contained: bool,
    /// Whether the indicator cycles through multiple colors
    pub multi_color: bool,
    /// Current morph fraction (0.0-7.0, cycles through shapes)
    pub morph_fraction: f32,
    /// Current rotation in degrees
    pub rotation: f32,
    /// Animation speed multiplier
    pub speed: f32,
    /// Current color index for multi-color mode
    pub color_index: usize,
}

impl MaterialLoadingIndicator {
    pub fn new() -> Self {
        Self {
            size: LOADING_INDICATOR_SIZE,
            fill_parent: false,
            contained: false,
            multi_color: false,
            morph_fraction: 0.0,
            rotation: 0.0,
            speed: 1.0,
            color_index: 0,
        }
    }

    pub fn contained(mut self) -> Self {
        self.contained = true;
        self
    }

    pub fn multi_color(mut self) -> Self {
        self.multi_color = true;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size.max(1.0);
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed.max(0.1);
        self
    }
}

impl Default for MaterialLoadingIndicator {
    fn default() -> Self {
        Self::new()
    }
}

/// Default loading indicator size
pub const LOADING_INDICATOR_SIZE: f32 = 48.0;

/// Shape size (indicator within container)
pub const LOADING_INDICATOR_SHAPE_SIZE: f32 = 38.0;

/// Duration per shape morph in seconds
pub const DURATION_PER_SHAPE: f32 = 0.65;

/// Constant rotation per shape in degrees
pub const CONSTANT_ROTATION_PER_SHAPE: f32 = 50.0;

/// Extra rotation per shape in degrees (spring-based)
pub const EXTRA_ROTATION_PER_SHAPE: f32 = 90.0;

/// Number of shapes in the morph sequence
pub const SHAPE_COUNT: usize = 7;

/// Shape types for morphing sequence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadingShape {
    SoftBurst, // 0
    Cookie9,   // 1
    Pentagon,  // 2
    Pill,      // 3
    Sunny,     // 4
    Cookie4,   // 5
    Oval,      // 6
}

impl LoadingShape {
    pub fn from_index(index: usize) -> Self {
        match index % SHAPE_COUNT {
            0 => LoadingShape::SoftBurst,
            1 => LoadingShape::Cookie9,
            2 => LoadingShape::Pentagon,
            3 => LoadingShape::Pill,
            4 => LoadingShape::Sunny,
            5 => LoadingShape::Cookie4,
            _ => LoadingShape::Oval,
        }
    }
}

/// Marker component for the shape renderer
#[derive(Component)]
struct LoadingIndicatorShape;

/// Number of dots in the indicator ring
pub const LOADING_INDICATOR_DOT_COUNT: usize = 12;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
struct LoadingIndicatorDot {
    index: usize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
struct LoadingIndicatorDotFor(Entity);

/// System to animate shape morphing
fn loading_indicator_morph_animation_system(
    time: Res<Time>,
    mut indicators: Query<&mut MaterialLoadingIndicator>,
) {
    for mut indicator in indicators.iter_mut() {
        // Advance morph fraction (0.0 to SHAPE_COUNT)
        indicator.morph_fraction += time.delta_secs() * indicator.speed / DURATION_PER_SHAPE;

        // Wrap around after completing all shapes
        if indicator.morph_fraction >= SHAPE_COUNT as f32 {
            indicator.morph_fraction -= SHAPE_COUNT as f32;
            indicator.color_index = (indicator.color_index + 1) % 4; // Cycle through 4 colors
        }
    }
}

/// System to animate rotation
fn loading_indicator_rotation_animation_system(
    time: Res<Time>,
    mut indicators: Query<&mut MaterialLoadingIndicator>,
) {
    for mut indicator in indicators.iter_mut() {
        let morph_factor_base = indicator.morph_fraction.floor();
        let morph_factor_per_shape = indicator.morph_fraction - morph_factor_base;

        // Calculate rotation components
        let constant_rotation =
            CONSTANT_ROTATION_PER_SHAPE * time.delta_secs() * indicator.speed / DURATION_PER_SHAPE;
        let _spring_rotation = EXTRA_ROTATION_PER_SHAPE * morph_factor_per_shape;

        indicator.rotation += constant_rotation;
        indicator.rotation %= 360.0;
    }
}

/// System to update shader material based on morph and rotation
fn loading_indicator_material_update_system(
    theme: Option<Res<MaterialTheme>>,
    indicators: Query<(&MaterialLoadingIndicator, &Children), Changed<MaterialLoadingIndicator>>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    material_query: Query<&MaterialNode<ShapeMorphMaterial>, With<LoadingIndicatorShape>>,
) {
    let Some(theme) = theme else { return };

    for (indicator, children) in indicators.iter() {
        // Get color based on multi-color mode
        let color = if indicator.multi_color {
            match indicator.color_index {
                0 => theme.primary,
                1 => theme.secondary,
                2 => theme.tertiary,
                _ => theme.error,
            }
        } else {
            theme.primary
        };

        // Update material for shape child
        for child in children.iter() {
            if let Ok(material_node) = material_query.get(child) {
                if let Some(material) = materials.get_mut(&material_node.0) {
                    // Calculate current and next shapes
                    let shape_index = indicator.morph_fraction.floor() as u32;
                    let morph_t = indicator.morph_fraction.fract();

                    material.shape_from = shape_index % 7;
                    material.shape_to = (shape_index + 1) % 7;
                    material.morph_t = morph_t;
                    material.rotation = indicator.rotation.to_radians();
                    material.color = LinearRgba::from(color);
                }
            }
        }
    }
}

/// Interpolate between two border radii
#[allow(dead_code)]
fn interpolate_border_radius(from: BorderRadius, to: BorderRadius, t: f32) -> BorderRadius {
    BorderRadius {
        top_left: interpolate_val(from.top_left, to.top_left, t),
        top_right: interpolate_val(from.top_right, to.top_right, t),
        bottom_left: interpolate_val(from.bottom_left, to.bottom_left, t),
        bottom_right: interpolate_val(from.bottom_right, to.bottom_right, t),
    }
}

#[allow(dead_code)]
fn interpolate_val(from: Val, to: Val, t: f32) -> Val {
    match (from, to) {
        (Val::Px(a), Val::Px(b)) => Val::Px(a + (b - a) * t),
        _ => from,
    }
}

/// System to refresh loading indicator colors when theme changes
fn loading_indicator_theme_refresh_system(
    theme: Option<Res<MaterialTheme>>,
    mut indicators: Query<
        (&MaterialLoadingIndicator, &mut BackgroundColor),
        Without<LoadingIndicatorShape>,
    >,
    _shapes: Query<&mut BackgroundColor, With<LoadingIndicatorShape>>,
) {
    let Some(theme) = theme else { return };
    if !theme.is_changed() {
        return;
    }

    for (indicator, mut bg) in indicators.iter_mut() {
        if indicator.contained {
            bg.0 = theme.surface_container_high;
        } else {
            bg.0 = Color::NONE;
        }
    }

    // Shape colors are updated in render system based on morph state
}

/// Builder for a loading indicator
pub struct LoadingIndicatorBuilder {
    indicator: MaterialLoadingIndicator,
}

impl LoadingIndicatorBuilder {
    pub fn new() -> Self {
        Self {
            indicator: MaterialLoadingIndicator::new(),
        }
    }

    pub fn contained(mut self) -> Self {
        self.indicator.contained = true;
        self
    }

    pub fn multi_color(mut self) -> Self {
        self.indicator.multi_color = true;
        self
    }

    pub fn fill(mut self) -> Self {
        self.indicator.fill_parent = true;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.indicator.size = size.max(1.0);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.indicator.speed = speed.max(0.1);
        self
    }

    pub fn build(self, theme: &MaterialTheme) -> impl Bundle {
        let size = self.indicator.size;
        let bg = if self.indicator.contained {
            theme.surface_container_high
        } else {
            theme.surface.with_alpha(0.0)
        };

        let (width, height) = if self.indicator.fill_parent {
            (Val::Percent(100.0), Val::Percent(100.0))
        } else {
            (Val::Px(size), Val::Px(size))
        };

        (
            self.indicator,
            Node {
                width,
                height,
                position_type: PositionType::Relative,
                overflow: Overflow::visible(),
                ..default()
            },
            BackgroundColor(bg),
            BorderRadius::all(Val::Px(size / 2.0)),
            Transform::default(),
            GlobalTransform::default(),
        )
    }
}

impl Default for LoadingIndicatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait to spawn loading indicators as children
pub trait SpawnLoadingIndicatorChild<'a> {
    fn spawn_loading_indicator(
        &mut self,
        theme: &MaterialTheme,
        materials: &mut Assets<ShapeMorphMaterial>,
    );

    fn spawn_loading_indicator_with(
        &mut self,
        theme: &MaterialTheme,
        materials: &mut Assets<ShapeMorphMaterial>,
        builder: LoadingIndicatorBuilder,
    );
}

impl<'a> SpawnLoadingIndicatorChild<'a> for ChildSpawnerCommands<'a> {
    fn spawn_loading_indicator(
        &mut self,
        theme: &MaterialTheme,
        materials: &mut Assets<ShapeMorphMaterial>,
    ) {
        self.spawn_loading_indicator_with(theme, materials, LoadingIndicatorBuilder::new());
    }

    fn spawn_loading_indicator_with(
        &mut self,
        theme: &MaterialTheme,
        materials: &mut Assets<ShapeMorphMaterial>,
        builder: LoadingIndicatorBuilder,
    ) {
        let color = theme.primary;

        let fill_parent = builder.indicator.fill_parent;
        let shape_size = LOADING_INDICATOR_SHAPE_SIZE;
        let container_size = builder.indicator.size;

        // Create material handle
        let material_handle = materials.add(ShapeMorphMaterial {
            shape_from: 0,
            shape_to: 1,
            morph_t: 0.0,
            rotation: 0.0,
            color: LinearRgba::from(color),
        });

        self.spawn(builder.build(theme)).with_children(|parent| {
            if fill_parent {
                // Fill entire parent container
                parent.spawn((
                    LoadingIndicatorShape,
                    MaterialNode(material_handle),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..default()
                    },
                ));
            } else {
                // Fixed size, centered
                parent.spawn((
                    LoadingIndicatorShape,
                    MaterialNode(material_handle),
                    Node {
                        width: Val::Px(shape_size),
                        height: Val::Px(shape_size),
                        position_type: PositionType::Absolute,
                        left: Val::Px((container_size - shape_size) / 2.0),
                        top: Val::Px((container_size - shape_size) / 2.0),
                        ..default()
                    },
                ));
            }
        });
    }
}
