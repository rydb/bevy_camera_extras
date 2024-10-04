//! A demo to show/test the different capabilities of the camera

use bevy::prelude::*;
use bevy_camera_extras::*;
use bevy_ui_extras::{visualize_components_for, visualize_resource, UiExtrasDebug};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiExtrasDebug::default())
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            keybinds_override: None,
            movement_settings_override: None,
        })
        //.add_plugins(WorldInspectorPlugin::default())
        .add_systems(
            Update,
            visualize_components_for::<CameraMode>(bevy_ui_extras::Display::Side(
                bevy_ui_extras::Side::Right,
            )),
        )
        .add_systems(
            Update,
            visualize_resource::<CamKeybinds>(bevy_ui_extras::Display::Window),
        )
        .add_systems(PostStartup, setup)
        .run();
}
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(10.0, 10.0),
        )),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // player
    let cube = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            Name::new("player"),
        ))
        .id();
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    let cam = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            CameraController {
                camera_mode: CameraMode::POV(POVCam {
                    target: cube,
                    pov: POV::FirstPerson,
                    settings: POVCamSettings::default(),
                }),
                restrained: CameraRestrained(true),
            },
        ))
        .id();

    commands.entity(cube).insert(ObservedBy(cam));
}
