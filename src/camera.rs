pub(crate) mod camera_controller;
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use camera_controller::{CameraController, CameraControllerPlugin};

#[derive(Component)]
/// Marker component for the main camera.
pub struct MainCamera;

/// Plugin responsible for setting up the camera.
pub struct CustomCameraPlugin;
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraControllerPlugin)
            .add_systems(Startup, setup_camera)
            //.add_systems(Update, draw_cursor)
            ;
    }
}

/// Setup the camera in a 3D scene.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(0.0, 500.0, 0.0).looking_at(Vec3::ZERO, -Vec3::Z),

            ..default()
        },
        // 3. Enable bloom for the camera
        BloomSettings::NATURAL,
        MainCamera,
        CameraController::default(),
    ));
}
