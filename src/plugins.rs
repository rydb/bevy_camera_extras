use bevy_app::prelude::*;
use bevy_ecs::system::Resource;
use super::systems::*;
use super::resources::*;
use crate::{InputState, KeyBindings, MovementSettings};

/// Includes everything to follow a set [`FlyCam`]
pub struct CameraExtrasPlugin {
    pub cursor_grabbed_by_default: bool,
    /// optional override for [`KeyBindings`], leave as None for default settings.
    pub keybinds_override: Option<KeyBindings>,
    /// optional override for [`MovementSettings`], leave as None for default settings.
    pub movement_settings_override: Option<MovementSettings>
}


impl Default for CameraExtrasPlugin {
    fn default() -> Self {
        Self {
            cursor_grabbed_by_default: true,
            keybinds_override: None,
            movement_settings_override: None,
        }
    }
}

// #[derive(Resource)]
// pub struct InitialGrabStateSet(pub bool);



impl Plugin for CameraExtrasPlugin {
    fn build(&self, app: &mut App) {
        
        app
        .init_resource::<InputState>()
        .init_resource::<RestraintsToggled>()
        .insert_resource(CursorGrabbed(self.cursor_grabbed_by_default))
        .insert_resource(self.keybinds_override.unwrap_or_default())
        .insert_resource(self.movement_settings_override.unwrap_or_default())
        
        .add_systems(PostStartup, set_intial_grab_state)
        .add_systems(Update, (follow_flagged, watch_flagged, move_to_attached))
        .add_systems(Update, camera_move)
        .add_systems(Update, camera_look)
        .add_systems(Update, cursor_grab)
        .add_systems(Update, check_for_restraints_toggle_press)
        ;
    }
}