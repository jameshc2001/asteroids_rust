use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    // Ship
    commands.spawn((
        Ship,
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(SHIP_COLOR)),
        Transform::from_translation(SHIP_STARTING_POSITION)
            .with_scale(Vec2::splat(SHIP_SCALE).extend(1.0)),
        Acceleration(Vec2::ZERO),
        Velocity(Vec2::ZERO),
        Damping(SHIP_DRAG),
        VelocityLimit(SHIP_VELOCITY_LIMIT),
    ));
}
