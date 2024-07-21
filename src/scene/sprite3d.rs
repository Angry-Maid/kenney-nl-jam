use std::hash::Hash;
use std::{f32::consts::PI, sync::LazyLock};

use bevy::{prelude::*, utils::HashMap};
use bevy_sprite3d::{Sprite3d, Sprite3dComponent, Sprite3dParams, Sprite3dPlugin};

#[derive(Component)]
pub enum BufferedSprite3d {
    Image(Sprite3d),
    Atlas(Sprite3d, TextureAtlas),
}

pub fn plugin(app: &mut App) {
    app.add_plugins(Sprite3dPlugin)
        .add_systems(Update, (load_buffered_sprites));
}

pub fn clone_sprite3d(s: &Sprite3d) -> Sprite3d {
    Sprite3d {
        transform: s.transform,
        image: s.image.clone(),
        pixels_per_metre: s.pixels_per_metre,
        pivot: s.pivot,
        alpha_mode: s.alpha_mode,
        unlit: s.unlit,
        double_sided: s.double_sided,
        emissive: s.emissive,
    }
}

pub fn get_sprite3d_from_buffer(b: &BufferedSprite3d) -> &Sprite3d {
    match b {
        BufferedSprite3d::Image(s) => s,
        BufferedSprite3d::Atlas(s, _) => s,
    }
}

pub fn load_buffered_sprites(
    mut c: Commands,
    mut sp: Sprite3dParams,
    q: Query<(Entity, &BufferedSprite3d)>,
    s: Res<AssetServer>,
) {
    q.iter().for_each(|(e, b)| {
        let sp3d = get_sprite3d_from_buffer(b);

        if s.get_load_state(sp3d.image.id()) != Some(bevy::asset::LoadState::Loaded) {
            return;
        }

        let mut e_c = c.entity(e);
        e_c.remove::<BufferedSprite3d>();

        match b {
            BufferedSprite3d::Image(s) => {
                e_c.insert(clone_sprite3d(s).bundle(&mut sp));
            }
            BufferedSprite3d::Atlas(s, a) => {
                e_c.insert(clone_sprite3d(s).bundle_with_atlas(&mut sp, a.clone()));
            }
        }
    })
}
