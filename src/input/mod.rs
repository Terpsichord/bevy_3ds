//! Input handling for the 3DS. The device is treated as a single gamepad and
//! sends native Bevy gamepad events for use with Bevy's [`InputSystem`].

use crate::input::converter::{convert_key, ConvertedKey};
use bevy::input::gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnection, GamepadConnectionEvent,
    GamepadEvent, GamepadInfo,
};
use bevy::input::touch::{TouchInput, TouchPhase};
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
                (input_update_system).before(bevy::input::InputSystem),
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

fn input_update_system(mut hid: NonSendMut<GamepadInput>, mut gamepad_events: EventWriter<GamepadEvent>, mut touch_events: EventWriter<TouchInput>) {
    hid.0.scan_input();

    for (keys, pressed) in [(hid.0.keys_down(), true), (hid.0.keys_up(), false)] {
        for key in keys {
            if key == KeyPad::TOUCH {
                let position = hid.0.touch_position();
                let phase = if pressed { TouchPhase::Started } else { TouchPhase::Ended };
                touch_events.send(create_touch_input(position, phase));
            } else {
                gamepad_events.send(create_gamepad_event(GAMEPAD, key, pressed));
            }
        }
    }

    // Touch moved
    if hid.0.keys_held().contains(KeyPad::TOUCH) & !hid.0.keys_down().contains(KeyPad::TOUCH) {
        touch_events.send(create_touch_input(hid.0.touch_position(), TouchPhase::Moved));
    }
}

fn create_gamepad_event(gamepad: Gamepad, key: KeyPad, pressed: bool) -> GamepadEvent {
    match convert_key(key) {
        ConvertedKey::Axis(axis) => {
            let value = if pressed { axis.sign } else { 0.0 };
            GamepadEvent::Axis(GamepadAxisChangedEvent::new(
                gamepad,
                axis.axis_type,
                value,
            ))
        }
        ConvertedKey::Button(button) => {
            // We don't get analog button input, so use "binary" values for press/unpress values.
            let value = if pressed { 1.0 } else { 0.0 };
            GamepadEvent::Button(GamepadButtonChangedEvent::new(
                gamepad, button, value,
            ))
        }
    }
}


fn create_touch_input(position: (u16, u16), phase: TouchPhase) -> TouchInput {
    TouchInput {
        phase,
        position: Vec2::new(position.0 as f32, position.1 as f32),
        window: Entity::PLACEHOLDER,
        force: None,
        id: 0,
    }
}
