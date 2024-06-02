use bevy::prelude::*;

#[derive(Bundle)]
/// A bundle of components for a Planet.
pub struct PlanetBundle {
    /// Holds a mesh and a material for rendering.
    pub pbr: PbrBundle,
    /// The planet's data.
    pub planet_data: PlanetData,
}

#[derive(Component, Debug, Clone)]
/// The data for a Planet
pub struct PlanetData {
    /// The planet's name, acts as an identifier.
    pub name: String,
    /// The planet's mass.
    pub mass: f32,
    /// The planet's radius.
    pub radius: f32,
    /// The planet's initial velocity.
    pub initial_velocity: Vec3,
    /// The planet's current velocity.
    pub velocity: Vec3,
    /// The planet's color.
    pub color: Color,
}

impl PlanetData {
    /// Constructs a new PlanetData.
    pub fn new(name: String, mass: f32, radius: f32, initial_velocity: Vec3, color: Color) -> Self {
        Self {
            name,
            mass,
            radius,
            initial_velocity,
            velocity: initial_velocity,
            color,
        }
    }

    /// Compute de velocity contribution from another planet for a given time step.
    pub fn compute_velocity(
        &self,
        position_1: Vec3,
        position_2: Vec3,
        mass_2: f32,
        g: f32,
        delta_seconds: f32,
    ) -> Vec3 {
        let squared_dist = position_1.distance_squared(position_2);
        let mass = self.mass;
        let force = (position_2 - position_1).normalize() * g * mass * mass_2 / squared_dist;

        delta_seconds * force / mass
    }
}
