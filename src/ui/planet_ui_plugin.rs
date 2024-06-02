use bevy::ecs::query;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_egui::{egui, EguiContexts};

use crate::camera::camera_plugin::MainCamera;
use crate::planets::planet_bundle::PlanetData;
use crate::planets::planet_plugin::SpatialBody;

#[derive(Component)]
struct SelectedPlanetMarker;

pub struct PlanetUiPlugin;
impl Plugin for PlanetUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_selection, display_selected_planet));
    }
}

fn clear_celection(
    commands: &mut Commands,
    selected: Result<(Entity, Mut<PlanetData>, &Transform), query::QuerySingleError>,
) {
    if let Ok((e, _, _)) = selected {
        commands
            .get_entity(e)
            .unwrap()
            .remove::<SelectedPlanetMarker>();
    }
}
fn check_selection(
    mut contexts: EguiContexts,

    mut commands: Commands,
    mut query: Query<
        (Entity, &mut PlanetData, &Transform),
        (With<SpatialBody>, Without<SelectedPlanetMarker>),
    >,
    mut query_selected: Query<(Entity, &mut PlanetData, &Transform), With<SelectedPlanetMarker>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left)
        || contexts.ctx_mut().wants_pointer_input()
        || contexts.ctx_mut().wants_keyboard_input()
    {
        return;
    }
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = q_windows.single().cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        clear_celection(&mut commands, query_selected.get_single_mut());
        return;
    };

    for (e, planet, transform) in query.iter_mut() {
        let l = ray.origin - transform.translation;
        if ray.direction.dot(l).abs() < 0. {
            continue;
        }

        let h = ray.origin
            + ray.direction.dot(l) * ray.direction.as_dvec3().as_vec3()
                / (ray.direction.length() * ray.direction.length());

        let d = (l.length_squared() - (h - ray.origin).length_squared()).sqrt();

        if d < planet.radius {
            clear_celection(&mut commands, query_selected.get_single_mut());
            commands.get_entity(e).unwrap().insert(SelectedPlanetMarker);
            return;
        }
    }
    clear_celection(&mut commands, query_selected.get_single_mut());
}
fn display_selected_planet(
    mut contexts: EguiContexts,
    query_selected: Query<(&mut PlanetData, &Transform), With<SelectedPlanetMarker>>,
) {
    if let Ok((planet, tfm)) = query_selected.get_single() {
        egui::Window::new(planet.name.clone()).show(contexts.ctx_mut(), |ui| {
            ui.label(&format!("Radius: {}", planet.radius));
            ui.label(&format!("Mass: {}", planet.mass));
            ui.label(&format!(
                "Position: ({}, {}, {})",
                tfm.translation.x, tfm.translation.y, tfm.translation.z
            ));
        });
    }
}
