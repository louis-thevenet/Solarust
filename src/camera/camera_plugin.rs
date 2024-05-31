use bevy::prelude::*;

use crate::ground::Ground;

use super::camera_controller_plugin::{CameraController, CameraControllerPlugin};

#[derive(Component)]
pub struct MainCamera;

pub struct CustomCameraPlugin;
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraControllerPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, draw_cursor);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
        CameraController::default(),
    ));
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation(), Plane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        point + ground.up() * 0.01,
        Direction3d::new_unchecked(ground.up()), // Up vector is already normalized.
        0.2,
        Color::WHITE,
    );
}
