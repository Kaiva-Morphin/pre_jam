#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct Inputs {
    time: f32,
}

@group(2) @binding(0) var<uniform> in: Inputs;
@group(2) @binding(1) var scene_sampler: sampler;
@group(2) @binding(2) var scene: texture_2d<f32>;
@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let k = apply_kernel(uv);
    return vec4(1.0, k, 0.0, 1.0);//textureSample(scene, scene_sampler, uv) * vec4(abs(sin(in.time)));
}


fn apply_kernel(
    uv: vec2<f32>,
) -> f32 {
    let global_offset = 2.0;
    var edge_dst = 1337.0;
    let kernelSize = 16;
    let halfKernel = kernelSize / 2;
    var total = 0.0;
    let texelSize = vec2<f32>(1.0 / f32(32), 1.0 / f32(32)) * 3.0;
    var weight_sum = 0.0;
    for (var x = -halfKernel; x <= halfKernel; x++) {
        for (var y = -halfKernel; y <= halfKernel; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texelSize;
            let dist = length(vec2<f32>(f32(x), f32(y)));
            let alpha = textureSample(scene, scene_sampler, uv + offset * global_offset).r;
            // air adds more shadow
            let weight = (1.0 - alpha) / (dist + 1.0); // prevent div by 0
            total += weight;
            weight_sum += 1.0 / (dist + 1.0);
        }
    }
    var result_global = total / weight_sum * textureSample(scene, scene_sampler, uv).a;
    return result_global;
}