// use crate::config::{DirectionalKeys, KeyToDirection, KeyToZoomDirection, ZoomKeys};

use crate::config::camera::{key_to_dir, key_to_zoom};

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

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

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, cam_control);
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
                .with_translation(Vec3::new(-10., 12., -16.)),
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
    mut q_cam: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Result::Ok((mut trans, mut proj)) = q_cam.get_single_mut() {
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
    }
}
