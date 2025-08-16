use std::ops::{Add, AddAssign, Mul, MulAssign};

use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::Rng;

mod constants;
use crate::constants::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(ThrustSpawnTimer(Timer::from_seconds(THRUST_SPAWN_FREQUENCY, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update,
                     (
                         ship_movement_input,
                         ship_shoot_input,
                         spawn_thrust_particles,
                         apply_acceleration,
                         limit_velocity,
                         apply_velocity,
                         apply_drag,
                         update_and_fade_lifetime,
                     ).chain())
        .run();
}

#[derive(Component)]
struct Ship;

#[derive(Component, Deref, DerefMut, Debug)]
struct Acceleration(Vec2);

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct VelocityLimit(f32);

#[derive(Component, Deref, DerefMut)]
struct Drag(f32);

#[derive(Resource)]
struct ThrustSpawnTimer(Timer);

#[derive(Component)]
struct Bullet;

#[derive(Component, Debug)]
struct Lifetime {
    age: f32,
    max: f32,
    fade: f32
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    //Ship
    commands.spawn((
        Ship,
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(SHIP_COLOR)),
        Transform::from_translation(SHIP_STARTING_POSITION)
            .with_scale(Vec2::splat(SHIP_SCALE).extend(1.0)),
        Acceleration(Vec2::ZERO),
        Velocity(Vec2::ZERO),
        Drag(SHIP_DRAG),
        VelocityLimit(SHIP_VELOCITY_LIMIT)
    ));
}

fn ship_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Acceleration, &mut Transform), With<Ship>>,
    time: Res<Time>,
) {
    let (mut ship_acceleration, mut ship_transform) = query.single_mut().unwrap();
    ship_acceleration.0 = Vec2::new(0.0, 0.0);
    let direction = get_ship_direction(&ship_transform.rotation);

    if keyboard_input.any_pressed(FORWARD_INPUT) {
        ship_acceleration.0 = direction * SHIP_ACCELERATION * time.delta_secs()
    }

    if keyboard_input.any_pressed(BACKWARD_INPUT) {
        ship_acceleration.0 = -direction * SHIP_DECELERATION * time.delta_secs()
    }

    if keyboard_input.any_pressed(LEFT_INPUT) {
        ship_transform.rotate_z(SHIP_ROTATION_SPEED * time.delta_secs());
    }

    if keyboard_input.any_pressed(RIGHT_INPUT) {
        ship_transform.rotate_z(-SHIP_ROTATION_SPEED * time.delta_secs());
    }
}

fn ship_shoot_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Ship>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !keyboard_input.just_pressed(SHOOT_INPUT) { return; }

    let ship_transform = query.single().unwrap();
    let ship_direction = get_ship_direction(&ship_transform.rotation);

    commands.spawn((
        Bullet,
        Mesh2d(meshes.add(Rectangle::new(BULLET_WIDTH, BULLET_HEIGHT))),
        MeshMaterial2d(materials.add(BULLET_COLOR)),
        Transform::from_translation(ship_transform.translation.truncate().extend(1.0))
            .with_scale(Vec2::splat(BULLET_SCALE).extend(1.0))
            .with_rotation(ship_transform.rotation),
        Velocity(ship_direction * BULLET_SPEED),
        Lifetime { age: 0.0, max: BULLET_LIFETIME, fade: BULLET_FADE_DURATION },
    ));
}

fn get_ship_direction(ship_rotation: &Quat) -> Vec2 {
    ship_rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0)).truncate()
}


fn spawn_thrust_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timer: ResMut<ThrustSpawnTimer>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Ship>>
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
        Drag(THRUST_DRAG * random_size_scale),
        Lifetime { age: 0.0, max: THRUST_LIFETIME, fade: THRUST_FADE_DURATION },
    ));
}

fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.add_assign(acceleration.mul(time.delta_secs()));
    }
}

fn apply_drag(mut query: Query<(&mut Velocity, &Drag)>, time: Res<Time>) {
    for (mut velocity, drag) in &mut query {
        velocity.mul_assign(1.0 - (drag.0 * time.delta_secs()));
    }
}

fn limit_velocity(mut query: Query<(&mut Velocity, &VelocityLimit)>) {
    for (mut velocity, limit) in &mut query {
        let speed = velocity.0.length();
        if speed > limit.0 { velocity.mul_assign(limit.0 / speed); }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.add_assign(velocity.mul(time.delta_secs()).extend(0.0));
    }
}

fn update_and_fade_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &mut Lifetime, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (entity, mut lifetime, material) in &mut query {
        lifetime.age += time.delta_secs();
        if lifetime.age <= lifetime.max { continue; }

        let time_since_max = ((lifetime.age - lifetime.max) / lifetime.fade).clamp(0.0, 1.0);
        if let Some(mat) = materials.get_mut(material) {
            mat.color = mat.color.with_alpha(1.0 - time_since_max);
        }
        if time_since_max >= 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn log(mut query: Query<(&Acceleration, &Velocity, &Transform), With<Ship>>) {
    let (a, v, t) = query.single_mut().unwrap();
    println!("{:?} {:?} {:?}", a, v, t.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0)));
}