@group(2) @binding(1)
var<uniform> GrassUniforms: struct {
    pos: vec2<f32>,
    _pad: vec2<f32>,
};
@group(2) @binding(6)
var<uniform> TimeUniforms: struct {
    time: f32,
    _pad: vec3<f32>,
};
@group(2) @binding(2) var sprite_texture: texture_2d<f32>;
@group(2) @binding(3) var sprite_texture_sampler: sampler;
@group(2) @binding(4) var velbuf_texture: texture_2d<f32>;
@group(2) @binding(5) var velbuf_texture_sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,   // local vertex position (e.g. [-0.5, 0.5])
    @location(1) uv: vec2<f32>,         // texture coordinates [0, 1]
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // used by rasterizer
    @location(0) uv: vec2<f32>,                  // pass through
    @location(1) screen_uv: vec2<f32>,           // screen-space UV for velocity lookup
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Grass quad assumed to be in NDC space or transformed externally
    let pos = vec4<f32>(input.position, 0.0, 1.0);
    output.clip_position = pos;
    output.uv = input.uv;

    // Convert from NDC [-1,1] to screen UV [0,1]
    let ndc = pos.xy / pos.w;
    output.screen_uv = ndc * 0.5 + vec2<f32>(0.5);

    return output;
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample velocity from screen-space UV
    let velocity = textureSample(velbuf_texture, velbuf_texture_sampler, input.screen_uv).xy;
    let decoded_velocity = (velocity - vec2<f32>(0.5)) * 2.0;

    // Optional animation via time
    let wind_wave = sin(TimeUniforms.time * 3.0 + input.uv.y * 10.0);

    // Distortion
    let distortion_strength = 0.00;
    let distorted_uv = input.uv + (decoded_velocity + vec2(wind_wave, 0.0)) * distortion_strength;

    // Final texture color
    let color = textureSample(sprite_texture, sprite_texture_sampler, distorted_uv);
    return color;
}