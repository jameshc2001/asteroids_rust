use std::ops::AddAssign;
use bevy::prelude::*;

use crate::components::*;
use crate::systems::asteroid::random_rotation;

pub fn bullet_asteroid_collision(
    mut commands: Commands,
    mut asteroid_query: Query<(&Transform, &mut RotationVelocity, &mut Asteroid, &Children), Without<AsteroidLine>>,
    mut asteroid_line_query: Query<(&Transform, &mut Velocity, &mut RotationVelocity, &mut Lifetime), With<AsteroidLine>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (asteroid_transform, mut asteroid_rotation_velocity, mut asteroid, asteroid_lines) in asteroid_query.iter_mut() {
            if asteroid_transform.translation.distance(bullet_transform.translation) < asteroid.max_radius {
                commands.entity(bullet_entity).despawn();

                asteroid.alive = false;
                asteroid_rotation_velocity.0 = 0.0;
                for line in asteroid_lines {
                    if let Ok((line_transform, mut line_velocity, mut line_rotation_velocity, mut line_lifetime)) = asteroid_line_query.get_mut(*line) {
                        line_velocity.add_assign(line_transform.translation.truncate());
                        line_rotation_velocity.add_assign(random_rotation());
                        line_lifetime.enabled = true
                    }
                }
            }
        }
    }
}
