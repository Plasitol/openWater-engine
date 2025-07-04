use crate::map::Sprite;
use crate::TILE_SIZE;
use macroquad::prelude::*;
use std::collections::HashMap;

pub fn render_sprites(
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    sprites: &Vec<Sprite>,
    textures: &HashMap<u8, Texture2D>,
) {
    for sprite in sprites {
        let dx = sprite.x - player_x;
        let dy = sprite.y - player_y;
        let distance = (dx * dx + dy * dy).sqrt();

        let angle = dy.atan2(dx) - player_angle;
        let fov = std::f32::consts::FRAC_PI_3;

        if angle.abs() < fov / 2.0 {
            let screen_x = (screen_width() / 2.0) * (1.0 + angle / (fov / 2.0));
            let sprite_height = (TILE_SIZE * screen_height()) / distance;
            let sprite_width = sprite_height;

            let texture = textures.get(&sprite.texture_id).unwrap();
            draw_texture_ex(
                texture,
                screen_x - sprite_width / 2.0,
                screen_height() / 2.0 - sprite_height / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(sprite_width, sprite_height)),
                    ..Default::default()
                },
            );
        }
    }
}
// 'NJ GHJCNJ GBPLTW CEEEERFFFFF!
