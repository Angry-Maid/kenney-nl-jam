use bevy::{gltf::GltfNode, prelude::*};
use bevy_sprite3d::Sprite3d;

use crate::{
    asset_management::{images::ImageKey, models::SceneKey, types::HandleMap},
    util::math::BLENDER_QUAT,
};

use super::{camera::Billboarded, sprite3d::BufferedSprite3d};

#[derive(Component)]
pub struct CameraPoint;

#[derive(Component)]
pub struct CameraIcon;

// This exists because parenting nullifies Transform rotation...
#[derive(Component)]
pub struct TranslationRelativeTo(pub Entity, pub Vec3);

#[derive(Resource, Default)]
pub struct CurrentScene(pub Option<SceneKey>);

pub fn plugin(app: &mut App) {
    app.init_resource::<CurrentScene>()
        // .add_systems(Startup, (spawn_gltf_objects, spawn_cube))
        .add_systems(Update, (put_relative, spawn_gltf_objects));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    mut r_current_scn: ResMut<CurrentScene>,
    time: Res<Time>,
    hm_scenes: Res<HandleMap<SceneKey>>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_nodes: Res<Assets<GltfNode>>,
    img_assets: Res<HandleMap<ImageKey>>,
) {
    // TODO:
    // Hacky way of avoiding dealing with asset loading events
    if time.elapsed_seconds() < 2. {
        return;
    };

    if let CurrentScene(Some(_)) = *r_current_scn {
        return;
    };

    // if the GLTF has loaded, we can navigate its contents
    // TODO:
    // Currently hard coded to a specific SceneKey.
    // We want to create a new scene for each new scene that's loaded,
    // without creating duplicates.
    if let Some(gltf) = assets_gltf.get(hm_scenes.get(&SceneKey::City).unwrap()) {
        println!("GOnna spawn big scity");
        *r_current_scn = CurrentScene(Some(SceneKey::City));

        // spawn the first scene in the file
        spawn_scene_with_cameras(&mut commands, gltf, &assets_gltf_nodes, &img_assets)
    }
}

// TODO:
// Currently sets camera's parent to the scene.
// If there are individual moving objects within the scene to which a camera is attached to,
// it won't work...
// I.e, the camera only moves relative to the entire scene.
fn spawn_scene_with_cameras(
    c: &mut Commands,
    g: &Gltf,
    assets_gltf_nodes: &Res<Assets<GltfNode>>,
    img_assets: &Res<HandleMap<ImageKey>>,
) {
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
                let cam_ent = c
                    .spawn((
                        Name::new(n.name.clone()),
                        n.transform,
                        //     .with_rotation(
                        //     *BLENDER_QUAT
                        //         * Quat::from_rotation_y(std::f32::consts::PI)
                        //         * n.transform.rotation,
                        // ),
                        CameraPoint,
                    ))
                    .set_parent(scene_ent)
                    .id();

                c.spawn((
                    BufferedSprite3d::Image(Sprite3d {
                        image: img_assets
                            .get(&ImageKey::CameraIcon)
                            .expect("Camera image should exist")
                            .clone(),
                        alpha_mode: AlphaMode::Blend,
                        double_sided: false,
                        pixels_per_metre: 512.,
                        ..Default::default()
                    }),
                    TranslationRelativeTo(cam_ent, 0.25 * Vec3::Y),
                    CameraIcon,
                    Billboarded,
                ));
            }
        });
}

fn put_relative(mut q: Query<(&mut Transform)>, q2: Query<(Entity, &TranslationRelativeTo)>) {
    q2.iter().for_each(|(e, &TranslationRelativeTo(r_e, pos))| {
        // TODO:
        // Avoid immutable borrow in a better way. Currently clones...
        let Ok(trans2) = q.get(r_e) else { return };
        let trans2 = trans2.clone();

        let Ok(mut trans1) = q.get_mut(e) else {
            return;
        };

        trans1.translation = trans2.translation + pos;
    })
}
