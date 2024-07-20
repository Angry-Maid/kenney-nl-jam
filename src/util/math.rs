use std::sync::LazyLock;

use bevy::{math::Quat, transform::components::Transform};

// NOTE:
// I might be wrong.
// BLENDER_QUAT * blender_transform.rotation = bevy_rotation
pub static BLENDER_QUAT: LazyLock<Quat> =
    LazyLock::new(|| Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
