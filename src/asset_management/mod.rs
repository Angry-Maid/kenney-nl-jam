use bevy::app::App;

use self::types::HandleMap;

pub mod audio;
pub mod fonts;
pub mod images;
pub mod models;
pub mod types;

pub(super) fn plugin(app: &mut App) {
    use self::audio::*;
    use self::fonts::FontKey;
    use self::images::ImageKey;
    use self::models::SceneKey;

    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<HandleMap<SceneKey>>();
    app.init_resource::<HandleMap<SceneKey>>();
    app.register_type::<HandleMap<FontKey>>();
    app.init_resource::<HandleMap<FontKey>>();
}
