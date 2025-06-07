struct Inputs {
    time: f32,
}

@group(2) @binding(0) var<uniform> in: Inputs;
@group(2) @binding(2) var prev_scene: texture_2d<f32>;
@group(2) @binding(3) var scene_sampler: sampler;
@group(2) @binding(4) var scene: texture_2d<f32>;
@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(scene, scene_sampler, uv) * textureSample(prev_scene, scene_sampler, uv);
}
