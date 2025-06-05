#import bevy_ui::ui_vertex_output::UiVertexOutput

struct GraphUniforms {
    a: f32,
    b: f32,
    ra: f32,
    rb: f32,
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
    let uv = input.uv;

    // Function definitions
    let f1 = sqrt(max((uv.x - graph_uniforms.a) * graph_uniforms.b, 0.0)) - uv.x + graph_uniforms.a;
    let f2 = sqrt(max((uv.x - graph_uniforms.ra) * graph_uniforms.rb, 0.0)) - uv.x + graph_uniforms.ra;

    // Map function values to UV space (assuming y in [0,1])
    let y1 = 0.5 + 0.4 * f1; // scale and center
    let y2 = 0.5 + 0.4 * f2;

    // Line thickness
    let thickness = 0.01;

    // Draw lines: red for (a, b), green for (ra, rb)
    let line1 = smoothstep(thickness, 0.0, abs(uv.y - y1));
    let line2 = smoothstep(thickness, 0.0, abs(uv.y - y2));

    // Base pixel
    let base_pixel = textureSample(base_sprite_texture, base_sprite_texture_sampler, uv);

    // Overlay lines
    let color = base_pixel.rgb +
        vec3(line1, line2, 0.0); // red and green channels

    return vec4(color, 1.0);
}