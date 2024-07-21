use bevy_app::prelude::*;
use bevy_ecs::system::Resource;
use super::systems::*;
use crate::{set_intial_grab_state, FlyCameraSystems};

/// Includes everything to follow a set [`FlyCam`]
pub struct CameraExtrasPlugin {
    pub cursor_grabbed_by_default: bool
}


#[derive(Resource)]
pub struct InitialGrabStateSet(pub bool);

#[derive(Default, Resource)]
pub struct CursorGrabbed(pub bool);

impl Plugin for CameraExtrasPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CursorGrabbed(self.cursor_grabbed_by_default))
        //.insert_resource(InitialGrabStateSet(false))
        .add_plugins(
            (
                FlyCameraSystems,
            )
        )
        .add_systems(PostStartup, set_intial_grab_state)
        .add_systems(Update, (follow_flagged, watch_flagged, move_to_attached))
        ;
    }
}