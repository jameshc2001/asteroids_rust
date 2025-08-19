use bevy::prelude::*;

use crate::components::*;

pub fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.0 += acceleration.0 * time.delta_secs();
    }
}

pub fn apply_drag(mut query: Query<(&mut Velocity, &Damping)>, time: Res<Time>) {
    for (mut velocity, damping) in &mut query {
        velocity.0 *= 1.0 - (damping.0 * time.delta_secs());
    }
}

pub fn limit_velocity(mut query: Query<(&mut Velocity, &VelocityLimit)>) {
    for (mut velocity, limit) in &mut query {
        let speed = velocity.0.length();
        if speed > limit.0 { velocity.0 *= limit.0 / speed; }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.0);
    }
}

pub fn apply_rotation_velocity(mut query: Query<(&mut Transform, &RotationVelocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.rotate_z(velocity.0 * time.delta_secs());
    }
}
