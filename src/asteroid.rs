use std::ops::Mul;
use bevy::prelude::*;
use crate::constants::SPAWN_ASTEROID_INPUT;

pub fn create_asteroid_vertices() -> Vec<Vec2> {
    let num_of_vertices: usize = 16;
    let radius: f32 = 100.0;

    let mut vertices = Vec::with_capacity(num_of_vertices + 1);
    for v in 0..num_of_vertices {
        let t = v as f32 / num_of_vertices as f32 * std::f32::consts::TAU; //+ random to rotate them
        vertices.push(Vec2::new(t.cos(), t.sin()).mul(radius));
    }
    vertices.push(vertices[0]);

    vertices
}

pub fn spawn_asteroid(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !keyboard_input.just_pressed(SPAWN_ASTEROID_INPUT) { return; }

    for pair in create_asteroid_vertices().windows(2) {
        let (curr, next) = (pair[0], pair[1]);
        spawn_line(&mut commands, &mut meshes, &mut materials, curr, next);
    }

}

fn spawn_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    a: Vec2,
    b: Vec2,
) {
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
    ));
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn create_asteroid_vertices_basic_properties() {
//         let vertices = create_asteroid_vertices();
//         assert_eq!(vertices.len(), 21);
//         println!("{:?}", vertices);
//     }
// }

