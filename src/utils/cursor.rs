use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource)]
pub struct CursorLockState {
    position: Option<Vec2>,
}

impl Default for CursorLockState {
    fn default() -> Self {
        Self { position: None }
    }
}

impl CursorLockState {
    pub fn is_locked(&self) -> bool {
        self.position.is_some()
    }

    pub fn lock_position(&self) -> Option<Vec2> {
        self.position
    }
}

#[derive(Event)]
pub struct LockCursor(pub bool);

pub fn preupdate_cursor_lock(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut lock: ResMut<CursorLockState>,
    mut events: EventReader<LockCursor>,
) {
    match events.iter().last() {
        // no event, do nothing
        None => (),
        Some(LockCursor(is_locked)) => {
            match window.single().cursor_position() {
                // cursor is not in the window, do nothing
                None => (),
                Some(position) => {
                    if *is_locked {
                        // lock the cursor
                        lock.position = Some(position)
                    } else {
                        // unlock the cursor
                        lock.position = None
                    }
                }
            }
        }
    }

    if lock.is_locked() {
        window.single_mut().set_cursor_position(lock.position);
    }
}
