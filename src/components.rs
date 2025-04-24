use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::Reflect;
use glam::Vec2;

// #[derive(Component)]
// pub struct Followed;

// #[derive(Component, Default)]
// pub struct Viewer {
//     pub offset: Vec3,
// }

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct PanOrbitCamera {}

pub struct CameraTarget;

// camera that is targeting this
#[derive(Component, Reflect)]
pub struct ObservedBy(pub Entity);

/// cached pov cam from camera when it was last in POV mode.
#[derive(Component, Clone, Copy, Reflect)]
pub struct POVCamCache(pub POVCam);

#[derive(Clone, Copy, PartialEq, Reflect, Debug)]
pub struct POVCamSettings {
    pub camera_distance_offset: Vec2,
    // The radius of the orbit, or the distance from the `focus` point.
    //pub radius: Option<f32>,
    // Rotation in radians around the global Y axis (longitudinal).
    //pub yaw: Option<f32>,
    // Rotation in radians around the local X axis (latitudinal).
    //pub pitch: Option<f32>,
    // The target focus point.
    //pub target_focus: Vec3,
    // The target yaw value.
    //pub target_yaw: f32,
    // The target pitch value.
    //pub target_pitch: f32,
    // The target radius value.
    //pub target_radius: f32,
    // The sensitivity of the orbiting motion.
    // Defaults to `1.0`.
    //pub orbit_sensitivity: f32,
    // How much smoothing is applied to the orbit motion.
    // Defaults to `0.8`.
    //pub orbit_smoothness: f32,
    //The sensitivity of moving the camera closer or further way using the scroll wheel.
    //pub zoom_sensitivity: f32,
    // How much smoothing is applied to the zoom motion.
    //pub zoom_smoothness: f32,

    // Whether to reverse the zoom direction.
    //pub reversed_zoom: bool,
}

impl Default for POVCamSettings {
    fn default() -> Self {
        Self {
            camera_distance_offset: Vec2::new(15.0, 10.0),
            // target_focus: Vec3::ZERO,
            // radius: None,
            // orbit_sensitivity: 1.0,
            // orbit_smoothness: 0.1,
            // reversed_zoom: false,
            // yaw: None,
            // pitch: None,
            // target_yaw: 0.0,
            // target_pitch: 0.0,
            // target_radius: 1.0,
            // zoom_sensitivity: 1.0,
            // zoom_smoothness: 0.1,
        }
    }
}

#[derive(Component, Clone, Copy, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub enum CameraMode {
    Free,
    POV(POVCam),
    Observer(ObserverCam),
}

#[derive(Clone, Copy, PartialEq, Reflect, Debug)]
pub struct POVCam {
    pub target: Entity,
    pub pov: POV,
    pub settings: POVCamSettings,
}

#[derive(Reflect, Copy, Clone, PartialEq, Debug)]
pub enum POV {
    FirstPerson,
    ThirdPerson,
    //Orbit,
}

#[derive(Reflect, Clone, Copy, PartialEq, Debug)]
pub enum ObserverCam {
    Orbit,
}

// #[derive(Clone, Copy, Reflect, PartialEq)]
// pub struct CameraDistanceOffset(pub Vec2);

// impl Default for CameraDistanceOffset {
//     fn default() -> Self {
//         Self(Vec2::new(15.0, 10.0))
//     }
// }

// #[derive(Component)]
// pub struct Watched;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraRestrained(pub bool);

// #[derive(Component, Reflect)]
// pub struct CameraTargeting(pub Option<Entity>);
