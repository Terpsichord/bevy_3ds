use bevy::input::gamepad::{GamepadAxisType, GamepadButtonType};
use ctru::services::hid::KeyPad;

pub struct Axis {
    pub axis_type: GamepadAxisType,
    // Either -1.0 or 1.0
    pub sign: f32,
}

pub enum ConvertedKey {
    Axis(Axis),
    Button(GamepadButtonType),
}

pub fn convert_key(key: KeyPad) -> ConvertedKey {
    match key {
        KeyPad::CPAD_UP => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::LeftStickY,
            sign: 1.0,
        }),
        KeyPad::CPAD_DOWN => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::LeftStickY,
            sign: -1.0,
        }),
        KeyPad::CPAD_LEFT => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::LeftStickX,
            sign: -1.0,
        }),
        KeyPad::CPAD_RIGHT => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::LeftStickX,
            sign: 1.0,
        }),
        KeyPad::CSTICK_UP => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::RightStickY,
            sign: 1.0,
        }),
        KeyPad::CSTICK_DOWN => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::RightStickY,
            sign: -1.0,
        }),
        KeyPad::CSTICK_LEFT => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::RightStickX,
            sign: -1.0,
        }),
        KeyPad::CSTICK_RIGHT => ConvertedKey::Axis(Axis {
            axis_type: GamepadAxisType::RightStickX,
            sign: 1.0,
        }),

        KeyPad::A => ConvertedKey::Button(GamepadButtonType::East),
        KeyPad::B => ConvertedKey::Button(GamepadButtonType::South),
        KeyPad::DPAD_DOWN => ConvertedKey::Button(GamepadButtonType::DPadDown),
        KeyPad::DPAD_LEFT => ConvertedKey::Button(GamepadButtonType::DPadLeft),
        KeyPad::DPAD_RIGHT => ConvertedKey::Button(GamepadButtonType::DPadRight),
        KeyPad::DPAD_UP => ConvertedKey::Button(GamepadButtonType::DPadUp),
        KeyPad::L => ConvertedKey::Button(GamepadButtonType::LeftTrigger),
        KeyPad::R => ConvertedKey::Button(GamepadButtonType::RightTrigger),
        KeyPad::SELECT => ConvertedKey::Button(GamepadButtonType::Select),
        KeyPad::START => ConvertedKey::Button(GamepadButtonType::Start),
        KeyPad::X => ConvertedKey::Button(GamepadButtonType::North),
        KeyPad::Y => ConvertedKey::Button(GamepadButtonType::West),
        KeyPad::ZL => ConvertedKey::Button(GamepadButtonType::LeftTrigger2),
        KeyPad::ZR => ConvertedKey::Button(GamepadButtonType::RightTrigger2),

        _ => unreachable!(),
    }

}
