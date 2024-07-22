pub mod plugins;
pub mod components;
pub mod resources;
mod systems;

//use bevy::ecs::event::{Events, ManualEventReader};
//use bevy::input::mouse::MouseMotion;
//use bevy::prelude::*;
//use bevy::window::{CursorGrabMode, PrimaryWindow};
//use bevy_component_extras::components::Debug;

use bevy_app::prelude::*;
use bevy_input::{mouse::MouseMotion, prelude::*};
use bevy_ecs::{event::ManualEventReader, prelude::*};
use bevy_log::prelude::*;
use bevy_render::camera::Camera;
use bevy_window::{prelude::*, CursorGrabMode, PrimaryWindow};
use bevy_time::prelude::*;
use bevy_transform::prelude::Transform;
use glam::{EulerRot, Quat, Vec3};
use crate::CursorGrabbed;
use crate::components::*;
use crate::resources::*;

pub mod prelude {
    pub use crate::*;
}



// /// Handles keyboard input and movement
// fn camera_move(
//     keys: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     primary_window: Query<&Window, With<PrimaryWindow>>,
//     settings: Res<MovementSettings>,
//     key_bindings: Res<KeyBindings>,
//     mut query: Query<(&CameraControls, &mut Transform), With<Camera>>, //    mut query: Query<&mut Transform, With<FlyCam>>,
// ) {
//     if let Ok(window) = primary_window.get_single() {
//         for (_camera, mut transform) in query.iter_mut() {
//             let mut velocity = Vec3::ZERO;
//             let local_z = transform.local_z();
//             let forward = -Vec3::new(local_z.x, 0., local_z.z);
//             let right = Vec3::new(local_z.z, 0., -local_z.x);

//             for key in keys.get_pressed() {
//                 match window.cursor.grab_mode {
//                     CursorGrabMode::None => (),
//                     _ => {
//                         let key = *key;
//                         if key == key_bindings.move_forward {
//                             velocity += forward;
//                         } else if key == key_bindings.move_backward {
//                             velocity -= forward;
//                         } else if key == key_bindings.move_left {
//                             velocity -= right;
//                         } else if key == key_bindings.move_right {
//                             velocity += right;
//                         } else if key == key_bindings.move_ascend {
//                             velocity += Vec3::Y;
//                         } else if key == key_bindings.move_descend {
//                             velocity -= Vec3::Y;
//                         }
//                     }
//                 }

//                 velocity = velocity.normalize_or_zero();

//                 transform.translation += velocity * time.delta_seconds() * settings.speed
//             }
//         }
//     } else {
//         warn!("Primary window not found for `player_move`!");
//     }
// }

// /// Handles looking around if cursor is locked
// fn camera_look(
//     settings: Res<MovementSettings>,
//     primary_window: Query<&Window, With<PrimaryWindow>>,
//     mut state: ResMut<InputState>,
//     motion: Res<Events<MouseMotion>>,
//     mut query: Query<&mut Transform, (With<Camera>, With<CameraControls>)>,
// ) {
//     if let Ok(window) = primary_window.get_single() {
//         for mut transform in query.iter_mut() {
//             //println!("making camera follow movement");

//             for ev in state.reader_motion.read(&motion) {
//                 let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
//                 match window.cursor.grab_mode {
//                     CursorGrabMode::None => (),
//                     _ => {
//                         // Using smallest of height or width ensures equal vertical and horizontal sensitivity
//                         let window_scale = window.height().min(window.width());
//                         pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
//                         yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
//                     }
//                 }

//                 pitch = pitch.clamp(-1.54, 1.54);

//                 // Order is important to prevent unintended roll
//                 transform.rotation =
//                     Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
//             }
//         }
//     } else {
//         warn!("Primary window not found for `player_look`!");
//     }
// }





// Grab cursor when an entity with FlyCam is added
// fn initial_grab_on_flycam_spawn(
//     mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
//     query_added: Query<Entity, (Added<Debug>, With<Camera>)>,
// ) {
//     if query_added.is_empty() {
//         return;
//     }

//     if let Ok(window) = &mut primary_window.get_single_mut() {
//         toggle_grab_cursor(window);
//     } else {
//         warn!("Primary window not found for `initial_grab_cursor`!");
//     }
// }

/// Adds all things required to manage a fly cam
pub struct FlyCameraSystems;
impl Plugin for FlyCameraSystems {
    fn build(&self, app: &mut App) {
        app
;
    }
}

//pub struct CameraGrabbed(bool);