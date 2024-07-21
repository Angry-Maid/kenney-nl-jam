use std::time::Duration;

use bevy::{ecs::query::QuerySingleError, prelude::*};

#[derive(Component)]
pub struct Robber;

#[derive(Component)]
struct LostTime {
    timer: Timer,
}

fn spawn_timer(commands: &mut Commands) {
    commands.spawn(
        (LostTime {
            timer: Timer::new(Duration::from_secs(30), TimerMode::Once),
        }),
    );
}

fn in_view(
    mut commands: Commands,
    mut timer: Query<(Entity, &mut LostTime)>,
    views: Query<&ViewVisibility, With<Robber>>,
    time: Res<Time>,
) {
    for visibility in views.iter() {
        if visibility.get() {
            if let Result::Ok((e, _)) = timer.get_single_mut() {
                commands.entity(e).despawn();
            }
        } else {
            match timer.get_single_mut() {
                Ok((_, mut t)) => {
                    t.timer.tick(time.delta());
                }
                Err(QuerySingleError::NoEntities(_)) => {
                    spawn_timer(&mut commands);
                }
                _ => {}
            }
        }
    }
}
