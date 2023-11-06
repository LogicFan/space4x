pub mod camera;
pub mod solar;

use crate::GameState;
use bevy::prelude::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<camera::CameraLostFocus>();
        app.add_systems(OnEnter(GameState::Game), (solar::init, camera::spawn))
            .add_systems(
                Update,
                camera::control_camera.run_if(in_state(GameState::Game)),
            );
    }
}
