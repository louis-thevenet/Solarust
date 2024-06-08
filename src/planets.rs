use std::collections::VecDeque;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

use planet_bundle::{CelestialBodyBundle, CelestialBodyData, CelestialBodyType};
use rand::Rng;

use crate::ui::SimulationState;

pub mod planet_bundle;

/// This plugin is responsible for setting up the simulation
/// and its associated systems such as rendering and physics.
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, (setup_mutual, setup_simple_stars))
            .add_systems(Update, rotate)
            .add_systems(
                FixedUpdate,
                (update_velocities, update_positions)
                    .chain()
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(Update, radius_changed);
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
#[allow(unused)]
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

    commands.spawn((CelestialBodyBundle {
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
    },));
    commands.spawn((CelestialBodyBundle {
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
    },));
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::ORANGE_RED,
        emissive: (Color::ORANGE_RED * 18.),
        ..default()
    });

    let debug_material = materials.add(Color::BLUE);

    let sun_mass = 1.0e6;
    let planet_mass = 1.0e6;
    let sun_radius = 10.0;
    let planet_radius = sun_radius;
    let planet_initial_velocity = Vec3::new(0., 0., 100.);
    let sun = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());
    let planet = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    commands
        .spawn((CelestialBodyBundle {
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
        },))
        .with_children(|p| {
            p.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::WHITE,
                    intensity: 1_000_000_000.0,
                    range: 1000.0,
                    radius: sun_radius,
                    ..default()
                },
                ..default()
            });
        });

    commands.spawn((CelestialBodyBundle {
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
    },));
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
    let stars_count = 5000;
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

/// detect new enemies and print their health
fn radius_changed(
    mut query: Query<(&mut Transform, &CelestialBodyData), Changed<CelestialBodyData>>,
) {
    for (mut tfm, data) in &mut query {
        tfm.scale = Vec3::ONE * data.radius
    }
}
