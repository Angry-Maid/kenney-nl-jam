//! Handle cars movements along their paths.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::{scene::objects::Path, screen::Screen};

use super::following::Robber;

#[derive(Resource)]
struct GlobalVelocity(f32);

#[derive(Event)]
pub struct PathFinished(pub Entity);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PathFinished>()
        .insert_resource::<GlobalVelocity>(GlobalVelocity(5.5))
        .add_systems(Update, move_along_path);
}

// TODO:
// Separate Win logic out
fn move_along_path(
    mut query: Query<(Entity, &mut Transform, &mut Path)>,
    mut e_p: EventWriter<PathFinished>,

    mut scrn: ResMut<NextState<Screen>>,
    q_robber: Query<&Robber>,
    time: Res<Time>,
    global_velocity: Res<GlobalVelocity>,
) {
    let velocity = global_velocity.0; // Access the global velocity

    for (e, mut transform, mut path) in query.iter_mut() {
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
        } else {
            if q_robber.contains(e) {
                scrn.set(Screen::Win);
                // e_p.send(PathFinished(e));
            }
        }
    }
}
