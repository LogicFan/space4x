use crate::utils::cursor::{CursorLockState, LockCursor};
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

/// The marker to indicate a entity is a main camera.
#[derive(Component)]
pub struct PrimaryCameraMarker;

/// Indicate which control mode the camera is in.
#[derive(Component)]
pub enum CameraControlMode {
    FreeMotion,
    Focus(Entity),
}

/// The bundle of the main camera. This camera serve dual purpose: used for
/// galaxy scene and for solar scene, differentiated by render layer.
/// # Invariant
/// - there should be at most 1 main camera in the world.
#[derive(Bundle)]
pub struct PrimaryCameraBundle {
    camera: Camera3dBundle,
    control: CameraControlMode,
    marker: PrimaryCameraMarker,
}

/// Spawn the primary camera in the world. This should only be called if there
/// is no primary camera existed already.
pub fn spawn(mut commands: Commands) {
    let camera = PrimaryCameraBundle {
        camera: Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 64.0),
            ..default()
        },
        control: CameraControlMode::FreeMotion,
        marker: PrimaryCameraMarker,
    };
    commands.spawn(camera);
}

/// Despawn the primary camera in the world.
pub fn despawn(mut commands: Commands, cameras: Query<Entity, With<PrimaryCameraMarker>>) {
    cameras.for_each(|camera| {
        commands.entity(camera).despawn();
    })
}

#[derive(Event)]
/// A event indicate camera has lost it's focus mode.
pub struct CameraLostFocus;

pub fn control_camera(
    mut cameras: Query<(&mut Transform, &CameraControlMode), With<PrimaryCameraMarker>>,
    mut mouse_scroll: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_button: Res<Input<MouseButton>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_state: Res<CursorLockState>,
    mut lost_focus: EventWriter<CameraLostFocus>,
    mut lock_cursor: EventWriter<LockCursor>,
) {
    cameras.for_each_mut(|camera| {
        let (mut transform, control_mode) = camera;

        let local_x = transform.local_x();
        let local_y = transform.local_y();
        let local_z = transform.local_z();

        // move camera forward and backward, TODO: support input config
        let dz = mouse_scroll
            .iter()
            .map(|e| match e.unit {
                MouseScrollUnit::Line => 150.0 * e.y,
                MouseScrollUnit::Pixel => e.y,
            })
            .reduce(|l, r| l + r)
            .unwrap_or(0.0);
        transform.translation -= local_z * dz;

        // move camera sideway
        match window.single().cursor_position() {
            None => return,
            Some(cursor) => {
                let width = window.single().width();
                let dx = if cursor.x < 0.01 * width {
                    -1.0
                } else if cursor.x > 0.99 * width {
                    1.0
                } else {
                    0.0
                };

                let height = window.single().height();
                let dy = if cursor.y < 0.01 * height {
                    1.0
                } else if cursor.y > 0.99 * height {
                    -1.0
                } else {
                    0.0
                };

                transform.translation += local_x * dx;
                transform.translation += local_y * dy;

                // skip the rotation and lost focus if we are moving horizontally.
                if dx != 0.0 || dy != 0.0 {
                    match control_mode {
                        CameraControlMode::Focus(_) => lost_focus.send(CameraLostFocus),
                        _ => (),
                    }
                    return;
                }
            }
        }

        // rotate camera
        // left hold and right hold has higher priority.
        if mouse_button.pressed(MouseButton::Left) {
            return;
        }
        if mouse_button.pressed(MouseButton::Right) {
            return;
        }

        if mouse_button.pressed(MouseButton::Middle) {
            if cursor_state.is_locked() {
                let angle = mouse_motion
                    .iter()
                    .map(|e| e.delta)
                    .reduce(|l, r| l + r)
                    .unwrap_or(Vec2::ZERO)
                    * 0.01;

                match control_mode {
                    CameraControlMode::FreeMotion => {
                        transform.rotate_local_x(angle.y);
                        transform.rotate_local_y(angle.x);
                    }
                    CameraControlMode::Focus(focal) => {
                        unimplemented!();
                    }
                }
            } else {
                lock_cursor.send(LockCursor(true));
            }
        } else {
            // release the cursor
            if cursor_state.is_locked() {
                lock_cursor.send(LockCursor(false));
            }
        }
    })
}
