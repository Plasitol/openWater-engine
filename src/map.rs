pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 10;
pub const TILE_SIZE: f32 = 64.0;

pub const MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 2, 0, 0, 3, 3, 0, 1],
    [1, 0, 2, 2, 0, 0, 3, 3, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 1, 0, 0, 3, 3, 0, 1],
    [1, 0, 2, 1, 0, 0, 3, 3, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub texture_id: u8,
}

pub fn get_sprites() -> Vec<Sprite> {
    vec![Sprite {
        x: 5.0 * TILE_SIZE,
        y: 5.0 * TILE_SIZE,
        texture_id: 4,
    }]
}

//я ебал спрайты че за хуйня
