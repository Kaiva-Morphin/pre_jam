use bevy::{asset::RenderAssetUsages, color::palettes::css::{PURPLE, YELLOW}, prelude::*, render::{mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexFormat}, render_resource::{AsBindGroup, ShaderRef}}, sprite::Material2d};
use bevy_rapier2d::prelude::*;

use crate::{camera::plugin::CameraFocus, physics::{controller::Controller, platforms::{MovingPlatform, MovingPlatformMode}}};



pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                init_scene
            ));
    }
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
