use macroquad::prelude::*;
use crate::audio_manager::AudioManager;

mod config;
mod crt_shader;
mod thruster;
mod audio_manager;
mod animation;
mod player;
mod enemy;
mod collision;
mod bullet;
mod spawner;
mod state_menu;
mod state_playing;
mod explosion;
mod star;
mod enemy_bullet;
mod state_paused;

use crate::crt_shader::load_crt_material;
use std::mem;
use crate::config::*;
use crate::player::Player;
use crate::animation::gerar_frames;
use crate::state_playing::PlayingState;
use crate::state_paused::{PausedState, PauseAction};
use crate::state_menu::{MenuState, MenuAction};
use crate::star::Star;
const W: f32 = INTERNAL_WIDTH as f32;
const H: f32 = INTERNAL_HEIGHT as f32;

enum GameState {
    Menu(MenuState),
    Playing(PlayingState),
    Paused(PlayingState, PausedState),
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Galaxy Forces".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let crt_material = load_crt_material();
    
    let render_target = render_target(
        INTERNAL_WIDTH,
        INTERNAL_HEIGHT,
    );
    render_target.texture.set_filter(FilterMode::Nearest);

    // --- ASSETS ---
    let player_texture = load_texture("sprites/spaceship.png").await.unwrap();
    let enemy_texture = load_texture("sprites/enemy.png").await.unwrap();
    let enemy2_texture = load_texture("sprites/enemy2.png").await.unwrap();
    let miniboss_texture = load_texture("sprites/miniboss.png").await.unwrap();
    let boss_texture = load_texture("sprites/boss.png").await.unwrap();
    let explosion_texture = load_texture("sprites/explosion.png").await.unwrap();
    let speaker_texture = load_texture("sprites/speaker.png").await.unwrap();
    speaker_texture.set_filter(FilterMode::Nearest);

    for tex in [
        &player_texture,
        &enemy_texture,
        &enemy2_texture,
        &miniboss_texture,
        &boss_texture,
        &explosion_texture,
    ] {
        tex.set_filter(FilterMode::Nearest);
    }

    let explosion_frames = gerar_frames(32.0, 32.0, 160.0, 32.0);

    let pixel_font = load_ttf_font("fonts/PressStart2P-Regular.ttf")
        .await
        .unwrap();

    // --- ENTIDADES ---
    let mut player = Player::new(player_texture.clone());
    let mut audio = AudioManager::new().await;

    // --- GAME STATE ---
	let mut game_state = GameState::Menu(MenuState::new(speaker_texture.clone()));
	audio.play_menu_music();

    let mut fade_alpha = 1.0;

    // --- BACKGROUND / CAMERA ---
    let mut stars: Vec<Star> = (0..120).map(|_| Star::new()).collect();
    let mut camera_offset = Vec2::ZERO;
    let mut shake_timer = 0.0;
    let mut shake_strength = 0.0;

    // --- LOOP ---
    loop {
        let dt = get_frame_time();

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            zoom: vec2(2.0 / INTERNAL_WIDTH as f32, -2.0 / INTERNAL_HEIGHT as f32),
            target: vec2(
                INTERNAL_WIDTH as f32 / 2.0 + camera_offset.x,
                INTERNAL_HEIGHT as f32 / 2.0 + camera_offset.y,
            ),
            ..Default::default()
        });

        clear_background(BLACK);

        // estrelas globais (jogo)
        for star in stars.iter_mut() {
            star.update();
        }

        if is_key_pressed(KeyCode::Escape) {
            game_state = match mem::replace(
                &mut game_state,
                GameState::Menu(MenuState::new(speaker_texture.clone()))
            ) {
                GameState::Playing(state) => {
                    GameState::Paused(
                        state,
                        PausedState::new(speaker_texture.clone())
                    )
                }

                GameState::Paused(state, _) => {
                    GameState::Playing(state)
                }

                other => other,
            };
        }

        match &mut game_state {
            GameState::Menu(menu) => {
                menu.update(dt);
                menu.draw(&pixel_font, audio.music_muted);

                match menu.draw_buttons(&pixel_font) {
                    MenuAction::Start => {
                        audio.click();
                        player.reset();
                        fade_alpha = 1.0;

                        game_state = GameState::Playing(
                            PlayingState::new(
                                enemy_texture.clone(),
                                enemy2_texture.clone(),
                                miniboss_texture.clone(),
                                boss_texture.clone(),
                                explosion_texture.clone(),
                                explosion_frames.clone(),
                            )
                            .await,
                        );
                    }

                    MenuAction::Mute => {
                        audio.click();
                        audio.toggle_music();
                    }

                    MenuAction::Quit => {
                        audio.click();
                        std::process::exit(0);
                    }

                    MenuAction::None => {}
                }
            }

            GameState::Playing(state) => {
                // --- PLAYER ---
                player.update();

                // --- CAMERA FOLLOW ---
                let screen_center_x = screen_width() * 0.5;
                let target_offset_x = (screen_center_x - player.pos.x) * 0.08;
                camera_offset.x += (target_offset_x - camera_offset.x) * 0.1;
                camera_offset.y = 0.0;

                // --- CAMERA SHAKE ---
                if shake_timer > 0.0 {
                    shake_timer -= dt;
                    camera_offset += vec2(
                        rand::gen_range(-1.0, 1.0),
                        rand::gen_range(-1.0, 1.0),
                    ) * shake_strength;
                }

                let was_flashing = player.is_flashing();

                // --- GAME UPDATE ---
                state.update(&mut player, dt);

                if player.is_flashing() && !was_flashing {
                    shake_timer = 0.15;
                    shake_strength = 6.0;
                }

                // --- DRAW ---
                state.draw(&player, &pixel_font);

                for star in stars.iter() {
                    star.draw(camera_offset);
                }
            }

            GameState::Paused(state, pause) => {

                state.draw(&player, &pixel_font);

                for star in stars.iter() {
                    star.draw(camera_offset);
                }

                match pause.draw(&pixel_font, audio.music_muted) {

                    PauseAction::Resume => {
                        if let GameState::Paused(state, _) =
                            mem::replace(&mut game_state, GameState::Menu(MenuState::new(speaker_texture.clone())))
                        {
                            game_state = GameState::Playing(state);
                        }
                    }

                    PauseAction::Menu => {
                        game_state = GameState::Menu(MenuState::new(speaker_texture.clone()));
                    }

                    PauseAction::Quit => {
                        std::process::exit(0);
                    }

                    PauseAction::ToggleSound => {
                        audio.toggle_music();
                    }

                    PauseAction::None => {}
                }
            }
        }

        // --- FADE VISUAL ---
        if fade_alpha > 0.0 {
            fade_alpha -= dt * 0.8;
            draw_rectangle(
                0.0,
                0.0,
                W, H,
                Color::new(0.0, 0.0, 0.0, fade_alpha),
            );
        }
        set_default_camera();

        gl_use_material(&crt_material);

        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width(),
                    screen_height(),
                )),
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();

        next_frame().await;
    }
}