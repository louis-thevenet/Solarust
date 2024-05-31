use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlanetBundle {
    pub pbr: PbrBundle,
    pub planet_data: PlanetData,
}

#[derive(Component)]
pub struct PlanetData {
    pub mass: f32,
    pub radius: f32,
    pub initial_velocity: Vec3,
    velocity: Vec3,
}

impl PlanetData {
    pub fn new(mass: f32, radius: f32, initial_velocity: Vec3) -> Self {
        Self {
            mass,
            radius,
            initial_velocity,
            velocity: initial_velocity,
        }
    }
}
