#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct Inputs {
    time: f32,
    width: u32,
    height: u32,
    _b: u32,
}

@group(2) @binding(0) var<uniform> in: Inputs;
@group(2) @binding(1) var<uniform> px_size: vec2<u32>;
@group(2) @binding(3) var col: texture_2d<f32>;
@group(2) @binding(4) var smpr: sampler;
@group(2) @binding(5) var mask: texture_2d<f32>;
@group(2) @binding(6) var norm: texture_2d<f32>;
@group(2) @binding(7) var noise: texture_3d<f32>;
@group(2) @binding(8) var noise_smpr: sampler;
@group(2) @binding(9) var light: texture_2d<f32>;

fn white_noise(pos: vec2<f32>) -> f32 {
    let dot_val = dot(pos, vec2<f32>(12.9898, 78.233));
    let sin_val = sin(dot_val) * 43758.5453;
    return fract(sin_val);
}

fn white_noise_3d(pos: vec3<f32>) -> f32 {
    let dot_val = dot(pos, vec3<f32>(12.9898, 78.233, 45.164));
    let sin_val = sin(dot_val) * 43758.5453;
    return fract(sin_val);
}


@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let up_uv = uv * vec2(1.0, 0.5);
    let width = f32(in.width);
    let height = f32(in.height) * 0.5;

    let texelSize = vec2<f32>(1.0 / f32(in.width), 1.0 / f32(in.height)) * 2.0;

    var color = vec4<f32>(0.0);
    // var masked_color = vec4<f32>(0.0);
    var kernelSize = 16;
    var halfKernel = kernelSize / 2;
    for (var x = -halfKernel; x <= halfKernel; x++) {
        for (var y = -halfKernel; y <= halfKernel; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texelSize;
            if (up_uv + offset).y > 0.5 {
                continue;
            }
            let v = textureSample(col, smpr, up_uv + offset);
            color += v;
            // masked_color += v * textureSample(mask, smpr, up_uv + offset);
        }
    }
    let totalSamples = f32(kernelSize * kernelSize);
    var result_color = color / totalSamples;
    // var result_masked = masked_color / totalSamples;
    result_color.r = min(result_color.r, 1.0);
    result_color.g = min(result_color.g, 1.0);
    result_color.b = min(result_color.b, 1.0);
    
    let e = 1.0 - (result_color.r + result_color.g + result_color.b) / 3.0;
    // var noise = textureSample(noise, noise_smpr, vec3(up_uv * width / height * 1.0, in.time * 0.5)).r;
    // noise *= 1.0;
    // // noise += 0.1;
    // let edgeFactor = 1.0 - (result_color.r + result_color.g + result_color.b) / 4.0;
    var u = up_uv * vec2(width, height);
    u.x = floor(u.x / 3.0) * 3.0;
    u.y = floor(u.y / 3.0) * 3.0;
    let noise = white_noise_3d(vec3(u, in.time * 0.01)) * white_noise_3d(vec3(uv, in.time * 0.01));
    // let noise = white_noise_3d(vec3(up_uv, in.time * 0.01));
    let noisyFalloff = result_color * mix(1.0, noise, e * noise);
    
    let mask_tex = textureSample(mask, smpr, uv);
    let mask = mask_tex.a;
    let c = textureSample(col, smpr, up_uv);
    var o = noisyFalloff.rgb;
    // var o = noisyFalloff.a;
    // o = vec3(apply_kernel(up_uv).rgb);
    // o = c.rgb;
    // c.a
    // o = textureSample(mask, smpr, up_uv).rgb;
    let tile_mask = mask;
    let tile_shadows = noisyFalloff.aaa;
    let lights = noisyFalloff.rgb;
    let normals = textureSample(col, smpr, uv * vec2(1.0, 0.5) + vec2(0.0, 0.5));
    o = normals.aaa * mask_tex.rgb * clamp(noisyFalloff.a, 0.01, 1.0) + mix(lights * tile_mask, lights, 1.0 - tile_shadows);
    return vec4(o, 1.0);
    // return vec4(c.rgb, 1.0);
} 

