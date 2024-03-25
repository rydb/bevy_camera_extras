use bevy_app::prelude::*;
use super::systems::*;
use crate::FlyCameraSystems;

/// Includes everything to follow a set [`FlyCam`]
pub struct DefaultCameraPlugin;

impl Plugin for DefaultCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
                FlyCameraSystems,
            )
        )
        .add_systems(Update, (follow_flagged, watch_flagged))
        ;
    }
}