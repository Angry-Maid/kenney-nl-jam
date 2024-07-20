use bevy::{gltf::GltfNode, prelude::*};

use crate::{
    asset_management::{models::SceneKey, types::HandleMap},
    util::math::BLENDER_QUAT,
};

#[derive(Component)]
pub struct CameraPoint;

pub fn plugin(app: &mut App) {
    app
        // .add_systems(Startup, (spawn_gltf_objects, spawn_cube))
        .add_systems(
            Update,
            spawn_gltf_objects.run_if(resource_changed::<HandleMap<SceneKey>>),
        );
}

fn spawn_gltf_objects(
    mut commands: Commands,
    hm_scenes: Res<HandleMap<SceneKey>>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_nodes: Res<Assets<GltfNode>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    // TODO:
    // Currently hard coded to a specific SceneKey.
    // We want to create a new scene for each new scene that's loaded,
    // without creating duplicates.
    if let Some(gltf) = assets_gltf.get(hm_scenes.get(&SceneKey::Ambulance).unwrap()) {
        // spawn the first scene in the file
        spawn_scene_with_cameras(&mut commands, gltf, &assets_gltf_nodes)
    }
}

// TODO:
// Currently sets camera's parent to the scene.
// If there are individual moving objects within the scene to which a camera is attached to,
// it won't work...
// I.e, the camera only moves relative to the entire scene.
fn spawn_scene_with_cameras(c: &mut Commands, g: &Gltf, assets_gltf_nodes: &Res<Assets<GltfNode>>) {
    let scene_ent = c
        .spawn(SceneBundle {
            scene: g.scenes[0].clone(),
            ..Default::default()
        })
        .id();

    g.nodes
        .iter()
        .map(|h| {
            assets_gltf_nodes
                .get(h)
                .expect("GltfNode should have loaded")
        })
        .for_each(|n| {
            if n.name.contains("Camera") {
                c.spawn((
                    Name::new(n.name.clone()),
                    n.transform.with_rotation(
                        *BLENDER_QUAT
                            * Quat::from_rotation_y(std::f32::consts::PI)
                            * n.transform.rotation,
                    ),
                    CameraPoint,
                ))
                .set_parent(scene_ent);
            }
        });
}
