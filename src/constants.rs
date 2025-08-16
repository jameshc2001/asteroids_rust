use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub const FORWARD_INPUT: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
pub const BACKWARD_INPUT: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
pub const LEFT_INPUT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
pub const RIGHT_INPUT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
pub const SHOOT_INPUT: KeyCode = KeyCode::Space;

pub const SHIP_COLOR: Color = Color::WHITE;
pub const SHIP_STARTING_POSITION: Vec3 = Vec3::new(50.0, -50.0, 100.0); //Always draw ship on top
pub const SHIP_SCALE: f32 = 25.0;
pub const SHIP_ACCELERATION: f32 = 50000.0;
pub const SHIP_DECELERATION: f32 = SHIP_ACCELERATION / 7.0;
pub const SHIP_ROTATION_SPEED: f32 = 5.0;
pub const SHIP_DRAG: f32 = 0.5;
pub const SHIP_VELOCITY_LIMIT: f32 = 600.0;

pub const THRUST_BASE_COLOR: Color = Color::srgb(1.0, 0.5, 0.1);
pub const THRUST_SCALE: f32 = 10.0;
pub const THRUST_START_SPEED: f32 = 150.0;
pub const THRUST_DRAG: f32 = 0.5;
pub const THRUST_SPAWN_FREQUENCY: f32 = 0.1;
pub const THRUST_LIFETIME: f32 = 3.0;
pub const THRUST_FADE_DURATION: f32 = 4.0;

pub const BULLET_COLOR: Color = Color::srgb(1.0, 0.1, 0.1);
pub const BULLET_SPEED: f32 = 1500.0;
pub const BULLET_WIDTH: f32 = 0.2;
pub const BULLET_HEIGHT: f32 = 1.0;
pub const BULLET_SCALE: f32 = 20.0;
pub const BULLET_LIFETIME: f32 = 2.0;
pub const BULLET_FADE_DURATION: f32 = 0.1;
