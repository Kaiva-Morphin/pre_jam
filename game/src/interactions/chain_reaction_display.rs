use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{AlphaMode2d, Material2d}};

use super::components::InInteractionArray;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChainGraphMaterial {
    #[uniform(1)]
    pub chain: f32,
    #[texture(2)]
    #[sampler(3)]
    pub sprite_handle: Handle<Image>,
}

const CHAINGRAPH_MATERIAL_PATH: &str = "shaders/chain_graph.wgsl";

impl Material2d for ChainGraphMaterial {
    fn fragment_shader() -> ShaderRef {
        CHAINGRAPH_MATERIAL_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Resource)]
pub struct ChainGraphMaterialHandle {
    pub handle: Handle<Image>
}

pub fn open_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    already_spawned: Local<Option<Entity>>,
    chain_graph_material_handle: Res<ChainGraphMaterialHandle>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_interaction[0] {
            commands.entity(entity).despawn();
        }
    } else {
        if in_interaction_array.in_interaction[0] {
            commands.spawn(
                Node {
                    left: Val::Percent(50.),
                    right: Val::Percent(50.),
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    ..default()
                }
            ).with_child(
                ImageNode {
                    image: chain_graph_material_handle.handle.clone(),
                    ..default()
                }
            );
        }
    }
}