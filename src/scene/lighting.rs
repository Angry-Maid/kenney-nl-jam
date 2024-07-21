use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::screen::Screen;

pub fn plugin(app: &mut App) {
    app.insert_resource(AmbientLight {
        brightness: 500.,
        ..Default::default()
    })
    .add_systems(
        OnEnter(Screen::Playing),
        sun.run_if(in_state(Screen::Playing)),
    )
    .add_systems(OnExit(Screen::Playing), despawn_sun);
}

#[derive(Component)]
struct Sun;

fn despawn_sun(mut commands: Commands, sun: Query<Entity, With<Sun>>) {
    if let Result::Ok(sun) = sun.get_single() {
        commands.entity(sun).despawn();
    }
}

fn sun(mut commands: Commands) {
    // directional 'sun' light
    commands.spawn((
        Sun,
        DirectionalLightBundle {
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
        },
    ));
}
