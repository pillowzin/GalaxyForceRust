use macroquad::audio::*;

pub struct AudioManager {
    theme: Sound,
    click: Sound,
    muted: bool,
}

impl AudioManager {
    pub async fn new() -> Self {
        let theme = load_sound("audio/theme.wav").await.unwrap();
        let click = load_sound("audio/click.wav").await.unwrap();

        Self {
            theme,
            click,
            muted: false,
        }
    }

    // toca a música do menu
    pub fn play_menu_music(&self) {
        if self.muted {
            return;
        }

        play_sound(
            &self.theme,
            PlaySoundParams {
                looped: true,
                volume: 0.5, // volume fixo
            },
        );
    }

    // para quando sair do menu
    pub fn stop_menu_music(&self) {
        stop_sound(&self.theme);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;

        if self.muted {
            stop_sound(&self.theme);
        } else {
            self.play_menu_music();
        }
    }

    pub fn click(&self) {
        if self.muted {
            return;
        }

        play_sound(
            &self.click,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }
}
