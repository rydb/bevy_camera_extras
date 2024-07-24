use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::Reflect;
use glam::{Vec2, Vec3};


#[derive(Component)]
pub struct Followed;

#[derive(Component, Default)]
pub struct Viewer {
    pub offset: Vec3,
}

// #[derive(Component, Clone, Copy, Reflect)]
// pub struct AttachedTo(pub Entity);

// pub struct Came {
//     pub attach_to: Entity,

// }


#[derive(Component, Clone, Copy, Reflect)]
pub struct CameraDistanceOffsetCache(pub CameraDistanceOffset);

#[derive(Clone, Copy, Reflect)]
pub enum CameraMode {
    FirstPerson,
    /// third person. Set distance from target with vec.
    ThirdPerson(CameraDistanceOffset),
}

#[derive(Clone, Copy, Reflect)]
pub struct CameraDistanceOffset(pub Vec2);

impl Default for CameraDistanceOffset {
    fn default() -> Self {
        Self(Vec2::new(15.0, 10.0))
    }
}

// #[derive(Component, Reflect)]
// pub struct ThirdPersonOffset {
//     pub offset: Vec2
// }

#[derive(Component)]
pub struct Watched;

/// A flag to mark an entity to have camera controllers.
/// add additional camera control flag components to create the camera kind you want.
#[derive(Component, Clone, Copy, Reflect)]
pub struct CameraControls {
    pub attach_to: Entity, 
    pub camera_mode: CameraMode,
}
