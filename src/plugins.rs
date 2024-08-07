use bevy_app::prelude::*;

use super::*;

/// Includes everything camera controllers for this plugin need to function
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

impl Plugin for CameraExtrasPlugin {
    fn build(&self, app: &mut App) {
        
        app
        .init_resource::<InputState>()
        //.init_resource::<RestraintsToggled>()
        .insert_resource(CursorGrabbed(self.cursor_grabbed_by_default))
        .insert_resource(self.keybinds_override.unwrap_or_default())
        .insert_resource(self.movement_settings_override.unwrap_or_default())
        
        .add_systems(PostStartup, set_intial_grab_state)
        // .add_systems(Update, (
        //     follow_flagged, 
        //     watch_flagged, 
        //     move_to_attached
        // ))
        .add_systems(Update, move_to_attached)
        .add_systems(Update, camera_move)
        .add_systems(Update, camera_look)
        .add_systems(Update, cursor_grab)
        .add_systems(Update, check_for_setting_toggles)
        ;
    }
}