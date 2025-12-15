//! UI-compatible 2D shape rendering
//! 
//! This module provides custom shape rendering that works within Bevy's UI layout system.
//! Shapes are rendered as 2D meshes positioned in UI space.

use bevy::prelude::*;

/// Plugin for UI shape rendering
pub struct UiShapePlugin;

impl Plugin for UiShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ui_shape_positions);
    }
}

/// Marker component for UI shapes that need position syncing
#[derive(Component)]
pub struct UiShapeMarker {
    /// Offset from parent UI node (in pixels)
    pub offset: Vec2,
}

/// A 2D path defined by vertices
#[derive(Clone, Debug)]
pub struct ShapePath {
    pub vertices: Vec<Vec2>,
    pub closed: bool,
}

impl ShapePath {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            closed: false,
        }
    }

    pub fn move_to(mut self, point: Vec2) -> Self {
        self.vertices.clear();
        self.vertices.push(point);
        self
    }

    pub fn line_to(mut self, point: Vec2) -> Self {
        self.vertices.push(point);
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    /// Create a regular polygon
    pub fn regular_polygon(sides: u32, radius: f32, center: Vec2) -> Self {
        use std::f32::consts::PI;
        let mut path = Self::new();
        
        for i in 0..sides {
            let angle = (i as f32) * 2.0 * PI / (sides as f32) - PI / 2.0;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        path.closed = true;
        path
    }

    /// Create a star shape
    pub fn star(points: u32, outer_radius: f32, inner_radius: f32, center: Vec2) -> Self {
        use std::f32::consts::PI;
        let mut path = Self::new();
        
        for i in 0..(points * 2) {
            let angle = (i as f32) * PI / (points as f32) - PI / 2.0;
            let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        path.closed = true;
        path
    }

    /// Create an ellipse
    pub fn ellipse(radii: Vec2, center: Vec2, segments: u32) -> Self {
        use std::f32::consts::PI;
        let mut path = Self::new();
        
        for i in 0..segments {
            let angle = (i as f32) * 2.0 * PI / (segments as f32);
            let x = center.x + radii.x * angle.cos();
            let y = center.y + radii.y * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        path.closed = true;
        path
    }

    /// Create a rounded rectangle
    pub fn rounded_rect(size: Vec2, radius: f32, center: Vec2, segments_per_corner: u32) -> Self {
        use std::f32::consts::PI;
        let mut path = Self::new();
        
        let half_w = size.x / 2.0;
        let half_h = size.y / 2.0;
        let r = radius.min(half_w).min(half_h);
        
        // Top-right corner
        for i in 0..segments_per_corner {
            let angle = (i as f32) * PI / 2.0 / (segments_per_corner as f32);
            let x = center.x + half_w - r + r * angle.cos();
            let y = center.y - half_h + r - r * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        // Bottom-right corner
        for i in 0..segments_per_corner {
            let angle = PI / 2.0 + (i as f32) * PI / 2.0 / (segments_per_corner as f32);
            let x = center.x + half_w - r + r * angle.cos();
            let y = center.y + half_h - r - r * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        // Bottom-left corner
        for i in 0..segments_per_corner {
            let angle = PI + (i as f32) * PI / 2.0 / (segments_per_corner as f32);
            let x = center.x - half_w + r + r * angle.cos();
            let y = center.y + half_h - r - r * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        // Top-left corner
        for i in 0..segments_per_corner {
            let angle = 3.0 * PI / 2.0 + (i as f32) * PI / 2.0 / (segments_per_corner as f32);
            let x = center.x - half_w + r + r * angle.cos();
            let y = center.y - half_h + r - r * angle.sin();
            path.vertices.push(Vec2::new(x, y));
        }
        
        path.closed = true;
        path
    }

    /// Tessellate the path into a triangle mesh using ear clipping
    pub fn tessellate(&self) -> Mesh {
        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            bevy::render::render_asset::RenderAssetUsages::default(),
        );

        if self.vertices.len() < 3 {
            return mesh;
        }

        // Simple ear-clipping triangulation for convex/simple polygons
        let indices = self.triangulate_ear_clipping();
        
        // Convert vertices to 3D positions
        let positions: Vec<[f32; 3]> = self.vertices
            .iter()
            .map(|v| [v.x, v.y, 0.0])
            .collect();

        let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; positions.len()];
        let uvs: Vec<[f32; 2]> = self.vertices
            .iter()
            .map(|v| [v.x, v.y])
            .collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

        mesh
    }

    /// Simple ear clipping triangulation
    fn triangulate_ear_clipping(&self) -> Vec<u32> {
        let mut indices = Vec::new();
        
        if self.vertices.len() < 3 {
            return indices;
        }

        // For simple convex polygons, use fan triangulation
        let n = self.vertices.len();
        for i in 1..(n - 1) {
            indices.push(0);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }

        indices
    }
}

impl Default for ShapePath {
    fn default() -> Self {
        Self::new()
    }
}

/// System to sync UI shape positions with their parent UI nodes
fn update_ui_shape_positions(
    ui_nodes: Query<(&Node, &GlobalTransform), Without<UiShapeMarker>>,
    mut ui_shapes: Query<(&UiShapeMarker, &ChildOf, &mut Transform)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    // Get the camera for UI space conversion
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };

    for (shape_marker, parent, mut transform) in ui_shapes.iter_mut() {
        if let Ok((node, global_transform)) = ui_nodes.get(parent.get()) {
            // Get the UI node's screen position
            let ui_pos = global_transform.translation();
            
            // Convert UI space to world space
            // UI coordinates are in logical pixels from top-left
            // We need to convert to world coordinates (center-origin)
            
            // Get viewport size
            if let Some(viewport_size) = camera.logical_viewport_size() {
                // UI node position in world space (centered)
                let world_x = ui_pos.x - viewport_size.x / 2.0 + shape_marker.offset.x;
                let world_y = -ui_pos.y + viewport_size.y / 2.0 + shape_marker.offset.y;
                
                transform.translation.x = world_x;
                transform.translation.y = world_y;
                transform.translation.z = ui_pos.z + 1.0; // Slightly in front of UI
            }
        }
    }
}

/// Builder for creating UI shapes
pub struct UiShapeBuilder {
    path: ShapePath,
    color: Color,
    offset: Vec2,
}

impl UiShapeBuilder {
    pub fn new(path: ShapePath) -> Self {
        Self {
            path,
            color: Color::WHITE,
            offset: Vec2::ZERO,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn build(
        self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> impl Bundle {
        let mesh = self.path.tessellate();
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(ColorMaterial::from(self.color));
        
        (
            UiShapeMarker {
                offset: self.offset,
            },
            mesh_handle.clone(),
            material_handle.clone(),
            Transform::from_xyz(0.0, 0.0, 1.0),
            Visibility::default(),
        )
    }
}
