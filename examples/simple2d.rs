use bevoids::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BoidsPlugin))
        .insert_resource(BoidsConfig {
            space: BoidSpace::TwoDimensional,
            debug: true,
        })
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for x in -10..10 {
        for y in -10..10 {
            let spacer = 20.0;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(10.0, 3).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_xyz(x as f32 * spacer, y as f32 * spacer, 0.0),
                    ..default()
                },
                Boid::new(
                    0.5,
                    BoidTurningConfig::new(1.0, 1.0, 1.0),
                    BoidViewConfig::new(90.0, 60.0),
                ),
            ));
        }
    }
}
