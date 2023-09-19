use bevy::prelude::*;
use super::systems::*;
use crate::NoCameraPlayerPlugin;

/// includes all systems required for managing cameras
pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (follow_flagged, watch_flagged))
        ;
    }
}

/// Spawns a debug fly cam + systems for managing cameras
pub struct DefaultCameraPlugin;

impl Plugin for DefaultCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
                NoCameraPlayerPlugin, CameraManagerPlugin
            )
        )
        .add_systems(Startup, spawn_debug_cam)
        ;
    }
}