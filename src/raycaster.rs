use crate::map::*;
use crate::player::Player;
use macroquad::prelude::*;
use std::collections::HashMap;

pub fn render_3d(player: &Player, textures: &HashMap<u8, Texture2D>) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let fov = std::f32::consts::FRAC_PI_3;
    let num_rays = screen_width as usize;
    let ray_angle_step = fov / num_rays as f32;

    for col in 0..num_rays {
        let ray_angle = player.angle - fov / 2.0 + col as f32 * ray_angle_step;
        let mut distance = 0.0;
        let max_depth = 800.0;
        let mut hit = false;
        let mut texture_x = 0.0;
        let mut tile_value = 1;

        while distance < max_depth && !hit {
            let hit_x = player.x + ray_angle.cos() * distance;
            let hit_y = player.y + ray_angle.sin() * distance;

            let test_x = (hit_x / TILE_SIZE) as usize;
            let test_y = (hit_y / TILE_SIZE) as usize;

            if test_x >= MAP_WIDTH || test_y >= MAP_HEIGHT {
                hit = true;
                break;
            }

            if MAP[test_y][test_x] != 0 {
                hit = true;
                tile_value = MAP[test_y][test_x];
                let hit_cell_x = hit_x % TILE_SIZE;
                let hit_cell_y = hit_y % TILE_SIZE;
                texture_x = if (hit_cell_x - TILE_SIZE).abs() < (hit_cell_y - TILE_SIZE).abs() {
                    hit_cell_x
                } else {
                    hit_cell_y
                };
            }
            distance += 1.0;
        }

        let corrected_distance = distance * (player.angle - ray_angle).cos();
        let wall_height = (screen_height * TILE_SIZE) / corrected_distance;

        let texture = textures.get(&tile_value).unwrap();
        let src_rect = Rect {
            x: (texture_x / TILE_SIZE) * texture.width(),
            y: 0.0,
            w: 1.0,
            h: texture.height(),
        };

        draw_texture_ex(
            texture,
            col as f32,
            (screen_height / 2.0 - wall_height / 2.0).max(0.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1.0, wall_height)),
                source: Some(src_rect),
                ..Default::default()
            },
        );
    }
}

pub fn draw_minimap(player: &Player) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if MAP[y][x] != 0 {
                draw_rectangle(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0, DARKGRAY);
            }
        }
    }
    draw_circle(
        player.x / TILE_SIZE * 8.0,
        player.y / TILE_SIZE * 8.0,
        3.0,
        YELLOW,
    );
    draw_line(
        player.x / TILE_SIZE * 8.0,
        player.y / TILE_SIZE * 8.0,
        (player.x + player.angle.cos() * 20.0) / TILE_SIZE * 8.0,
        (player.y + player.angle.sin() * 20.0) / TILE_SIZE * 8.0,
        1.0,
        RED,
    );
}
