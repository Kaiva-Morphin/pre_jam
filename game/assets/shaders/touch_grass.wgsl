@group(2) @binding(1) var<uniform> pos: vec2<f32>;
@group(2) @binding(2) var sprite_texture: texture_2d<f32>;
@group(2) @binding(3) var sprite_texture_sampler: sampler;
@group(2) @binding(4) var velbuf_texture: texture_2d<f32>;
@group(2) @binding(5) var velbuf_texture_sampler: sampler;

struct VertexOutput {
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let value = textureSample(sprite_texture, sprite_texture_sampler, in.uv);
    return value;
}