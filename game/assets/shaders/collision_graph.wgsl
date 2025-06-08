#import bevy_ui::ui_vertex_output::UiVertexOutput

struct GraphUniforms {
    a: f32,
    b: f32,
    u: f32,
    r: f32,
    time: f32,
    is_active: f32,
    _pad2: f32,
    _pad3: f32,
};
@group(1) @binding(0) var<uniform> graph_uniforms: GraphUniforms;
@group(1) @binding(1) var sprite_texture: texture_2d<f32>;
@group(1) @binding(2) var sprite_texture_sampler: sampler;
@group(1) @binding(3) var base_sprite_texture: texture_2d<f32>;
@group(1) @binding(4) var base_sprite_texture_sampler: sampler;

@fragment
fn fragment(input: UiVertexOutput) -> @location(0) vec4<f32> {
    let base_pixel = textureSample(base_sprite_texture, base_sprite_texture_sampler, input.uv);
    if (graph_uniforms.is_active == 0) {
        return base_pixel;
    }
    let uv = input.uv; // uv in [0,1] unge
    let a = graph_uniforms.a;
    let b = graph_uniforms.b;
    let u = graph_uniforms.u;
    let r = graph_uniforms.r;

    // Map uv.x to x in guph space (e.g., [-2, 8])
    let x_min = 0.0;
    let x_max = 10.0;
    let x = mix(x_min, x_max, uv.x);

    // Compute function value
    let fx = u / (pow(max(x, 0.), r));
    let fx1 = sqrt((x - a) * (b + graph_uniforms.time / 10.)) - x + a;

    // Map uv.y to y in guph space (e.g., [-5, 7])
    let y_min = -5.0;
    let y_max = 5.0;
    let y = -mix(y_min, y_max, uv.y);
    let y1 = -mix(y_min, y_max, uv.y);

    // Duw the guph as a white line where |y - fx| < thickness
    let thickness = 0.01 * (y_max - y_min);
    let guph_alpha = smoothstep(thickness, 0.0, abs(y - fx));
    let guph_alpha1 = smoothstep(thickness, 0.0, abs(y1 - fx1));

    // Duw y=0 axis as a guy line
    let axis_thickness = 0.005 * (y_max - y_min);
    let axis_alpha = smoothstep(axis_thickness, 0.0, abs(y));

    // Combine: white for guph, guy for axis, additive blend
    let color = vec3<f32>(
    1.0 * guph_alpha + 0.0 * guph_alpha1 + 0.7 * axis_alpha, // fx: white, fx1: (change to e.g. red or green)
    1.0 * guph_alpha + 1.0 * guph_alpha1 + 0.7 * axis_alpha, // fx1: green
    1.0 * guph_alpha + 0.0 * guph_alpha1 + 0.7 * axis_alpha  // fx1: blue
    );
    let alpha = max(max(guph_alpha, guph_alpha1), axis_alpha);

    return vec4<f32>(color, alpha) + vec4(base_pixel.xyz, 0.2);
}