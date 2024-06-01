use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlanetBundle {
    pub pbr: PbrBundle,
    pub planet_data: PlanetData,
}

#[derive(Component, Debug)]
pub struct PlanetData {
    pub mass: f32,
    pub radius: f32,
    pub initial_velocity: Vec3,
    pub velocity: Vec3,
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

    pub fn compute_velocity(
        &self,
        position_1: Vec3,
        position_2: Vec3,
        mass_2: f32,
        g: f32,
        delta_seconds: f32,
    ) -> Vec3 {
        let sqrt_dist = position_1.distance_squared(position_2);
        let mass = self.mass;
        let force = (position_2 - position_1).normalize() * g * mass * mass_2 / sqrt_dist;

        delta_seconds * force / mass
    }
}
