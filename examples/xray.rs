use bevy::prelude::*;
use bevy::render::mesh::shape::Cube;
use bevy_plugin_billboard::{BillboardComponents, BillboardMaterial, BillboardPlugin};

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(BillboardPlugin)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut billboards: ResMut<Assets<BillboardMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/icon.png")
        .unwrap();

    let billboard = billboards.add(BillboardMaterial {
        albedo_texture: Some(texture_handle),
        ..Default::default()
    });

    commands
        .spawn(BillboardComponents {
            billboard,
            translation: Translation::new(0.0, 0.0, 1.5),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..BillboardComponents::xray()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Cube { size: 1.0 }.into()),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(-3.0, 3.0, 4.0),
            ..Default::default()
        })
        .spawn((
            Transform::default(),
            Translation::default(),
            Rotation::default(),
            Scale::default(),
            Rotator,
        ))
        .with_children(|parent| {
            parent.spawn(Camera3dComponents {
                translation: Translation::new(0.0, 0.0, -15.0),
                rotation: Rotation::from_rotation_x(3.14),
                ..Default::default()
            });
        });
}

struct Rotator;

fn rotator_system(time: Res<Time>, mut query: Query<(&Rotator, &mut Rotation)>) {
    for (_rotator, mut rotation) in &mut query.iter() {
        rotation.0 *= Quat::from_rotation_y(1.0 * time.delta_seconds);
    }
}
