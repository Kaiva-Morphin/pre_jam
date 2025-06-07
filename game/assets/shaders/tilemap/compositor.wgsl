#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct Inputs {
    time: f32,
    width: u32,
    height: u32,
    _b: u32,
}

@group(2) @binding(0) var<uniform> in: Inputs;

@group(2) @binding(1) var occ_sampler: sampler;
@group(2) @binding(2) var occlusion: texture_2d<f32>;
@group(2) @binding(3) var scene_sampler: sampler;
@group(2) @binding(4) var scene: texture_2d<f32>;

@group(2) @binding(5) var noise: texture_3d<f32>;
@group(2) @binding(6) var noise_smpr: sampler;

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
            let v = textureSample(occlusion, occ_sampler, uv + offset);
            color += v;
            // masked_color += v * textureSample(mask, smpr, uv + offset);
        }
    }
    let totalSamples = f32(kernelSize * kernelSize);
    var result_color = color / totalSamples;
    // var result_masked = masked_color / totalSamples;
    result_color.r = min(result_color.r, 1.0);
    result_color.g = min(result_color.g, 1.0);
    result_color.b = min(result_color.b, 1.0);


    result_color.r = max(result_color.r, 0.2);
    result_color.g = max(result_color.g, 0.2);
    result_color.b = max(result_color.b, 0.2);



    let e = 1.0 - (result_color.r + result_color.g + result_color.b) / 3.0;
    // var noise = textureSample(noise, noise_smpr, vec3(uv * width / height * 1.0, in.time * 0.5)).r;
    // noise *= 1.0;
    // // noise += 0.1;
    // let edgeFactor = 1.0 - (result_color.r + result_color.g + result_color.b) / 4.0;
    var u = uv * vec2(width, height);
    u.x = floor(u.x / 3.0) * 3.0;
    u.y = floor(u.y / 3.0) * 3.0;
    let noise = white_noise_3d(vec3(u, in.time * 0.01)) * white_noise_3d(vec3(uv, in.time * 0.01));
    // let noise = white_noise_3d(vec3(uv, in.time * 0.01));
    let noisyFalloff = result_color * mix(1.0, noise, e * noise);
    let mask_tex = textureSample(scene, scene_sampler, uv);
    let mask = mask_tex.a;
    let c = textureSample(occlusion, occ_sampler, uv);
    var o = noisyFalloff.rgb;
    let tile_mask = mask;
    let tile_shadows = noisyFalloff.aaa;
    let lights = noisyFalloff.rgb;
    o = mask_tex.rgb * clamp(noisyFalloff.a, 0.01, 1.0) + mix(lights * tile_mask, lights, 1.0 - tile_shadows);

    
    // return vec4(noisyFalloff.rgb, 1.0)  * textureSample(scene, scene_sampler, uv);
    let scene_color = textureSample(scene, scene_sampler, uv).rgb;
    var light_color = 0.0;
    var ambient_light = vec3(0.2);
    var diffuse_light = light_color;
    // return vec4(light_color + ambient_light, 1.0);
    // if (mask > 0.0) {
    //     return vec4(textureSample(scene, scene_sampler, uv).aaa, 1.0);
    // } else {
    //     return vec4(o, 1.0);
    // }
    return vec4(0.0);
    // let r = textureSample(occlusion, occ_sampler, uv);
    // let s = textureSample(scene, scene_sampler, uv);
    // vec4(uv, 1.0, 1.0) * 
    // return vec4(r.aaa, s.a);
    // return vec4(r.rgb, s.a) ;
    // return vec4(c.rgb, 1.0);
} 

