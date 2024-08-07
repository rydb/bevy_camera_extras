use bevy_ecs::{event::ManualEventReader, prelude::*};
use bevy_input::{mouse::MouseMotion, prelude::*};

/// Key configuration for camera
#[derive(Resource, Clone, Copy)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
    /// toggle insertered restraints on camera.
    pub toggle_restraints: KeyCode,
    
    /// Switched between different camera modes
    pub switch_camera_mode: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            toggle_grab_cursor: KeyCode::Escape,
            toggle_restraints: KeyCode::ControlLeft,
            switch_camera_mode: KeyCode::Tab,
        }
    }
}

// Weather the window should grab or not grab the cursor. 
#[derive(Default, Resource)]
pub struct CursorGrabbed(pub bool);

/// Mouse sensitivity and movement speed
#[derive(Resource, Clone, Copy)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
pub struct InputState {
    pub reader_motion: ManualEventReader<MouseMotion>,
}