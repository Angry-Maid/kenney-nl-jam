//! Game mechanics and content.

use bevy::prelude::*;

pub mod audio;
pub mod sample;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((sample::plugin, audio::plugin));
}
