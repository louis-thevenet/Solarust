use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Bundle)]
/// A bundle of components for a `CelestialBody`.
pub struct CelestialBodyBundle {
    /// Holds a mesh and a material for rendering.
    pub pbr: PbrBundle,
    /// The body's data.
    pub body_data: CelestialBodyData,
}

#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Different types of `CelestialBodies`
pub enum CelestialBodyType {
    Planet,
    Star(f32),
}

#[derive(Component, Debug, Clone, Deserialize, Serialize)]
/// The data for a Body
pub struct CelestialBodyData {
    /// The body's name, acts as an identifier.
    pub name: String,
    /// The body's type
    pub body_type: CelestialBodyType,
    /// The body's mass.
    pub mass: f32,
    /// The body's radius.
    pub radius: f32,
    /// The body's initial velocity.
    pub initial_velocity: Vec3,
    /// The body's current velocity.
    pub velocity: Vec3,
    /// The body's color.
    pub color: [f32; 3],
}

impl CelestialBodyData {
    /// Constructs a new `CelestialBodyData`.
    pub fn new(
        name: String,
        body_type: CelestialBodyType,
        mass: f32,
        radius: f32,
        initial_velocity: Vec3,
        color: Color,
    ) -> Self {
        Self {
            name,
            body_type,
            mass,
            radius,
            initial_velocity,
            velocity: initial_velocity,
            color: color.rgb_linear_to_vec3().to_array(),
        }
    }

    /// Compute de velocity contribution from another body for a given time step.
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
