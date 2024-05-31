use bevy::prelude::*;
use camera::camera_plugin::CustomCameraPlugin;
use planets::planet_plugin::PlanetPlugin;
mod camera;
mod planets;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CustomCameraPlugin, PlanetPlugin))
        .run();
}
