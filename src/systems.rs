use std::*;

use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_input::ButtonInput;
use bevy_render::camera::Camera;
use bevy_transform::components::Transform;
use glam::Vec3;

use bevy_input::mouse::MouseMotion;
use bevy_log::prelude::*;
use bevy_time::prelude::*;
use bevy_window::{prelude::*, CursorGrabMode, PrimaryWindow};
use glam::{EulerRot, Quat};

use crate::*;

pub fn check_for_setting_toggles(
    //mut restraints_toggle: ResMut<RestraintsToggled>,
    //cameras: Query<(Entity, Option<&RestraintsToggled>), With<CameraControls>>,
    camera_keybinds: Res<CamKeybinds>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cameras: Query<(Entity, &mut CameraMode, Option<&mut CameraRestrained>)>,
    mut commands: Commands,
    pov_cam_settings: Query<&POVCamCache>,
) {
    if keys.just_pressed(camera_keybinds.toggle_restraints) {
        for (_, _, restraints_check) in cameras.iter_mut() {
            if let Some(mut toggle) = restraints_check {
                toggle.0 ^= true;
            }
        }
        //restraints_toggle.0 ^= true;
    }
    if keys.just_pressed(camera_keybinds.switch_camera_mode) {
        for (e, mut camera_mode, _) in cameras.iter_mut() {
            *camera_mode = match *camera_mode {
                CameraMode::POV(cam) => {
                    let settings = match pov_cam_settings.get(e) {
                        Ok(item) => item.0.settings,
                        Err(_) => POVCamSettings::default(),
                    };
                    let pov = match cam.pov {
                        POV::FirstPerson => POV::ThirdPerson,
                        POV::ThirdPerson => POV::FirstPerson,
                        // POV::FirstPerson => POV::ThirdPerson,
                        // POV::ThirdPerson => POV::Orbit,
                        // POV::Orbit => POV::FirstPerson,
                    };
                    CameraMode::POV(POVCam {
                        target: cam.target,
                        pov: pov,
                        settings: settings,
                    })
                }
                CameraMode::Observer(..) => *camera_mode,
            }
        }
    }
    if keys.just_pressed(camera_keybinds.switch_camera_kind) {
        for (e, mut camera_mode, _) in cameras.iter_mut() {
            *camera_mode = match *camera_mode {
                CameraMode::POV(cam) => {
                    commands.entity(e).insert(POVCamCache(cam));
                    CameraMode::Observer(ObserverCam::Orbit)
                }
                CameraMode::Observer(..) => {
                    let cam = match pov_cam_settings.get(e) {
                        Ok(item) => item.0,
                        Err(_) => {
                            warn!("switching to camera without previously set POV cam settings not implemented, ignoring this attempt");
                            return;
                        }
                    };
                    CameraMode::POV(cam)
                }
            }
        }
    }
}

pub fn move_camera_based_on_mode(
    to_watch_querry: Query<Entity, With<ObservedBy>>,
    mut cameras: Query<
        (
            Entity,
            &mut Transform,
            &mut Projection,
            &CameraMode,
            Option<&CameraRestrained>,
        ),
        With<Camera>,
    >,
    transforms: Query<&Transform, Without<Camera>>,
    pov_cam_settings: Query<&POVCamCache>,
    //keys: Res<ButtonInput<KeyCode>>,
    //restraints_toggle: Res<RestraintsToggled>,
) {
    //if restraints_toggle.0 == true {
    for (cam_entity, mut cam_trans, mut _projection, cam_info, restrained) in cameras.iter_mut() {
        //let Ok(cam_trans) = transforms.get_mut(camera_entity) else {return;};
        //let Some(attach_target) = targeting.target else {return;};
        let restraints_toggled = match restrained {
            Some(toggle) => toggle.0,
            None => false,
        };
        if restraints_toggled == true {
            let settings = match pov_cam_settings.get(cam_entity) {
                Ok(item) => item.0.settings,
                Err(_) => POVCamSettings::default(),
            };
            match cam_info {
                CameraMode::POV(cam) => {
                    let Ok(target_trans) = transforms.get(cam.target) else {
                        return;
                    };

                    match cam.pov {
                        POV::FirstPerson => {
                            //let Some(target) = targeting.0 else {return};
                            cam_trans.translation = target_trans.translation
                        }
                        POV::ThirdPerson => {
                            let car_position = target_trans.translation;
                            let car_forward = target_trans.forward();

                            // Camera should follow the car from above and slightly behind it
                            // let follow_distance = 15.0;
                            // let follow_height = 10.0;
                            let follow_distance = settings.camera_distance_offset.x;
                            let follow_height = settings.camera_distance_offset.y;

                            // Calculate desired camera position behind the car
                            let mut desired_camera_position =
                                car_position - car_forward * follow_distance;
                            desired_camera_position.y += follow_height;

                            // Smoothly move the camera to the desired position
                            cam_trans.translation = desired_camera_position;

                            // Make the camera look at the car with a slight downward angle
                            cam_trans.look_at(car_position, Vec3::Y);
                        }
                        // POV::Orbit => {
                        //     if !settings.initialized {
                        //         // Calculate yaw, pitch, and radius from the camera's position. If user sets all
                        //         // these explicitly, this calculation is wasted, but that's okay since it will only run
                        //         // once on init.
                        //         let (yaw, pitch, radius) =
                        //             calculate_from_translation_and_focus(cam_trans.translation, target_trans.translation);
                        //         let &mut mut yaw = settings.yaw.get_or_insert(yaw);
                        //         let &mut mut pitch = settings.pitch.get_or_insert(pitch);
                        //         let &mut mut radius = settings.radius.get_or_insert(radius);

                        //         // Set initial values
                        //         settings.yaw = Some(yaw);
                        //         settings.pitch = Some(pitch);
                        //         settings.radius = Some(radius);
                        //         settings.target_yaw = yaw;
                        //         settings.target_pitch = pitch;
                        //         settings.target_radius = radius;
                        //         settings.target_focus = target_trans.translation;

                        //         update_orbit_transform(
                        //             yaw,
                        //             pitch,
                        //             radius,
                        //             target_trans.translation,
                        //             &mut cam_trans,
                        //             &mut projection,
                        //         );

                        //         //settings.initialized = true;
                        //     }

                        //     // 1 - Get Input

                        //     let mut orbit = Vec2::ZERO;
                        //     let mut pan = Vec2::ZERO;
                        //     let mut scroll_line = 0.0;
                        //     let mut scroll_pixel = 0.0;
                        //     let mut orbit_button_changed = false;

                        //     let mut should_get_input = true;

                        //     // The reason we only skip getting input if the camera is inactive/disabled is because
                        //     // it might still be moving (lerping towards target values) when the user is not
                        //     // actively controlling it.
                        //     if should_get_input {
                        //         let zoom_direction = match settings.reversed_zoom {
                        //             true => -1.0,
                        //             false => 1.0,
                        //         };

                        //         orbit = mouse_key_tracker.orbit * settings.orbit_sensitivity;
                        //         pan = mouse_key_tracker.pan * settings.pan_sensitivity;
                        //         scroll_line =
                        //             mouse_key_tracker.scroll_line * zoom_direction * settings.zoom_sensitivity;
                        //         scroll_pixel =
                        //             mouse_key_tracker.scroll_pixel * zoom_direction * settings.zoom_sensitivity;
                        //         orbit_button_changed = mouse_key_tracker.orbit_button_changed;
                        //     }

                        //     // 2 - Process input into target yaw/pitch, or focus, radius

                        //     if orbit_button_changed {
                        //         // Only check for upside down when orbiting started or ended this frame,
                        //         // so we don't reverse the yaw direction while the user is still dragging
                        //         let wrapped_pitch = (settings.target_pitch % TAU).abs();
                        //         settings.is_upside_down = wrapped_pitch > TAU / 4.0 && wrapped_pitch < 3.0 * TAU / 4.0;
                        //     }

                        //     let mut has_moved = false;
                        //     if orbit.length_squared() > 0.0 {
                        //         // Use window size for rotation otherwise the sensitivity
                        //         // is far too high for small viewports
                        //         if let Some(win_size) = active_cam.window_size {
                        //             let delta_x = {
                        //                 let delta = orbit.x / win_size.x * PI * 2.0;
                        //                 if settings.is_upside_down {
                        //                     -delta
                        //                 } else {
                        //                     delta
                        //                 }
                        //             };
                        //             let delta_y = orbit.y / win_size.y * PI;
                        //             settings.target_yaw -= delta_x;
                        //             settings.target_pitch += delta_y;

                        //             has_moved = true;
                        //         }
                        //     }
                        //     if pan.length_squared() > 0.0 {
                        //         // Make panning distance independent of resolution and FOV,
                        //         if let Some(vp_size) = active_cam.viewport_size {
                        //             let mut multiplier = 1.0;
                        //             match *projection {
                        //                 Projection::Perspective(ref p) => {
                        //                     pan *= Vec2::new(p.fov * p.aspect_ratio, p.fov) / vp_size;
                        //                     // Make panning proportional to distance away from focus point
                        //                     if let Some(radius) = settings.radius {
                        //                         multiplier = radius;
                        //                     }
                        //                 }
                        //                 Projection::Orthographic(ref p) => {
                        //                     pan *= Vec2::new(p.area.width(), p.area.height()) / vp_size;
                        //                 }
                        //             }
                        //             // Translate by local axes
                        //             let right = transform.rotation * Vec3::X * -pan.x;
                        //             let up = transform.rotation * Vec3::Y * pan.y;
                        //             let translation = (right + up) * multiplier;
                        //             settings.target_focus += translation;
                        //             has_moved = true;
                        //         }
                        //     }
                        //     if (scroll_line + scroll_pixel).abs() > 0.0 {
                        //         // Calculate the impact of scrolling on the reference value
                        //         let line_delta = -scroll_line * (settings.target_radius) * 0.2;
                        //         let pixel_delta = -scroll_pixel * (settings.target_radius) * 0.2;

                        //         // Update the target value
                        //         settings.target_radius += line_delta + pixel_delta;

                        //         // If it is pixel-based scrolling, add it directly to the current value
                        //         settings.radius = settings
                        //             .radius
                        //             .map(|value| apply_zoom_limits(value + pixel_delta));

                        //         has_moved = true;
                        //     }

                        //     // 3 - Apply constraints

                        //     settings.target_yaw = apply_yaw_limits(settings.target_yaw);
                        //     settings.target_pitch = apply_pitch_limits(settings.target_pitch);
                        //     settings.target_radius = apply_zoom_limits(settings.target_radius);

                        //     if !settings.allow_upside_down {
                        //         settings.target_pitch = settings.target_pitch.clamp(-PI / 2.0, PI / 2.0);
                        //     }

                        //     // 4 - Update the camera's transform based on current values

                        //     if let (Some(yaw), Some(pitch), Some(radius)) =
                        //         (settings.yaw, settings.pitch, settings.radius)
                        //     {
                        //         if has_moved
                        //             // For smoothed values, we must check whether current value is different from target
                        //             // value. If we only checked whether the values were non-zero this frame, then
                        //             // the camera would instantly stop moving as soon as you stopped moving it, instead
                        //             // of smoothly stopping
                        //             || settings.target_yaw != yaw
                        //             || settings.target_pitch != pitch
                        //             || settings.target_radius != radius
                        //             || settings.target_focus != settings.focus
                        //             || settings.force_update
                        //         {
                        //             // Interpolate towards the target values
                        //             let new_yaw = util::lerp_and_snap_f32(
                        //                 yaw,
                        //                 settings.target_yaw,
                        //                 settings.orbit_smoothness,
                        //                 time.delta_seconds(),
                        //             );
                        //             let new_pitch = util::lerp_and_snap_f32(
                        //                 pitch,
                        //                 settings.target_pitch,
                        //                 settings.orbit_smoothness,
                        //                 time.delta_seconds(),
                        //             );
                        //             let new_radius = util::lerp_and_snap_f32(
                        //                 radius,
                        //                 settings.target_radius,
                        //                 settings.zoom_smoothness,
                        //                 time.delta_seconds(),
                        //             );
                        //             let new_focus = util::lerp_and_snap_vec3(
                        //                 settings.focus,
                        //                 settings.target_focus,
                        //                 settings.pan_smoothness,
                        //                 time.delta_seconds(),
                        //             );

                        //             util::update_orbit_transform(
                        //                 new_yaw,
                        //                 new_pitch,
                        //                 new_radius,
                        //                 new_focus,
                        //                 &mut transform,
                        //                 &mut projection,
                        //             );

                        //             // Update the current values
                        //             settings.yaw = Some(new_yaw);
                        //             settings.pitch = Some(new_pitch);
                        //             settings.radius = Some(new_radius);
                        //             settings.focus = new_focus;
                        //             settings.force_update = false;
                        //         }
                        //     }

                        // }
                    }
                }
                CameraMode::Observer(observer_kind) => {
                    match observer_kind {
                        ObserverCam::Orbit => {
                            let mut point_count = 0.0;
                            let mut cord_total = Vec3::new(0.0, 0.0, 0.0);

                            if to_watch_querry.iter().len() > 0 {
                                for e in to_watch_querry.iter() {
                                    if let Ok(trans) = transforms.get(e) {
                                        point_count += 1.0;
                                        cord_total += trans.translation;
                                    }
                                }
                                //cam_trans.translation = Vec3::new(settings.camera_distance_offset.x, cam_trans.translation.y, settings.camera_distance_offset.y);

                                cam_trans.look_at(
                                    cord_total / Vec3::new(point_count, point_count, point_count),
                                    Vec3::new(0.0, 0.0, 0.0),
                                );
                            }
                        }
                    }
                }
            };
        }
    }
    //}
}

/// Handles keyboard input and movement
pub fn camera_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<CamKeybinds>,
    //restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<(&CameraMode, &mut Transform, Option<&CameraRestrained>), With<Camera>>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to move camera, Reason: {:#}", err);
            return;
        }
    };

    for (camera, mut transform, target_config) in query.iter_mut() {
        let restraints_toggled = match target_config {
            Some(toggle) => toggle.0,
            None => false,
        };
        if restraints_toggled == false || camera == &CameraMode::Observer(ObserverCam::Orbit) {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == key_bindings.move_forward {
                            velocity += forward;
                        }
                        if key == key_bindings.move_backward {
                            velocity -= forward;
                        }
                        if key == key_bindings.move_left {
                            velocity -= right;
                        }
                        if key == key_bindings.move_right {
                            velocity += right;
                        }
                        if key == key_bindings.move_ascend {
                            velocity += Vec3::Y;
                        }
                        if key == key_bindings.move_descend {
                            velocity -= Vec3::Y;
                        }
                    }
                }
            }
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * settings.speed
        }
    }
}

/// Handles looking around if cursor is locked
pub fn camera_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    //restraints_toggled: Res<RestraintsToggled>,
    mut query: Query<(&mut Transform, &CameraMode, Option<&CameraRestrained>), With<Camera>>,
) {
    let window = match primary_window.get_single() {
        Ok(win) => win,
        Err(err) => {
            warn!("Unable to rotate camera, Reason: {:#}", err);
            return;
        }
    };

    //if restraints_toggled.0 == false {
    for (mut transform, camera_controls, restraints) in query.iter_mut() {
        let restraints_toggled = match restraints {
            Some(toggle) => toggle.0,
            None => false,
        };
        let first_person_look = match camera_controls {
            CameraMode::POV(cam) => match cam.pov {
                POV::FirstPerson => true,
                POV::ThirdPerson => false,
                //POV::Orbit => false,
            },
            CameraMode::Observer(..) => false,
        };

        if restraints_toggled == false || first_person_look {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

pub fn cursor_grab(
    mut grabbed: ResMut<CursorGrabbed>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<CamKeybinds>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window, &mut grabbed);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

/// Grabs/ungrabs mouse cursor
pub fn toggle_grab_cursor(window: &mut Window, grabbed: &mut ResMut<CursorGrabbed>) {
    match grabbed.0 {
        false => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
            window.cursor.hit_test = true;

            grabbed.0 = true;
        }
        true => {
            //window.set_cursor_position(Some(Vec2::new(0.0, 0.0)));

            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
            window.cursor.hit_test = false;

            grabbed.0 = false;
        }
    }
}

pub fn set_intial_grab_state(
    mut grabbed: ResMut<CursorGrabbed>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        info!("setting initial grab state");

        toggle_grab_cursor(&mut window, &mut grabbed);
    }
}
