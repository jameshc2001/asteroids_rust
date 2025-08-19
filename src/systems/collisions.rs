use bevy::prelude::*;

use crate::components::*;

pub fn bullet_asteroid_collision(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform, &Asteroid)>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (asteroid_entity, asteroid_transform, asteroid) in asteroid_query.iter() {
            if asteroid_transform.translation.distance(bullet_transform.translation) < asteroid.max_radius {
                commands.entity(asteroid_entity).despawn();
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
