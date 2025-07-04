use crate::map::{MAP, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
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

        let mut new_x = self.x;
        let mut new_y = self.y;

        if is_key_down(KeyCode::W) {
            new_x += self.angle.cos() * move_speed;
            new_y += self.angle.sin() * move_speed;
        }
        if is_key_down(KeyCode::S) {
            new_x -= self.angle.cos() * move_speed;
            new_y -= self.angle.sin() * move_speed;
        }
        if is_key_down(KeyCode::A) {
            self.angle -= rot_speed;
        }
        if is_key_down(KeyCode::D) {
            self.angle += rot_speed;
        }

        // коллизия запомни
        let tile_x = (new_x / TILE_SIZE) as usize;
        let tile_y = (self.y / TILE_SIZE) as usize;
        if MAP[tile_y][tile_x] == 0 {
            self.x = new_x;
        }

        let tile_x = (self.x / TILE_SIZE) as usize;
        let tile_y = (new_y / TILE_SIZE) as usize;
        if MAP[tile_y][tile_x] == 0 {
            self.y = new_y;
        }
    }
}
