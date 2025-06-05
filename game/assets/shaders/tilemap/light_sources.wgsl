#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct Inputs {
    time: f32,
    width: u32,
    height: u32,
    emitters: u32,
}

struct LightEmitter {
    pos: vec2<f32>,
    radius: f32,
    angle: f32,
    color: vec4<f32>,
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
@group(2) @binding(10) var<uniform> emitters: array<LightEmitter, 64>;

fn to_up_uv(p: vec2<f32>, px: vec2<f32>) -> vec2<f32> {
    return p / px;
}
fn to_px(p: vec2<f32>, px: vec2<f32>) -> vec2<f32> {
    return p * px;
}


fn apply_kernel(
    up_uv: vec2<f32>,
) -> f32 {
    let global_offset = 2.0;
    var edge_dst = 1337.0;
    let kernelSize = 16;
    let halfKernel = kernelSize / 2;
    var total = 0.0;
    let texelSize = vec2<f32>(1.0 / f32(in.width), 1.0 / f32(in.height)) * 3.0;
    var weight_sum = 0.0;
    for (var x = -halfKernel; x <= halfKernel; x++) {
        for (var y = -halfKernel; y <= halfKernel; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texelSize;
            let dist = length(vec2<f32>(f32(x), f32(y)));
            let alpha = textureSample(mask, smpr, up_uv + offset * global_offset).a;
            // air adds more shadow
            let weight = (1.0 - alpha) / (dist + 1.0); // prevent div by 0
            total += weight;
            weight_sum += 1.0 / (dist + 1.0);
        }
    }
    var result_global = total / weight_sum * textureSample(mask, smpr, up_uv).a;
    return result_global;
}

@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let up_uv = uv * vec2(1.0, 2.0) % vec2(1.0, 1.0);
    let width = f32(in.width);
    let height = f32(in.height);
    let px_size = vec2(width, height);
    let px_up_uv = up_uv * px_size - px_size * 0.5;
    let aspect = height / width;

    let result_global = apply_kernel(up_uv);

    let light_pos = vec2(cos(in.time) * 130.0 * 3.0, 10.0 + cos(in.time));
    let light_radius = 250.0;
    let light_color = vec3(0.92, 0.8, 0.9);

    let MAX_SAMPLES = 128.0;
    let step = 4.0;

    let t = px_up_uv - light_pos;
    let dist = length(t);
    let dir = normalize(t);
    
    var falloff = pow(1.0 - clamp(dist / light_radius, 0.0, 1.0), 2.0);
    var occlusion = 0.0;
    var samples = 0.0;
    for (var i = 0; i < i32(in.emitters); i = i + 1) {
        
        for (var i = 0.0; i < MAX_SAMPLES; i = i + 1.0) {
            let walked = i * step;
            let p = light_pos + dir * walked;
            if walked >= dist {
                break;
            }
            samples += 1.0;
            let u = (p + px_size * 0.5) / px_size;
            let v = textureSample(mask, smpr, u).a;
            occlusion += v;
        }
    }
    
    
    // occlusion /= samples;
    // occlusion = 1.0 - occlusion;
    // falloff = smoothstep(light_radius, 0.0, dist);
     // pow(dist / light_radius, 2.0

    // occlusion = min(occlusion, 1.0);
    // var falloff = 1.0 - dist / light_radius;
    // falloff = pow(1.0 - clamp(dist / light_radius, 0.0, 1.0), 2.0);
    // var raw_light = falloff * (1.0 - occlusion);
    // let noise = pow(textureSample(noise, noise_smpr, vec3(up_uv * 1.0, in.time * 0.4)).r, 2.0);
    // let noise_edge = (0.25 - (raw_light * 0.25)) * 4.0 - 0.75;
    // let noised =  raw_light * mix(1.0, noise, noise_edge * noise);
    // return vec4(vec3(noised), 1.0);
    // occlusion = min(occlusion, 1.0);

    var v = 1.0 - clamp(occlusion / samples, 0.0, 1.0);
    var o = 0.0;
    if v > 0.7 {
        o = v;
    }
    var raw_light = falloff * (1.0 - occlusion);
    raw_light = falloff * o;
    let noise_val = pow(textureSample(noise, noise_smpr, vec3(up_uv * aspect * 1.4, in.time * 0.3)).r, 2.0);
    let d = clamp(dist / light_radius, 0.0, 1.0);
    let edge_blend = smoothstep(0.3, 1.0, d); 
    let noise_strength = mix(0.0, 1.0, edge_blend);
    let noise_factor = mix(1.0, noise_val, noise_strength);
    let noisy_light = raw_light * noise_factor;

    var normal = normalize(vec3(textureSample(norm, smpr, up_uv).rg, 1.0) * 2.0 - 1.0);
    var dot = dot(normal, normalize(vec3(up_uv - ((light_pos) + px_size * 0.5) / px_size, 0.0)));
    dot = dot + 0.5;
    // + dot
    // if up_uv
    if uv.y > 0.5 {
        return vec4(normal, dot);
    } else {
        return vec4(noisy_light * light_color, result_global); //light_color * noisy_light
    }
    // return vec4(noisy_light * light_color, result_global); //light_color * noisy_light
} 

// var normal = normalize(vec3(textureSample(norm, smpr, up_uv).rg, 1.0) * 2.0 - 1.0);
    // var d = dot(normal, normalize(vec3(up_uv - ((light_pos) + px_size * 0.5) / px_size, 0.0)));
    // d = d + 0.5;
    // return vec4(d * textureSample(col, smpr, up_uv).rgb, 1.0);

    //aspect * 


 // if normal_rgb.x > 0.25 {
    //     col.x = 1.0;
    // }
    // if normal_rgb.x > 0.25 {
    //     col.x = 1.0;
    // }
    // col.x = normal_rgb.x;
    // col.y = normal_rgb.y;
    // col.z = 1.0;
    // if normal_rgb.y > 0.25 {
    //     col.y = 1.0;
    // }
