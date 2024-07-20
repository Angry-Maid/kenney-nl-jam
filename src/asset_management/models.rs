use super::types::{AssetKey, HandleMap};
use bevy::prelude::*;

// TODO:
// Actually load our assets...
#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SceneKey {
    Taxi,
    ATM,
}

impl AssetKey for SceneKey {
    type Asset = Gltf;
}

impl FromWorld for HandleMap<SceneKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (SceneKey::Taxi, asset_server.load("scenes/taxi.gltf")),
            // (SceneKey::ATM, asset_server.load("dump/atm.gltf")),
        ]
        .into()
    }
}
