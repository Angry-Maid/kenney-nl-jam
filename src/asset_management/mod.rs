use bevy::app::App;

use self::types::HandleMap;

pub mod images;
pub mod misc;
pub mod types;

pub(super) fn plugin(app: &mut App) {
    use self::images::ImageKey;
    use self::misc::*;

    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
}
