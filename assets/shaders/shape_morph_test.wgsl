#import bevy_ui::ui_vertex_output::UiVertexOutput

struct ShapeMorphMaterial {
    shape_from: u32,
    shape_to: u32,
    morph_t: f32,
    rotation: f32,
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: ShapeMorphMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // Simple test: just return the color to verify rendering works
    return material.color;
}
