use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

use space4x::scene::game::GameScenePlugin;
use space4x::utils::UtilsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(UtilsPlugin)
        .add_plugins(GameScenePlugin)
        .run();
}
