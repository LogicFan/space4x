pub mod cursor;
pub mod state;
pub mod uid;

use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<state::GameState>();

        app.add_event::<cursor::LockCursor>()
            .insert_resource(cursor::CursorLockState::default())
            .add_systems(PreUpdate, cursor::preupdate_cursor_lock);
    }
}
