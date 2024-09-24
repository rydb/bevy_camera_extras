mod components;
mod plugins;
mod resources;
mod systems;

use bevy_ecs::prelude::Bundle;
use bevy_render::prelude::*;
use bevy_transform::prelude::*;
pub use components::*;
use glam::{Quat, Vec3};
pub use plugins::*;
pub use resources::*;
use systems::*;

/// Camera attached to something. Specfic behaviour depends on [`CameraMode`]
#[derive(Bundle)]
pub struct CameraController {
    pub restrained: CameraRestrained,
    pub camera_mode: CameraMode,
    //pub targeting: CameraTargeting,
}

pub fn calculate_from_translation_and_focus(translation: Vec3, focus: Vec3) -> (f32, f32, f32) {
    let comp_vec = translation - focus;
    let mut radius = comp_vec.length();
    if radius == 0.0 {
        radius = 0.05; // Radius 0 causes problems
    }
    let yaw = if comp_vec.x == 0.0 && comp_vec.z >= 0.0 {
        0.0
    } else {
        (comp_vec.z / (comp_vec.x.powi(2) + comp_vec.z.powi(2)).sqrt()).acos()
    };
    let pitch = (comp_vec.y / radius).asin();
    (yaw, pitch, radius)
}

/// Update `transform` based on yaw, pitch, and the camera's focus and radius
pub fn update_orbit_transform(
    yaw: f32,
    pitch: f32,
    mut radius: f32,
    focus: Vec3,
    transform: &mut Transform,
    projection: &mut Projection,
) {
    let mut new_transform = Transform::IDENTITY;
    if let Projection::Orthographic(ref mut p) = *projection {
        p.scale = radius;
        // (near + far) / 2.0 ensures that objects near `focus` are not clipped
        radius = (p.near + p.far) / 2.0;
    }
    new_transform.rotation *= Quat::from_rotation_y(yaw) * Quat::from_rotation_x(-pitch);
    new_transform.translation += focus + new_transform.rotation * Vec3::new(0.0, 0.0, radius);
    *transform = new_transform;
}
