use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::Reflect;
use glam::{Vec2, Vec3};

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
    /// The radius of the orbit, or the distance from the `focus` point.
    pub radius: Option<f32>,
    /// Rotation in radians around the global Y axis (longitudinal). Updated automatically.
    /// If both `yaw` and `pitch` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// You should not update this after initialization - use `target_yaw` instead.
    /// Defaults to `None`.
    pub yaw: Option<f32>,
    /// Rotation in radians around the local X axis (latitudinal). Updated automatically.
    /// If both `yaw` and `pitch` are `0.0`, then the camera will be looking forward, i.e. in
    /// the `Vec3::NEG_Z` direction, with up being `Vec3::Y`.
    /// If set to `None`, it will be calculated from the camera's current position during
    /// initialization.
    /// You should not update this after initialization - use `target_pitch` instead.
    /// Defaults to `None`.
    pub pitch: Option<f32>,
    /// The target focus point. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `Vec3::ZERO`.
    pub target_focus: Vec3,
    /// The target yaw value. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `0.0`.
    pub target_yaw: f32,
    /// The target pitch value. The camera will smoothly transition to this value Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `0.0`.
    pub target_pitch: f32,
    /// The target radius value. The camera will smoothly transition to this value. Updated
    /// automatically, but you can also update it manually to control the camera independently of
    /// the mouse controls, e.g. with the keyboard.
    /// Defaults to `1.0`.
    pub target_radius: f32,
    /// The sensitivity of the orbiting motion.
    /// Defaults to `1.0`.
    pub orbit_sensitivity: f32,
    /// How much smoothing is applied to the orbit motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing.
    /// Defaults to `0.8`.
    pub orbit_smoothness: f32,
    /// The sensitivity of moving the camera closer or further way using the scroll wheel.
    /// Defaults to `1.0`.
    pub zoom_sensitivity: f32,
    /// How much smoothing is applied to the zoom motion. A value of `0.0` disables smoothing,
    /// so there's a 1:1 mapping of input to camera position. A value of `1.0` is infinite
    /// smoothing.
    /// Defaults to `0.8`.
    /// Note that this setting does not apply to pixel-based scroll events, as they are typically
    /// already smooth. It only applies to line-based scroll events.
    pub zoom_smoothness: f32,

    /// Whether to reverse the zoom direction.
    /// Defaults to `false`.
    pub reversed_zoom: bool,
}

impl Default for POVCamSettings {
    fn default() -> Self {
        Self {
            camera_distance_offset: Vec2::new(15.0, 10.0),
            target_focus: Vec3::ZERO,
            radius: None,
            orbit_sensitivity: 1.0,
            orbit_smoothness: 0.1,
            reversed_zoom: false,
            yaw: None,
            pitch: None,
            target_yaw: 0.0,
            target_pitch: 0.0,
            target_radius: 1.0,
            zoom_sensitivity: 1.0,
            zoom_smoothness: 0.1,
        }
    }
}

#[derive(Component, Clone, Copy, Reflect, PartialEq, Debug)]
pub enum CameraMode {
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
pub struct CameraRestrained(pub bool);

// #[derive(Component, Reflect)]
// pub struct CameraTargeting(pub Option<Entity>);
