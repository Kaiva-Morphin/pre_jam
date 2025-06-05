use bevy::{platform::collections::HashSet, prelude::*};

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::target::LowresUiContainer, utils::custom_material_loader::PipesAtlasHandles};

#[derive(Component)]
pub struct Pipe {
    pub flat_id: usize,
}

pub const PIPE_GRID_SIZE: f32 = 50.;

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
            let mut childern = vec![];
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    childern.push(commands.spawn((
                        Node {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            left: Val::Px(PIPE_GRID_SIZE * x as f32),
                            bottom: Val::Px(PIPE_GRID_SIZE * y as f32),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ImageNode::from_atlas_image(
                            pipes_atlas_handles.image_handle.clone(),
                            TextureAtlas::from(pipes_atlas_handles.layout_handle.clone())
                        ),
                        Pipe {flat_id: x + y * GRID_SIZE},
                        Button,
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
    fn random() -> Self {
        let conn_type = ((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * 3.) as usize;
        Self {
            conn_type,
            rot_state: 0,
            neighbors: CONNECTIONS[conn_type],
        }
    }
    fn rotate(&mut self) {
        self.rot_state = (self.rot_state + 1) % 4;
        self.neighbors = [self.neighbors[3], self.neighbors[0], self.neighbors[1], self.neighbors[2]];
    }
    fn default() -> Self {
        Self {
            conn_type: 0,
            rot_state: 0,
            neighbors: [0,0,0,0],
        }
    }
}

#[derive(Resource, Default)]
pub struct PipeGrid {
    pub data: Vec<Connection>,
    pub is_loaded: bool,
}

pub fn init_grid(
    mut pipe_grid: ResMut<PipeGrid>,
) {
    pipe_grid.data = vec![Connection::default(); GRID_SIZE * GRID_SIZE];
    pipe_grid.is_loaded = true;
    let mut final_path = vec![];

    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]]; // up, right, down, left

    'outer: loop {
        let mut visited = vec![false; GRID_SIZE * GRID_SIZE];
        let mut path = vec![[0, 0]];
        visited[0] = true;
        let mut curr = [0, 0];
        let goal = [GRID_SIZE as i32 - 1, GRID_SIZE as i32 - 1];

        for _ in 0..(GRID_SIZE * GRID_SIZE * 4) {
            // Find all valid moves
            let mut moves = vec![];
            for dir in directions {
                let nx = curr[0] + dir[0];
                let ny = curr[1] + dir[1];
                if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
                    let idx = nx as usize + ny as usize * GRID_SIZE;
                    if !visited[idx] {
                        moves.push([nx, ny]);
                    }
                }
            }
            if moves.is_empty() {
                // Dead end, restart
                break;
            }
            let next = moves[((getrandom::u32().unwrap() as f32 / u32::MAX as f32) * moves.len() as f32) as usize];
            let idx = next[0] as usize + next[1] as usize * GRID_SIZE;
            visited[idx] = true;
            path.push(next);
            curr = next;
            if curr == goal {
                final_path = path;
                break 'outer;
            }
        }
        // If we exit the for loop without reaching the goal, restart
    }

    for pos in 1..final_path.len() {
        let prev = final_path[pos - 1];
        let curr = final_path[pos];
        let curr_flat = curr[0] as usize + curr[1] as usize * GRID_SIZE;
        let prev_flat = prev[0] as usize + prev[1] as usize * GRID_SIZE;
        let d = [curr[0] - prev[0], curr[1] - prev[1]];
        if d[0] < 0 {
            pipe_grid.data[curr_flat].neighbors[3] = 1;
            pipe_grid.data[prev_flat].neighbors[1] = 1;
        } else if d[0] > 0 {
            pipe_grid.data[curr_flat].neighbors[1] = 1;
            pipe_grid.data[prev_flat].neighbors[3] = 1;
        }
        if d[1] < 0 {
            pipe_grid.data[curr_flat].neighbors[2] = 1;
            pipe_grid.data[prev_flat].neighbors[0] = 1;
        } else if d[1] > 0 {
            pipe_grid.data[curr_flat].neighbors[0] = 1;
            pipe_grid.data[prev_flat].neighbors[2] = 1;
        }
        let conn = &mut pipe_grid.data[curr_flat];
        let (conn_type, rot) = match conn.neighbors {
            [1,0,1,0] => (0, 0), // |
            [0,1,0,1] => (0, 1), // -
            [1,1,0,0] => (1, 0), // L
            [0,1,1,0] => (1, 1), // L
            [0,0,1,1] => (1, 2), // L
            [1,0,0,1] => (1, 3), // L
            [1,1,1,0] => (2, 0), // T
            [0,1,1,1] => (2, 1), // T
            [1,0,1,1] => (2, 2), // T
            [1,1,0,1] => (2, 3), // T
            [1,1,1,1] => (3, 0), // +
            // Handle endpoints (single connection)
            [1,0,0,0] => (0, 0), // treat as vertical
            [0,1,0,0] => (0, 1), // treat as horizontal
            [0,0,1,0] => (0, 0), // treat as vertical
            [0,0,0,1] => (0, 1), // treat as horizontal
            a => panic!("{:?} {:?}", a, pos)
        };
        conn.conn_type = conn_type;
        conn.rot_state = rot;
        let conn = &mut pipe_grid.data[prev_flat];
        let (conn_type, rot) = match conn.neighbors {
            [1,0,1,0] => (0, 0), // |
            [0,1,0,1] => (0, 1), // -
            [1,1,0,0] => (1, 0), // L
            [0,1,1,0] => (1, 1), // L
            [0,0,1,1] => (1, 2), // L
            [1,0,0,1] => (1, 3), // L
            [1,1,1,0] => (2, 0), // T
            [0,1,1,1] => (2, 1), // T
            [1,0,1,1] => (2, 2), // T
            [1,1,0,1] => (2, 3), // T
            [1,1,1,1] => (3, 0), // +
            // Handle endpoints (single connection)
            [1,0,0,0] => (0, 0), // treat as vertical
            [0,1,0,0] => (0, 1), // treat as horizontal
            [0,0,1,0] => (0, 0), // treat as vertical
            [0,0,0,1] => (0, 1), // treat as horizontal
            a => panic!("{:?} {:?}", a, pos)
        };
        conn.conn_type = conn_type;
        conn.rot_state = rot;
    }
}

pub fn update_pipes(
    pipe_image_nodes: Query<(&Pipe, &mut ImageNode, &Interaction), Changed<Interaction>>,
    mut pipe_grid: ResMut<PipeGrid>,
) {
    if pipe_grid.is_loaded {
        for (pipe, mut pipe_image_node, pipe_interaction) in pipe_image_nodes {
            if let Some(texture_atlas) = &mut pipe_image_node.texture_atlas {
                let conn = &mut pipe_grid.data[pipe.flat_id];
                if *pipe_interaction == Interaction::Pressed {
                    conn.rotate();
                }
                texture_atlas.index = conn.rot_state + conn.conn_type * 4;
            }
        }
    }
}
