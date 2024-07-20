use bevy::{input::keyboard::KeyCode, math::Vec3};

pub fn key_to_dir(k: &KeyCode) -> Vec3 {
    match k {
        // NOTE:
        // Directions are intentionally backwards.
        // See `scene::camera::config::CAMERA_QUATS`
        KeyCode::KeyW => -Vec3::Z,
        KeyCode::KeyS => Vec3::Z,
        KeyCode::KeyA => -Vec3::X,
        KeyCode::KeyD => Vec3::X,
        _ => Vec3::ZERO,
    }
}

pub fn key_to_zoom(k: &KeyCode) -> f32 {
    match k {
        KeyCode::KeyQ => -1.,
        KeyCode::KeyE => 1.,
        _ => 0.,
    }
}
