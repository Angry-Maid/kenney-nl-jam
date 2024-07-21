// use crate::config::{DirectionalKeys, KeyToDirection, KeyToZoomDirection, ZoomKeys};

use crate::config::camera::{key_to_dir, key_to_zoom};

use bevy::math::bounding::{Aabb3d, BoundingVolume, RayCast3d};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;
use log::info;

use super::objects::{CameraIcon, CameraPoint, TranslationRelativeTo};

pub mod config {
    use std::{f32::consts::PI, sync::LazyLock};

    use bevy::prelude::{Quat, Vec3};

    pub const CAMERA_STEP: f32 = 0.05;
    pub const ZOOM_STEP: f32 = 0.05;
    pub const HEIGHT_BOUNDS: (f32, f32) = (3., 90.);

    pub static CAMERA_QUATS: LazyLock<(Quat, Quat, Quat)> = LazyLock::new(|| {
        let yaw = Quat::from_axis_angle(Vec3::Y, -0.75 * PI);
        let pitch = Quat::from_axis_angle(Vec3::X, -0.25 * PI);

        (yaw, pitch, yaw * pitch)
    });
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Billboarded;

pub fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.6, 0.7, 1.0)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (jump_to_camera, billboarded_stuff));
}

pub fn spawn_camera(mut commands: Commands) {
    let (_, _, isometric_quat) = *config::CAMERA_QUATS;

    commands.spawn((
        MainCamera,
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 5.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..Default::default()
            }
            .into(),
            // NOTE:
            // Distance might affect shadows and clipping / culling)
            // TODO:
            // Provide a Quaternion instead. Better for relocating the camera and maintaining a set
            // orientation.
            transform: Transform::from_rotation(isometric_quat)
                .with_translation(Vec3::new(-5., 12., -5.)),
            ..Default::default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}

fn cam_control(
    mut q_cam: Query<
        (&mut Transform, &mut Projection, &Camera),
        (With<MainCamera>, Without<CameraPoint>),
    >,
    q_c_points: Query<(&Transform), With<CameraPoint>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Result::Ok((mut trans, mut proj, cam)) = q_cam.get_single_mut() {
        if cam.is_active {
            let pan_delta = keyboard_input
                .get_pressed()
                .map(key_to_dir)
                .sum::<Vec3>()
                .normalize_or_zero();

            let zoom_delta = keyboard_input.get_pressed().map(key_to_zoom).sum::<f32>();

            // trans.y = max; trans.y >= min
            // trans.y = ok + excess
            // scaled.y = delta.y - excess = k * delta.y -> k = 1 - excess / delta.y;

            let (yaw_quat, _, _) = *config::CAMERA_QUATS;
            trans.translation += config::CAMERA_STEP * (yaw_quat * pan_delta);

            if let Projection::Orthographic(ref mut proj1) = &mut *proj {
                proj1.scale = f32::clamp(
                    proj1.scale + config::ZOOM_STEP * (zoom_delta as f32),
                    config::HEIGHT_BOUNDS.0,
                    config::HEIGHT_BOUNDS.1,
                );
            }

            if keyboard_input.just_pressed(KeyCode::KeyR) {
                // TODO:
                // This sucks, but convenient.
                q_c_points.iter().for_each(|t| {
                    *trans = *t;
                })
            }
        }
    }
}

fn billboarded_stuff(
    // mut g: Gizmos,
    mut q: Query<&mut Transform, With<Billboarded>>,
    q_cam: Query<(&Transform, &Camera), Without<Billboarded>>,
) {
    if let Some(active_camera_transform) =
        q_cam
            .iter()
            .find_map(|(t, c)| if c.is_active { Some(t) } else { None })
    {
        q.iter_mut().for_each(|mut t| {
            t.rotation = active_camera_transform.rotation;
        })
    }
}

fn jump_to_camera(
    mut g: Gizmos,
    mut q_transform: Query<&mut Transform>,
    q_camera: Query<(Entity, &Camera, &GlobalTransform), With<MainCamera>>,
    r_click: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_c_points: Query<(Entity, &TranslationRelativeTo), (With<CameraIcon>, Without<MainCamera>)>,
) {
    if !r_click.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok((cam_ent, camera, camera_gtransform)) = q_camera.get_single() else {
        return;
    };

    let Ok(window) = q_window.get_single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(Ray3d { origin, direction }) =
        camera.viewport_to_world(camera_gtransform, cursor_position)
    else {
        return;
    };

    if let Some((_, &next_cam_transform)) = q_c_points
        .iter()
        .filter_map(|(e, TranslationRelativeTo(r_e, _))| {
            let Ok(&Transform {
                translation,
                rotation,
                ..
            }) = q_transform.get(e)
            else {
                return None;
            };

            let relative_location = rotation.inverse().mul_vec3(origin - translation);
            let relative_direction = rotation.inverse().mul_vec3((direction).into());

            g.arrow(
                relative_location,
                relative_location + 3. * relative_direction,
                Color::LinearRgba(LinearRgba::GREEN),
            );

            (RayCast3d::new(
                relative_location,
                Dir3::new_unchecked(relative_direction),
                std::f32::MAX,
            ))
            .aabb_intersection_at(&Aabb3d {
                min: (-0.5 * Vec3::new(1., 1., 0.)).into(),
                max: (0.5 * Vec3::new(1., 1., 1.)).into(),
            })
            .map(|d| (d, q_transform.get(*r_e).unwrap()))
        })
        .fold(None, |acc, (distance, transform)| {
            if let Some((o_distance, _)) = acc {
                if distance < o_distance {
                    Some((distance, transform))
                } else {
                    acc
                }
            } else {
                Some((distance, transform))
            }
        })
    {
        *q_transform.get_mut(cam_ent).unwrap() = next_cam_transform;
    }
}
