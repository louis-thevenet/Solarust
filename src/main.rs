use bevy::prelude::*;
use camera::camera_plugin::CustomCameraPlugin;
use planets::planet_plugin::PlanetPlugin;
use ui::UIPlugin;
mod camera;
mod planets;
mod ui;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UIPlugin, CustomCameraPlugin, PlanetPlugin))
        .run();
}
