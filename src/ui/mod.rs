use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use debug_ui_plugin::DebugUiPlugin;
use planet_ui_plugin::PlanetUiPlugin;
use crate::camera::camera_controller_plugin::CameraController;

use crate::camera::camera_plugin::MainCamera;

mod debug_ui_plugin;
pub(crate) mod planet_ui_plugin;
mod move_body_plugin;

#[derive(Default, States, Debug, Hash, Eq, Clone, Copy, PartialEq)]
/// The state of the application.
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}

#[derive(Resource)]
/// The configuration of the application.
pub struct AppConfig {
    pub draw_velocities: bool,
    pub draw_trajectories: bool,
    pub trajectories_number_iterationss: usize,
    pub draw_unit_vectors: bool,
    pub add_new_planet: bool,
}

impl Default for AppConfig {
    /// Default configuration of the application.
    fn default() -> Self {
        Self {
            draw_velocities: true,
            draw_trajectories: true,
            trajectories_number_iterationss: 500,
            draw_unit_vectors: false,
            add_new_planet: false,
        }
    }
}

/// The UI plugin of the application.
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppConfig>()
            .init_state::<SimulationState>()
            .add_plugins(EguiPlugin)
            .add_plugins((DebugUiPlugin, PlanetUiPlugin))
            .add_systems(Update, (build_ui, ui_controls));
    }
}

/// Builds the side panel of the application.
fn build_ui(
    mut contexts: EguiContexts,
    mut app_config: ResMut<AppConfig>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    query_cam: Query<&Transform, With<MainCamera>>,
) {
    egui::SidePanel::left("Menu")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            match *sim_state.get() {
                SimulationState::Paused => {
                    ui.heading("Paused");
                    if ui.button("Resume").clicked() {
                        next_sim_state.set(SimulationState::Running);
                    }
                }
                SimulationState::Running => {
                    ui.heading("Running");
                    if ui.button("Pause").clicked() {
                        next_sim_state.set(SimulationState::Paused);
                    }
                }
            }

            let cam = query_cam.single().translation;

            ui.label(format!("Cam position : ({:.1}, {:.1}, {:.1})", cam.x, cam.y, cam.z));
            ui.label(format!("{}",CameraController::default()));
            

            if ui.button("Add new planet").clicked() {
                app_config.add_new_planet = true;
            };

            ui.checkbox(&mut app_config.draw_velocities, "Draw velocities");
            ui.checkbox(&mut app_config.draw_trajectories, "Draw trajectories");

            ui.horizontal(|ui| {
                ui.add(egui::widgets::DragValue::new(&mut app_config.trajectories_number_iterationss).speed(100));
                ui.label("Future trajectories steps");
            });

            ui.collapsing("Debug", |ui| {
                ui.checkbox(&mut app_config.draw_unit_vectors, "Draw unit vectors (XYZ=RGB)");
            });


            if ui.button("Quit").clicked() {
                app_exit_events.send(AppExit);
            };
        });
}

/// Handles controls for the simulation.
fn ui_controls(
    keys: Res<ButtonInput<KeyCode>>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match sim_state.get() {
            SimulationState::Paused => next_sim_state.set(SimulationState::Running),
            SimulationState::Running => next_sim_state.set(SimulationState::Paused),
        }
    }
}
