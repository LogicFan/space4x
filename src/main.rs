use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use space4x::scene::galaxy::solar::init;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, init)
        .run();
}
