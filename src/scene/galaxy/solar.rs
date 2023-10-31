use crate::utils::uid::ObjectId;
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

#[derive(Component)]
pub struct SolarSystemMarker;

#[derive(Bundle)]
pub struct SolarSystemBundle {
    id: ObjectId,
    core: PbrBundle,
    marker: SolarSystemMarker,
}

pub fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let between = Uniform::new(-256.0, 256.0);
    let mut rng = thread_rng();

    for i in 0..4096 {
        let x: f32 = between.sample(&mut rng);
        let y: f32 = between.sample(&mut rng);

        commands.spawn(SolarSystemBundle {
            id: ObjectId::new_random(),
            marker: SolarSystemMarker,
            core: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
        });
    }
}
