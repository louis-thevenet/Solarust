use bevy::ecs::query;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

use crate::planets::planet_bundle::{CelestialBodyBundle, CelestialBodyType};
use crate::ui::AppConfig;
use crate::{camera::MainCamera, planets::planet_bundle::CelestialBodyData};

#[derive(Resource, Default)]
struct Duplicate(bool);

#[derive(Component)]
/// Marker component for the currently selected planet.
pub struct SelectedPlanetMarker;

/// Plugin responsible for displaying the planets related UI.
/// This includes the currently selected planet's details for now.
pub struct SelectedPlanetUiPlugin;

impl Plugin for SelectedPlanetUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Duplicate>().add_systems(
            Update,
            (
                check_selection,
                display_selected_planet_window,
                add_new_planet.run_if(run_if_add_new_planet),
                duplicate_planet.run_if(run_if_duplicate_planet),
            ),
        );
    }
}

/// Clears the selected planet.
fn clear_selection(
    commands: &mut Commands,
    selected: Result<(Entity, Mut<CelestialBodyData>, &Transform), query::QuerySingleError>,
) {
    if let Ok((e, _, _)) = selected {
        commands
            .get_entity(e)
            .unwrap()
            .remove::<SelectedPlanetMarker>();
    }
}

/// Checks if the user has clicked on a planet and selects it.
#[allow(clippy::type_complexity)]
fn check_selection(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut CelestialBodyData, &Transform),
        (With<CelestialBodyData>, Without<SelectedPlanetMarker>),
    >,
    mut query_selected: Query<
        (Entity, &mut CelestialBodyData, &Transform),
        With<SelectedPlanetMarker>,
    >,
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
        //clear_selection(&mut commands, query_selected.get_single_mut());
        return;
    };

    for (e, planet, transform) in &mut query {
        let l = ray.origin - transform.translation;
        if ray.direction.dot(l).abs() < 0. {
            continue;
        }

        let h = ray.origin
            + ray.direction.dot(l) * ray.direction.as_dvec3().as_vec3()
                / (ray.direction.length() * ray.direction.length());

        let d = (l.length_squared() - (h - ray.origin).length_squared()).sqrt();

        if d < planet.radius {
            clear_selection(&mut commands, query_selected.get_single_mut());
            commands.get_entity(e).unwrap().insert(SelectedPlanetMarker);
            return;
        }
    }
    //clear_selection(&mut commands, query_selected.get_single_mut());
}

/// Displays the selected planet's data in a floating window.
fn display_selected_planet_window(
    mut duplicate: ResMut<Duplicate>,
    mut contexts: EguiContexts,
    mut query_selected: Query<(&mut CelestialBodyData, &mut Transform), With<SelectedPlanetMarker>>,
    mut gizmos: Gizmos,
) {
    // show selection by drawing unit vectors on the selection
    for (body_data, transform) in &query_selected {
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

    // selection window
    if let Ok((mut planet, mut tfm)) = query_selected.get_single_mut() {
        egui::Window::new(planet.name.clone()).show(contexts.ctx_mut(), |ui| {
            if ui.button("Duplicate").clicked() {
                duplicate.0 = true;
            }

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut planet.radius));
                ui.label("Radius");
            });

            ui.horizontal(|ui| {
                let speed = (planet.mass / 10.0 + 1.0).abs();
                ui.add(egui::DragValue::new(&mut planet.mass).speed(speed));
                ui.label("Mass");
            });

            ui.add(egui::Slider::new(&mut planet.color[0], 0.0_f32..=1.0_f32).text("Red"));
            ui.add(egui::Slider::new(&mut planet.color[1], 0.0_f32..=1.0_f32).text("Green"));
            ui.add(egui::Slider::new(&mut planet.color[2], 0.0_f32..=1.0_f32).text("Blue"));

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut tfm.translation.x));
                ui.add(egui::DragValue::new(&mut tfm.translation.y));
                ui.add(egui::DragValue::new(&mut tfm.translation.z));
                ui.label("Position");
            });

            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut planet.velocity.x));
                ui.add(egui::DragValue::new(&mut planet.velocity.y));
                ui.add(egui::DragValue::new(&mut planet.velocity.z));
                ui.label("Velocity");
            })
        });
    }
}

fn run_if_add_new_planet(app_config: Res<AppConfig>) -> bool {
    app_config.add_new_planet
}

fn add_new_planet(
    mut app_config: ResMut<AppConfig>,
    mut commands: Commands,
    query: Query<(&Transform, &CelestialBodyData)>,
    mut query_selected: Query<Entity, With<SelectedPlanetMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Ok(entity) = query_selected.get_single_mut() {
        commands.entity(entity).remove::<SelectedPlanetMarker>();
    }

    let color = Color::rgb_from_array([
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    ]);
    let material = materials.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });

    app_config.add_new_planet = false;

    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

    let mut radius = 0.;
    let mut mass = 0.;
    let mut position = Vec3::ZERO;
    let mut velocity = Vec3::ZERO;
    for (tfm, data) in &query {
        radius += data.radius;
        mass += data.mass;
        position += tfm.translation;
        velocity += data.velocity;
    }

    let cnt = query.iter().count() as f32;

    radius /= cnt;
    mass /= cnt;
    position /= cnt;
    velocity /= cnt;
    commands.spawn((
        CelestialBodyBundle {
            pbr: PbrBundle {
                mesh,
                material,
                transform: Transform {
                    translation: position
                        + if cnt == 1.0 {
                            Vec3::ONE * radius
                        } else {
                            Vec3::ZERO
                        },
                    scale: Vec3::ONE * radius,
                    ..default()
                },
                ..Default::default()
            },

            body_data: CelestialBodyData::new(
                String::from("New Planet"),
                CelestialBodyType::Planet,
                mass,
                radius,
                velocity,
                color,
            ),
        },
        SelectedPlanetMarker,
    ));
}

fn run_if_duplicate_planet(duplicate: Res<Duplicate>) -> bool {
    duplicate.0
}

fn duplicate_planet(
    mut duplicate: ResMut<Duplicate>,
    mut commands: Commands,
    selected_query: Query<
        (&Transform, &Handle<Mesh>, &CelestialBodyData),
        With<SelectedPlanetMarker>,
    >,
    mut selected_entity_query: Query<Entity, With<SelectedPlanetMarker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = Color::rgb_from_array([
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    ]);
    let material = materials.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });

    duplicate.0 = false;

    if let Ok((tfm, mesh, data)) = selected_query.get_single() {
        commands.spawn((
            CelestialBodyBundle {
                pbr: PbrBundle {
                    mesh: mesh.clone(),
                    material,
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
            SelectedPlanetMarker,
        ));
    }

    // since we're in duplicate, this should always be Ok(_)
    if let Ok(entity) = selected_entity_query.get_single_mut() {
        commands.entity(entity).remove::<SelectedPlanetMarker>();
    }
}
