use macroquad::prelude::*;

#[derive(Clone)]
pub struct Star {
    pub pos: Vec2,
    pub speed: f32,
    pub size: f32,
}

impl Star {
    pub fn new() -> Self {
        Self {
            pos: vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            ),
            speed: rand::gen_range(30.0, 90.0), // velocidade real (dt)
            size: rand::gen_range(1.0, 2.5),
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.pos.y += self.speed * dt
            * (1.0 + (get_time() as f32 * 0.5).sin() * 0.02);

        if self.pos.y > screen_height() {
            self.pos.y = 0.0;
            self.pos.x = rand::gen_range(0.0, screen_width());
        }
    }

    pub fn draw(&self, camera_offset: Vec2) {
        let depth = (self.speed / 90.0).clamp(0.2, 1.0);
        let alpha = depth.clamp(0.4, 1.0);

        draw_circle(
            self.pos.x + camera_offset.x * depth,
            self.pos.y + camera_offset.y * depth,
            self.size,
            Color::new(1.0, 1.0, 1.0, alpha),
        );
    }

}
