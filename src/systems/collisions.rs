use bevy::prelude::*;

use crate::components::*;

pub fn bullet_asteroid_collision(
    mut commands: Commands,
    mut asteroid_query: Query<(Entity, &Transform, &mut Asteroid)>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (asteroid_entity, asteroid_transform, mut asteroid) in asteroid_query.iter_mut() {
            if asteroid_transform.translation.distance(bullet_transform.translation) < asteroid.max_radius {
                commands.entity(bullet_entity).despawn();

                asteroid.alive = false;

                commands.entity(asteroid_entity).despawn();
            }
        }
    }
}
