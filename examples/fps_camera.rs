use std::f32::consts::{FRAC_PI_2, TAU};
use bevy::{log, prelude::*};
use ctru::applets::error;
use bevy::input::ButtonInput;
use bevy::input::gamepad::{GamepadButton, GamepadButtonType};
use bevy::hierarchy::BuildChildren;

fn main() {
    error::set_panic_hook(false);

    App::new()
        .add_plugins((bevy_3ds::DefaultPlugins
                          .set(log::LogPlugin {
                              level: log::Level::DEBUG,
                              ..default()
                          }),))
        .add_systems(Startup, (spawn_player, spawn_cubes))
        .add_systems(Update, (exit_on_start, move_player))
        .run();
}

#[derive(Component)]
struct Player {
    move_speed: f32,
    camera_sensitivity: f32,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player {
                move_speed: 4.0,
                camera_sensitivity: 0.2,
            },
            Transform::default(),
        )).with_children(|parent| {
            parent.spawn(bevy_3ds::render::Camera { fov: 45.0 });
        });
}

fn spawn_cubes(mut commands: Commands) {
    let n = 10;
    for i in 0..n {
        let angle = i as f32 * TAU / (n as f32);
        commands.spawn((
            Transform::from_xyz(angle.cos() * 5.0, 0.0, angle.sin() * 5.0),
            bevy_3ds::render::Cube,
        ));
    }
}

fn move_player(mut player: Query<(&mut Transform, &Player)>, axes: Res<Axis<GamepadAxis>>, time: Res<Time>) {
    let Ok((mut transform, player)) = player.get_single_mut() else { return };

    let gamepad = Gamepad { id: 0 };

    let mut movement = Vec3::ZERO;
    let mut yaw = 0.0;
    let mut pitch = 0.0;

    if let Some(lx) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)) {
        movement.x = lx * player.move_speed * time.delta_seconds();
    }
    if let Some(ly) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
        movement.z = ly * player.move_speed * time.delta_seconds();
    }
    if let Some(rx) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)) {
        yaw = rx * player.camera_sensitivity * time.delta_seconds();
    }
    if let Some(ry) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)) {
        pitch = (ry * player.camera_sensitivity * time.delta_seconds()).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    let right = transform.right();
    let forward = transform.forward();
    transform.translation += movement.x * right + movement.z * forward;
    transform.rotate_y(-yaw);
    transform.rotate_x(-pitch);
}

fn exit_on_start(
    button_inputs: Res<ButtonInput<GamepadButton>>,
    mut exit_events: EventWriter<AppExit>,
) {
    if button_inputs.just_pressed(GamepadButton::new(
        bevy_3ds::input::GAMEPAD,
        GamepadButtonType::Start,
    )) {
        exit_events.send(AppExit::Success);
    }
}
