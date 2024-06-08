use bevy::prelude::*;
use crate::planets::planet_bundle::CelestialBodyData;
use crate::ui::AppConfig;

pub struct PlanetUiPlugin;

impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_vectors.run_if(run_if_draw_velocities))
            .add_systems(Update, draw_trajectories.run_if(run_if_draw_trajectories))
            .add_systems(Update, draw_unit_vectors.run_if(run_if_draw_unit_vectors));
    }
}


/// Returns true if the app is configured to draw velocities.
fn run_if_draw_velocities(app_config: Res<AppConfig>) -> bool {
    app_config.draw_velocities
}

/// Draws velocity vectors for all bodies.
fn draw_vectors(
    mut gizmos: Gizmos,
    query: Query<(&CelestialBodyData, &Transform), With<CelestialBodyData>>,
) {
    for (body_data, transform) in &query {
        let body_position = transform.translation;
        let body_velocity = body_data.velocity;

        gizmos.arrow(
            body_position,
            body_position + 2. * body_velocity / body_data.radius,
            Color::YELLOW,
        );
    }
}

/// Returns true if the app is configured to draw velocities.
fn run_if_draw_unit_vectors(app_config: Res<AppConfig>) -> bool {
    app_config.draw_unit_vectors
}

/// Draws velocity vectors for all bodies.
fn draw_unit_vectors(
    mut gizmos: Gizmos,
    query: Query<(&CelestialBodyData, &Transform), With<CelestialBodyData>>,
) {
    for (body_data, transform) in &query {
        let body_position = transform.translation;

        gizmos.arrow(
            body_position,
            body_position + Vec3::X * 2. * body_data.radius,
            Color::RED,
        );

        gizmos.arrow(
            body_position,
            body_position + Vec3::Y * 2. * body_data.radius,
            Color::GREEN,
        );

        gizmos.arrow(
            body_position,
            body_position + Vec3::Z * 2. * body_data.radius,
            Color::BLUE,
        );
    }
}

/// Returns true if the app is configured to draw trajectories.
fn run_if_draw_trajectories(app_config: Res<AppConfig>) -> bool {
    app_config.draw_trajectories
}

/// Draws trajectories for all bodies by simulating their future positions over time.
fn draw_trajectories(
    mut gizmos: Gizmos,
    query: Query<(&CelestialBodyData, &Transform), With<CelestialBodyData>>,
    app_config: Res<AppConfig>,
) {
    let g = 1.;
    let delta_seconds = 0.01;
    let mut bodies_and_positions = query
        .iter()
        .map(|(bd, tfm)| (bd, tfm.translation, bd.velocity))
        .collect::<Vec<_>>();

    for _ in 0..app_config.trajectories_number_iterationss {
        let old_bodies_and_positions = bodies_and_positions.clone();

        for i in 0..bodies_and_positions.len() {
            let mut total_velocity_to_add = Vec3::ZERO;
            for j in 0..bodies_and_positions.len() {
                if i == j {
                    continue;
                }
                total_velocity_to_add += bodies_and_positions[i].0.compute_velocity(
                    old_bodies_and_positions[i].1,
                    old_bodies_and_positions[j].1,
                    old_bodies_and_positions[j].0.mass,
                    g,
                    delta_seconds,
                );
            }
            bodies_and_positions[i].2 += total_velocity_to_add;
            bodies_and_positions[i].1 =
                bodies_and_positions[i].1 + bodies_and_positions[i].2 * delta_seconds;
            gizmos.line(
                old_bodies_and_positions[i].1,
                bodies_and_positions[i].1,
                Color::rgb_from_array(bodies_and_positions[i].0.color),
            );
        }
    }
}
