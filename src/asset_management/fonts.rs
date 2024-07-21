use bevy::prelude::*;

use super::types::{AssetKey, HandleMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum FontKey {
    FontAwesome,
}

impl AssetKey for FontKey {
    type Asset = Font;
}

impl FromWorld for HandleMap<FontKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            FontKey::FontAwesome,
            asset_server.load("fonts/fa_6_free-solid-900.otf"),
        )]
        .into()
    }
}
