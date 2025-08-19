use bevy::prelude::*;

mod constants;
mod components;
mod systems;

use crate::components::ThrustSpawnTimer;
use crate::constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(ThrustSpawnTimer(Timer::from_seconds(THRUST_SPAWN_FREQUENCY, TimerMode::Repeating)))
        .add_systems(Startup, systems::setup::setup)
        .add_systems(Update, (
            systems::input::ship_movement_input,
            systems::input::ship_shoot_input,
            systems::asteroid::spawn_asteroid,
            systems::particles::spawn_thrust_particles,
            systems::physics::apply_acceleration,
            systems::physics::limit_velocity,
            systems::physics::apply_velocity,
            systems::physics::apply_rotation_velocity,
            systems::physics::apply_drag,
            systems::lifetime::update_and_fade_lifetime,
            systems::collisions::bullet_asteroid_collision,
        ))
        .run();
}