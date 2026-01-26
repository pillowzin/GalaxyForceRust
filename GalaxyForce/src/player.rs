use macroquad::prelude::*;
use crate::config::{SPRITE_SIZE, SCALE};

pub struct Player {
    pub pos: Vec2,
    texture: Texture2D,
    hit_timer: f32,
    shake_timer: f32,

    last_hit_dir: Vec2,
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            pos: vec2(200.0, 400.0),
            texture,
            hit_timer: 0.0,
            shake_timer: 0.0,
            last_hit_dir: Vec2::ZERO,
        }
    }
    fn size(&self) -> f32 {
        SPRITE_SIZE * SCALE
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.hit_timer = (self.hit_timer - dt).max(0.0);
        self.shake_timer = (self.shake_timer - dt).max(0.0);

        let (mx, my) = mouse_position();
        let size = self.size();

        self.pos.x = (mx - size / 2.0)
            .clamp(0.0, screen_width() - size);
        self.pos.y = (my - size / 2.0)
            .clamp(0.0, screen_height() - size);
    }

    pub fn hitbox(&self) -> Rect {
        let size = self.size();
        Rect::new(self.pos.x, self.pos.y, size, size)
    }

    pub fn hit(&mut self, from: Vec2) {
        self.hit_timer = 0.2;
        self.shake_timer = 0.12;

        self.last_hit_dir = (self.pos - from).normalize_or_zero();
    }

    pub fn is_flashing(&self) -> bool {
        self.hit_timer > 0.0
    }

    pub fn draw(&self) {
        let size = self.size();

        // ---- SHAKE LOCAL ----
        let mut draw_pos = self.pos;
        if self.shake_timer > 0.0 {
            let strength = 4.0 * (self.shake_timer / 0.12);
            draw_pos += self.last_hit_dir * strength;
        }

        // ---- FLASH ----
        let flashing = self.hit_timer > 0.0;
        let flash_alpha = (self.hit_timer / 0.2).clamp(0.0, 1.0);

        // sprite base (normal)
        draw_texture_ex(
            &self.texture,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );

        // ---- GLOW + FLASH ----
        if flashing {
            draw_texture_ex(
                &self.texture,
                draw_pos.x - 2.0,
                draw_pos.y - 2.0,
                Color::new(1.0, 1.0, 1.0, flash_alpha),
                DrawTextureParams {
                    dest_size: Some(vec2(size + 4.0, size + 4.0)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn reset(&mut self) {
        self.pos = vec2(
            screen_width() * 0.5,
            screen_height() * 0.7,
        );

        self.hit_timer = 0.0;
        self.shake_timer = 0.0;
        self.last_hit_dir = Vec2::ZERO;
    }

}