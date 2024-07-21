use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub fn plugin(app: &mut App) {
    app.insert_resource(AmbientLight {
        brightness: 500.,
        ..Default::default()
    })
    .add_systems(Startup, sun);
}

fn sun(mut commands: Commands) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-100., 100., -100.))
            .looking_at(Vec3::ZERO, Vec3::Y),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder { ..default() }.into(),
        ..default()
    });
}
