use crate::constants::{ASTEROID_MAX_RADIUS, ASTEROID_MIN_RADIUS, ASTEROID_VERTICES, SPAWN_ASTEROID_INPUT};
use crate::{Asteroid, RotationVelocity};
use bevy::prelude::*;
use bevy::reflect::List;
use rand::Rng;
use std::ops::Mul;

pub fn create_asteroid_vertices() -> Vec<Vec2> {
    let radius: f32 = rand::rng().random_range(ASTEROID_MIN_RADIUS .. ASTEROID_MAX_RADIUS);

    let mut vertices = Vec::with_capacity(ASTEROID_VERTICES + 1);
    for v in 0..ASTEROID_VERTICES {
        let t = v as f32 / ASTEROID_VERTICES as f32 * std::f32::consts::TAU;// + random_rotation;
        let random_radius_scale = rand::rng().random_range(0.75 .. 1.25f32);
        vertices.push(Vec2::new(t.cos(), t.sin()).mul(radius * random_radius_scale));
    }
    vertices.push(vertices[0]);

    vertices
}

pub fn spawn_asteroid(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Asteroid>>,
) {
    if !keyboard_input.just_pressed(SPAWN_ASTEROID_INPUT) { return; }

    if let Ok(existing_asteroid_entity) = query.single() {
        commands.entity(existing_asteroid_entity).despawn();
    }

    let vertices = create_asteroid_vertices();
    
    let mut max_radius: f32 = 0.0;
    for v in &vertices {
        max_radius = max_radius.max(v.length());
    }

    let parent = commands.spawn((
        Asteroid { max_radius },
        Transform::from_xyz(200.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(rand::rng().random_range(0.0 .. std::f32::consts::TAU))),
        RotationVelocity(random_rotation()),
        GlobalTransform::default(),
    )).id();

    for pair in vertices.windows(2) {
        let (curr, next) = (pair[0], pair[1]);
        let child = spawn_line(&mut commands, &mut meshes, &mut materials, curr, next);
        commands.entity(parent).add_child(child);
    }
}

fn random_rotation() -> f32 {
    if rand::rng().random_bool(0.5) {
        rand::rng().random_range(-1.2 .. -0.2)
    } else {
        rand::rng().random_range(0.2 .. 1.2)
    }
}

fn spawn_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    a: Vec2,
    b: Vec2,
) -> Entity {
    let delta = b - a;
    let length = delta.length().max(0.0001);
    let angle = delta.y.atan2(delta.x);
    let midpoint = (a + b) * 0.5;
    
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(length, 2.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: midpoint.extend(0.1),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::splat(1.0),
            ..default()
        },
    )).id()
}
