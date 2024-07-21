//! Handle cars movements along their paths.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::scene::objects::Path;

#[derive(Resource)]
struct GlobalVelocity(f32);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource::<GlobalVelocity>(GlobalVelocity(0.5))
        .add_systems(Update, move_car);
}

fn move_car(
    time: Res<Time>,
    global_velocity: Res<GlobalVelocity>,
    mut query: Query<(&mut Transform, &mut Path)>,
) {
    let velocity = global_velocity.0; // Access the global velocity

    for (mut transform, mut path) in query.iter_mut() {
        if path.step < path.points.len() - 1 {
            let target = path.points[path.step + 1];
            let direction = (target - transform.translation).normalize();
            let distance_to_move = velocity * time.delta_seconds();
            let distance_to_target = transform.translation.distance(target);

            if distance_to_move >= distance_to_target {
                // Snap to the target and move to the next point
                transform.translation = target;
                path.step += 1;
            } else {
                // Move towards the target
                transform.translation += direction * distance_to_move;
            }

            // Adjust rotation to face the target
            let angle = Vec3::Z.angle_between(-direction);
            let axis = Vec3::Z.cross(-direction).normalize();
            transform.rotation = Quat::from_axis_angle(axis, angle);
        }
    }
}
