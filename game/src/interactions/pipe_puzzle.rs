use bevy::{platform::collections::HashSet, prelude::*};

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::target::LowresUiContainer, utils::custom_material_loader::PipesAtlasHandles};

#[derive(Component)]
pub struct Pipe {
    pub flat_id: usize,
}

pub fn open_pipe_puzzle_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    pipes_atlas_handles: Res<PipesAtlasHandles>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::PipePuzzle && in_interaction_array.in_any_interaction {
            const GRID_SCREEN_SIZE: f32 = 200.;
            let mut childern = vec![];
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    childern.push(commands.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            left: Val::Px(GRID_SCREEN_SIZE / x as f32),
                            bottom: Val::Px(GRID_SCREEN_SIZE / y as f32),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ImageNode::from_atlas_image(
                            pipes_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(pipes_atlas_handles.layout_handle.clone())
                        ),
                        Pipe {flat_id: x + y * GRID_SIZE},
                    )).id());
                }
            }
            let entity = commands.spawn((
                BackgroundColor::from(Color::Srgba(Srgba::new(0., 1., 0., 0.5))),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            )).add_children(&childern).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

const GRID_SIZE: usize = 5;
/*
conns: is it connected to [up, right, down, left]
0: | [1,0,1,0]
1: L [1,1,0,0]
2: T [0,1,1,1]
3: + [1,1,1,1]
*/
const CONNECTIONS: [[usize; 4]; 4] = [
    [1,0,1,0],
    [1,1,0,0],
    [0,1,1,1],
    [1,1,1,1],
];

#[derive(Default, Clone)]
pub struct Connection {
    pub conn_type: usize,
    pub rot_state: usize,
    pub neighbors: [usize; 4],
}

impl Connection {
    fn rotate(&mut self) {

    }
    fn random() -> Self {
        let conn_type = ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * 3.) as usize;
        Self {
            conn_type,
            rot_state: 0,
            neighbors: CONNECTIONS[conn_type],
        }
    }
}

#[derive(Resource, Default)]
pub struct PipeGrid {
    pub data: Vec<Connection>,
    pub is_loaded: bool,
}

pub fn init_grid(
    mut grid: ResMut<PipeGrid>,
) {
    grid.data = vec![Connection::default(); GRID_SIZE * GRID_SIZE];
    grid.is_loaded = true;

    let mut stack = vec![0];
    let mut visited = HashSet::new();

    // Start with a random connection at the root
    grid.data[0] = Connection::random();

    while let Some(current) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }

        let neighbors = grid.data[current].neighbors;
        let x = current % GRID_SIZE;
        let y = current / GRID_SIZE;

        // Directions: [up, right, down, left]
        let directions = [
            (0, 1),   // up
            (1, 0),   // right
            (0, -1),  // down
            (-1, 0),  // left
        ];

        for (i, (dx, dy)) in directions.iter().enumerate() {
            if neighbors[i] == 0 {
                continue;
            }
            let nx = x as isize + dx * neighbors[i] as isize;
            let ny = y as isize + dy * neighbors[i] as isize;

            // Bounds check
            if nx < 0 || nx >= GRID_SIZE as isize || ny < 0 || ny >= GRID_SIZE as isize {
                continue;
            }
            let neighbor_idx = (ny as usize) * GRID_SIZE + (nx as usize);
            if !visited.contains(&neighbor_idx) {
                grid.data[neighbor_idx] = Connection::random();
                stack.push(neighbor_idx);
            }
        }
    }
}

pub fn update_pipes(
    pipe_image_nodes: Query<(&Pipe, &mut ImageNode)>,
    pipe_grid: Res<PipeGrid>,
) {
    if pipe_grid.is_loaded {
        for (pipe, mut pipe_image_node) in pipe_image_nodes {
            if let Some(texture_atlas) = &mut pipe_image_node.texture_atlas {
                let conn = &pipe_grid.data[pipe.flat_id];
                texture_atlas.index = conn.rot_state + conn.conn_type * GRID_SIZE;
            }
        }
    }
}
