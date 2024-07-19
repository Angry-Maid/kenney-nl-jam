use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_cube);
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::srgb(1.0, 0.2, 0.3)),
        transform: Transform::default().with_scale(3. * Vec3::ONE),
        ..Default::default()
    });
}
