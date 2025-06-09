use bevy::{platform::collections::HashSet, prelude::*};
use bevy_tailwind::tw;

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::{components::{containers::{base::*, text_display::{text_display_green_handle, ui_text_display_green_with_text}}, hack_button::*}, target::LowresUiContainer}, utils::{debree::{get_random_range, Malfunction, MalfunctionType, Resolved}, spacial_audio::PlaySoundEvent}};

// ALSO CHANGE TW VALUE!
pub const HACK_GRID_SIZE: u32 = 5;

pub const HACK_PIXEL_GRID_SIZE: u32 = 50;
pub const HACK_ATLAS_COLUMNS: u32 = 6;
pub const HACK_ATLAS_ROWS: u32 = 6;
pub const NUM_HACK_BUTTON_TYPES: f32 = 7.;

#[derive(Component)]
pub struct HackButtonBase {
    pub pos: UVec2,
}

#[derive(Component)]
pub struct GoalText;

#[derive(Component)]
pub struct BufferText;

pub fn open_hack_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    hack_grid: Res<HackGrid>,
    malfunction: Res<Malfunction>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::HackMinigame && in_interaction_array.in_any_interaction {
            event_writer.write(PlaySoundEvent::OpenUi);
            let mut is_active = false;
            if malfunction.malfunction_types.contains(&MalfunctionType::Hack) {
                is_active = true;
            }
            let hack = hack_button_bundle(&asset_server, &mut texture_atlases);
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);
            let text_bundle = text_display_green_handle(&asset_server);
            
            let goal_text = "Goal: ";
            let goal_text_entity = commands.spawn(
            ui_main_container(&main, ())
            ).with_children(|cmd|{
                cmd.spawn(
                ui_text_display_green_with_text(&text_bundle, (GoalText, GoalText), goal_text, &asset_server)
                ).insert(tw!("w-[230px] items-center justify-center p-[5px]"));
            }).id();



            let buffer_text = "Selected: ";
            let buffer_text_entity = commands.spawn(
            ui_main_container(&main, ())
            ).with_children(|cmd|{
                cmd.spawn(
                ui_text_display_green_with_text(&text_bundle, (BufferText, BufferText), buffer_text, &asset_server)
                ).insert(tw!("w-[230px] items-center justify-center p-[5px]"));
            }).id();

            let mut children = vec![];
            for y in 0..HACK_GRID_SIZE {
                for x in 0..HACK_GRID_SIZE {
                    let mut state = HackButtonState::Disabled;
                    let mut index = 0;
                    if is_active {
                        index = hack_grid.grid[(x + y * HACK_GRID_SIZE) as usize];
                        if y == 0 {
                            state = HackButtonState::Active
                        } else {
                            state = HackButtonState::Enabled
                        }
                    }
                    children.push(commands.spawn(
                    ui_main_container(&main, children![(
                        ui_hack_button(&hack, HackButton{state: state, index: index}, HackButtonBase {pos: UVec2::new(x, y)}),
                    )])).id());
                }
            }
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd|{
                cmd.spawn(ui_main_container(&main, ())).with_children(|cmd| {
                    cmd.spawn(ui_sub_container(&sub, ())).with_children(|cmd| {
                        cmd.spawn(tw!("items-center justify-center w-full h-full grid grid-cols-5 grid-rows-5 gap-x-px gap-y-px"),)
                        .add_children(&children);
                    });
                });
                cmd.spawn(ui_main_container(&main, ())).insert(
                        tw!("flex flex-col-reverse")
                    ).with_children(|cmd|{
                        cmd.spawn(ui_sub_container(&sub, ()))
                        .with_children(|cmd| {
                            cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                            .add_child(goal_text_entity);
                        });
                        cmd.spawn(ui_sub_container(&sub, ()))
                        .with_children(|cmd| {
                            cmd.spawn(tw!("items-center justify-center w-full h-full"),)
                            .add_child(buffer_text_entity);
                        });
                    });
            }).id();
            
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

#[derive(Resource, Default)]
pub struct HackGrid {
    pub is_loaded: bool,
    pub grid: Vec<usize>,
    pub win_seq: Vec<usize>,
}

pub fn init_hack_display(
    malfunction: Res<Malfunction>,
    mut hack_grid: ResMut<HackGrid>,
) {
    if malfunction.is_changed() && malfunction.malfunction_types.contains(&MalfunctionType::Hack) && !hack_grid.is_loaded {
        hack_grid.is_loaded = true;
        hack_grid.grid = vec![0; (HACK_GRID_SIZE * HACK_GRID_SIZE) as usize];
        for y in 0..HACK_GRID_SIZE as usize {
            loop {
                let mut have_different = false;
                for x in 0..HACK_GRID_SIZE as usize {
                    let flat_id = x + y * HACK_GRID_SIZE as usize;
                    let index = get_random_range(0., NUM_HACK_BUTTON_TYPES) as usize;
                    hack_grid.grid[flat_id] = index;
                    if hack_grid.grid[y * HACK_GRID_SIZE as usize] != index {
                        have_different = true;
                    }
                }
                if have_different {
                    break;
                }
            }
        }
        let hor_entry1 = get_random_range(1., HACK_GRID_SIZE as f32) as usize;
        let vert_entry1 = get_random_range(1., HACK_GRID_SIZE as f32) as usize;
        let mut hor_entry2 = get_random_range(-(hor_entry1 as f32), HACK_GRID_SIZE as f32 - hor_entry1 as f32) as usize;
        loop {
            if hor_entry2 == 0 {
                hor_entry2 = get_random_range(-(hor_entry1 as f32), HACK_GRID_SIZE as f32 - hor_entry1 as f32) as usize;
            } else {
                break;
            }
        }
        hack_grid.win_seq = vec![
            hack_grid.grid[hor_entry1],
            hack_grid.grid[hor_entry1 + vert_entry1 * HACK_GRID_SIZE as usize],
            hack_grid.grid[hor_entry1 + hor_entry2 + vert_entry1 * HACK_GRID_SIZE as usize]
        ];
        println!("{},0 {},{} {},{} ({}) {:?}", hor_entry1, hor_entry1, vert_entry1, hor_entry1 + hor_entry2, vert_entry1, hor_entry2, hack_grid.win_seq.iter().map(|index| HACK_BUTTON_NAMES[*index]).collect::<Vec<&str>>());
        // println!("{:?}", (0..100).map(|_| get_random_range(1., HACK_GRID_SIZE as f32 - 1.) as usize).collect::<Vec<usize>>());
    }
}

pub fn update_hack_display(
    mut malfunction: ResMut<Malfunction>,
    mut hack_grid: ResMut<HackGrid>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut ImageNode,
            &mut HackButton,
            &HackButtonBase
        ),
    >,
    changed_interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut selected_seq_pos: Local<Vec<UVec2>>,
    mut selected_seq_index: Local<Vec<usize>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut prev_state: Local<Interaction>,
    goal_text: Query<&mut Text, With<GoalText>>,
    buffer_text: Query<&mut Text, (With<BufferText>, Without<GoalText>)>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    for mut text in goal_text {
        let winseq = hack_grid.win_seq.iter().map(|index| HACK_BUTTON_NAMES[*index]).collect::<Vec<&str>>();
        let mut seq = String::from("Goal: ");
        for i in winseq {
            seq += &format!("{} ", i);
        }
        text.0 = seq;
    }
    for mut text in buffer_text {
        let sel_seq = selected_seq_index.iter().map(|index| HACK_BUTTON_NAMES[*index]).collect::<Vec<&str>>();
        let mut seq = String::from("Selected: ");
        for i in sel_seq {
            seq += &format!("{} ", i);
        }
        text.0 = seq;
    }
    let curr_type = MalfunctionType::Hack;
    if malfunction.malfunction_types.contains(&curr_type) && hack_grid.is_loaded {
        for (entity, interaction, mut node, mut hack, base) in
            &mut interaction_query
        {
            let ver_hor_lightup_condition = match selected_seq_pos.len() {
                0 => {
                    base.pos.y == 0
                },
                1 => {
                    base.pos.x == selected_seq_pos[0].x
                },
                2 => {
                    base.pos.y == selected_seq_pos[1].y
                },
                _ => {return;}
            };
            let spec_condition = match selected_seq_pos.len() {
                0 => {
                    base.pos.y == 0
                },
                1 => {
                    base.pos.x == selected_seq_pos[0].x && base.pos.y != 0
                },
                2 => {
                    base.pos.y == selected_seq_pos[1].y && base.pos.x != selected_seq_pos[0].x
                },
                _ => {return;}
            };
            if ver_hor_lightup_condition {
                hack.state = HackButtonState::Active;
                if let Ok(interaction) = changed_interaction_query.get(entity) {
                    // println!("{:?} {}", prev_state, mouse_button.just_released(MouseButton::Left));
                    if *prev_state == Interaction::Pressed && mouse_button.just_released(MouseButton::Left) && spec_condition{
                        if let Some(a) = &mut node.texture_atlas {
                            a.index = hack.get_idx(false, true);
                            selected_seq_pos.push(base.pos);
                            selected_seq_index.push(hack.index);
                        }
                    }
                    *prev_state = *interaction;
                } else {
                    if let Some(a) = &mut node.texture_atlas {
                        a.index = hack.get_idx(*interaction == Interaction::Hovered, *interaction == Interaction::Pressed);
                    }
                }
            }
            if selected_seq_pos.contains(&base.pos) {
                hack.state = HackButtonState::SuperActive;
                if let Some(a) = &mut node.texture_atlas {
                    a.index = hack.get_idx(false, false);
                }
            } 
            if selected_seq_pos.len() == hack_grid.win_seq.len() {
                println!("{:?}", selected_seq_index.iter().map(|index| HACK_BUTTON_NAMES[*index]).collect::<Vec<&str>>());
                hack_grid.is_loaded = false;
                let mut failed = true;
                if *selected_seq_index == hack_grid.win_seq {
                    event_writer.write(PlaySoundEvent::Success);
                    failed = false;
                };
                malfunction.resolved.push(Resolved {resolved_type: curr_type.clone(), failed});
                *prev_state = Interaction::default();
                *selected_seq_pos = vec![];
                *selected_seq_index = vec![];
            }
        }
    }
}