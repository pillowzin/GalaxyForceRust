pub const SCALE: f32 = 2.5;
pub const SPRITE_SIZE: f32 = 16.0;

pub const WINDOW_COLS: u32 = 10;
pub const WINDOW_ROWS: u32 = 15;

pub const WINDOW_WIDTH: i32 =
    (SPRITE_SIZE * SCALE * WINDOW_COLS as f32) as i32;

pub const WINDOW_HEIGHT: i32 =
    (SPRITE_SIZE * SCALE * WINDOW_ROWS as f32) as i32;
