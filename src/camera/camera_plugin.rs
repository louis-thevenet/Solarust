use bevy::prelude::*;

use super::camera_controller_plugin::{CameraController, CameraControllerPlugin};

#[derive(Component)]
pub struct MainCamera;

pub struct CustomCameraPlugin;
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraControllerPlugin)
            .add_systems(Startup, setup_camera)
            //.add_systems(Update, draw_cursor)
            ;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 500.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),
            ..default()
        },
        MainCamera,
        CameraController::default(),
    ));
}
