use crate::map::TILE_SIZE;
use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }

    pub fn update(&mut self) {
        let move_speed = 100.0 * get_frame_time();
        let rot_speed = 2.0 * get_frame_time();

        if is_key_down(KeyCode::W) {
            self.x += self.angle.cos() * move_speed;
            self.y += self.angle.sin() * move_speed;
        }
        if is_key_down(KeyCode::S) {
            self.x -= self.angle.cos() * move_speed;
            self.y -= self.angle.sin() * move_speed;
        }
        if is_key_down(KeyCode::A) {
            self.angle -= rot_speed;
        }
        if is_key_down(KeyCode::D) {
            self.angle += rot_speed;
        }
    }
}
