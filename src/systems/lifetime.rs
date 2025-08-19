use bevy::prelude::*;

use crate::components::*;

impl Default for Lifetime {
    fn default() -> Self {
        Self {
            enabled: true,
            max: 5.0,
            age: 0.0,
            fade: 5.0,
        }
    }
}

pub fn update_and_fade_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &mut Lifetime, &MeshMaterial2d<ColorMaterial>)>,
) {
    for (entity, mut lifetime, material) in &mut query {
        if !lifetime.enabled { continue; }
        
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
