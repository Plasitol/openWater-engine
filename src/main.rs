mod map;
mod player;
mod raycaster;
mod textures;

use crate::map::*;
use crate::player::*;
use crate::raycaster::*;
use crate::textures::*;

use macroquad::prelude::*;
use std::collections::HashMap;

#[macroquad::main("OpenWater")]
async fn main() {
    let textures = load_textures().await;

    let mut player = Player::new(3.0 * TILE_SIZE, 3.0 * TILE_SIZE, 0.0);

    loop {
        player.update();
        clear_background(BLACK);

        render_3d(&player, &textures);
        draw_minimap(&player);

        next_frame().await;
    }
}
