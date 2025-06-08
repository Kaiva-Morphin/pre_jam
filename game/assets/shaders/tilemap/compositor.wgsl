#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct Inputs {
    time: f32,
    width: u32,
    height: u32,
    _b: u32,
}

@group(2) @binding(0) var<uniform> in: Inputs;

@group(2) @binding(1) var light_sampler: sampler;
@group(2) @binding(2) var light: texture_2d<f32>;

@group(2) @binding(3) var occluders_sampler: sampler;
@group(2) @binding(4) var occluders: texture_2d<f32>;

@group(2) @binding(5) var scene_sampler: sampler;
@group(2) @binding(6) var scene: texture_2d<f32>;

@group(2) @binding(7) var noise: texture_3d<f32>;
@group(2) @binding(8) var noise_smpr: sampler;

@group(2) @binding(9) var bg_sampler: sampler;
@group(2) @binding(10) var bg: texture_2d<f32>;


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
    let height = f32(in.height);
    let texelSize = vec2<f32>(1.0 / f32(in.width), 1.0 / f32(in.height)) * 2.0;
    var color = vec4<f32>(0.0);
    // var masked_color = vec4<f32>(0.0);


    // wibecode 🤘
    let kernelSize = 6u;
    let numDirections = 32u;
    let numSteps = 5u;
    for (var d = 0u; d < numDirections; d = d + 1u) {
        let angle = f32(d) * 6.2831853 / f32(numDirections); // 2π
        let dir = vec2<f32>(cos(angle), sin(angle));

        for (var s = 1u; s <= numSteps; s = s + 1u) {
            let stepSize = f32(s) / f32(numSteps);
            let offset = dir * stepSize * f32(kernelSize) * texelSize;
            let v = textureSample(light, light_sampler, uv + offset);
            color += v;
        }
    }

    // Add center sample if you want to include the original pixel
    color += textureSample(light, light_sampler, uv);

    let totalSamples = f32(numDirections * numSteps + 1u);
    let result_color = color / totalSamples;
    // var kernelSize = 16;
    // var halfKernel = kernelSize / 2;
    // for (var x = -halfKernel; x <= halfKernel; x++) {
    //     for (var y = -halfKernel; y <= halfKernel; y++) {
    //         let offset = vec2<f32>(f32(x), f32(y)) * texelSize;
    //         let v = textureSample(light, light_sampler, uv + offset);
    //         color += v;
    //         // masked_color += v * textureSample(mask, smpr, uv + offset);
    //     }
    // }
    // let totalSamples = f32(kernelSize * kernelSize);
    // var result_color = color / totalSamples;

    // var result_masked = masked_color / totalSamples;
    // result_color.r = min(result_color.r, 1.0);
    // result_color.g = min(result_color.g, 1.0);
    // result_color.b = min(result_color.b, 1.0);





    let e = 1.0 - (result_color.r + result_color.g + result_color.b) / 3.0;
    // var noise = textureSample(noise, noise_smpr, vec3(uv * width / height * 1.0, in.time * 0.5)).r;
    // noise *= 1.0;
    // // noise += 0.1;
    // let edgeFactor = 1.0 - (result_color.r + result_color.g + result_color.b) / 4.0;
    var u = uv * vec2(width, height);
    u.x = floor(u.x / 2.0) * 2.0;
    u.y = floor(u.y / 2.0) * 2.0;
    let noise = white_noise_3d(vec3(u, in.time * 0.01)) * white_noise_3d(vec3(uv * vec2(width, height), in.time * 0.01));
    // let noise = white_noise_3d(vec3(uv, in.time * 0.01));
    let noisyFalloff = result_color * mix(1.0, noise, e * noise);
    let mask_tex = textureSample(occluders, occluders_sampler, uv);
    let mask = mask_tex.a;
    var o = noisyFalloff.rgb;
    let tile_mask = mask;
    let non_tiles_mask = 1.0 - mask;
    let tile_shadows = noisyFalloff.aaa;
    let lights = noisyFalloff.rgb;
    o = mask_tex.rgb * clamp(noisyFalloff.a, 0.01, 1.0);

    let occluders_color = textureSample(occluders, occluders_sampler, uv).rgb;
    
    
    // return vec4(noisyFalloff.rgb, 1.0)  * textureSample(occluders, occluders_sampler, uv);
    let scene_color = textureSample(scene, scene_sampler, uv).rgba;
    var light_color = 0.0;
    var ambient_light = vec3(0.03);
    var diffuse_light = light_color;


    // let tiles = (ambient_light  + (mix(lights * tile_mask, lights, 1.0 - tile_shadows)) * tile_mask) * occluders_color;
    let tiles = (ambient_light * tile_shadows + ambient_light  + (mix(lights * tile_mask, lights, 1.0 - tile_shadows)) * tile_mask) * occluders_color;

    let scene = scene_color.rgb * (ambient_light + noisyFalloff.rgb) * non_tiles_mask * scene_color.a;

    let bg = textureSample(bg, bg_sampler, uv).rgb * non_tiles_mask * (1.0 - scene_color.a);
    // return vec4(noise, noise, noise, 1.0); 
    // return vec4(scene + tiles, 1.0);
    return vec4(scene + tiles + bg, 1.0);
    // let result = mix(lights, occluders_color, clamp(lights, vec3(0.0), vec3(1.0)));
    // mix(0.0, lights.r, 1.0 - clamp(lights.r, 0.0, 1.0)),
    // return vec4(
    //     mask 
    //     *  occluders_color
    //     + lights 
    //     * tile_mask 
    //     + lights 
    //     * tile_mask 
    //     + scene
    //     , 1.0); 
    // if (mask > 0.0) {
    // } else {
        // return vec4(mask, 1.0);
    // }

    // return vec4(textureSample(light, light_sampler, uv).rgb,1.0);
    // let r = textureSample(occlusion, occ_sampler, uv);
    // let s = textureSample(occluders, occluders_sampler, uv);
    // vec4(uv, 1.0, 1.0) * 
    // return vec4(r.aaa, s.a);
    // return vec4(r.rgb, s.a) ;
    // return vec4(c.rgb, 1.0);
} 


fn overlay(base: f32,  blend: f32) -> f32 {
    if (base < 0.5) {
        return 2.0 * base * blend;
    } else {
        return 1.0 - 2.0 * (1.0 - base) * (1.0 - blend);
    }
}

fn overlay_vec3(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return vec3(
        overlay(base.r, blend.r),
        overlay(base.g, blend.g),
        overlay(base.b, blend.b)
    );
}