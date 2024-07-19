use super::types::{AssetKey, HandleMap};
use bevy::prelude::*;

// TODO:
// Actually load our assets...
#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ModelKey {
    MyCube,
}

impl AssetKey for ModelKey {
    type Asset = Mesh;
}

impl FromWorld for HandleMap<ModelKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(ModelKey::MyCube, asset_server.load("images/ducky.png"))].into()
    }
}
