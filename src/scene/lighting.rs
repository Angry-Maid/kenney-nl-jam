use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(AmbientLight {
        brightness: 500.,
        ..Default::default()
    });
}
