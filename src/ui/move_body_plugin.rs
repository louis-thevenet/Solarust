use std::f32::consts::PI;

use bevy::prelude::Visibility::{Hidden, Visible};
use bevy::prelude::*;

use crate::planets::planet_bundle::CelestialBodyData;
use crate::ui::planet_ui_plugin::SelectedPlanetMarker;

#[derive(Component)]
struct ArrowMarker;

pub struct MoveBodyUiPlugin;

impl Plugin for MoveBodyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_move_ui)
            .add_systems(Update, draw_move_ui);
    }
}

fn build_move_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arrow_mesh = meshes.add(Cuboid::new(10.0, 1.0, 1.0).mesh());
    let arrow_mat = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..default()
    });

    // -> Y
    commands.spawn((
        PbrBundle {
            mesh: arrow_mesh.clone(),
            material: arrow_mat.clone(),
            visibility: Visibility::Hidden,
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, PI / 2.)),
            ..default()
        },
        ArrowMarker,
    ));

    // -> X
    commands.spawn((
        PbrBundle {
            mesh: arrow_mesh.clone(),
            material: arrow_mat.clone(),
            visibility: Visibility::Hidden,
            transform: default(),

            ..default()
        },
        ArrowMarker,
    ));

    // -> Z
    commands.spawn((
        PbrBundle {
            mesh: arrow_mesh.clone(),
            material: arrow_mat.clone(),
            visibility: Visibility::Hidden,
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.)),
            ..default()
        },
        ArrowMarker,
    ));
}

#[allow(clippy::type_complexity)]
fn draw_move_ui(
    selected_query: Query<
        (&Transform, &CelestialBodyData),
        (With<SelectedPlanetMarker>, Without<ArrowMarker>),
    >,
    mut arrow_query: Query<(&mut Transform, &mut Visibility), With<ArrowMarker>>,
) {
    match selected_query.get_single() {
        Ok((tfm, body_data)) => {
            for (mut tfm_arrow, mut visibility) in &mut arrow_query {
                tfm_arrow.translation = tfm.translation
                    + body_data.radius * tfm_arrow.rotation.mul_vec3(Vec3::X).normalize_or_zero();
                println!("make visible on {}", tfm_arrow.translation);
                *visibility = Visible;
            }
        }
        Err(_) => {
            for (_, mut visibility) in &mut arrow_query {
                *visibility = Hidden;
            }
        }
    }
}
