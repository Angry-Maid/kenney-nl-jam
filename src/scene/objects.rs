use bevy::prelude::*;

use crate::asset_management::{models::SceneKey, types::HandleMap};

pub fn plugin(app: &mut App) {
    app
        // .add_systems(Startup, (spawn_gltf_objects, spawn_cube))
        .add_systems(
            Update,
            spawn_gltf_objects.run_if(resource_changed::<Assets<Gltf>>),
        );
}

fn spawn_gltf_objects(
    mut commands: Commands,
    hm_scenes: Res<HandleMap<SceneKey>>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(hm_scenes.get(&SceneKey::Taxi).unwrap()) {
        // spawn the first scene in the file
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });
        // PERF: the `.clone()`s are just for asset handles, don't worry :)
    }
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::srgb(1.0, 0.2, 0.3)),
        transform: Transform::default().with_scale(3. * Vec3::ONE),
        ..Default::default()
    });
}
