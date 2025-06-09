use std::collections::{HashMap, HashSet};

use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_tailwind::tw;
use debug_utils::overlay_text;
use pixel_utils::camera::{PixelCamera, TARGET_HEIGHT, TARGET_WIDTH};

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::{components::{containers::{base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, text_display::text_display_green_handle, viewport_container::viewport_handle}, spinny::ui_spinny, ui_submit_button::submit_button_bundle, wire_inlet::{ui_wire_inlet, wire_inlet_bundle}}, target::LowresUiContainer}, utils::{custom_material_loader::SpriteAssets, debree::{get_random_range, Malfunction, Resolved}, mouse::CursorPosition, spacial_audio::PlaySoundEvent}};




#[derive(Resource)]
pub struct WireMinigame {
    pub locked_id: Option<usize>,
    pub socket_positions: HashMap<usize, Vec2>,
    pub task: HashMap<usize, usize>,
    pub colors: HashMap<usize, Color>,
    pub connected: HashMap<usize, usize>,
    pub allow_unordered: bool,
}

impl WireMinigame {
    pub fn calc_color(&mut self, idx: usize, task: usize){
        self.colors.insert(idx, Color::from(Color::hsl(
            (360.0) / WIRES as f32 * ((task as f32 + 1.0) % (WIRES as f32)),
            1.0,0.5
        )));
    }
    pub fn get_color(&self, idx: usize) -> Color {
        self.colors.get(&idx).copied().unwrap_or(Color::WHITE)
    }
}

impl Default for WireMinigame {
    fn default() -> Self {
        Self {
            locked_id: None,
            task: HashMap::new(),
            socket_positions: HashMap::new(),
            colors: HashMap::new(),
            connected: HashMap::new(),
            allow_unordered: true,
        }
    }
}

#[derive(Component)]
pub struct Wire {
    pub id: usize,
    pub left: bool,
}


#[derive(Component)]
pub struct WireContainer;

const WIRES : usize = 2;
const WIRE_SOCKETS : usize = WIRES * 2;

pub fn refresh_game(
    g: &mut ResMut<WireMinigame>,
) {
    g.task = HashMap::new();
    g.connected = HashMap::new();
    // g.allow_unordered = getrandom::u32() & 2 == 0;

    let mut used = HashSet::new();

    let allow_unordered = true;
    let mut get_free_ids = |mut a: usize, mut b: usize| -> (usize, usize) {
        let mut i = 0;
        while used.contains(&a) {
            a = (a + 1) % if allow_unordered {WIRE_SOCKETS} else {WIRES};
            if i > WIRE_SOCKETS {
                warn!("ITER LIMIT FOR 'from' REACHED FOR REFRESH WIRES");
                break;
            } else {
                i += 1;
            }
        };
        let mut i = 0;
        used.insert(a);
        while used.contains(&((b + WIRES) % WIRE_SOCKETS)) {
            b = (b + 1) % if allow_unordered {WIRE_SOCKETS} else {WIRES};
            if i > WIRE_SOCKETS {
                warn!("ITER LIMIT FOR 'to' REACHED FOR REFRESH WIRES");
                break;
            } else {
                i += 1;
            }
        };
        b = (b + WIRES) % WIRE_SOCKETS;
        used.insert(b);
        (a.min(b), a.max(b))
    };


    for i in 0..WIRES {
        let a = getrandom::u32().unwrap() as usize % WIRE_SOCKETS;
        let b = getrandom::u32().unwrap() as usize % WIRE_SOCKETS;
        let (a, b) = get_free_ids(a, b);
        info!("Bind {} to {}", a, b);
        g.calc_color(a, i);
        g.calc_color(b, i);
        g.task.insert(a, b);
        g.task.insert(b, a);
    }
    // info!("Task: {:?}", g.task);
}


pub fn open_wires_display(
    mut commands: Commands,
    in_interaction_array: Res<InInteractionArray>,
    mut already_spawned: Local<Option<Entity>>,
    // spinny_atlas_handles: Res<SpinnyAtlasHandles>,
    lowres_container: Single<Entity, With<LowresUiContainer>>,
    // mut wave_graph_material: ResMut<Assets<WaveGraphMaterial>>,
    // mut modulator_consts: ResMut<WaveModulatorConsts>,
    images: Res<Assets<Image>>,
    sprite_assets: Res<SpriteAssets>,
    asset_server: Res<AssetServer>,
    mut malfunction: ResMut<Malfunction>,
    
    mut wires: ResMut<WireMinigame>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // TODO: add touch and open sfx and success sfx
    if let Some(entity) = *already_spawned {
        if !in_interaction_array.in_any_interaction {
            commands.entity(entity).despawn();
            *already_spawned = None;
        }
    } else {
        if in_interaction_array.in_interaction == InteractionTypes::WiresMinigame && in_interaction_array.in_any_interaction {
            let main = main_container_handle(&asset_server);
            let sub = sub_container_handle(&asset_server);
            let text_bundle = text_display_green_handle(&asset_server);
            let submit_bundle = submit_button_bundle(&asset_server, &mut texture_atlases);
            let wire_bundle = wire_inlet_bundle(&asset_server);
            refresh_game(&mut wires);

            let mut wires_e = vec![];
            for i in 0..WIRE_SOCKETS {
                wires_e.push(commands.spawn(
                ui_main_container(&main, children![
                    ui_wire_inlet(&wire_bundle, 
                        wires.get_color(i),
                        (
                            Wire {id: i, left: i < WIRES},
                        ))
                ])).id());
            };
            
            
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd| {
                cmd.spawn(ui_main_container(&main, ()))
                .with_children(|cmd| {
                    cmd.spawn((
                        tw!("flex flex-row gap-[100px]"),
                        RelativeCursorPosition::default(),
                        WireContainer
                    ))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("flex flex-col"))
                            .add_children(&wires_e[0..WIRES]);
                        cmd.spawn(tw!("flex flex-col"))
                            .add_children(&wires_e[WIRES..WIRE_SOCKETS]);
                    });
                });
            }).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}


#[derive(Component)]
pub struct ToWorld;

#[derive(Component)]
pub struct ToWorldEnd;

type Parent = ChildOf;

#[derive(Component)]
pub struct ToWorldWire;


pub fn touch_wires_inlet(
    mut commands: Commands,
    
    children_q: Query<(&Parent, &Transform)>,
    computed_nodes: Query<&ComputedNode>,

    wires_q: Query<(Entity ,&RelativeCursorPosition, &Wire, &GlobalTransform,  &Transform, &ComputedNode)>,
    wires_container: Query<(Entity, &RelativeCursorPosition, &ComputedNode), With<WireContainer>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut wires: ResMut<WireMinigame>,
    ui_scale : Res<UiScale>,
    sprite_assets: Res<SpriteAssets>,
    cursor: Res<CursorPosition>,
    window: Single<&Window>,

    grabbed_wire_rot: Query<(Entity, &Children), With<GrabbedWireRot>>,
    grabbed_wire_size: Query<Entity, With<GrabbedWire>>,
    mut malfunction: ResMut<Malfunction>,
    mut event_writer: EventWriter<PlaySoundEvent>,
) {
    let prev_locked = wires.locked_id.clone();
    if mouse_button.just_released(MouseButton::Left) {
        wires.locked_id = None;
    }
    let mut need_remove = mouse_button.just_released(MouseButton::Left);
    for (_container_entity, cursor_rel_pos, node) in wires_container {
        if let Some(wire_id) = wires.locked_id {
            if let Some(rel_pos) = cursor_rel_pos.normalized {
                let size = node.size;
                let Some(start) = wires.socket_positions.get(&wire_id) else {warn!("SOCKET POS NOT FOUND"); continue;};
                let start = start / ui_scale.0 + size / 2.0 / ui_scale.0 ;
                let end = rel_pos * node.size / ui_scale.0;

                let delta = end - start;
                let distance = delta.length();
                let angle = delta.y.atan2(delta.x);
                
                for (wire, child) in grabbed_wire_rot {
                    for c in child.iter() {
                        let Ok(c) = grabbed_wire_size.get(c) else {continue;};
                        commands.entity(c).insert((
                            Node {
                                margin: UiRect {
                                    top: Val::Px(-5.), ..default()
                                },
                                min_width: Val::Px(distance),
                                height: Val::Px(10.),
                                ..default()
                            },
                        ));
                    }
                    commands.entity(wire).insert((
                        Transform::from_rotation(Quat::from_rotation_z(angle)),
                        Node {
                            position_type: PositionType::Absolute,
                            max_height: Val::Px(0.),
                            max_width: Val::Px(10.),
                            left: Val::Px(start.x),
                            top: Val::Px(start.y),
                            width: Val::Px(10.),
                            height: Val::Px(0.),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
    for (_e, cursor_rel_pos, wire, global_transform, transform, node) in wires_q {
        if let Some(_rel_pos) = cursor_rel_pos.normalized {
            if cursor_rel_pos.mouse_over() &&
            mouse_button.just_pressed(MouseButton::Left) {
                wires.locked_id = Some(wire.id);
                let relative = global_transform.translation().xy() - window.size() * 0.5;
                wires.socket_positions.insert(wire.id, relative);
                info!("SOCKET POS: {:?}", relative);
                for (container_entity, _cursor_rel_pos, node) in wires_container {
                    let start = relative / ui_scale.0 + node.size / 2.0 / ui_scale.0 ;
                    let n = commands.spawn((
                        GrabbedWireRot,
                        Transform::from_rotation(Quat::from_rotation_z(0.0)),
                        Node {
                            position_type: PositionType::Absolute,
                            max_height: Val::Px(0.),
                            max_width: Val::Px(10.),
                            left: Val::Px(start.x),
                            top: Val::Px(start.y),
                            width: Val::Px(10.),
                            height: Val::Px(10.),
                            ..default()
                        },
                        children![
                            (
                                GrabbedWire,
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(-5.), ..default()
                                    },
                                    ..default()
                                },
                                ImageNode{
                                    image: sprite_assets.wire.clone(),
                                    color: wires.get_color(wire.id),
                                    image_mode: NodeImageMode::Sliced(TextureSlicer {
                                        border: BorderRect::axes(6.0, 0.0),
                                        center_scale_mode: SliceScaleMode::Tile{stretch_value: 1.0},
                                        sides_scale_mode: SliceScaleMode::Tile{stretch_value: 1.0},
                                        max_corner_scale: 1.0,
                                    }),
                                    ..default()
                                }
                            )
                        ]
                    )).id();
                    commands.entity(container_entity).add_child(n);
                    break;
                }
            }
            // handle wire connection
            let Some(locked_id) = prev_locked else {continue;};
            if locked_id == wire.id {continue;};
            
            if mouse_button.just_released(MouseButton::Left) {
                if cursor_rel_pos.mouse_over() {
                    if wires.connected.contains_key(&wire.id) || wires.connected.contains_key(&locked_id) {
                        need_remove = true;
                        info!("ALREADY CONNECTED");
                        continue;
                    };
                    let relative = global_transform.translation().xy() - window.size() * 0.5;
                    wires.socket_positions.insert(wire.id, relative);
                    for (_container_entity, _cursor_rel_pos, node) in wires_container {
                        info!("Try connect: {} -> {}", wires.connected.len(), wires.task.len());
                        if wires.task.get(&locked_id) != Some(&wire.id) {
                            info!("ASASASAASSSSSSSSSSSSSSSSSSSSSSSSS");
                            malfunction.resolved.push(Resolved {
                                resolved_type: crate::utils::debree::MalfunctionType::Reactor,
                                failed: true,
                            });
                        } else if wires.connected.len() == wires.task.len() {
                            event_writer.write(PlaySoundEvent::Success);
                            malfunction.resolved.push(Resolved {
                                resolved_type: crate::utils::debree::MalfunctionType::Reactor,
                                failed: false,
                            });
                        };
                        let Some(start) = wires.socket_positions.get(&locked_id) else {warn!("SOCKET POS NOT FOUND"); continue;};
                        let Some(end) = wires.socket_positions.get(&wire.id) else {warn!("SOCKET POS NOT FOUND"); continue;};
                        let start = start / ui_scale.0 + node.size / 2.0 / ui_scale.0 ;
                        let end = end / ui_scale.0 + node.size / 2.0 / ui_scale.0 ;
                        let delta = end - start;
                        let relative = global_transform.translation().xy() - window.size() * 0.5;
                        wires.socket_positions.insert(wire.id, relative);
                        for (wire, child) in grabbed_wire_rot {
                            commands.entity(wire).remove::<GrabbedWireRot>().insert((
                                ConnectedWire,
                            ));
                            for c in child {
                                commands.entity(*c).remove::<GrabbedWire>();
                            }
                        }
                        need_remove = false;
                        
                        wires.connected.insert(locked_id, wire.id);
                        wires.connected.insert(wire.id, locked_id);
                        break;
                    }
                }
            }
        }
    }
    
    if need_remove {
        for (wire, _child) in grabbed_wire_rot {
            commands.entity(wire).despawn();
        };
    }   

    
}



#[derive(Component)]
pub struct ConnectedWire;
#[derive(Component)]
pub struct GrabbedWire;
#[derive(Component)]
pub struct GrabbedWireRot;

