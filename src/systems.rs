use std::*;
use std::ops::BitXorAssign;

use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_render::camera::Camera;
use bevy_transform::commands;
use bevy_transform::components::Transform;
use bevy_input::prelude::*;
use glam::Vec3;

use bevy_input::mouse::MouseMotion;
use bevy_log::prelude::*;
use bevy_window::{prelude::*, CursorGrabMode, PrimaryWindow};
use bevy_time::prelude::*;
use glam::{EulerRot, Quat};
use crate::components::*;
use crate::resources::*;

use crate::*;
//use bevy_component_extras::components::*;
/// follow behind entities marked for following

pub fn follow_flagged (
    //mut commands: Commands,
    to_watch_querry: Query<Entity, With<Followed>>,
    viewer_querry: Query<(Entity, &Viewer)>,
    mut transform_querry: Query<&mut Transform>,
) {
    let mut cord_total = Vec3::new(0.0,0.0,0.0);

    if to_watch_querry.iter().len() > 0 {
        for e in to_watch_querry.iter() {
            if let Ok(trans) = transform_querry.get(e) {
                cord_total += trans.translation;
            }
        }
        for (e, viewer) in viewer_querry.iter() {
            if let Ok(mut trans) = transform_querry.get_mut(e) {
                //println!("following {:#?}", e);
                //println!("new trans for FOLLOW is {:#}", new_trans.translation);
                trans.translation = cord_total + viewer.offset;
                //println!("following all followed entities at: {:#?}", new_trans.translation);
                //commands.entity(e).insert(new_trans);
            }
    
        }
    }

}

pub fn check_for_setting_toggles(
    mut restraints_toggle: ResMut<RestraintsToggled>,
    camera_keybinds: Res<KeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(Entity, &mut CameraControls)>,
    mut commands: Commands,
    cached_offsets: Query<&CameraDistanceOffsetCache>,
) {
    if keys.just_pressed(camera_keybinds.toggle_restraints) {
        restraints_toggle.0 ^= true;
    }
    if keys.just_pressed(camera_keybinds.switch_camera_mode) {
        for (e, mut camera) in cameras.iter_mut() {
            camera.camera_mode = match camera.camera_mode {
                CameraMode::FirstPerson => {
                    let offset = match cached_offsets.get(e) {
                        Ok(item) => item.0,
                        Err(_) => CameraDistanceOffset::default(),
                    };
                    CameraMode::ThirdPerson(offset)
                },
                CameraMode::ThirdPerson(offset) => {
                    commands.entity(e).insert(CameraDistanceOffsetCache(offset));
                    CameraMode::FirstPerson
                },
            }
        }
    }
}

pub fn move_to_attached(
    mut attaching_cameras: Query<(&mut Transform, &CameraControls), With<Camera>>,
    transforms: Query<&Transform, Without<Camera>>,
    //keys: Res<ButtonInput<KeyCode>>,
    restraints_toggle: Res<RestraintsToggled>,
) {
    if restraints_toggle.0 == true {
        for (mut cam_trans, cam_info) in attaching_cameras.iter_mut() {
        
            //let Ok(cam_trans) = transforms.get_mut(camera_entity) else {return;};
            let Ok(target_trans) = transforms.get(cam_info.attach_to) else {return;};

            match cam_info.camera_mode {
                CameraMode::FirstPerson => {
                    cam_trans.translation = target_trans.translation
                },
                CameraMode::ThirdPerson(offset) => {
                    let car_position = target_trans.translation;
                    let car_forward = target_trans.forward();
        
                    // Camera should follow the car from above and slightly behind it
                    // let follow_distance = 15.0;
                    // let follow_height = 10.0;
                    let follow_distance = offset.0.x;
                    let follow_height = offset.0.y;
        
                    // Calculate desired camera position behind the car
                    let mut desired_camera_position = car_position - car_forward * follow_distance;
                    desired_camera_position.y += follow_height;
        
                    // Smoothly move the camera to the desired position
                    cam_trans.translation = desired_camera_position;
        
                    // Make the camera look at the car with a slight downward angle
                    cam_trans.look_at(car_position, Vec3::Y);
                },
            };    
    
        }
    }
}

/// rotates camera to watch entities marked for watching
pub fn watch_flagged(
    //mut commands: Commands,
    to_watch_querry: Query<Entity, With<Watched>>,
    viewer_querry: Query<Entity, With<Viewer>>,
    mut transform_querry: Query<&mut Transform>,

) {
    if to_watch_querry.iter().len() > 0 {
        let mut point_count = 0.0;
        let mut cord_total = Vec3::new(0.0,0.0,0.0);

        for e in to_watch_querry.iter() {
            if let Ok(trans) = transform_querry.get(e) {
                point_count += 1.0;
                cord_total += trans.translation;
            }
        }
        for e in viewer_querry.iter() {
            if let Ok(mut trans) = transform_querry.get_mut(e) {
                trans.look_at(cord_total / Vec3::new(point_count, point_count, point_count), Vec3::new(0.0,0.0,0.0));
            }

        }
    }
}

/// Handles keyboard input and movement
pub fn camera_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<(&CameraControls, &mut Transform), With<Camera>>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to move camera, Reason: {:#}", err);
            return;
        }
    };

    if restraints_toggled.0 == false {
        for (_camera, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);


            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == key_bindings.move_forward {
                            velocity += forward;
                        } if key == key_bindings.move_backward {
                            velocity -= forward;
                        } if key == key_bindings.move_left {
                            velocity -= right;
                        } if key == key_bindings.move_right {
                            velocity += right;
                        } if key == key_bindings.move_ascend {
                            velocity += Vec3::Y;
                        } if key == key_bindings.move_descend {
                            velocity -= Vec3::Y;
                        }
                    }
                }
            }
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * settings.speed

        }
    } 
}


/// Handles looking around if cursor is locked
pub fn camera_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<(&mut Transform, &CameraControls), (With<Camera>)>,
) {

    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to rotate camera, Reason: {:#}", err);
            return;
        }
    };

    //if restraints_toggled.0 == false {
        for (mut transform, camera_controls) in query.iter_mut() {
            if restraints_toggled.0 == false || camera_controls.camera_mode == CameraMode::FirstPerson {
                for ev in state.reader_motion.read(&motion) {
                    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                    match window.cursor.grab_mode {
                        CursorGrabMode::None => (),
                        _ => {
                            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                            let window_scale = window.height().min(window.width());
                            pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                            yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                        }
                    }
    
                    pitch = pitch.clamp(-1.54, 1.54);
    
                    transform.rotation =
                        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
                }
            } else {
            match camera_controls.camera_mode {
                CameraMode::FirstPerson => {
                    //Freefly look is first person look at the moment, skip
                    continue;
                },
                CameraMode::ThirdPerson(_) => {
                    //TODO
                },
            }
            }
            


        }
    //}
}

pub fn cursor_grab(
    grabbed: ResMut<CursorGrabbed>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window, grabbed);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

/// Grabs/ungrabs mouse cursor
pub fn toggle_grab_cursor(window: &mut Window, mut grabbed: ResMut<CursorGrabbed>) {
    match grabbed.0 {
        false => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
            
            grabbed.0 = true;
        },
        true => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
            
            grabbed.0 = false;
        },
    }
}


pub fn set_intial_grab_state(
    grabbed: ResMut<CursorGrabbed>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        println!("setting initial grab state");

        toggle_grab_cursor(&mut window, grabbed);
    }
}