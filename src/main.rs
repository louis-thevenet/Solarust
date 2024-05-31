use bevy::{prelude::*, render::camera::CameraPlugin};
use camera::camera_plugin::CustomCameraPlugin;
use ground::setup_ground;
mod camera;
mod ground;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CustomCameraPlugin)
        .add_systems(Startup, (setup_ground))
        .run();
}
