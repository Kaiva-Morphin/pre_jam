#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct Inputs {
    time: f32,
}

@group(2) @binding(0) var<uniform> in: Inputs;
@group(2) @binding(1) var scene_sampler: sampler;
@group(2) @binding(2) var scene: texture_2d<f32>;
@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4(1.0, 0.0, 0.0, 1.0);
}
