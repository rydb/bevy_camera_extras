use bevy_ecs::component::Component;
use glam::Vec3;


#[derive(Component)]
pub struct Followed;

#[derive(Component, Default)]
pub struct Viewer {
    pub offset: Vec3,
}

#[derive(Component)]
pub struct Watched;

#[derive(Component)]
pub struct FlyCam;
