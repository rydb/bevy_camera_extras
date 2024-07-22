use bevy_ecs::{event::ManualEventReader, prelude::*};
use bevy_input::{mouse::MouseMotion, prelude::*};

/// Key configuration
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
            toggle_restraints: KeyCode::ControlLeft
        }
    }
}

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

#[derive(Resource)]
/// camera restraints. Enables freefly if this is set to false
pub struct RestraintsToggled(pub bool);

impl Default for RestraintsToggled {
    fn default() -> Self {
        Self(true)
    }
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
pub struct InputState {
    pub reader_motion: ManualEventReader<MouseMotion>,
}