use std::ops::BitOr;
use std::ops::BitXorAssign;

use bevy_ecs::prelude::*;
use bevy_input::ButtonInput;
use bevy_render::camera::Camera;
use bevy_transform::components::Transform;
use bevy_input::prelude::*;
use glam::Vec3;

use bevy_app::prelude::*;
use bevy_input::{mouse::MouseMotion, prelude::*};
use bevy_ecs::{event::ManualEventReader, prelude::*};
use bevy_log::prelude::*;
use bevy_window::{prelude::*, CursorGrabMode, PrimaryWindow};
use bevy_time::prelude::*;
use glam::{EulerRot, Quat};
use crate::components::*;
use crate::resources::*;

use crate::{AttachedTo, Followed, Viewer, Watched};
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

pub fn check_for_restraints_toggle_press(
    mut restraints_toggle: ResMut<RestraintsToggled>,
    camera_controls: Res<KeyBindings>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(camera_controls.toggle_restraints) {
        restraints_toggle.0 ^= true;
    }
}

pub fn move_to_attached(
    mut attaching_cameras: Query<(&mut Transform, &AttachedTo), With<Camera>>,
    transforms: Query<&Transform, Without<Camera>>,
    //keys: Res<ButtonInput<KeyCode>>,
    restraints_toggle: Res<RestraintsToggled>,
) {
    if restraints_toggle.0 == true {
        for (mut cam_trans, target) in attaching_cameras.iter_mut() {
        
            //let Ok(cam_trans) = transforms.get_mut(camera_entity) else {return;};
            
            let Ok(target_trans) = transforms.get(target.0) else {return;};
    
    
            cam_trans.translation = target_trans.translation
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
                //let mut new_trans = *trans;
                //println!("new trans for ROTATION is: {:#?}", new_trans.translation);
                // look at the median cordinate between all "watched" entities
                trans.look_at(cord_total / Vec3::new(point_count, point_count, point_count), Vec3::new(0.0,0.0,0.0));
                //println!("looking at {:#?}", new_trans.rotation);
                //commands.entity(e).insert(new_trans);
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
    mut query: Query<(&CameraControls, &mut Transform), With<Camera>>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
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
                        } else if key == key_bindings.move_backward {
                            velocity -= forward;
                        } else if key == key_bindings.move_left {
                            velocity -= right;
                        } else if key == key_bindings.move_right {
                            velocity += right;
                        } else if key == key_bindings.move_ascend {
                            velocity += Vec3::Y;
                        } else if key == key_bindings.move_descend {
                            velocity -= Vec3::Y;
                        }
                    }
                }

                velocity = velocity.normalize_or_zero();

                transform.translation += velocity * time.delta_seconds() * settings.speed
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles looking around if cursor is locked
pub fn camera_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, (With<Camera>, With<CameraControls>)>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            //println!("making camera follow movement");

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

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
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

// sets a camera in the world to a debug camera, or if one doesn't exist, spawns one(!!!THIS BREAKS IF THERE IS MORE THEN ONE CAMERA!!!)
// pub fn set_debug_cam(
//     mut commands:Commands,
//     camera_query: Query<Entity, With<Camera>>
// ) {
//     //commands.insert_resource(RaycastPluginState::<Selectable>::default().with_debug_cursor());
//     if camera_query.iter().len() <= 0 {
//         commands.spawn(
//             (
//     Camera3dBundle {
//                 transform: Transform::from_xyz(5.0, 4.0, 5.0).with_rotation(Quat::from_rotation_y(PI / 2.5)),
//                 ..default()
//             },
//             FlyCam,
//             //RaycastSource::<Selectable>::new(),
//             //SelectionMode::default(),
//             Viewer{offset: Vec3::new(5.0, 5.0, 5.0)},
    
//         )
//         )
//         ;
//     } else {
//         for e in camera_query.iter() {
//             commands.entity(e)
//             .insert(FlyCam)
//             .insert(Viewer{offset: Vec3::new(5.0, 5.0, 5.0)})
//             ;
//         }
//     }

// }