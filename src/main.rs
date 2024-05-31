use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use camera::camera_plugin::CustomCameraPlugin;
use ground::setup_ground;
mod camera;
mod ground;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CustomCameraPlugin)
        .add_systems(Startup, setup_ground)
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
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let planet = meshes.add(Sphere::default().mesh().ico(5).unwrap());
    //let planet2 = meshes.add(Sphere::default().mesh().uv(32, 18));

    commands.spawn((
        PbrBundle {
            mesh: planet.clone(),
            material: debug_material.clone(),
            transform: Transform::from_xyz(-5.0, 4.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..Default::default()
        },
        SpatialBody,
    ));

    commands.spawn((
        PbrBundle {
            mesh: planet,
            material: debug_material.clone(),
            transform: Transform::from_xyz(5.0, 4.0, 0.0),
            ..Default::default()
        },
        SpatialBody,
    ));
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
