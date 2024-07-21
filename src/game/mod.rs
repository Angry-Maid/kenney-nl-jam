//! Game mechanics and content.

use bevy::prelude::*;

use crate::asset_management;

pub mod audio;
pub mod sample;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((asset_management::plugin, sample::plugin, audio::plugin));
}
