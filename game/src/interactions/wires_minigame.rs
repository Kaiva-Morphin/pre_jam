use bevy::{prelude::*, ui::RelativeCursorPosition};
use bevy_tailwind::tw;
use pixel_utils::camera::{TARGET_HEIGHT, TARGET_WIDTH};

use crate::{interactions::components::{InInteractionArray, InteractionTypes}, ui::{components::{containers::{base::{main_container_handle, sub_container_handle, ui_main_container, ui_sub_container}, text_display::text_display_green_handle, viewport_container::viewport_handle}, spinny::ui_spinny, ui_submit_button::submit_button_bundle, wire_inlet::{ui_wire_inlet, wire_inlet_bundle}}, target::LowresUiContainer}, utils::{custom_material_loader::SpriteAssets, debree::Malfunction, mouse::CursorPosition}};

#[derive(Resource, Default)]
pub struct Wires {
    pub is_locked: bool,
    pub locked_id: usize,
}

#[derive(Component)]
pub struct WireId {
    pub id: usize,
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
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
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

            let mut children = vec![];
            for i in 0..8 {
                children.push(commands.spawn(
                ui_main_container(&main, children![
                    ui_wire_inlet(&wire_bundle, WireId {id: i})
                ])).id());
            };
            
            let entity = commands.spawn(
                tw!("items-center justify-center w-full h-full"),
            ).with_children(|cmd| {
                cmd.spawn(ui_main_container(&main, ()))
                .with_children(|cmd| {
                    cmd.spawn(tw!("flex flex-row gap-[100px]"))
                    .with_children(|cmd| {
                        cmd.spawn(tw!("flex flex-col"))
                            .add_children(&children[0..4]);
                        cmd.spawn(tw!("flex flex-col"))
                            .add_children(&children[4..8]);
                    });
                });
            }).id();
            *already_spawned = Some(entity);
            commands.entity(*lowres_container).add_child(entity);
        }
    }
}

pub fn touch_wires_inlet(
    mut commands: Commands,
    wires_q: Query<(&RelativeCursorPosition, &WireId, &GlobalTransform, &ComputedNode)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut wires: ResMut<Wires>,
    sprite_assets: Res<SpriteAssets>,
    cursor: Res<CursorPosition>
) {
    if mouse_button.just_released(MouseButton::Left) {
        wires.is_locked = false;
    }
    for (cursor_rel_pos, wire_id, global_transform, node) in wires_q {
        if let Some(rel_pos) = cursor_rel_pos.normalized {
            if cursor_rel_pos.mouse_over() &&
            mouse_button.just_pressed(MouseButton::Left) {
                wires.is_locked = true;
                wires.locked_id = wire_id.id;
            }
            if wires.is_locked && wires.locked_id == wire_id.id {
                
                let size = node.size;
                let start = global_transform.translation().xy() - size - Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32) / 2.;
                let end = start + rel_pos * size;
                println!("{:?} {:?} {:?} {:?}", size, start, rel_pos, cursor.screen_position);

                let delta = end - start;
                let distance = delta.length();
                let angle = delta.y.atan2(delta.x);

                let midpoint = (start + end) / 2.0;
                commands.spawn((
                    ImageNode::from(sprite_assets.wire.clone()),
                    Transform::from_rotation(Quat::from_rotation_z(angle)),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(midpoint.x),
                        top: Val::Px(midpoint.y),
                        width: Val::Px(distance),
                        height: Val::Px(10.),
                        ..default()
                    }
                ));
            }
        }
    }
}



#[derive(Component)]
pub struct ToWorld;

pub fn get_pos(
    mut commands: Commands,
    wires_q: Query<(&RelativeCursorPosition, &WireId, &GlobalTransform, &ComputedNode)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut wires: ResMut<Wires>,
    sprite_assets: Res<SpriteAssets>,
    cursor: Res<CursorPosition>,

    pic: Option<Single<(Entity, &mut Transform), With<ToWorld>>>
) {
    // let Some((e, mut t)) = pic else {
    //     commands.spawn(

    //     );
    //     return;
    // };
    // if mouse_button.just_released(MouseButton::Left) {
    //     wires.is_locked = false;
    // }
    for (cursor_rel_pos, wire_id, global_transform, node) in wires_q {
        info!("Curs: {:?}", cursor_rel_pos.mouse_over());
        if let Some(rel_pos) = cursor_rel_pos.normalized {
            if cursor_rel_pos.mouse_over() &&
            mouse_button.just_pressed(MouseButton::Left) {
                wires.is_locked = true;
                wires.locked_id = wire_id.id;
            }
            if wires.is_locked && wires.locked_id == wire_id.id {
                
                let size = node.size;
                let start = global_transform.translation().xy() - size - Vec2::new(TARGET_WIDTH as f32, TARGET_HEIGHT as f32) / 2.;
                let end = start + rel_pos * size;
                println!("{:?} {:?} {:?} {:?}", size, start, rel_pos, cursor.screen_position);

                let delta = end - start;
                let distance = delta.length();
                let angle = delta.y.atan2(delta.x);

                let midpoint = (start + end) / 2.0;
                commands.spawn((
                    ImageNode::from(sprite_assets.wire.clone()),
                    Transform::from_rotation(Quat::from_rotation_z(angle)),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(midpoint.x),
                        top: Val::Px(midpoint.y),
                        width: Val::Px(distance),
                        height: Val::Px(10.),
                        ..default()
                    }
                ));
            }
        }
    }
}