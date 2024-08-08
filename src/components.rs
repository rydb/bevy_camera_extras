use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::Reflect;
use enumset::EnumSetType;
use glam::Vec2;


// #[derive(Component)]
// pub struct Followed;

// #[derive(Component, Default)]
// pub struct Viewer {
//     pub offset: Vec3,
// }

pub struct CameraTarget;

// camera that is targeting this
#[derive(Component, Reflect)]
pub struct ObservedFrom(pub Entity);

/// cached pov cam from camera when it was last in POV mode.
#[derive(Component, Clone, Copy, Reflect)]
pub struct POVCamCache(pub POVCam);


#[derive(Clone, Copy, PartialEq, Reflect)]
pub struct POVCamSettings {
    pub camera_distance_offset: Vec2
}

impl Default for POVCamSettings {
    fn default() -> Self {
        Self {
            camera_distance_offset: Vec2::new(15.0, 10.0)
        }
    }
}

#[derive(Component, Clone, Copy, Reflect, PartialEq)]
pub enum CameraMode {
    POV(POVCam),
    Observer,
}

#[derive(Clone, Copy, PartialEq, Reflect)]
pub struct POVCam {
    pub target: Entity,
    pub pov: POV,
    pub settings: POVCamSettings
}

#[derive(Reflect, Copy, Clone, PartialEq)]
pub enum POV {
    FirstPerson,
    ThirdPerson,
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
