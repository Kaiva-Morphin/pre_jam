#import bevy_ui::ui_vertex_output::UiVertexOutput

struct GraphUniforms {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    ra: f32,
    rb: f32,
    rc: f32,
    rd: f32,
    time: f32,
    _pad1: f32,
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
    let x = input.uv.x * 2.0 * 3.1415926;
    let y = input.uv.y;

    // Animate both waves by shifting phase with time
    let speed = 1.0; // adjust for desired speed
    let f = graph_uniforms.a + graph_uniforms.b * sin((x + graph_uniforms.c + graph_uniforms.time * speed) * graph_uniforms.d);
    let t = graph_uniforms.ra + graph_uniforms.rb * sin((x + graph_uniforms.rc + graph_uniforms.time * speed) * graph_uniforms.rd);

    let dist_f = abs(y - f);
    let dist_t = abs(y - t);

    let thickness = 0.01;
    let aa = 0.0005;

    let line_f = smoothstep(thickness + aa, thickness - aa, dist_f);
    let line_t = smoothstep(thickness + aa, thickness - aa, dist_t);

    let base_pixel = textureSample(base_sprite_texture, base_sprite_texture_sampler, input.uv);

    // Draw f as red, t as green, both as white where they overlap
    let color = vec4(line_f, line_t, 0.0, 0.0);

    return base_pixel + color;
    // return vec4(graph_uniforms.a);
}