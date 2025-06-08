#import bevy_ui::ui_vertex_output::UiVertexOutput

struct ChainUniforms {
    chain: array<vec4<f32>, 10>,
};

@group(1) @binding(0) var<uniform> chain_uniforms: ChainUniforms;
@group(1) @binding(1) var sprite_texture: texture_2d<f32>;
@group(1) @binding(2) var sprite_texture_sampler: sampler;
@group(1) @binding(3) var base_sprite_texture: texture_2d<f32>;
@group(1) @binding(4) var base_sprite_texture_sampler: sampler;

@fragment
fn fragment(input: UiVertexOutput) -> @location(0) vec4<f32> {
    let x = input.uv.x;
    let y = input.uv.y;

    // Map x in [0,1] to index in [0,39]
    let idx_f = clamp(x * 39.0, 0.0, 39.0);
    let idx = u32(idx_f);
    let idx_next = min(idx + 1u, 39u);

    // Linear interpolate between two nearest points for smoothness
    let t = idx_f - f32(idx);
    let v0 = chain_uniforms.chain[idx / 4u][idx % 4u];
    let v1 = chain_uniforms.chain[idx_next / 4u][idx_next % 4u];
    let value = mix(v0, v1, t);

    // Map value from [0,100] to [0,1] (y axis, bottom = 0, top = 1)
    let graph_y = value / 100.0;

    // Invert y so 0 is bottom and 1 is top
    let dist = abs((1.0 - y) - graph_y);
    let thickness = 0.01;
    let line = smoothstep(thickness, 0.0, dist);

    let base_pixel = textureSample(base_sprite_texture, base_sprite_texture_sampler, input.uv);
    return base_pixel + vec4(line, line, line, 0.0);
}