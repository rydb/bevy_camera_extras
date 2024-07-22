use bevy_ecs::{component::Component, entity::Entity};
use bevy_reflect::Reflect;
use glam::Vec3;


#[derive(Component)]
pub struct Followed;

#[derive(Component, Default)]
pub struct Viewer {
    pub offset: Vec3,
}

#[derive(Component, Clone, Copy, Reflect)]
pub struct AttachedTo(pub Entity);

#[derive(Component)]
pub struct Watched;

/// A flag to mark an entity to have camera controllers.
/// add additional camera control flag components to create the camera kind you want.
#[derive(Component)]
pub struct CameraControls;
