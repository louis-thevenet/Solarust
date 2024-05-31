use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use camera::camera_plugin::CustomCameraPlugin;
use planet_bundle::{PlanetBundle, PlanetData};
mod camera;
mod planet_bundle;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CustomCameraPlugin)
        .add_systems(Startup, setup_test)
        .add_systems(FixedUpdate, rotate)
        .run();
}

#[derive(Component)]
struct SpatialBody;

fn rotate(mut query: Query<&mut Transform, With<SpatialBody>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

fn setup_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sun_material = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Mask(0.0),
        ..Default::default()
    });

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let sun_radius = 10.0;
    let planet_radius = sun_radius * 0.3;

    let sun = meshes.add(Sphere::new(sun_radius).mesh().ico(5).unwrap());
    let planet = meshes.add(Sphere::new(planet_radius).mesh().ico(5).unwrap());

    commands.spawn((
        PlanetBundle {
            pbr: PbrBundle {
                mesh: sun,
                material: sun_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            planet_data: PlanetData::new(1., sun_radius, Vec3::ZERO),
        },
        SpatialBody,
    ));

    commands.spawn((
        PlanetBundle {
            pbr: PbrBundle {
                mesh: planet,
                material: debug_material.clone(),
                transform: Transform::from_xyz(100.0, 0.0, 0.0),
                ..Default::default()
            },
            planet_data: PlanetData::new(1., planet_radius, Vec3::ZERO),
        },
        SpatialBody,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
