use std::collections::VecDeque;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

use crate::ui::{AppConfig, SimulationState};

use super::planet_bundle::{CelestialBodyBundle, CelestialBodyData, CelestialBodyType};

/// This plugin is responsible for setting up the simulation
/// and its associated systems such as rendering and physics.
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_mutual)
            .add_systems(Update, rotate)
            .add_systems(
                FixedUpdate,
                (update_velocities, update_positions)
                    .chain()
                    .run_if(in_state(SimulationState::Running)),
            ).add_systems(Update, radius_changed)
            .add_systems(Update, add_new_planet.run_if(run_if_add_new_planet))
            .add_systems(Update, draw_vectors.run_if(run_if_draw_velocities))
            .add_systems(Update, draw_trajectories.run_if(run_if_draw_trajectories))
            .add_systems(Update, draw_unit_vectors.run_if(run_if_draw_unit_vectors));
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

/// Sets up a simple scene.
fn setup_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_material = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Mask(0.0),
        ..Default::default()
    });

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let sun_mass = 1.0e6;
    let planet_mass = 1.0e3;
    let sun_radius = 10.0;
    let planet_radius = sun_radius * 0.3;
    let planet_initial_velocity = Vec3::new(0., 0., 100.);
    let sun = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());
    let planet = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    commands.spawn((
        CelestialBodyBundle {
            pbr: PbrBundle {
                mesh: sun,
                material: sun_material.clone(),
                transform: Transform {
                    translation: Default::default(),
                    rotation: Default::default(),
                    scale: Vec3::ONE * sun_radius,
                },
                ..Default::default()
            },
            body_data: CelestialBodyData::new(
                String::from("Sun"),
                CelestialBodyType::Star(1.),
                sun_mass,
                sun_radius,
                Vec3::ZERO,
                Color::YELLOW,
            ),
        },
    ));
    commands.spawn((
        CelestialBodyBundle {
            pbr: PbrBundle {
                mesh: planet,
                material: debug_material.clone(),
                transform: Transform {
                    translation: Vec3::new(100.0, 0.0, 0.0),
                    scale: Vec3::ONE * planet_radius,
                    ..default()
                },
                ..Default::default()
            },

            body_data: CelestialBodyData::new(
                String::from("Planet"),
                CelestialBodyType::Planet,
                planet_mass,
                planet_radius,
                planet_initial_velocity,
                Color::BLUE,
            ),
        },
    ));
    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Sets up a simple scene.
fn setup_mutual(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::YELLOW,
        ..Default::default()
    });

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let sun_mass = 1.0e6;
    let planet_mass = 1.0e6;
    let sun_radius = 10.0;
    let planet_radius = sun_radius;
    let planet_initial_velocity = Vec3::new(0., 0., 100.);
    let sun = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());
    let planet = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    commands.spawn((
        CelestialBodyBundle {
            pbr: PbrBundle {
                mesh: sun,
                material: sun_material.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    rotation: Default::default(),
                    scale: Vec3::ONE * sun_radius,
                },
                ..Default::default()
            },
            body_data: CelestialBodyData::new(
                String::from("Sun"),
                CelestialBodyType::Star(1.),
                sun_mass,
                sun_radius,
                Vec3::ZERO,
                Color::YELLOW,
            ),
        },
    ));
    commands.spawn((
        CelestialBodyBundle {
            pbr: PbrBundle {
                mesh: planet,
                material: debug_material.clone(),
                transform: Transform {
                    translation: Vec3::new(100.0, 0.0, 0.0),
                    scale: Vec3::ONE * planet_radius,
                    ..default()
                },
                ..Default::default()
            },

            body_data: CelestialBodyData::new(
                String::from("Planet"),
                CelestialBodyType::Planet,
                planet_mass,
                planet_radius,
                planet_initial_velocity,
                Color::BLUE,
            ),
        },
    ));
    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn run_if_add_new_planet(app_config: Res<AppConfig>) -> bool {
    app_config.add_new_planet
}

fn add_new_planet(mut app_config: ResMut<AppConfig>, mut commands: Commands,
                  selected_query: Query<
                      (&Transform, &Handle<Mesh>, &CelestialBodyData),
                      (With<crate::ui::planet_ui_plugin::SelectedPlanetMarker>)>,
                  mut images: ResMut<Assets<Image>>,
                  mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::rgb_from_array([rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()]);
    let material = materials.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });


    app_config.add_new_planet = false;

    if let Ok((tfm, mesh, data)) = selected_query.get_single() {
        commands.spawn((
            CelestialBodyBundle {
                pbr: PbrBundle {
                    mesh: mesh.clone(),
                    material: material,
                    transform: Transform {
                        translation: tfm.translation + Vec3::ONE * data.radius,
                        scale: Vec3::ONE * data.radius,
                        ..default()
                    },
                    ..Default::default()
                },

                body_data: CelestialBodyData::new(
                    String::from("Planet"),
                    CelestialBodyType::Planet,
                    data.mass,
                    data.radius,
                    data.velocity,
                    color,
                ),
            },
        ));
    }
}


/// Returns true if the app is configured to draw velocities.
fn run_if_draw_velocities(app_config: Res<AppConfig>) -> bool {
    app_config.draw_velocities
}

/// Draws velocity vectors for all bodies.
fn draw_vectors(mut gizmos: Gizmos, query: Query<(&CelestialBodyData, &Transform), With<CelestialBodyData>>) {
    for (body_data, transform) in &query {
        let body_position = transform.translation;
        let body_velocity = body_data.velocity;

        gizmos.arrow(
            body_position,
            body_position + body_velocity / body_data.radius,
            Color::YELLOW,
        );
    }
}

/// Returns true if the app is configured to draw velocities.
fn run_if_draw_unit_vectors(app_config: Res<AppConfig>) -> bool {
    app_config.draw_unit_vectors
}

/// Draws velocity vectors for all bodies.
fn draw_unit_vectors(mut gizmos: Gizmos, query: Query<(&CelestialBodyData, &Transform), With<CelestialBodyData>>) {
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

/// detect new enemies and print their health
fn radius_changed(
    mut query: Query<(&mut Transform, &CelestialBodyData), Changed<CelestialBodyData>>
) {
    for (mut tfm, data) in &mut query {
        tfm.scale = Vec3::ONE * data.radius
    }
}