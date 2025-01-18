//! Iterates and prints gamepad input and connection events.
#![feature(lazy_cell)]

use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonInput,
        GamepadConnectionEvent, GamepadEvent,
    },
    log,
    prelude::*,
};
use ctru::applets::error;
use ctru::services::gfx::Gfx;
use ctru::prelude::Console;


fn main() {
    error::set_panic_hook(false);

    App::new()
        // Add default bevy_3ds plugins
        .add_plugins((
            bevy_3ds::DefaultPlugins
                // Configure logging to debug level
                .set(log::LogPlugin {
                    level: log::Level::DEBUG,
                    ..default()
                }),
        ))
        .add_systems(Update, (exit_on_start, gamepad_events, gamepad_ordered_events).chain())
        .run();

}

fn gamepad_events(
    mut connection_events: EventReader<GamepadConnectionEvent>,
    mut axis_changed_events: EventReader<GamepadAxisChangedEvent>,
    // Handles the continuous measure of how far a button has been pressed down, as measured
    // by `Axis<GamepadButton>`. Whenever that value changes, this event is emitted.
    mut button_changed_events: EventReader<GamepadButtonChangedEvent>,
    // Handles the boolean measure of whether a button is considered pressed or unpressed, as
    // defined by the thresholds in `GamepadSettings::button_settings` and measured by
    // `Input<GamepadButton>`. When the threshold is crossed and the button state changes,
    // this event is emitted.
    mut button_input_events: EventReader<GamepadButtonInput>,
) {

    for connection_event in connection_events.read() {
        info!("{:?}", connection_event);
    }

    for axis_changed_event in axis_changed_events.read() {
        info!(
            "{:?} of {:?} is changed to {}",
            axis_changed_event.axis_type, axis_changed_event.gamepad, axis_changed_event.value
        );
    }
    for button_changed_event in button_changed_events.read() {
        info!(
            "{:?} of {:?} is changed to {}",
            button_changed_event.button_type,
            button_changed_event.gamepad,
            button_changed_event.value
        );
    }
    for button_input_event in button_input_events.read() {
        info!("{:?}", button_input_event);
    }
}

// If you require in-frame relative event ordering, you can also read the `Gamepad` event
// stream directly. For standard use-cases, reading the events individually or using the
// `Input<T>` or `Axis<T>` resources is preferable.
fn gamepad_ordered_events(mut gamepad_events: EventReader<GamepadEvent>, gfx: NonSend<Gfx>) {
    let _console = Console::new(gfx.bottom_screen.borrow_mut());

    for gamepad_event in gamepad_events.read() {
        match gamepad_event {
            GamepadEvent::Connection(connection_event) => info!("{:?}", connection_event),
            GamepadEvent::Button(button_event) => info!("{:?}", button_event),
            GamepadEvent::Axis(axis_event) => info!("{:?}", axis_event),
        }
    }
}

fn exit_on_start(button_inputs: Res<ButtonInput<GamepadButton>>, mut exit_events: EventWriter<AppExit>) {
    if button_inputs.just_pressed(GamepadButton::new(bevy_3ds::input::GAMEPAD, GamepadButtonType::Start)) {
        exit_events.send(AppExit::Success);
    }
}