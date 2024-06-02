use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy::ecs::system::Resource;
use bevy_egui::egui::Pos2;
use bevy_egui::{egui, EguiContexts};

use crate::camera::camera_plugin::MainCamera;
use crate::planets::planet_bundle::PlanetData;
use crate::planets::planet_plugin::SpatialBody;

#[derive(Resource, Default)]
struct SelectedPlanet {
    data: Option<(PlanetData, Vec3)>,
}

pub struct PlanetUiPlugin;
impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPlanet>()
            .add_systems(Update, (check_selection, display_selected_planet));
    }
}
fn check_selection(
    mut selected_planet: ResMut<SelectedPlanet>,
    mut query: Query<(&'static mut PlanetData, &Transform), With<SpatialBody>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor_position) = q_windows.single().cursor_position() else {
        selected_planet.as_mut().data = None;
        return;
    };

    let (camera, camera_transform) = camera_query.single();

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        selected_planet.as_mut().data = None;

        return;
    };

    for (planet, transform) in query.iter_mut() {
        let l = ray.origin - transform.translation;
        if ray.direction.dot(l).abs() < 0. {
            continue;
        }

        let h = ray.origin
            + ray.direction.dot(l) * ray.direction.as_dvec3().as_vec3()
                / (ray.direction.length() * ray.direction.length());

        let d = (l.length_squared() - (h - ray.origin).length_squared()).sqrt();

        if d < planet.radius {
            selected_planet.as_mut().data = Some((planet.clone(), transform.translation));
            return;
        }
    }
    selected_planet.as_mut().data = None;
}
fn display_selected_planet(mut contexts: EguiContexts, selected_planet: Res<SelectedPlanet>) {
    if let Some((planet, position)) = &selected_planet.data {
        egui::Window::new(planet.name.clone())
            .current_pos(Pos2 { x: 0.5, y: 0. })
            .show(contexts.ctx_mut(), |ui| {
                ui.label(&format!("Radius: {}", planet.radius));
                ui.label(&format!("Mass: {}", planet.mass));
                ui.label(&format!(
                    "Position: ({}, {}, {})",
                    position.x, position.y, position.z
                ));
            });
    };
}
