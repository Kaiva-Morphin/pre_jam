@group(2) @binding(1) var<uniform> time: f32;
@group(2) @binding(2) var sprite_texture: texture_2d<f32>;
@group(2) @binding(3) var sprite_texture_sampler: sampler;

struct VertexOutput {
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(sprite_texture, 0));
    let uv = input.uv;
    let pixel = 1.0 / tex_size;

    let center_alpha = textureSample(sprite_texture, sprite_texture_sampler, uv).a;

    // Check 1-pixel neighbors
    let left   = textureSample(sprite_texture, sprite_texture_sampler, uv + vec2(-pixel.x, 0.0)).a;
    let right  = textureSample(sprite_texture, sprite_texture_sampler, uv + vec2(pixel.x, 0.0)).a;
    let up     = textureSample(sprite_texture, sprite_texture_sampler, uv + vec2(0.0, -pixel.y)).a;
    let down   = textureSample(sprite_texture, sprite_texture_sampler, uv + vec2(0.0, pixel.y)).a;

    // "Outer edge": transparent pixel with at least one opaque neighbor
    let is_outer_edge = center_alpha < 0.1 && (
        left > 0.1 || right > 0.1 || up > 0.1 || down > 0.1
    );

    // Glow color (e.g., animated)
    let glow = vec3<f32>(1.0, 1.0, 0.0) * abs(sin(time));

    // Output
    if (is_outer_edge) {
        return vec4<f32>(glow, 1.0);
    } else if (center_alpha > 0.1) {
        // Normal sprite pixel
        return textureSample(sprite_texture, sprite_texture_sampler, uv);
    } else {
        // Fully transparent
        return vec4<f32>(0.0);
    }
}