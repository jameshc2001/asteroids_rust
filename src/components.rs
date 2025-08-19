use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct Ship;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct VelocityLimit(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct RotationVelocity(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Damping(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct RotationDamping(pub f32);

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Debug)]
pub struct Asteroid {
    pub max_radius: f32,
    pub alive: bool,
}

#[derive(Component)]
pub struct AsteroidLine;

#[derive(Component, Debug)]
pub struct Lifetime {
    pub age: f32,
    pub max: f32,
    pub fade: f32,
    pub enabled: bool,
}

// Resources
#[derive(Resource)]
pub struct ThrustSpawnTimer(pub Timer);
