use macroquad::prelude::*;
use crate::player::Player;
use crate::config::INTERNAL_HEIGHT;

const HEART_SIZE: f32 = 16.0;

pub fn draw(
    player: &Player,
    heart_texture: &Texture2D,
    skull_texture: &Texture2D,
    kills: u32,
    camera_offset: Vec2,
) {

    let y = INTERNAL_HEIGHT as f32 - 22.0 + camera_offset.y;
    let base_x = 10.0 + camera_offset.x;

    for i in 0..player.hp {

        let x = base_x + (i as f32 * 18.0);

        let color = if i < player.hp {
            WHITE
        } else {
            Color::new(0.2,0.2,0.2,1.0)
        };

        draw_texture_ex(
            heart_texture,
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(HEART_SIZE, HEART_SIZE)),
                ..Default::default()
            },
        );
    }

    let skull_x = 120.0 + camera_offset.x;

    draw_texture_ex(
        skull_texture,
        skull_x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(16.0, 16.0)),
            ..Default::default()
        },
    );

    let text = format!("{:04}", kills);

    draw_text(
        &text,
        skull_x + 22.0,
        y + 14.0,
        18.0,
        WHITE,
    );
}