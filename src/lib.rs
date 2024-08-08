mod plugins;
mod components;
mod resources;
mod systems;

use bevy_ecs::prelude::Bundle;
pub use components::*;
pub use plugins::*;
pub use resources::*;
use systems::*;


/// Free fly cam.
#[derive(Bundle)]
pub struct CameraControllerFree {
    pub restrained: CameraRestrained
}

/// Camera attached to something. Specfic behaviour depends on [`CameraMode`]
#[derive(Bundle)]
pub struct CameraController {
    pub restrained: CameraRestrained,
    pub camera_mode: CameraMode,
    //pub targeting: CameraTargeting,
}
