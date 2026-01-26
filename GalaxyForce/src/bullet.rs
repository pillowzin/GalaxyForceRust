use macroquad::prelude::*;

pub struct Bullet {
    pub pos: Vec2,
    speed: f32,
}

impl Bullet {
    pub fn new(origin: Vec2) -> Self {
        Self {
            pos: origin,
            speed: 500.0,
        }
    }

    pub fn update(&mut self) {
        self.pos.y -= self.speed * get_frame_time();
    }

    pub fn draw(&self) {
        let size = 6.0;
        draw_rectangle(
            self.pos.x - size / 2.0,
            self.pos.y - size,
            size,
            size * 2.0,
            RED,
        );
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x - 3.0, self.pos.y - 6.0, 6.0, 12.0)
    }

    pub fn offscreen(&self) -> bool {
        self.pos.y < -20.0
    }
}
