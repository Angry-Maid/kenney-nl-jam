//! Game mechanics and content.

use bevy::prelude::*;

use crate::asset_management;

pub mod audio;
pub mod movement;
pub mod following;
pub mod sample;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((sample::plugin, audio::plugin, movement::plugin));
}
