use bevy_ecs::{event::ManualEventReader, prelude::*};
use bevy_input::{mouse::MouseMotion, prelude::*};
use bevy_reflect::Reflect;
use glam::Vec3;

/// Key configuration for camera
#[derive(Resource, Reflect, Clone, Copy, Debug)]
pub struct CamKeybinds {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
    /// toggle insertered restraints on camera.
    pub toggle_restraints: KeyCode,
    
    /// Switch between different modes of current camera kind.
    pub switch_camera_mode: KeyCode,

    /// Switch between different camera kinds(observer, POV, etc..)
    pub switch_camera_kind: KeyCode,

    /// Button used to orbit the camera.
    /// Defaults to `Button::Left`.
    pub orbit_drag_button: MouseButton,

    /// Key that must be pressed for `button_orbit` to work.
    /// Defaults to `None` (no modifier).
    pub modifier_orbit: Option<KeyCode>,

}

impl Default for CamKeybinds {
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
            switch_camera_kind: KeyCode::Backquote,
            orbit_drag_button: MouseButton::Left,
            modifier_orbit: None,


        }
    }
}

// Weather the window should grab or not grab the cursor. 
#[derive(Default, Resource, Debug)]
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