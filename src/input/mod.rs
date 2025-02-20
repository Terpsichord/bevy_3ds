//! Input handling for the 3DS. The device is treated as a single gamepad and
//! sends native Bevy gamepad events for use with Bevy's [`InputSystem`].

use crate::input::converter::{convert_key, ConvertedKey};
use bevy::input::gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnection, GamepadConnectionEvent,
    GamepadEvent, GamepadInfo,
};
use bevy::prelude::*;
use ctru::services::hid::{Hid, KeyPad};

/// There is only one "gamepad" on the 3DS, so its ID is always zero.
pub const GAMEPAD: Gamepad = Gamepad { id: 0 };

mod converter;

#[derive(Default)]
pub struct InputPlugin;

struct GamepadInput(Hid);

impl Default for GamepadInput {
    fn default() -> Self {
        let hid = Hid::new().expect("failed to init HID");
        Self(hid)
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<GamepadInput>()
            // TODO: reread https://bevyengine.org/news/bevy-0-10/#ecs-schedule-v3
            // and make sure these are scheduled correctly
            .add_systems(PreStartup, gamepad_startup_system)
            .add_systems(
                PreUpdate,
                gamepad_update_system.before(bevy::input::InputSystem),
            );
    }
}

fn gamepad_startup_system(mut events: EventWriter<GamepadEvent>) {
    events.send(
        GamepadConnectionEvent::new(
            GAMEPAD,
            GamepadConnection::Connected(GamepadInfo {
                name: String::from("3DS Gamepad"),
            }),
        )
        .into(),
    );
}

fn gamepad_update_system(mut hid: NonSendMut<GamepadInput>, mut events: EventWriter<GamepadEvent>) {
    hid.0.scan_input();

    for key in hid.0.keys_down().iter() {
        if let Some(event) = create_input_event(GAMEPAD, key, true) {
            events.send(event);
        }
    }

    // TODO: should we even bother sending events for keys_held() ?
    for key in hid.0.keys_held().iter() {
        if let Some(event) = create_input_event(GAMEPAD, key, true) {
            events.send(event);
        }
    }

    for key in hid.0.keys_up() {
        if let Some(event) = create_input_event(GAMEPAD, key, false) {
            events.send(event);
        }
    }
}

fn create_input_event(gamepad: Gamepad, key: KeyPad, pressed: bool) -> Option<GamepadEvent> {
    match convert_key(key) {
        ConvertedKey::Axis(axis) => {
            let value = if pressed { axis.sign } else { 0.0 };
            Some(GamepadEvent::Axis(GamepadAxisChangedEvent::new(
                gamepad,
                axis.axis_type,
                value,
            )))
        }
        ConvertedKey::Button(button) => {
            // We don't get analog button input, so use "binary" values for press/unpress values.
            let value = if pressed { 1.0 } else { 0.0 };
            Some(GamepadEvent::Button(GamepadButtonChangedEvent::new(
                gamepad, button, value,
            )))
        }
    }
}
