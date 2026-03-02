use macroquad::prelude::*;
use rand::gen_range;
use crate::config::{INTERNAL_WIDTH, INTERNAL_HEIGHT};

const W: f32 = INTERNAL_WIDTH as f32;
const H: f32 = INTERNAL_HEIGHT as f32;
const TILE: f32 = 8.0;
const TITLE_Y: f32 = 6.0 * TILE;      // 48
const SHIP_Y: f32 = 14.0 * TILE;       // 112
const MENU_START_Y: f32 = 22.0 * TILE; // 176
const MENU_GAP: f32 = 3.0 * TILE;      // 24
const STAR_COUNT: usize = 100;

#[derive(Clone)]
struct Star {
    pos: Vec2,
    speed: f32,
}

pub struct MenuState {
    time: f32,
    stars: Vec<Star>,
    fade: f32,
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Start,
    Mute,
    Quit,
    None,
}

impl MenuState {
    pub fn new() -> Self {
        let stars = (0..STAR_COUNT)
            .map(|_| Star {
                pos: vec2(
                    gen_range(0.0, W),
                    gen_range(0.0, H),
                ),
                speed: gen_range(10.0, 40.0),
            })
            .collect();

        Self {
            time: 0.0,
            stars,
            fade: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        if self.fade > 0.0 {
            self.fade -= dt * 0.8;
        }

        for star in self.stars.iter_mut() {
            star.pos.y += star.speed * dt;
            if star.pos.y > H {
                star.pos.y = 0.0;
                star.pos.x = gen_range(0.0, W);
            }
        }
    }

    pub fn draw(&self, font: &Font, ship: &Texture2D) {
        clear_background(BLACK);

        for star in &self.stars {
            draw_rectangle(star.pos.x, star.pos.y, 1.0, 1.0, WHITE);
        }

        let offset = (self.time * 1.5).sin() * 2.0;
        let center_x = W * 0.5;
        let title_size = 8;

        let a = "Galaxy";
        let b = "Force";

        let da = measure_text(a, Some(font), title_size, 1.0);
        let db = measure_text(b, Some(font), title_size, 1.0);

        draw_text_ex(
            a,
            center_x - da.width / 2.0,
            TITLE_Y + offset,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: RED,
                ..Default::default()
            },
        );

        draw_text_ex(
            b,
            center_x - db.width / 2.0,
            TITLE_Y + 2.0 * TILE + offset,
            TextParams {
                font: Some(font),
                font_size: title_size,
                color: BLUE,
                ..Default::default()
            },
        );

        draw_texture_ex(
            ship,
            center_x - 16.0,
            SHIP_Y + offset,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );

        if self.fade > 0.0 {
            draw_rectangle(
                0.0,
                0.0,
                W,
                H,
                Color::new(0.0, 0.0, 0.0, self.fade),
            );
        }
    }

    pub fn draw_buttons(&self, font: &Font) -> MenuAction {
        let center_x = W * 0.5;
        let y = MENU_START_Y;

        Self::button(center_x, y, "INICIAR", font, MenuAction::Start)
            .or(Self::button(center_x, y + MENU_GAP, "MUTAR AUDIO", font, MenuAction::Mute))
            .or(Self::button(center_x, y + MENU_GAP * 2.0, "SAIR", font, MenuAction::Quit))
            .unwrap_or(MenuAction::None)
    }

    fn button(
        x: f32,
        y: f32,
        label: &str,
        font: &Font,
        action: MenuAction,
    ) -> Option<MenuAction> {
        let bw = 14.0 * TILE;
        let bh = 2.0 * TILE;

        let (mx, my) = mouse_position();
        let mx = mx * (INTERNAL_WIDTH as f32 / screen_width());
        let my = my * (INTERNAL_HEIGHT as f32 / screen_height());

        let hovered = Rect::new(x - bw / 2.0, y, bw, bh)
            .contains(vec2(mx, my));

        let scale = if hovered { 1.05 } else { 1.0 };
        let color = if hovered { YELLOW } else { GRAY };

        draw_rectangle(
            x - bw * scale / 2.0,
            y,
            bw * scale,
            bh,
            color,
        );

        let dim = measure_text(label, Some(font), 8, 1.0);
        draw_text_ex(
            label,
            x - dim.width / 2.0,
            y + bh / 2.0 + dim.height / 2.0,
            TextParams {
                font: Some(font),
                font_size: 8,
                color: BLACK,
                ..Default::default()
            },
        );

        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            Some(action)
        } else {
            None
        }
    }
}