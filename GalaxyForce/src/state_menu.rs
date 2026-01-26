use macroquad::prelude::*;
use rand::gen_range;

const STAR_COUNT: usize = 200;

pub struct MenuState {
    blink_timer: f32,
    show_text: bool,
    stars: Vec<(Vec2, Color)>,
}

impl MenuState {
    pub fn new() -> Self {
        let mut stars = Vec::with_capacity(STAR_COUNT);

        for _ in 0..STAR_COUNT {
            stars.push((
                vec2(
                    gen_range(0.0, screen_width()),
                    gen_range(0.0, screen_height()),
                ),
                Color::new(
                    gen_range(0.0, 1.0),
                    gen_range(0.0, 1.0),
                    gen_range(0.0, 1.0),
                    1.0,
                ),
            ));
        }

        Self {
            blink_timer: 0.0,
            show_text: true,
            stars,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.blink_timer += dt;

        if self.blink_timer >= 0.5 {
            self.blink_timer = 0.0;
            self.show_text = !self.show_text;
        }

        is_key_pressed(KeyCode::Space)
    }

    pub fn draw(&self, font: &Font, player_texture: &Texture2D) {
        clear_background(BLACK);

        // --- fundo estrelado ---
        for (pos, color) in &self.stars {
            draw_rectangle(pos.x, pos.y, 2.0, 2.0, *color);
        }

        // --- títulos ---
        let title_font_size = 36;
        let small_font_size = 10;

        let title_a = "Galaxy";
        let title_b = "Force";

        let dim_a = measure_text(title_a, Some(font), title_font_size, 1.0);
        let dim_b = measure_text(title_b, Some(font), title_font_size, 1.0);

        let center_x = screen_width() * 0.5;
        let base_y = screen_height() / 8.0;

        draw_text_ex(
            title_a,
            center_x - dim_a.width / 2.0,
            base_y,
            TextParams {
                font: Some(font),
                font_size: title_font_size,
                color: Color::from_rgba(255, 30, 0, 255),
                ..Default::default()
            },
        );

        draw_text_ex(
            title_b,
            center_x - dim_b.width / 2.0,
            base_y + 40.0,
            TextParams {
                font: Some(font),
                font_size: title_font_size,
                color: Color::from_rgba(0, 30, 255, 255),
                ..Default::default()
            },
        );

        // --- naves decorativas ---
        let ship_size = 16.0 * 2.5;
        let ship_y = screen_height() / 4.0 - ship_size;

        draw_texture_ex(
            player_texture,
            center_x - dim_a.width / 2.0 - ship_size - 8.0,
            ship_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(ship_size, ship_size)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            player_texture,
            center_x + dim_a.width / 2.0 + 8.0,
            ship_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(ship_size, ship_size)),
                ..Default::default()
            },
        );

        // --- texto de start (pisca + sombra) ---
        if self.show_text {
            let text = "Pressione SPACE para Iniciar";
            let dim = measure_text(text, Some(font), small_font_size, 1.0);

            let x = center_x - dim.width / 2.0;
            let y = screen_height() / 4.0 + 40.0;

            // sombra
            draw_text_ex(
                text,
                x + 2.0,
                y + 2.0,
                TextParams {
                    font: Some(font),
                    font_size: small_font_size,
                    color: BLACK,
                    ..Default::default()
                },
            );

            // texto
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: small_font_size,
                    color: YELLOW,
                    ..Default::default()
                },
            );
        }

        // --- crédito ---
        let credit = "feito por jakezin";
        let credit_dim = measure_text(credit, Some(font), small_font_size, 1.0);

        draw_text_ex(
            credit,
            center_x - credit_dim.width / 2.0,
            dim_b.height + credit_dim.height,
            TextParams {
                font: Some(font),
                font_size: small_font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
