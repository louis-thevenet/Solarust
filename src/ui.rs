use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, States, Debug, Hash, Eq, Clone, Copy, PartialEq)]

pub enum SimulationState {
    #[default]
    Paused,
    Running,
}

#[derive(Default, Resource)]
pub struct AppConfig {
    pub draw_velocities: bool,
    pub draw_trajectories: bool,
    pub trajectories_iterations: usize,
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppConfig>()
            .init_state::<SimulationState>()
            .add_plugins(EguiPlugin)
            .add_systems(Update, (build_ui, ui_controls));
    }
}

fn build_ui(
    mut contexts: EguiContexts,
    mut app_config: ResMut<AppConfig>,
    sim_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    egui::SidePanel::left("Menu")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            match *sim_state.get() {
                SimulationState::Paused => {
                    ui.heading("Paused");
                    if ui.button("Resume").clicked() {
                        next_sim_state.set(SimulationState::Running)
                    }
                }
                SimulationState::Running => {
                    ui.heading("Running");
                    if ui.button("Pause").clicked() {
                        next_sim_state.set(SimulationState::Paused)
                    }
                }
            }
            ui.checkbox(&mut app_config.draw_velocities, "Draw velocities");
            ui.checkbox(&mut app_config.draw_trajectories, "Draw trajectories");
            ui.add(
                egui::widgets::Slider::new(&mut app_config.trajectories_iterations, 1..=9999)
                    .text("Trajectory iterations"),
            );

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
}

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
