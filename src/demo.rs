use core::f32::consts::PI;

use libm::{cos, cosf, sinf};
use num_traits::ToPrimitive;

use crate::joystick::JoystickReport;
#[derive(Default)]
pub struct Rotator {
    angle: u32
}

const RADIUS: f32 = 110.0;

impl Iterator for Rotator {
    type Item = JoystickReport;

    fn next(&mut self) -> Option<Self::Item> {
        let angel_rad = self.angle.to_f32().unwrap() * PI / 180.0;
        let x = RADIUS * cosf(angel_rad);
        let y = RADIUS * sinf(angel_rad);

        self.angle += 1;
        Some(
            JoystickReport { x: x.to_i8().unwrap(), y: y.to_i8().unwrap(), rx: 0, ry: 0, buttons: 0 }
        )
    }
}