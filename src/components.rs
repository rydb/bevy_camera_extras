use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::Reflect;
use glam::Vec2;


// #[derive(Component)]
// pub struct Followed;

// #[derive(Component, Default)]
// pub struct Viewer {
//     pub offset: Vec3,
// }

#[derive(Component, Clone, Copy, Reflect)]
pub struct CameraDistanceOffsetCache(pub CameraDistanceOffset);

#[derive(Component, Clone, Copy, Reflect, PartialEq)]
pub enum CameraMode {
    FirstPerson,
    /// third person. Set distance from target with vec.
    ThirdPerson(CameraDistanceOffset),
}

#[derive(Clone, Copy, Reflect, PartialEq)]
pub struct CameraDistanceOffset(pub Vec2);

impl Default for CameraDistanceOffset {
    fn default() -> Self {
        Self(Vec2::new(15.0, 10.0))
    }
}

// #[derive(Component)]
// pub struct Watched;

#[derive(Component)]
pub struct CameraRestrained(pub bool);


#[derive(Component)]
pub struct CameraTargeting(pub Entity);
