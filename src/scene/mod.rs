use bevy::app::App;

pub mod camera;
pub mod lighting;
pub mod objects;
pub mod sprite3d;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        self::camera::plugin,
        self::sprite3d::plugin,
        self::objects::plugin,
        self::lighting::plugin,
    ));
}
