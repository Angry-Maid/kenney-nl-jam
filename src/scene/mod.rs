use bevy::app::App;

pub mod camera;
pub mod lighting;
pub mod objects;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        self::camera::plugin,
        self::objects::plugin,
        self::lighting::plugin,
    ));
}
