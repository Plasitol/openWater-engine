use macroquad::prelude::*;
use std::collections::HashMap;

pub async fn load_textures() -> HashMap<u8, Texture2D> {
    let mut textures = HashMap::new();
    let brick = load_texture("assets/brick.png").await.unwrap();
    let stone = load_texture("assets/stone.png").await.unwrap();
    let metal = load_texture("assets/metal.png").await.unwrap();

    let tree = load_texture("assets/tree.png").await.unwrap();

    brick.set_filter(FilterMode::Nearest);
    stone.set_filter(FilterMode::Nearest);
    metal.set_filter(FilterMode::Nearest);
    metal.set_filter(FilterMode::Nearest);

    textures.insert(1, brick);
    textures.insert(2, stone);
    textures.insert(3, metal);
    textures.insert(4, tree);

    textures
}
