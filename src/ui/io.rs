use std::fs;

use crate::{
    camera::MainCamera,
    planets::planet_bundle::{CelestialBodyBundle, CelestialBodyData, CelestialBodyType},
};

use super::SimulationState;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use serde::{Deserialize, Serialize};
use tinyfiledialogs::{open_file_dialog_multi, save_file_dialog};

#[derive(Serialize, Deserialize)]
struct CelestialBodyRelevantData {
    body_data: CelestialBodyData,
    position: Vec3,
}

#[derive(Default, Serialize, Deserialize)]
struct AppData {
    celestial_bodies: Vec<CelestialBodyRelevantData>,
    camera_position: Vec3,
}

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                save_scene.run_if(in_state(SimulationState::SaveSceneFile)),
                load_scene.run_if(in_state(SimulationState::PickSceneFile)),
            ),
        );
    }
}

fn save_scene(
    mut contexts: EguiContexts,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    query: Query<(&Transform, &CelestialBodyData), Without<MainCamera>>,
    query_cam: Query<&Transform, With<MainCamera>>,
) {
    egui::Window::new("Save File").show(contexts.ctx_mut(), |ui| {
        ui.spinner();

        let path_to_save = match save_file_dialog("Save current scene", "") {
            Some(path) => {
                info!("{}", path);
                path
            }
            None => {
                ui.label("An error occured");
                info!("Error while fetching path");
                return;
            }
        };
        let mut data = AppData {
            camera_position: query_cam.single().translation,
            ..Default::default()
        };

        for (transform, body) in &query {
            data.celestial_bodies.push(CelestialBodyRelevantData {
                body_data: body.clone(),
                position: transform.translation,
            });
        }

        let serialized_data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(e) => {
                info!("Error : {}", e);
                return;
            }
        };

        let res = fs::write(&path_to_save, serialized_data);
        match res {
            Ok(_) => info!("File saved to {}", path_to_save),
            Err(e) => info!("Error : {}", e),
        }
    });

    next_sim_state.set(SimulationState::Paused);
}

fn load_scene(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    egui::Window::new("Open File").show(contexts.ctx_mut(), |ui| {
        ui.spinner();
        let list_path_to_open = match open_file_dialog_multi("Open scene file", "", None) {
            Some(path) => {
                info!("{:?}", path);
                path
            }
            None => {
                ui.label("An error occured");
                info!("Error while fetching path");
                return;
            }
        };

        for path in list_path_to_open {
            match fs::read_to_string(&path) {
                Err(e) => info!("Error while reading {} : {}", path, e),
                Ok(data) => {
                    let app_data = serde_json::from_str::<AppData>(&data);
                    match app_data {
                        Err(e) => info!("Error while deserializing data from {} : {}", path, e),
                        Ok(app_data) => {
                            for body in app_data.celestial_bodies {
                                let color = Color::rgb_from_array(body.body_data.color);
                                let material = materials.add(StandardMaterial {
                                    base_color: color,
                                    emissive: color * body.body_data.emissive_factor,

                                    ..Default::default()
                                });

                                let mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

                                let mut entity_command = commands.spawn((CelestialBodyBundle {
                                    pbr: PbrBundle {
                                        mesh,
                                        material,
                                        transform: Transform {
                                            translation: body.position,
                                            scale: Vec3::ONE * body.body_data.radius,
                                            ..default()
                                        },
                                        ..Default::default()
                                    },

                                    body_data: CelestialBodyData::new(
                                        body.body_data.name,
                                        body.body_data.body_type.clone(),
                                        body.body_data.mass,
                                        body.body_data.radius,
                                        body.body_data.velocity,
                                        color,
                                    ),
                                },));
                                if body.body_data.body_type == CelestialBodyType::Star {
                                    entity_command.with_children(|p| {
                                        p.spawn(PointLightBundle {
                                            point_light: PointLight {
                                                color: Color::WHITE,
                                                intensity: body.body_data.light_factor,
                                                range: 1000.0,
                                                radius: body.body_data.radius,
                                                ..default()
                                            },
                                            ..default()
                                        });
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        //     let mut data = AppData {
        //         camera_position: query_cam.single().translation,
        //         ..Default::default()
        //     };

        //     for (transform, body) in &query {
        //         data.celestial_bodies.push(CelestialBodyRelevantData {
        //             body_data: body.clone(),
        //             position: transform.translation,
        //         });
        //     }

        //     let serialized_data = match serde_json::to_string(&data) {
        //         Ok(data) => data,
        //         Err(e) => {
        //             info!("Error : {}", e);
        //             return;
        //         }
        //     };

        //     let res = fs::write(&path_to_save, serialized_data);
        //     match res {
        //         Ok(_) => info!("File saved to {}", path_to_save),
        //         Err(e) => info!("Error : {}", e),
        //     }
    });

    next_sim_state.set(SimulationState::Paused);
}
