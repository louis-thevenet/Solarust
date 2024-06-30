use std::collections::VecDeque;

use bevy::prelude::*;

use planet_bundle::CelestialBodyData;
use rand::Rng;

use crate::ui::SimulationState;

pub mod planet_bundle;

/// This plugin is responsible for setting up the simulation
/// and its associated systems such as rendering and physics.
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, setup_simple_stars)
            .add_systems(Update, (rotate, radius_changed))
            .add_systems(
                FixedUpdate,
                (update_velocities, update_positions)
                    .chain()
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

/// Rotates the bodies in the simulation.
/// This is a simple way to make the planets look like they're moving.
fn rotate(mut query: Query<&mut Transform, With<CelestialBodyData>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Updates the velocities of bodies by calculating their gravitational
/// forces on each other.
fn update_velocities(
    mut query: Query<(&mut CelestialBodyData, &Transform), With<CelestialBodyData>>,
    time: Res<Time>,
) {
    let g = 1.;

    let mut operations = VecDeque::new();
    for (bd1, tfm1) in &query {
        let mut total_velocity_to_add = Vec3::ZERO;
        for (bd2, tfm2) in query.iter() {
            if tfm1 == tfm2 {
                continue;
            }
            total_velocity_to_add += bd1.compute_velocity(
                tfm1.translation,
                tfm2.translation,
                bd2.mass,
                g,
                time.delta_seconds(),
            );
        }
        operations.push_back(total_velocity_to_add);
    }

    for (mut db, _) in &mut query {
        db.velocity += operations.pop_front().unwrap();
    }
}

/// Updates the positions of bodies based on their velocities.
fn update_positions(
    mut query: Query<(&CelestialBodyData, &mut Transform), With<CelestialBodyData>>,
    time: Res<Time>,
) {
    for (bd, mut tfm) in &mut query {
        tfm.translation += bd.velocity * time.delta_seconds();
    }
}

/// Runs when the radius of a celestial body changes to update the scale of its transform.
fn radius_changed(
    mut query: Query<(&mut Transform, &CelestialBodyData), Changed<CelestialBodyData>>,
) {
    for (mut tfm, data) in &mut query {
        tfm.scale = Vec3::ONE * data.radius
    }
}

fn setup_simple_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: (Color::WHITE * 18.),
        ..default()
    });
    let radius = 10.0;
    let star_mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());
    let inner_bound = 5000.0;
    let outer_bound = 50000.0;
    let stars_count = if cfg!(target_arch = "wasm32") {
        3000
    } else {
        10000
    };
    for _ in 0..stars_count {
        let x = rng.gen_range(0.0..(outer_bound - inner_bound))
            * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        let y = rng.gen_range(0.0..(outer_bound - inner_bound))
            * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        let z = rng.gen_range(0.0..(outer_bound - inner_bound))
            * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        commands
            .spawn(PbrBundle {
                mesh: star_mesh.clone(),
                material: star_material.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, z),
                    scale: Vec3::ONE * radius,
                    ..default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::WHITE,
                        intensity: 1_000_000_000.0,
                        range: 1000.0,
                        radius,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}
