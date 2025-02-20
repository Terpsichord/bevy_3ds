use std::f32::consts::PI;
use ctru::services::hid::{Hid, KeyPad};
use citro3d::math::Matrix4;


pub struct Player {
    pos_x: f32,
    pos_z: f32,
    yaw: f32,
    pitch: f32,
    move_speed: f32,
    turn_speed: f32,
}

impl Player {
    pub fn new(move_speed: f32, turn_speed: f32) -> Self {
        Player { pos_x: 0.0, pos_z: 0.0, yaw: 0.0, pitch: 0.0, move_speed, turn_speed}
    }

    pub fn calculate_view(&self) -> Matrix4 {
        let mut view = Matrix4::identity();

        view.translate(-self.pos_x, 0.0, -self.pos_z);
        view.rotate_y(self.yaw);
        view.rotate_x(self.pitch);

        view
    }

    pub fn update(&mut self, hid: &Hid) {
        if hid.keys_held().contains(KeyPad::CPAD_RIGHT) {
            self.pos_x += self.yaw.cos() * self.move_speed;
            self.pos_z += self.yaw.sin() * self.move_speed;
        }

        if hid.keys_held().contains(KeyPad::CPAD_LEFT) {
            self.pos_x -= self.yaw.cos() * self.move_speed;
            self.pos_z -= self.yaw.sin() * self.move_speed;
        }

        if hid.keys_held().contains(KeyPad::CPAD_DOWN) {
            self.pos_x -= self.yaw.sin() * self.move_speed;
            self.pos_z += self.yaw.cos() * self.move_speed;
        }

        if hid.keys_held().contains(KeyPad::CPAD_UP) {
            self.pos_x += self.yaw.sin() * self.move_speed;
            self.pos_z -= self.yaw.cos() * self.move_speed;
        }

        if hid.keys_held().contains(KeyPad::CSTICK_RIGHT) {
            self.yaw += self.turn_speed.to_radians();
            if self.yaw > 2.0 * PI {
                self.yaw -= 2.0 * PI;
            }
        }

        if hid.keys_held().contains(KeyPad::CSTICK_LEFT) {
            self.yaw -= self.turn_speed.to_radians();
            if self.yaw < 0.0 {
                self.yaw += 2.0 * PI;
            }
        }
    }
}
