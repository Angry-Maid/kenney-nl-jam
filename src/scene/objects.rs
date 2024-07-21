use std::collections::HashMap;

use bevy::{gltf::GltfNode, prelude::*};
use bevy_sprite3d::Sprite3d;

use rand::seq::SliceRandom;

use crate::{
    asset_management::{images::ImageKey, models::SceneKey, types::HandleMap},
    game::following::Robber,
    screen::Screen,
};

#[cfg(feature = "dev")]
use crate::dev_tools::DevState;

use super::{camera::Billboarded, sprite3d::BufferedSprite3d};

#[derive(Component)]
pub struct CameraPoint;

#[derive(Component)]
pub struct CameraIcon;

#[derive(Component)]
pub struct Scene;

// This exists because parenting nullifies Transform rotation...
#[derive(Component)]
pub struct TranslationRelativeTo(pub Entity, pub Vec3);

#[derive(Resource, Default)]
pub struct CurrentScene(pub Option<SceneKey>);
#[derive(Component, Default)]
pub struct Path {
    pub points: Vec<Vec3>,
    pub step: usize,
}

const ROBBER_SPAWN_NODE_PREFIX: &str = "Robber";
const POINT_SUFFIX: &str = "Point";

pub fn plugin(app: &mut App) {
    app.init_resource::<CurrentScene>()
        .add_systems(OnEnter(Screen::Playing), (spawn_gltf_objects,))
        .add_systems(OnExit(Screen::Playing), (despawn_gltf_objects,))
        .add_systems(Update, (put_relative,).run_if(in_state(Screen::Playing)));

    #[cfg(feature = "dev")]
    app.add_systems(Update, spawn_path_nodes.run_if(in_state(DevState::On)));
}

fn despawn_gltf_objects(
    mut commands: Commands,
    mut r_current_scn: ResMut<CurrentScene>,
    scene: Query<Entity, With<Scene>>,
    camera_points: Query<Entity, With<CameraPoint>>,
    camera_icons: Query<Entity, With<CameraIcon>>,
) {
    if let Result::Ok(scene) = scene.get_single() {
        commands.entity(scene).despawn();
    }
    for cam_point in camera_points.iter() {
        commands.entity(cam_point).despawn();
    }
    for cam_icon in camera_icons.iter() {
        commands.entity(cam_icon).despawn();
    }

    *r_current_scn = CurrentScene(None);
}

pub fn spawn_gltf_objects(
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
        *r_current_scn = CurrentScene(Some(SceneKey::City));

        // spawn the first scene in the file
        spawn_scene_with_cameras(&mut commands, gltf, &assets_gltf_nodes, &img_assets);
        if let Some(robber_gltf) = assets_gltf.get(hm_scenes.get(&SceneKey::Taxi).unwrap()) {
            // spawn the first scene in the file
            spawn_robber(&mut commands, robber_gltf, gltf, &assets_gltf_nodes);
        }
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
        .spawn((
            Scene,
            SceneBundle {
                scene: g.scenes[0].clone(),
                ..Default::default()
            },
        ))
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
                let mut rot_t = n.transform;
                rot_t.rotate_local_y(std::f32::consts::PI);

                let cam_ent = c
                    .spawn((Name::new(n.name.clone()), rot_t, CameraPoint))
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

fn put_relative(mut q: Query<&mut Transform>, q2: Query<(Entity, &TranslationRelativeTo)>) {
    q2.iter().for_each(|(e, &TranslationRelativeTo(r_e, pos))| {
        let Ok(trans2) = q.get(r_e) else { return };
        let trans2 = *trans2;

        let Ok(mut trans1) = q.get_mut(e) else {
            return;
        };

        trans1.translation = trans2.translation + pos;
    })
}

// TODO:
// Currently sets camera's parent to the scene.
// If there are individual moving objects within the scene to which a camera is attached to,
// it won't work...
// I.e, the camera only moves relative to the entire scene.
fn spawn_robber(
    c: &mut Commands,
    g: &Gltf,
    city_g: &Gltf,
    assets_gltf_nodes: &Res<Assets<GltfNode>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    // TODO:
    // Collect all possible paths that should be named as follow: Robber1Point1, RObber1Point2, Robber2Point1, etc and then get one at random for robber
    let path: Vec<Vec3> = get_random_robber_path(city_g, assets_gltf_nodes);
    info!("path: {:?}", path.clone());

    let starting_position = path.first().cloned().unwrap_or_default();
    info!("Robber position: {:?}", starting_position.clone());
    c.spawn((
        SceneBundle {
            scene: g.scenes[0].clone(),
            transform: Transform::from_translation(starting_position),
            ..Default::default()
        },
        Path {
            points: path,
            step: 0,
        },
        Robber,
    ));
}

fn get_random_robber_path(city_g: &Gltf, assets_gltf_nodes: &Res<Assets<GltfNode>>) -> Vec<Vec3> {
    get_robber_paths(city_g, assets_gltf_nodes)
        .choose(&mut rand::thread_rng())
        .cloned()
        .expect("Robber doesn't have set path")
}

fn get_robber_paths(city_g: &Gltf, assets_gltf_nodes: &Res<Assets<GltfNode>>) -> Vec<Vec<Vec3>> {
    let paths: Vec<Vec<Vec3>> = city_g
        .nodes
        .iter()
        .filter_map(|handle| {
            assets_gltf_nodes.get(handle).and_then(|node| {
                if node.name.contains(ROBBER_SPAWN_NODE_PREFIX) {
                    extract_group_and_suffix(&node.name).map(|(group_number, suffix_number)| {
                        (group_number, suffix_number, node.transform.translation)
                    })
                } else {
                    None
                }
            })
        })
        .fold(
            HashMap::<usize, Vec<(usize, Vec3)>>::new(),
            |mut groups, (group_number, suffix_number, translation)| {
                groups
                    .entry(group_number)
                    .or_default()
                    .push((suffix_number, translation));
                groups
            },
        )
        .into_values()
        .map(|mut node_vec| {
            node_vec.sort_by_key(|(suffix, _)| *suffix);
            node_vec
                .into_iter()
                .map(|(_, translation)| translation)
                .collect()
        })
        .collect();

    paths
}

// Extract group and suffix numbers from node names
fn extract_group_and_suffix(name: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = name.split(POINT_SUFFIX).collect();
    if parts.len() == 2 {
        let group_number = parts[0]
            .strip_prefix(ROBBER_SPAWN_NODE_PREFIX)
            .and_then(|s| s.parse().ok())?;
        let suffix_number = parts[1].parse().ok()?;
        Some((group_number, suffix_number))
    } else {
        None
    }
}

fn spawn_path_nodes(
    mut c: Commands,
    paths_query: Query<&Path>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::srgb(0.8, 0.0, 0.0));

    let sphere = meshes.add(Mesh::from(Sphere { radius: 0.1 }));

    paths_query.iter().for_each(|path| {
        path.points.iter().for_each(|point| {
            c.spawn(PbrBundle {
                mesh: sphere.clone(),
                material: material.clone(),
                transform: Transform::from_translation(*point),
                ..default()
            });
        })
    });
}
