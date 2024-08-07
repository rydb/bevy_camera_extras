use std::*;

use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_render::camera::Camera;
use bevy_transform::components::Transform;
use bevy_input::prelude::*;
use glam::Vec3;

use bevy_input::mouse::MouseMotion;
use bevy_log::prelude::*;
use bevy_window::{prelude::*, CursorGrabMode, PrimaryWindow};
use bevy_time::prelude::*;
use glam::{EulerRot, Quat};

use crate::*;

pub fn check_for_setting_toggles(
    //mut restraints_toggle: ResMut<RestraintsToggled>,
    //cameras: Query<(Entity, Option<&RestraintsToggled>), With<CameraControls>>,
    camera_keybinds: Res<KeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(Entity, &mut CameraMode, Option<&mut CameraRestrained>)>,
    mut commands: Commands,
    pov_cam_settings: Query<&POVCamCache>,
) {
    if keys.just_pressed(camera_keybinds.toggle_restraints) {
        for (_, _, restraints_check) in cameras.iter_mut() {
            if let Some(mut toggle) = restraints_check {
            
                toggle.0 ^= true;
            }
        }
        //restraints_toggle.0 ^= true;
    }
    if keys.just_pressed(camera_keybinds.switch_camera_mode) {
        for (e, mut camera_mode, _) in cameras.iter_mut() {
            
            *camera_mode = match *camera_mode {
                CameraMode::POV(cam) => {
                    let settings = match pov_cam_settings.get(e) {
                        Ok(item) => item.0.settings,
                        Err(_) => POVCamSettings::default(),
                    };
                    let pov = match cam.pov {
                        POV::FirstPerson => POV::ThirdPerson,
                        POV::ThirdPerson => POV::FirstPerson,
                    };
                    CameraMode::POV(POVCam {
                        target: cam.target,
                        pov: pov,
                        settings: settings
                    })
                },
                CameraMode::Observer => *camera_mode
            }
        }
    }
    if keys.just_pressed(camera_keybinds.switch_camera_kind) {
        for (e, mut camera_mode, _) in cameras.iter_mut() {
            *camera_mode = match *camera_mode {
                CameraMode::POV(cam) => 
                {
                    commands.entity(e).insert(POVCamCache(cam));
                    CameraMode::Observer
                }
                CameraMode::Observer => {
                    let cam = match pov_cam_settings.get(e) {
                        Ok(item) => item.0,
                        Err(_) => {
                            warn!("switching to camera without previously set POV cam settings not implemented, ignoring this attempt");
                            return
                        },
                    };
                    CameraMode::POV(cam)
                },
            }
        }
    }
}

pub fn move_camera_based_on_mode(
    to_watch_querry: Query<Entity, With<ObservedFrom>>,
    mut cameras: Query<(Entity, &mut Transform, &CameraMode, Option<&CameraRestrained>), With<Camera>>,
    transforms: Query<&Transform, Without<Camera>>,
    pov_cam_settings: Query<&POVCamCache>,
    //keys: Res<ButtonInput<KeyCode>>,
    //restraints_toggle: Res<RestraintsToggled>,
) {
    //if restraints_toggle.0 == true {
    for (cam_entity, mut cam_trans, cam_info, restrained) in cameras.iter_mut() {
    
        //let Ok(cam_trans) = transforms.get_mut(camera_entity) else {return;};
        //let Some(attach_target) = targeting.target else {return;};
        let restraints_toggled = match restrained {
            Some(toggle) => toggle.0,
            None => false,
        };
        if restraints_toggled == true {
            match cam_info{
                CameraMode::POV(cam) => {
                    let settings = match pov_cam_settings.get(cam_entity) {
                        Ok(item) => item.0.settings,
                        Err(_) => POVCamSettings::default(),
                    };
                    match cam.pov {
                        POV::FirstPerson => {
                            //let Some(target) = targeting.0 else {return};
                            let Ok(target_trans) = transforms.get(cam.target) else {return;};

                            cam_trans.translation = target_trans.translation
                        },
                        POV::ThirdPerson => {
                            
                            let Ok(target_trans) = transforms.get(cam.target.clone()) else {return;};

                            let car_position = target_trans.translation;
                            let car_forward = target_trans.forward();
                
                            // Camera should follow the car from above and slightly behind it
                            // let follow_distance = 15.0;
                            // let follow_height = 10.0;
                            let follow_distance = settings.camera_distance_offset.x;
                            let follow_height = settings.camera_distance_offset.y;
                
                            // Calculate desired camera position behind the car
                            let mut desired_camera_position = car_position - car_forward * follow_distance;
                            desired_camera_position.y += follow_height;
                
                            // Smoothly move the camera to the desired position
                            cam_trans.translation = desired_camera_position;
                
                            // Make the camera look at the car with a slight downward angle
                            cam_trans.look_at(car_position, Vec3::Y);
                        },
                    }
                },
                CameraMode::Observer => {
                    if to_watch_querry.iter().len() > 0 {
                        let mut point_count = 0.0;
                        let mut cord_total = Vec3::new(0.0,0.0,0.0);
                
                        for e in to_watch_querry.iter() {
                            if let Ok(trans) = transforms.get(e) {
                                point_count += 1.0;
                                cord_total += trans.translation;
                            }
                        }
                        cam_trans.look_at(cord_total / Vec3::new(point_count, point_count, point_count), Vec3::new(0.0,0.0,0.0));
                
                    }
                }     


            };    
        }


    }
    //}
}

/// Handles keyboard input and movement
pub fn camera_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    //restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<(&CameraMode, &mut Transform, Option<&CameraRestrained>), With<Camera>>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to move camera, Reason: {:#}", err);
            return;
        }
    };

    for (_camera, mut transform, target_config) in query.iter_mut() {
        let restraints_toggled = match target_config {
            Some(toggle) => toggle.0,
            None => false,
        };
        if restraints_toggled == false{
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
    //restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<(&mut Transform, &CameraMode, Option<&CameraRestrained>), With<Camera>>,
) {

    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to rotate camera, Reason: {:#}", err);
            return;
        }
    };


    //if restraints_toggled.0 == false {
    for (mut transform, camera_controls, restraints) in query.iter_mut() {
        let restraints_toggled = match restraints {
            Some(toggle) => toggle.0,
            None => false,
        };
        let first_person_look = match camera_controls {
            CameraMode::POV(cam) => match cam.pov {
                POV::FirstPerson => true,
                POV::ThirdPerson => false

            },
            CameraMode::Observer => false
        };
        
        if restraints_toggled == false || first_person_look {
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
        } 
    }
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