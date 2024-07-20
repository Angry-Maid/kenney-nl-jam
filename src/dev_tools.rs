//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, log::LogPlugin, math::VectorSpace, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

use crate::screen::Screen;

#[derive(Resource, Deref, DerefMut)]
pub struct IsInDevMode(pub bool);

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.insert_resource(IsInDevMode(false))
        .add_plugins((NoCameraPlayerPlugin,))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                switch_to_dev_mode,
                (change_cams).run_if(resource_changed::<IsInDevMode>),
            ),
        )
        .add_systems(Update, log_transitions::<Screen>);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                is_active: false,
                ..Default::default()
            },
            transform: Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
    ));
}

fn switch_to_dev_mode(mut r_dmode: ResMut<IsInDevMode>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyF) {
        **r_dmode = !(**r_dmode);
    }
}

fn change_cams(
    mut q_cams: Query<(Entity, &mut Camera)>,
    r_in_dev: Res<IsInDevMode>,
    q_f: Query<&FlyCam>,
) {
    let IsInDevMode(val) = *r_in_dev;

    q_cams.iter_mut().for_each(|(e, mut c)| {
        if q_f.contains(e) {
            c.is_active = val;
        } else {
            c.is_active = !val;
        }
    })
}
