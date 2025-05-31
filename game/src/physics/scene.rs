use bevy::{asset::RenderAssetUsages, color::palettes::css::{PURPLE, YELLOW}, prelude::*, render::{mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexFormat}, render_resource::{AsBindGroup, ShaderRef}}, sprite::Material2d};
use bevy_rapier2d::prelude::*;

use crate::{camera::plugin::CameraFocus, physics::{controller::Controller, platforms::{MovingPlatform, MovingPlatformMode}}};



pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_player, 
                init_scene
            ));
    }
}




#[derive(Component)]
pub struct Player;


pub fn spawn_player(
    mut cmd: Commands,
    assets: Res<AssetServer>,

){
    cmd.spawn((
        RigidBody::Dynamic,
        Transform::from_xyz(0.0, 100.0, 0.0),
        Velocity::zero(),
        Player,
        Dominance::group(0),
        GravityScale(0.0),
        Name::new("Player"),
        Collider::capsule(vec2(0.0, 6.0), vec2(0.0, -6.0), 6.0),
        Sprite::from_image(assets.load("pixel/test.png")),
        LockedAxes::ROTATION_LOCKED,
        Sleeping::disabled(),
        Ccd::enabled(),
        CameraFocus{priority: 0},
        Controller{
            horisontal_velocity: 0.0,
            max_horisontal_velocity: 100.0,
            total_air_jumps: 2,
            ..default()
        }
    ));
}

pub fn init_scene(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    cmd.spawn((
        RigidBody::Fixed,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(100.0, 5.0),
    ));
    cmd.spawn((
        RigidBody::Dynamic,
        Dominance::group(0),
        Name::new("Box"),
        // GravityScale(1.0),
        // Velocity::zero(),
        Transform::from_xyz(125.0, 25.0, 0.0),
        Collider::cuboid(10.0, 10.0),
        // Sleeping::disabled(),
        // Ccd::enabled(),

    ));
    // bevy::render::render_resource::RenderPipelineDescriptor ;
    cmd.spawn((
        Collider::cuboid(25.0, 5.0),
        MovingPlatform::bundle(
            vec![
                vec2(125.0, 0.0),
                vec2(125.0, 100.0),
                vec2(225.0, 100.0),
                vec2(225.0, 0.0),
            ],
            2.0, 
            MovingPlatformMode::Loop
        ),
    ));
    let mut line = Mesh::new(
        PrimitiveTopology::LineList,
        RenderAssetUsages::RENDER_WORLD,
    );
    line.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![vec3(10.0, 5.0, 0.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0), vec3(15.0, 15.0, 0.0)]);
    let v_color: Vec<[f32; 4]> = vec![[0.4, 0.6, 1.0, 1.0]; 4];
    line.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);
    line.insert_indices(Indices::U32(vec![0, 1, 2, 3]));
    cmd.spawn((
        Mesh2d(meshes.add(line)),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        // MeshMaterial2d::from(),
    ));

    let mut star = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Vertices need to have a position attribute. We will use the following
    // vertices (I hope you can spot the star in the schema).
    //
    //        1
    //
    //     10   2
    // 9      0      3
    //     8     4
    //        6
    //   7        5
    //
    // These vertices are specified in 3D space.
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..10 {
        // The angle between each vertex is 1/10 of a full rotation.
        let a = i as f32 * std::f32::consts::PI / 5.0;
        // The radius of inner vertices (even indices) is 100. For outer vertices (odd indices) it's 200.
        let r = (1 - i % 2) as f32 * 100.0 + 100.0;
        // Add the vertex position.
        v_pos.push([r * ops::sin(a), r * ops::cos(a), 0.0]);
    }
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);


    let mut indices = vec![0, 1, 10];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    star.insert_indices(Indices::U32(indices));
    cmd.spawn((
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Mesh2d(meshes.add(star)),
    ));
}

#[derive(TypePath, Clone, AsBindGroup, Asset)]
pub struct RopeMaterial{
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}
impl Material2d for RopeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rope.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/rope.wgsl".into()
    }
}
