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
    Star,
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
    /// How emissive the material is
    pub emissive_factor: f32,
    /// How much light is sent to other bodies (onyl if  it's a star)
    pub light_factor: f32,
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
        let (emissive, light) = match body_type {
            CelestialBodyType::Planet => (0.0, 0.0),
            CelestialBodyType::Star => (18.0, 1_000_000_000.0),
        };
        Self {
            name,
            body_type,
            mass,
            radius,
            initial_velocity,
            velocity: initial_velocity,
            color: color.rgb_linear_to_vec3().to_array(),
            emissive_factor: emissive,
            light_factor: light,
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
