use bevy::{platform::collections::HashSet, prelude::*};
use bevy_tailwind::tw;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::{components::containers::base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, target::LowresUiContainer}, utils::{custom_material_loader::PipesAtlasHandles, debree::{Malfunction, MalfunctionType, Resolved}, spacial_audio::PlaySoundEvent}};

// WIBECODE RULES 🤘🧑‍🎤
const ROWS: usize = 6;
const COLS: usize = 6;

pub const SINGLE_PIPE_TEX_SIZE : f32 = 16.;
const PIPE_GRID_SIZE : f32 = 25.0;

pub fn open_pipe_puzzle_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    pipes_atlas_handles: Res<PipesAtlasHandles>,
    asset_server: Res<AssetServer>,
    mut pipes: ResMut<PipeMinigame>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    // TODO: add pipe sounds
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        // pipes.fill_solved();
        if in_interaction_array.in_interaction == InteractionTypes::PipePuzzle && in_interaction_array.in_any_interaction {
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);
            
            
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(ui_main_container(&main, ())).with_children(|cmd| {
                    cmd.spawn(ui_sub_container(&sub, ())).with_children(|cmd| {
                        //info!("{:?}", tw!("items-center justify-center w-full h-full grid grid-cols-6 grid-rows-6"));
                        /*
                        Node { display: Grid, box_sizing: BorderBox, position_type: Relative, overflow: Overflow { x: Visible, y: Visible }, overflow_clip_margin: OverflowClipMargin { visual_box: ContentBox, margin: 0.0 }, left: Auto, right: Auto, top: Auto, bottom: Auto, width: Percent(100.0), height: Percent(100.0), min_width: Auto, min_height: Auto, max_width: Auto, max_height: Auto, aspect_ratio: None, align_items: Center, justify_items: Default, align_self: Auto, justify_self: Auto, align_content: Default, justify_content: Center, margin: UiRect { left: Px(0.0), right: Px(0.0), top: Px(0.0), bottom: Px(0.0) }, padding: UiRect { left: Px(0.0), right: Px(0.0), top: Px(0.0), bottom: Px(0.0) }, border: UiRect { left: Px(0.0), right: Px(0.0), top: Px(0.0), bottom: Px(0.0) }, flex_direction: Row, flex_wrap: NoWrap, flex_grow: 0.0, flex_shrink: 1.0, flex_basis: Auto, row_gap: Px(0.0), column_gap: Px(0.0), grid_auto_flow: Row, grid_template_rows: [RepeatedGridTrack { repetition: Count(6), tracks: [GridTrack { min_sizing_function: Px(0.0), max_sizing_function: Fraction(1.0) }] }], grid_template_columns: [RepeatedGridTrack { repetition: Count(6), tracks: [GridTrack { min_sizing_function: Px(0.0), max_sizing_function: Fraction(1.0) }] }], grid_auto_rows: [], grid_auto_columns: [], grid_row: GridPlacement { start: None, span: Some(1), end: None }, grid_column: GridPlacement { start: None, span: Some(1), end: None } }
                        */
                        cmd.spawn(Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            // align_self: AlignSelf::Center,
                            // align_items: AlignItems::Center,
                            // justify_content: JustifyContent::Center,
                            // position_type: PositionType::Absolute,
                            // flex_direction: FlexDirection::Row,
                            // align_items: AlignItems::Center,
                            // justify_content: JustifyContent::Center,
                            // width: Val::Px(PIPE_GRID_SIZE * COLS as f32),
                            // height: Val::Px(PIPE_GRID_SIZE * ROWS as f32),
                            // display: Display::Flex,
                            display: Display::Grid,
                            row_gap: Val::Px(-0.4),
                            grid_auto_flow: GridAutoFlow::Column,
                            column_gap: Val::Px(-0.4),
                            grid_template_rows: vec![RepeatedGridTrack::flex(ROWS as u16, 1.)],
                            grid_template_columns: vec![RepeatedGridTrack::flex(COLS as u16, 1.)],
                            ..Default::default()
                        })
                        .with_children(|cmd|{
                            for x in 0..COLS {
                                for y in 0..ROWS {
                                    let pipe = pipes.get_pipe(x, ROWS - y - 1);
                                    cmd.spawn((
                                        Node {
                                            width: Val::Px(PIPE_GRID_SIZE),
                                            height: Val::Px(PIPE_GRID_SIZE),
                                            // position_type: PositionType::Absolute,
                                            // left: Val::Px(PIPE_GRID_SIZE * x as f32),
                                            // bottom: Val::Px(PIPE_GRID_SIZE * y as f32),
                                            ..default()
                                        },
                                        ImageNode::from_atlas_image(
                                            pipes_atlas_handles.image_handle.clone(),
                                            TextureAtlas{
                                                layout: pipes_atlas_handles.layout_handle.clone(),
                                                index: pipe.as_ref().map(|v|v.get_index()).unwrap_or(15)
                                            },
                                        ),
                                        PipeEntity{
                                            pipe: pipe.cloned(),
                                            position: uvec2(x as u32, (ROWS - y - 1) as u32), 
                                        },
                                        Button,
                                    ));
                                }
                            }
                        });
                    });
                });
            }).id();
            
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

pub fn update_pipes(
    mut pipe_image_nodes: Query<(&mut PipeEntity, &mut ImageNode, &Interaction), Changed<Interaction>>,
    mut pipes: ResMut<PipeMinigame>,
    mut malfunction: ResMut<Malfunction>,
    mut event_writer: EventWriter<PlaySoundEvent>,
    in_interaction_array: Res<InInteractionArray>,
){
    if in_interaction_array.in_interaction == InteractionTypes::PipePuzzle && in_interaction_array.in_any_interaction {
        for (mut pipe, mut pipe_image_node, pipe_interaction) in pipe_image_nodes.iter_mut() {
            if let Some(texture_atlas) = &mut pipe_image_node.texture_atlas {
                if *pipe_interaction == Interaction::Pressed {
                    pipes.rotate(pipe.position);
                    if let Some(p) = pipes.get_pipe(pipe.position.x as usize, pipe.position.y as usize) {
                        texture_atlas.index = p.get_index();
                    }
                    if pipes.is_solved() {
                        event_writer.write(PlaySoundEvent::Success);
                    malfunction.resolved.push(Resolved {
                            resolved_type: MalfunctionType::Engine,
                            failed: false,
                        });
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PipeType {
    SINGLE,
    LINE,
    CORNER,
    TEE,
    CROSS
}

type PipeRotation = u8;
type PipeSide = u8;

#[derive(Clone, Debug)]
pub struct Pipe {
    variant: PipeType,
    rotation: PipeRotation 
}
#[derive(Component, Clone)]
pub struct PipeEntity {
    pipe: Option<Pipe>,
    position: UVec2
}

impl PipeEntity {
    fn rotate(&mut self){
        if let Some(p) = &mut self.pipe {
            p.rotation = (p.rotation + 1) % 4;
        }
    }
    fn get_index(&self) -> usize {
        self.pipe.as_ref().map(|v|v.get_index()).unwrap_or(15)
    }
}


impl Pipe {
    pub fn get_sides(&self) -> Vec<PipeSide> {
        let dirs: Vec<u8> = match self.variant {
            PipeType::SINGLE => vec![0],
            PipeType::LINE => vec![0, 2],
            PipeType::CORNER => vec![0, 1],
            PipeType::TEE => vec![0, 1, 3],
            PipeType::CROSS => vec![0, 1, 2, 3],
        };
        
        dirs.into_iter()
            .map(|d| (d + self.rotation) % 4)
            .collect()
    }
}


impl PipeType {
    pub fn all() -> &'static [PipeType] {
        // статический срез всех вариантов PipeType
        &[
            PipeType::SINGLE,
            PipeType::LINE,
            PipeType::CORNER,
            PipeType::TEE,
            PipeType::CROSS,
        ]
    }
}

pub fn get_sides(pipe_type: &PipeType, rotation: PipeRotation) -> Vec<PipeSide> {
    let dirs = match pipe_type {
        PipeType::SINGLE => vec![0],
        PipeType::LINE => vec![0, 2],
        PipeType::CORNER => vec![0, 1],
        PipeType::TEE => vec![0, 1, 3],
        PipeType::CROSS => vec![0, 1, 2, 3],
    };
    dirs.into_iter().map(|d| (d + rotation) % 4).collect()
}

pub fn get_candidates(optional: &[PipeSide], include: &[PipeSide]) -> Vec<Pipe> {
    use std::collections::HashSet;

    let allowed_sides: HashSet<_> = optional.iter().chain(include.iter()).cloned().collect();
    let include_set: HashSet<_> = include.iter().cloned().collect();

    let mut candidates: Vec<_> = Vec::new();

    for pipe_type in PipeType::all() {
        for rotation in 0..4 {
            let sides: HashSet<_> = get_sides(pipe_type, rotation).into_iter().collect();
            if !include_set.is_subset(&sides) {
                continue;
            }
            if !sides.is_subset(&allowed_sides) {
                continue;
            }

            candidates.push(Pipe{variant: *pipe_type, rotation});
        }
    }

    candidates
}

fn random_u32() -> u32 {
    getrandom::u32().unwrap()
}



fn pipe_weight(pipe_type: PipeType) -> usize {
    match pipe_type {
        PipeType::SINGLE => 10,
        PipeType::LINE => 10,
        PipeType::CORNER => 40,
        PipeType::TEE => 20,
        PipeType::CROSS => 10,
    }
}

fn pick_candidate(candidates: &[Pipe]) -> Option<Pipe> {
    if candidates.is_empty() {
        return None;
    }

    let weights: Vec<usize> = candidates.iter()
        .map(|pipe| pipe_weight(pipe.variant))
        .collect();

    let total_weight: usize = weights.iter().sum();
    if total_weight == 0 {
        return None;
    }

    // Получаем случайное число в диапазоне [0, total_weight)
    let rnd = (random_u32() as usize) % total_weight;

    // Находим индекс, соответствующий rnd
    let mut acc = 0;
    for (i, w) in weights.iter().enumerate() {
        acc += *w;
        if rnd < acc {
            return Some(candidates[i].clone());
        }
    }

    // Some(candidates[candidates.len() - 1])
    warn!("No candidate!");
    None
}


impl Pipe {
    fn get_index(&self) -> usize {
        match self.variant {
            PipeType::LINE => {
                match self.rotation % 2 {
                    0 => 7,
                    1 => 13,
                    _ => 15,
                }
            }
            PipeType::TEE => {
                match self.rotation {
                    0 => 1,
                    1 => 4,
                    2 => 9,
                    3 => 6,
                    _ => 15
                }
            }
            PipeType::CORNER => {
                match self.rotation {
                    0 => 0,
                    1 => 8,
                    2 => 10,
                    3 => 2,
                    _ => 15
                }
            }
            PipeType::SINGLE => {
                match self.rotation {
                    0 => 3,
                    1 => 12,
                    2 => 11,
                    3 => 14,
                    _ => 15
                }
            }
            PipeType::CROSS => {5}
        }
    }
}

fn filter_grid_corners(pos: (usize, usize), inc_sides: &[u8]) -> Vec<u8> {
    let (r, c) = pos;
    let mut allowed = Vec::new();

    for &side in inc_sides {
        match side {
            0 if r == 0 => continue,
            1 if c == COLS - 1 => continue,
            2 if r == ROWS - 1 => continue,
            3 if c == 0 => continue,
            _ => allowed.push(side),
        }
    }

    allowed
}

fn neighbors(r: usize, c: usize) -> Vec<((usize, usize), u8)> {
    let mut result = Vec::new();

    if r > 0 {
        result.push(((r - 1, c), 0));
    }
    if c < COLS - 1 {
        result.push(((r, c + 1), 1));
    }
    if r < ROWS - 1 {
        result.push(((r + 1, c), 2));
    }
    if c > 0 {
        result.push(((r, c - 1), 3));
    }

    result
}

fn opposite_side(side: u8) -> u8 {
    (side + 2) % 4
}


fn allowed_sides_grid_corners(pos: (usize, usize), sides: &mut Vec<u8>) -> Vec<u8> {
    let (r, c) = pos;
    let mut allowed = sides.clone();

    if r == 0 {
        allowed.retain(|&s| s != 0); 
    }
    if c == COLS - 1 {
        allowed.retain(|&s| s != 1); 
    }
    if r == ROWS - 1 {
        allowed.retain(|&s| s != 2); 
    }
    if c == 0 {
        allowed.retain(|&s| s != 3); 
    }

    allowed
}

fn get_include_exclude_sides(
    r: usize,
    c: usize,
    grid: &Vec<Vec<Option<Pipe>>>,
) -> (Vec<u8>, Vec<u8>) {
    let mut include = Vec::new();
    let mut exclude = Vec::new();

    for ((nr, nc), side_to_neighbor) in neighbors(r, c).into_iter() {
        if let Some(neighbor_tile) = &grid[nr][nc] {
            let opposite = opposite_side(side_to_neighbor);
            let neighbor_sides = get_sides(&neighbor_tile.variant, neighbor_tile.rotation);

            if neighbor_sides.contains(&opposite) {
                include.push(side_to_neighbor);
            } else {
                exclude.push(side_to_neighbor);
            }
        }
    }

    (include, exclude)
}




#[derive(Resource)]
pub struct PipeMinigame {
    grid: Vec<Vec<Option<Pipe>>>,    
}

impl PipeMinigame {
    pub fn rotate(&mut self, pos: UVec2) {
        let Some(grid) = self.grid.get_mut(pos.y as usize) else {return;};
        let Some(v) = grid.get_mut(pos.x as usize) else {return;};
        let Some(p) = v else {return;};
        p.rotation = (p.rotation + 1) % 4;
    }
    pub fn fill_solved(&mut self) {
        let start_r = random_u32() as usize % ROWS;
        let start_c = random_u32() as usize % COLS;

        let allowed = allowed_sides_grid_corners((start_r, start_c), &mut vec![0, 1, 2, 3]);
        let candidates = get_candidates(&allowed, &[]);
        if candidates.is_empty() {
            panic!("No candidates for start cell");
        }

        let first = pick_candidate(&candidates).expect("No candidate for first");
        self.grid[start_r][start_c] = Some(first);

        let mut queue = vec![(start_r, start_c)];

        while let Some((r, c)) = queue.pop() {
            for ((nr, nc), _) in neighbors(r, c) {
                if self.grid[nr][nc].is_some() {
                    continue;
                }

                let (include, exclude) = get_include_exclude_sides(nr, nc, &self.grid);
                let allowed = allowed_sides_grid_corners((nr, nc), &mut vec![0, 1, 2, 3]);

                let optional: Vec<u8> = allowed
                    .into_iter()
                    .filter(|s| !include.contains(s) && !exclude.contains(s))
                    .collect();

                let candidates = get_candidates(&optional, &include);
                if candidates.is_empty() {
                    continue;
                }

                self.grid[nr][nc] = pick_candidate(&candidates);
                queue.push((nr, nc));
            }
        }
    }
    pub fn shuffle(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                if let Some(pipe) = cell {
                    pipe.rotation = (random_u32() % 4) as u8;
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        for r in 0..ROWS {
            for c in 0..COLS {
                let pipe = match &self.grid[r][c] {
                    Some(p) => p,
                    None => continue,
                };

                let sides = pipe.get_sides();

                for ((nr, nc), side_to_neighbor) in neighbors(r, c) {
                    if nr >= ROWS || nc >= COLS {
                        continue;
                    }
                    let neighbor = match &self.grid[nr][nc] {
                        Some(n) => n,
                        None => continue, 
                    };
                    let neighbor_sides = neighbor.get_sides();
                    let opposite = opposite_side(side_to_neighbor);

                    if sides.contains(&side_to_neighbor) != neighbor_sides.contains(&opposite) {
                        return false;
                    }
                }
            }
        }
        true
    }


    pub fn clear(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
    }
    pub fn get_pipe(&self, x: usize, y: usize) -> Option<&Pipe> {
        if x < COLS && y < ROWS {
            self.grid[y][x].as_ref()
        } else {
            None
        }
    }
}


impl Default for PipeMinigame {
    fn default() -> Self {
        let mut s = Self {
            grid: vec![vec![None; COLS]; ROWS],
        };
        s.fill_solved();
        // s.shuffle();
        s
    }
}
