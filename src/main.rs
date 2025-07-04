mod map;
mod player;
mod raycaster;
mod render_sprites;
mod textures;

use crate::{map::*, player::*, raycaster::*, render_sprites::*, textures::*}; // <-- импорт функции render_sprites
use macroquad::prelude::*;
use std::collections::HashMap;

#[macroquad::main("OpenWater")]
async fn main() {
    let textures = load_textures().await;

    let mut player = Player::new(5.0 * TILE_SIZE, 5.0 * TILE_SIZE, 0.0);

    let sprites = map::get_sprites(); // <- получаем спрайты ДО цикла

    loop {
        player.update();
        clear_background(BLACK);

        render_3d(&player, &textures);
        draw_minimap(&player);

        render_sprites(player.x, player.y, player.angle, &sprites, &textures);

        next_frame().await;
    }
}
