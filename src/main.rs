use std::ops::{Mul, MulAssign};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BACKGROUND_COLOR: Color = Color::BLACK;

const SHIP_COLOR: Color = Color::WHITE;
const SHIP_STARTING_POSITION: Vec3 = Vec3::new(50.0, -50.0, 1.0);
const SHIP_SCALE: f32 = 50.0;
const SHIP_ACCELERATION: f32 = 50000.0;
const SHIP_DECELERATION: f32 = SHIP_ACCELERATION / 7.0;
const SHIP_ROTATION_SPEED: f32 = 5.0;
const SHIP_DRAG: f32 = 0.5;
const SHIP_VELOCITY_LIMIT: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update,
                     (
                         ship_input,
                         apply_acceleration,
                         limit_velocity,
                         apply_velocity,
                         apply_drag
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Triangle2d::default()).into(),
            material: materials.add(SHIP_COLOR),
            transform: Transform::from_translation(SHIP_STARTING_POSITION).with_scale(Vec2::splat(SHIP_SCALE).extend(1.0)),
            ..default()
        },
        Ship,
        Acceleration(Vec2::new(0.0, 0.0)),
        Velocity(Vec2::new(0.0, 0.0)),
        Drag(SHIP_DRAG),
        VelocityLimit(SHIP_VELOCITY_LIMIT)
    ));
}

fn ship_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Acceleration, &mut Transform), With<Ship>>,
    time: Res<Time>,
) {
    let (mut ship_acceleration, mut ship_transform) = query.single_mut();
    ship_acceleration.0 = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        let direction = ship_transform.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0)).truncate();
        ship_acceleration.0 = direction * SHIP_ACCELERATION * time.delta_seconds()
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        let direction = ship_transform.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0)).truncate();
        ship_acceleration.0 = -direction * SHIP_DECELERATION * time.delta_seconds()
    }

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        ship_transform.rotate_z(SHIP_ROTATION_SPEED * time.delta_seconds());
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        ship_transform.rotate_z(-SHIP_ROTATION_SPEED * time.delta_seconds());
    }
}

fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.x += acceleration.x * time.delta_seconds();
        velocity.y += acceleration.y * time.delta_seconds();
    }
}

fn apply_drag(mut query: Query<(&mut Velocity, &Drag)>, time: Res<Time>) {
    for (mut velocity, drag) in &mut query {
        let scale = 1.0 - (drag.0 * time.delta_seconds());
        velocity.x *= scale;
        velocity.y *= scale;
    }
}

fn limit_velocity(mut query: Query<(&mut Velocity, &VelocityLimit)>, time: Res<Time>) {
    for (mut velocity, limit) in &mut query {
        let speed = velocity.0.length();
        if speed > limit.0 {
            velocity.mul_assign(limit.0 / speed);
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn log(mut query: Query<(&Acceleration, &Velocity, &Transform), With<Ship>>) {
    let (a, v, t) = query.single_mut();
    println!("{:?} {:?} {:?}", a, v, t.rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0)));
}