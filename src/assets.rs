use macroquad::prelude::*;
use macroquad::audio::*;

pub struct Assets {
    // texturas
    pub player: Texture2D,
    pub enemy: Texture2D,
    pub enemy2: Texture2D,
    pub miniboss: Texture2D,
    pub boss: Texture2D,
    pub explosion: Texture2D,

    // fontes
    pub pixel_font: Font,

    // sons
    pub theme: Sound,
    pub click: Sound,
    pub laser: Sound,
    pub collide: Sound,

    // animações
    pub explosion_frames: Vec<Rect>,
}

impl Assets {
    pub async fn load() -> Self {
        let player = load_texture("sprites/spaceship.png").await.unwrap();
        let enemy = load_texture("sprites/enemy.png").await.unwrap();
        let enemy2 = load_texture("sprites/enemy2.png").await.unwrap();
        let miniboss = load_texture("sprites/miniboss.png").await.unwrap();
        let boss = load_texture("sprites/boss.png").await.unwrap();
        let explosion = load_texture("sprites/explosion.png").await.unwrap();

        for tex in [
            &player, &enemy, &enemy2, &miniboss, &boss, &explosion
        ] {
            tex.set_filter(FilterMode::Nearest);
        }

        let pixel_font =
            load_ttf_font("fonts/PressStart2P-Regular.ttf").await.unwrap();

        let theme = load_sound("audio/theme.wav").await.unwrap();
        let click = load_sound("audio/click.wav").await.unwrap();
        let laser = load_sound("audio/laser.wav").await.unwrap();
        let collide = load_sound("audio/collide.wav").await.unwrap();

        let explosion_frames =
            crate::animation::gerar_frames(32.0, 32.0, 160.0, 32.0);

        Self {
            player,
            enemy,
            enemy2,
            miniboss,
            boss,
            explosion,
            pixel_font,
            theme,
            click,
            laser,
            collide,
            explosion_frames,
        }
    }
}
