use std::collections::VecDeque;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

use crate::ui::{AppConfig, SimulationState};

use super::planet_bundle::{PlanetBundle, PlanetData};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_test)
            .add_systems(Update, rotate)
            .add_systems(
                FixedUpdate,
                (update_velocities, update_positions)
                    .chain()
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(Update, draw_vectors.run_if(run_if_draw_velocities))
            .add_systems(Update, draw_trajectories.run_if(run_if_draw_trajectories));
    }
}

#[derive(Component)]
struct SpatialBody;

fn rotate(mut query: Query<&mut Transform, With<SpatialBody>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

fn update_velocities(
    mut query: Query<(&mut PlanetData, &Transform), With<SpatialBody>>,
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

    for (mut db, _) in query.iter_mut() {
        db.velocity += operations.pop_front().unwrap();
    }
}

fn update_positions(
    mut query: Query<(&PlanetData, &mut Transform), With<SpatialBody>>,
    time: Res<Time>,
) {
    for (bd, mut tfm) in query.iter_mut() {
        tfm.translation += bd.velocity * time.delta_seconds();
    }
}

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

    let sun = meshes.add(Sphere::new(sun_radius).mesh().ico(5).unwrap());
    let planet = meshes.add(Sphere::new(planet_radius).mesh().ico(5).unwrap());

    commands.spawn((
        PlanetBundle {
            pbr: PbrBundle {
                mesh: sun,
                material: sun_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            planet_data: PlanetData::new(sun_mass, sun_radius, Vec3::ZERO, Color::YELLOW),
        },
        SpatialBody,
    ));
    commands.spawn((
        PlanetBundle {
            pbr: PbrBundle {
                mesh: planet,
                material: debug_material.clone(),
                transform: Transform::from_xyz(100.0, 0.0, 0.0),
                ..Default::default()
            },

            planet_data: PlanetData::new(
                planet_mass,
                planet_radius,
                planet_initial_velocity,
                Color::BLUE,
            ),
        },
        SpatialBody,
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

fn run_if_draw_velocities(app_config: Res<AppConfig>) -> bool {
    app_config.draw_velocities
}

fn draw_vectors(mut gizmos: Gizmos, query: Query<(&PlanetData, &Transform), With<SpatialBody>>) {
    for (planet_data, transform) in &query {
        let planet_position = transform.translation;
        let planet_velocity = planet_data.velocity;

        gizmos.arrow(
            planet_position,
            planet_position + planet_velocity / planet_data.radius,
            Color::YELLOW,
        );
    }
}

fn run_if_draw_trajectories(app_config: Res<AppConfig>) -> bool {
    app_config.draw_trajectories
}

fn draw_trajectories(
    mut gizmos: Gizmos,
    query: Query<(&PlanetData, &Transform), With<SpatialBody>>,
    app_config: Res<AppConfig>,
) {
    let g = 1.;
    let delta_seconds = 0.01;
    let mut bodies_and_positions = query
        .iter()
        .map(|(bd, tfm)| (bd, tfm.translation, bd.velocity))
        .collect::<Vec<_>>();

    for _ in 0..app_config.trajectories_iterations {
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
                bodies_and_positions[i].0.color,
            )
        }
    }
}
