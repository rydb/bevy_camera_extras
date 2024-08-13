//! A demo to show/test the different capabilities of the camera

use bevy::prelude::*;
use bevy_camera_extras::*;
use bevy_inspector_egui::{bevy_egui::EguiContext, quick::WorldInspectorPlugin};
use bevy_ui_extras::systems::visualize_right_sidepanel_for;
use bevy_window::PrimaryWindow;
use egui::{text::LayoutJob, Color32, FontId, RichText, TextFormat};
use egui_extras::syntax_highlighting::CodeTheme;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            keybinds_override: None,
            movement_settings_override: None,
        })
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, display_controls)
        .add_systems(Update, visualize_right_sidepanel_for::<CameraMode>)
        .run();
}
pub fn display_controls(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    keybinds: Res<CamKeybinds>,
    camera: Query<&CameraMode>
) {


    //let pretty_camera_controls = egui::RichText::new(camera_controls)
    for mut context in primary_window.iter_mut() {
        let camera_controls = format!("{:#?}", *keybinds);

        let Ok(camera_mode) = camera.get_single() else {return;};
        let lines = camera_controls.split("\n")
        .enumerate()
        .map(|(i, line)| {
            let mut job = LayoutJob::default();
            let tokens = line.split(&[':', '{', '}'][..]).collect::<Vec<_>>();
            println!("tokens for line {:#}, are {:#?}", i, tokens);
            if i == 0 {
                for (n, token) in tokens.iter().enumerate() {
                    let color = if n == 0 {
                        Color32::GREEN
                    } else {
                        Color32::WHITE
                    };
                    job.append(token, 0.0, TextFormat {
                        font_id: FontId::new(14.0, egui::FontFamily::Proportional),
                        color: color,
                        //background: Color32::WHITE,
                        ..Default::default()
                    });
                }

                return job
            };
    
            for (n, token) in tokens.iter().enumerate() {
                let color = if Some(token) == tokens.last() {
                    Color32::WHITE
                } else if n == 0 {
                    Color32::LIGHT_BLUE
                } else {
                    Color32::WHITE
                };
                job.append(&(token.to_string() + " "), 0.0, TextFormat {
                    font_id: FontId::new(14.0, egui::FontFamily::Proportional),
                    color: color,
                    //background: Color32::WHITE,
                    ..Default::default()
                })
            }
            job
        })
        .collect::<Vec<_>>();

        egui::Window::new("Camera Controls")
        .show(context.get_mut(), |ui| {
            //egui_extras::syntax_highlighting::highlight(ui.ctx(), &CodeTheme::dark(), &camera_controls, "rs");

            for line in lines {
                ui.label(line);
            }
            ui.label(format!("{:#?}", camera_mode));
        });
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::new(0.0, 1.0, 0.0), Vec2::new(10.0, 10.0))),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    
    // player
    let cube = commands.spawn(
    (
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
                
            },
            Name::new("player"),
        )

    ).id();
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
    let cam = commands.spawn(
    (
        Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
        CameraController {
            camera_mode: CameraMode::POV(POVCam {
                target: cube,
                pov: POV::FirstPerson,
                settings: POVCamSettings::default()
            }),
            restrained: CameraRestrained(true),
            //targeting: CameraTargeting(Some(cube)),
        }
    )).id();    
    
    commands.entity(cube).insert(ObservedBy(cam));
}
