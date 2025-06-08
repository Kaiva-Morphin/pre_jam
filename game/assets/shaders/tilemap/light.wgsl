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
    color: vec3<f32>,
    rotation: f32,
    intensity: f32
}

@group(2) @binding(0) var<uniform> in: Inputs;

@group(2) @binding(1) var<uniform> emitters: array<LightEmitter, 64>;

@group(2) @binding(2) var occluders_sampler: sampler;
@group(2) @binding(3) var occluders: texture_2d<f32>;

@group(2) @binding(4) var noise_sampler: sampler;
@group(2) @binding(5) var noise: texture_3d<f32>;

fn apply_kernel(
    uv: vec2<f32>,
) -> f32 {
    let global_offset = 2.0;
    var edge_dst = 1337.0;
    let kernelSize = 8;
    let halfKernel = kernelSize / 2;
    var total = 0.0;
    let texelSize = vec2<f32>(1.0 / f32(in.width), 1.0 / f32(in.height)) * 1.5;
    var weight_sum = 0.0;
    for (var x = -halfKernel; x <= halfKernel; x++) {
        for (var y = -halfKernel; y <= halfKernel; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texelSize;
            let dist = length(vec2<f32>(f32(x), f32(y)));
            let alpha = textureSample(occluders, occluders_sampler, uv + offset * global_offset).a;
            // air adds more shadow
            let weight = (1.0 - alpha) / (dist + 1.0); // prevent div by 0
            total += weight;
            weight_sum += 1.0 / (dist + 1.0);
        }
    }
    var result_global = total / weight_sum * textureSample(occluders, occluders_sampler, uv).a;
    return result_global;
}
@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let px_size = vec2(f32(in.width), f32(in.height));
    let px_uv = uv * px_size - px_size * 0.5;
    let aspect = f32(in.height) / f32(in.width);

    let result_global = apply_kernel(uv);
    let MAX_SAMPLES = 512.0;
    let step = 2.0;

    var total_light = vec3(0.0);
    var v = 0;
    // { radius_px: 6.0, spot: 1.0, color_and_rotation: Vec4(0.007843138, 0.011764706, 0.015686275, 7.0) }
    let em = emitters[0];
    
    for (var idx = 0; idx < i32(in.emitters); idx = idx + 1) {
        let emitter = emitters[idx];
        v += 1;
        let light_pos = emitter.pos * vec2(1.0, -1.0);
        let light_radius = emitter.radius;
        let light_color = emitter.color.rgb;

        let t = px_uv - light_pos;
        let dist = length(t);
        if dist > light_radius {
            continue;
        }

        let dir = normalize(t);
        var angular_falloff = 1.0;
        var angular_noise_d = 1.0;
        if emitter.spot > 0.0 {
            let rot = emitter.rotation / 180.0 * 3.1415926;
            let spot_dir = vec2(cos(rot), sin(rot));
            let to_pixel_dir = dir;
            let cutoff = cos(emitter.spot / 360.0 * 3.1415926);
            let dot_val = clamp(dot(spot_dir, to_pixel_dir), -1.0, 1.0);
            let angle = acos(dot_val);
            angular_falloff = smoothstep(emitter.spot / 360.0 * 3.14, -emitter.spot / 360.0 * 3.14 , angle);
            // let outer_cutoff = cos((rot));
            // let t = clamp((dot_val - outer_cutoff) / (cutoff - outer_cutoff), 0.0, 1.0);
            // angular_falloff = t * t * t * (t * (6.0 * t - 15.0) + 10.0);
            // angular_falloff = pow(t, 1.0);
            // angular_falloff = t * t * (3.0 - 2.0 * t);
            // let half_angle = radians(emitter.spot * 0.5);
            // let angle_diff = abs(half_angle * 2.0);
            // let angular_falloff = smoothstep(half_angle, -half_angle, angle_diff);
            
            angular_noise_d = angular_falloff ;
            if dot_val < cutoff * 0.8 {
                continue;
            }
        }


        // SHADOW TRACING
        var occlusion = 0.0;
        var samples = 0.0;
        var occlusion_in_a_row = 0;
        for (var i = 0.0; i < MAX_SAMPLES; i = i + 1.0) {
            let walked = i * step;
            if walked >= dist {
                occlusion_in_a_row = 0;
                break;
            }
            let p = light_pos + dir * walked;
            let u = (p + px_size * 0.5) / px_size;
            let v = textureSample(occluders, occluders_sampler, u).a;
            occlusion += v;
            samples += 1.0;
        }
        

        let v = 1.0 - clamp(occlusion / samples, 0.0, 1.0);
        let falloff = pow(1.0 - clamp(dist / light_radius, 0.0, 1.0), 2.0);
        if v > 0.8 {
            let noise_val = pow(textureSample(noise, noise_sampler, vec3(uv * aspect * 1.4, in.time * 0.3)).r, 2.0);
            let d = clamp((dist / light_radius * angular_noise_d), 0.0, 1.0);

            // let edge_blend = smoothstep(0.85, 1.0, 1.0 - falloff * angular_falloff);
            let edge_blend = smoothstep(0.3, 1.0, d);
            let noise_strength = mix(0.0, 1.0, edge_blend);
            let noise_factor = mix(1.0, noise_val, noise_strength);
            total_light += light_color * falloff * v * noise_factor * emitter.intensity * angular_falloff;
        }
    }
    // if total_light.b > 0.0 {return vec4(1.0, 1.0, 0.0, 1.0);};
    // var normal = normalize(vec3(textureSample(norm, smpr, uv).rg, 1.0) * 2.0 - 1.0);
    // var dot_val = dot(normal, normalize(vec3(uv - ((emitters[0].pos) + px_size * 0.5) / px_size, 0.0))) + 0.5;
    
    // return vec4(1.0, 1.0, 1.0, 1.0);
    // if uv.y > 0.5 {
    // return vec4(total_light, result_global);

    

    // return vec4(res, 1.0);
    return vec4(total_light, result_global);
    // return vec4(result_global);
    // } else {
    // }
}
