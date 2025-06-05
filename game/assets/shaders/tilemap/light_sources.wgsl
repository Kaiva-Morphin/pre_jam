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
    spot: f32,
    rotation: f32,
    color: vec3<f32>,
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
    let px_size = vec2(f32(in.width), f32(in.height));
    let px_up_uv = up_uv * px_size - px_size * 0.5;
    let aspect = f32(in.height) / f32(in.width);

    let result_global = apply_kernel(up_uv);

    let MAX_SAMPLES = 128.0;
    let step = 4.0;

    var total_light = vec3(0.0);
    for (var idx = 0; idx < i32(in.emitters); idx = idx + 1) {
        let emitter = emitters[idx];

        let light_pos = emitter.pos;
        let light_radius = emitter.radius;
        let light_color = emitter.color.rgb;

        let t = px_up_uv - light_pos;
        let dist = length(t);
        if dist > light_radius {
            continue;
        }

        let dir = normalize(t);

        if emitter.spot > 0.0 {
            let spot_dir = vec2(cos(emitter.rotation), sin(emitter.rotation));
            let to_pixel_dir = normalize(px_up_uv - emitter.pos);
            let cutoff = cos(emitter.spot);
            if dot(spot_dir, to_pixel_dir) < cutoff {
                continue;
            }
        }


        // SHADOW TRACING
        var occlusion = 0.0;
        var samples = 0.0;
        for (var i = 0.0; i < MAX_SAMPLES; i = i + 1.0) {
            let walked = i * step;
            if walked >= dist {
                break;
            }
            let p = light_pos + dir * walked;
            let u = (p + px_size * 0.5) / px_size;
            let v = textureSample(mask, smpr, u).a;
            occlusion += v;
            samples += 1.0;
        }

        let v = 1.0 - clamp(occlusion / samples, 0.0, 1.0);
        let falloff = pow(1.0 - clamp(dist / light_radius, 0.0, 1.0), 2.0);

        if v > 0.7 {
            let noise_val = pow(textureSample(noise, noise_smpr, vec3(up_uv * aspect * 1.4, in.time * 0.3)).r, 2.0);
            let d = clamp(dist / light_radius, 0.0, 1.0);
            let edge_blend = smoothstep(0.3, 1.0, d);
            let noise_strength = mix(0.0, 1.0, edge_blend);
            let noise_factor = mix(1.0, noise_val, noise_strength);

            total_light += light_color * falloff * v * noise_factor;
        }
    }

    if uv.y > 0.5 {
        var normal = normalize(vec3(textureSample(norm, smpr, up_uv).rg, 1.0) * 2.0 - 1.0);
        var dot_val = dot(normal, normalize(vec3(up_uv - ((emitters[0].pos) + px_size * 0.5) / px_size, 0.0))) + 0.5;
        return vec4(normal, dot_val);
    } else {
        return vec4(total_light, result_global);
    }


    if uv.y > 0.5 {
        let normal = normalize(vec3(textureSample(norm, smpr, up_uv).rg, 1.0) * 2.0 - 1.0);
        let dotval = dot(normal, normalize(vec3(up_uv - ((emitters[0].pos) + px_size * 0.5) / px_size, 0.0))) + 0.5;
        return vec4(normal, dotval);
    } else {
        return vec4(total_light, result_global);
    }
}
