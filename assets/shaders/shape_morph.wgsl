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

// Constants for shape types matching LoadingShape enum
const SHAPE_SOFT_BURST: u32 = 0u;  // 10-point star
const SHAPE_COOKIE_9: u32 = 1u;    // 9-point star
const SHAPE_PENTAGON: u32 = 2u;    // Pentagon
const SHAPE_PILL: u32 = 3u;        // Capsule
const SHAPE_SUNNY: u32 = 4u;       // 8-point star
const SHAPE_COOKIE_4: u32 = 5u;    // 4-point star
const SHAPE_OVAL: u32 = 6u;        // Ellipse

const PI: f32 = 3.14159265359;

// ===== SDF Helper Functions =====

fn rotate2d(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec2<f32>(
        p.x * c - p.y * s,
        p.x * s + p.y * c
    );
}

// SDF for circle
fn sd_circle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

// SDF for regular polygon
fn sd_polygon(p: vec2<f32>, n: f32, r: f32) -> f32 {
    let an = PI / n;
    let en = PI / n;
    let acs = vec2<f32>(cos(an), sin(an));
    let ecs = vec2<f32>(cos(en), sin(en));
    
    var bn = atan2(abs(p.x), p.y) % (2.0 * an) - an;
    var q = length(p) * vec2<f32>(cos(bn), abs(sin(bn)));
    q = q - r * acs;
    q = q + ecs * clamp(-dot(q, ecs), 0.0, r * acs.y / ecs.y);
    return length(q) * sign(q.x);
}

// SDF for star shape
fn sd_star(p: vec2<f32>, points: f32, outer_radius: f32, inner_radius: f32) -> f32 {
    let angle = atan2(p.y, p.x);
    let sector = 2.0 * PI / points;
    let half_sector = sector * 0.5;
    
    // Determine which sector we're in
    var local_angle = (angle + PI) % sector;
    if local_angle < 0.0 {
        local_angle += sector;
    }
    
    let dist = length(p);
    
    // Calculate distance to star outline
    var radius: f32;
    if local_angle < half_sector {
        let t = local_angle / half_sector;
        radius = mix(outer_radius, inner_radius, t);
    } else {
        let t = (local_angle - half_sector) / half_sector;
        radius = mix(inner_radius, outer_radius, t);
    }
    
    return dist - radius;
}

// SDF for capsule (pill shape)
fn sd_capsule(p: vec2<f32>, width: f32, height: f32) -> f32 {
    let half_height = height * 0.5;
    let radius = width * 0.5;
    
    var p_local = p;
    p_local.y = abs(p_local.y);
    p_local.y = p_local.y - max(0.0, half_height - radius);
    
    return length(p_local) - radius;
}

// SDF for ellipse
fn sd_ellipse(p: vec2<f32>, r: vec2<f32>) -> f32 {
    let k0 = length(p / r);
    let k1 = length(p / (r * r));
    return k0 * (k0 - 1.0) / k1;
}

// ===== Shape-specific SDFs =====

fn sdf_soft_burst(p: vec2<f32>) -> f32 {
    // 10-pointed star
    return sd_star(p, 10.0, 0.48, 0.30);
}

fn sdf_cookie_9(p: vec2<f32>) -> f32 {
    // 9-pointed star
    return sd_star(p, 9.0, 0.47, 0.35);
}

fn sdf_pentagon(p: vec2<f32>) -> f32 {
    // Regular pentagon
    return sd_polygon(p, 5.0, 0.44);
}

fn sdf_pill(p: vec2<f32>) -> f32 {
    // Capsule/pill shape (horizontal)
    return sd_capsule(p, 0.40, 0.70);
}

fn sdf_sunny(p: vec2<f32>) -> f32 {
    // 8-pointed star
    return sd_star(p, 8.0, 0.48, 0.34);
}

fn sdf_cookie_4(p: vec2<f32>) -> f32 {
    // 4-pointed star (diamond-like)
    return sd_star(p, 4.0, 0.48, 0.37);
}

fn sdf_oval(p: vec2<f32>) -> f32 {
    // Ellipse
    return sd_ellipse(p, vec2<f32>(0.50, 0.40));
}

// Get SDF for a specific shape
fn get_shape_sdf(p: vec2<f32>, shape: u32) -> f32 {
    switch shape {
        case 0u: { return sdf_soft_burst(p); }
        case 1u: { return sdf_cookie_9(p); }
        case 2u: { return sdf_pentagon(p); }
        case 3u: { return sdf_pill(p); }
        case 4u: { return sdf_sunny(p); }
        case 5u: { return sdf_cookie_4(p); }
        case 6u: { return sdf_oval(p); }
        default: { return sd_circle(p, 0.4); }
    }
}

// Smooth minimum for blending SDFs
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // Convert UV to centered coordinates (-0.5 to 0.5)
    var uv = in.uv - vec2<f32>(0.5, 0.5);
    
    // Scale up to make shapes fill the space better (UVs are -0.5 to 0.5, we want shapes to fill)
    uv = uv * 2.0;  // Now uv ranges from -1.0 to 1.0
    
    // Apply rotation
    uv = rotate2d(uv, material.rotation);
    
    // Get SDFs for both shapes
    let sdf_from = get_shape_sdf(uv, material.shape_from);
    let sdf_to = get_shape_sdf(uv, material.shape_to);
    
    // Smooth interpolation between shapes
    let sdf = mix(sdf_from, sdf_to, material.morph_t);
    
    // Anti-aliasing using fwidth
    let edge_distance = fwidth(sdf) * 0.5;
    let alpha = 1.0 - smoothstep(-edge_distance, edge_distance, sdf);
    
    // Apply alpha to color
    var color = material.color;
    color.a *= alpha;
    
    return color;
}
