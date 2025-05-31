@group(2) @binding(1) var<uniform> screen_size: vec2<f32>;
@group(2) @binding(2) var<uniform> player_pos: vec2<f32>; // this doesnt fetch
@group(2) @binding(3) var buffer_texture: texture_2d<f32>;
@group(2) @binding(4) var buffer_texture_sampler: sampler;
@group(2) @binding(5) var debug_texture: texture_2d<f32>;
@group(2) @binding(6) var debug_texture_sampler: sampler;

struct VertexOutput {
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let value = textureSample(debug_texture, debug_texture_sampler, in.uv).xy;
    return vec4(value, 0., 1.);
}

@group(0) @binding(0) var compute_buffer_texture: texture_storage_2d<rgba32float, read_write>;
@group(0) @binding(1) var compute_debug_texture: texture_storage_2d<rgba32float, read_write>;
@group(0) @binding(2) var<storage, read_write> parameters: array<vec2<f32>>;
@group(0) @binding(3) var<storage, read_write> compute_player_pos: array<vec2<f32>>;

@compute @workgroup_size(8, 8)
fn update(@builtin(global_invocation_id) id: vec3<u32>) {
    var screen_size = parameters[0];
    let coords = vec2<f32>(id.xy) / screen_size;
    let player_pos = compute_player_pos[0];
    let player_vel = compute_player_pos[1] * 80.;
    let player_box_hs = vec2(0.025, 0.05);
    let box_min = player_pos - player_box_hs;
    let box_max = player_pos + player_box_hs;

    var box = 0.0;
    if (coords.x >= box_min.x && coords.x <= box_max.x &&
        coords.y >= box_min.y && coords.y <= box_max.y) {
        box = 1.0;
    }
    //-stiffness * deviation + vec4(player_vel * damping * box, 0.,0.,);
    // velocity
    let prev_velocity = textureLoad(compute_buffer_texture, id.xy).zw;
    let velocity_deviation = vec2(0.5, 0.5) - prev_velocity;
    let acceleration = velocity_deviation * 0.05 + player_vel * box;
    let new_acceleration = vec4(0., 0., prev_velocity + acceleration);
    // displacement
    let prev_displacement = textureLoad(compute_buffer_texture, id.xy).xy;
    let displacement_deviation = vec2(0.5, 0.5) - prev_displacement;
    let displacement_velocity = displacement_deviation * 0.05 + player_vel * box + acceleration * 2.; // need accel for momentum
    let new_displacement_velocity = vec4(prev_displacement + displacement_velocity, 0., 0.);

    let new_texture = new_acceleration + new_displacement_velocity;
    textureStore(compute_buffer_texture, id.xy, new_texture);
    // textureStore(compute_debug_texture, id.xy, vec4(displacement_velocity,0.,0.));
    textureStore(compute_debug_texture, id.xy, new_displacement_velocity);
}

@compute @workgroup_size(10, 10)
fn init(@builtin(global_invocation_id) id: vec3<u32>) {
    // let coords = vec2<i32>(id.xy);
    // // Example: Write white to every pixel
    // textureStore(compute_buffer_texture, coords, vec4<f32>(1.0, 1.0, 1.0, 1.0));
}