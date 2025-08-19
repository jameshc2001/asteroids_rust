use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::Rng;
use std::ops::{Add, Mul};

use crate::components::*;
use crate::constants::*;
use crate::systems::input::get_ship_direction;

pub fn spawn_thrust_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timer: ResMut<ThrustSpawnTimer>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Ship>>,
) {
    if !keyboard_input.any_pressed(FORWARD_INPUT) { return; }
    if keyboard_input.any_just_pressed(FORWARD_INPUT) {
        timer.0.reset();
    } else {
        if !timer.0.tick(time.delta()).just_finished() { return; }
    }

    let ship_transform = query.single().unwrap();
    let ship_direction = get_ship_direction(&ship_transform.rotation);
    let ship_perpendicular = ship_direction.perp();

    let random_rotation = rand::rng().random_range(-0.5 .. 0.5f32);
    let particle_direction = Vec2::new(
        random_rotation.cos(),
        random_rotation.sin()
    ).rotate(-ship_direction);

    let random_perp_dist = rand::rng().random_range(-7.0..7.0f32);
    let particle_start_location = ship_transform.translation
        .truncate()
        .add(particle_direction.mul(SHIP_SCALE / 4.0))
        .add(ship_perpendicular.mul(random_perp_dist))
        .extend(0.0);

    let random_size_scale = rand::rng().random_range(0.5 .. 1.25f32);
    let random_speed_scale = rand::rng().random_range(0.8 .. 1.2f32);

    let random_color = Color::srgb(
        rand::rng().random_range(0.8 .. 1.0f32),
        rand::rng().random_range(0.4 .. 0.6f32),
        rand::rng().random_range(0.1 .. 0.3f32)
    );

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: random_color,
            alpha_mode: AlphaMode2d::Blend,
            ..default()
        })),
        Transform::from_translation(particle_start_location)
            .with_scale(Vec2::splat(THRUST_SCALE * random_size_scale).extend(1.0)),
        Velocity(particle_direction.mul(THRUST_START_SPEED * random_speed_scale)),
        Damping(THRUST_DRAG * random_size_scale),
        Lifetime { age: 0.0, max: THRUST_LIFETIME, fade: THRUST_FADE_DURATION },
    ));
}
