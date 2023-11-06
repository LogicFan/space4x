use crate::utils::uid::ObjectId;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

#[derive(Component, Debug, Clone, Copy)]
pub struct SolarSystemMarker;

#[derive(Bundle, Clone)]
pub struct SolarSystemBundle {
    id: ObjectId,
    core: PbrBundle,
    marker: SolarSystemMarker,
}

/// systems below
pub fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let horizontal = Uniform::new(-256.0, 256.0);
    let vertical = Uniform::new(-8.0, 8.0);
    let mut rng = thread_rng();

    for i in 0..4096 {
        let x: f32 = horizontal.sample(&mut rng);
        let y: f32 = horizontal.sample(&mut rng);
        let z: f32 = vertical.sample(&mut rng);

        commands.spawn(SolarSystemBundle {
            id: ObjectId::new_random(),
            marker: SolarSystemMarker,
            core: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 1.0,
                    ..default()
                })),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
        });
    }
}
