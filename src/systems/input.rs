use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn ship_movement_input(
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

pub fn ship_shoot_input(
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

pub fn get_ship_direction(ship_rotation: &Quat) -> Vec2 {
    ship_rotation.mul_vec3(Vec3::new(0.0, 1.0, 0.0)).truncate()
}
