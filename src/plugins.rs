use bevy_app::prelude::*;
use super::systems::*;
use crate::FlyCameraSystems;

/// includes all systems required for managing cameras
pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (follow_flagged, watch_flagged))
        ;
    }
}

/// Sets a debug cam(or spawns one if one doesn't exist) + adds systems for managing cameras
pub struct DefaultCameraPlugin;

impl Plugin for DefaultCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
                FlyCameraSystems, CameraManagerPlugin
            )
        )
        .add_systems(PostStartup, set_debug_cam)
        ;
    }
}