use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::GameState;

pub const DEFAULT_CAMERA_SENSITIVITY: f32 = 6.0;
pub const KEYBOARD_ACCELERATION: f32 = 200.0;
pub const MAX_SPEED: f32 = 0.5;

#[derive(Default, Component)]
pub struct PlayerController {
    yaw: f32,
    pitch: f32,
    velocity: Vec3,
}

/// TODO: find a way to default to Locked mode
fn handle_mouse_grab_mode(
    input: Res<Input<KeyCode>>,
    // The primary window is a marker component for determining which window is currently active
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = primary_window.single_mut();

    if input.just_pressed(KeyCode::Escape) {
        let updated_grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
            CursorGrabMode::None => CursorGrabMode::Locked,
        };

        // Locked not available on windows, Confined not available on osx, treat them as the same
        match updated_grab_mode {
            CursorGrabMode::Locked | CursorGrabMode::Confined => window.cursor.visible = false,
            _ => (),
        };
        window.cursor.grab_mode = updated_grab_mode;
    }
}

fn handle_player_keyboard(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerController, &mut Transform)>,
) {
    let (mut controller, mut transform) = query.single_mut();

    let x_movement = match (input.pressed(KeyCode::A), input.pressed(KeyCode::D)) {
        (true, false) => -1.,
        (false, true) => 1.,
        _ => 0.,
    };

    let z_movement = match (input.pressed(KeyCode::W), input.pressed(KeyCode::S)) {
        (true, false) => -1.,
        (false, true) => 1.,
        _ => 0.,
    };

    let y_movement = match (
        input.pressed(KeyCode::LShift),
        input.pressed(KeyCode::Space),
    ) {
        (true, false) => -1.,
        (false, true) => 1.,
        _ => 0.,
    };

    let (axis_h, axis_v, axis_float) = (x_movement, z_movement, y_movement);
    let rotation = transform.rotation;

    let forward_vector = rotation.mul_vec3(Vec3::Z).normalize();
    let forward_plane_vector = Vec3::new(forward_vector.x, 0., forward_vector.z).normalize();
    let strafe_vector = Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_plane_vector)
        .normalize();

    let accel: Vec3 =
        (strafe_vector * axis_h) + (forward_plane_vector * axis_v) + (Vec3::Y * axis_float);

    let accel = if accel.length() != 0. {
        accel.normalize()
    } else {
        Vec3::ZERO
    };

    let acceleration = match input.pressed(KeyCode::LControl) {
        true => 8.0 * KEYBOARD_ACCELERATION,
        false => KEYBOARD_ACCELERATION,
    };

    controller.velocity = acceleration * accel * time.delta_seconds();
    controller.velocity = match controller.velocity.length() > MAX_SPEED {
        true => controller.velocity.normalize() * MAX_SPEED,
        false => controller.velocity,
    };

    transform.translation += controller.velocity;
}

fn handle_player_mouse(
    time: Res<Time>,
    mut mouse_motion_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut PlayerController, &mut Transform)>,
) {
    let (mut controller, mut transform) = query.single_mut();

    let delta: Vec2 = mouse_motion_reader.iter().map(|event| event.delta).sum();
    if delta.is_nan() {
        return;
    }

    controller.yaw -= delta.x * DEFAULT_CAMERA_SENSITIVITY * time.delta_seconds();
    controller.pitch +=
        (delta.y * DEFAULT_CAMERA_SENSITIVITY * time.delta_seconds()).clamp(-89., 89.9);

    transform.rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw.to_radians())
        * Quat::from_axis_angle(-Vec3::X, controller.pitch.to_radians());
}

fn player_plugin_startup(mut commands: Commands) {
    // Spawn camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2., 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                fov: FRAC_PI_2,
                far: 2048.0,
                ..Default::default()
            }),
            ..default()
        })
        .insert(PlayerController::default());
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_plugin_startup.in_schedule(OnEnter(GameState::Running)))
            .add_system(handle_player_keyboard.in_set(OnUpdate(GameState::Running)))
            .add_system(handle_player_mouse.in_set(OnUpdate(GameState::Running)))
            .add_system(handle_mouse_grab_mode.in_set(OnUpdate(GameState::Running)));
    }
}
