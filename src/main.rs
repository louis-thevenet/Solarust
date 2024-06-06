use bevy::prelude::*;
use camera::CustomCameraPlugin;
use planets::PlanetPlugin;
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
