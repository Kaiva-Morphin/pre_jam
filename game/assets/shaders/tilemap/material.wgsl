#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;
@group(2) @binding(3) var norm_tex: texture_2d<f32>;
@group(2) @binding(4) var norm_sampler: sampler;

fn rotate2d(vec: vec2<f32>, angle: f32) -> vec2<f32> {
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    let rotation_matrix = mat2x2<f32>(
        cos_a, -sin_a,
        sin_a,  cos_a
    );
    return rotation_matrix * vec;
}

fn compute_lighting(normal: vec3<f32>, light_dir: vec3<f32>) -> f32 {
    return max(dot(normalize(normal), normalize(light_dir)), 0.0);
}

fn get_light_dir(time: f32) -> vec3<f32> {
    let x = cos(time);
    let y = sin(time);
    return normalize(vec3<f32>(x, y, 1.0));
}


@fragment
fn fragment(@location(2) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let n = normalize(vec3(textureSample(norm_tex, norm_sampler, uv).xy, 1.0) * 2.0 - 1.0); 
    let c = textureSample(base_color_texture, base_color_sampler, uv); 
    var s = get_light_dir(0.5);
    // c.x = dot(s, n);
    // c.y = dot(s, n);0
    var l = max(dot(s, n), 0.5);
    l = l * l;
    // return vec4(l, l, l, 1.0);
    return c * l;
}


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
