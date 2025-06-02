struct ChainUniforms {
    chain: f32,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
};
@group(2) @binding(0) var<uniform> chain_uniforms: ChainUniforms;
@group(2) @binding(1) var sprite_texture: texture_2d<f32>;
@group(2) @binding(2) var sprite_texture_sampler: sampler;
@group(2) @binding(3) var base_sprite_texture: texture_2d<f32>;
@group(2) @binding(4) var base_sprite_texture_sampler: sampler;

struct VertexOutput {
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let x = input.uv.x * 2.0 * 3.1415926;
    let y = input.uv.y;
    let f = 0.5 * sin(x + chain_uniforms.chain) + 0.5;
    let dist = abs(y - f);
    let thickness = 0.01;
    let line = smoothstep(thickness, 0.0, dist);
    let base_pixel = textureSample(base_sprite_texture, base_sprite_texture_sampler, input.uv);
    return base_pixel + vec4(line, line, line, 0.);
    // return base_pixel;
}