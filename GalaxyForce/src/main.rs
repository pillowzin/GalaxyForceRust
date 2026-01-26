mod config;
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

use macroquad::prelude::*;
use star::Star;
use crate::config::*;
use crate::player::Player;
use crate::animation::gerar_frames;
use crate::state_playing::PlayingState;
use state_menu::MenuState;

enum GameState {
    Menu(MenuState),
    Playing(PlayingState),
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
	// --- ASSETS ---
	let player_texture = load_texture("sprites/spaceship.png").await.unwrap();
	let enemy_texture = load_texture("sprites/enemy.png").await.unwrap();
	let enemy2_texture = load_texture("sprites/enemy2.png").await.unwrap();
	let miniboss_texture = load_texture("sprites/miniboss.png").await.unwrap();
	let explosion_texture = load_texture("sprites/explosion.png").await.unwrap();
	let boss_texture = load_texture("sprites/boss.png").await.unwrap();

	player_texture.set_filter(FilterMode::Nearest);
	enemy_texture.set_filter(FilterMode::Nearest);
	enemy2_texture.set_filter(FilterMode::Nearest);
	miniboss_texture.set_filter(FilterMode::Nearest);
	boss_texture.set_filter(FilterMode::Nearest);
	explosion_texture.set_filter(FilterMode::Nearest);

	let explosion_frames = gerar_frames(
	    32.0, 32.0,
	    160.0, 32.0,
	);

	let pixel_font = load_ttf_font("fonts/PressStart2P-Regular.ttf")
	    .await
	    .unwrap();

	// --- ENTIDADES ---
	let mut player = Player::new(player_texture.clone());

	// ---- GAME STATES ----
	let mut game_state = GameState::Menu(MenuState::new());

	// let mut playing_state = PlayingState::new(
	//     enemy_texture.clone(),
	//     enemy2_texture.clone(),
	//     miniboss_texture.clone(),
	//     boss_texture.clone(),
	//     explosion_texture,
	//     explosion_frames,
	// );

	// ESTRELINHAS!
	let mut camera_offset = Vec2::ZERO;
	let mut shake_timer = 0.0;
	let mut shake_strength = 0.0;
	let mut stars: Vec<Star> = (0..120).map(|_| Star::new()).collect();

    // --- LOOP ---
	loop {
	    let dt = get_frame_time();
	    clear_background(BLACK);

	    // --- UPDATE STARS (global) ---
	    for star in stars.iter_mut() {
	        star.update();
	    }

		match &mut game_state {
		    GameState::Menu(menu) => {
		        let start = menu.update(dt);
		        menu.draw(&pixel_font, &player_texture);

		        for star in stars.iter() {
		            star.draw(Vec2::ZERO);
		        }

				if start {
				    player.reset();

				    game_state = GameState::Playing(
				        PlayingState::new(
				            enemy_texture.clone(),
				            enemy2_texture.clone(),
				            miniboss_texture.clone(),
				            boss_texture.clone(),
				            explosion_texture.clone(),
				            explosion_frames.clone(),
				        )
				    );
				}

		    }

	        GameState::Playing(state) => {
	            // --- PLAYER ---
	            player.update();

	            // --- CAMERA OFFSET BASE (juice) ---
	            let screen_center_x = screen_width() * 0.5;
	            let target_offset_x = (screen_center_x - player.pos.x) * 0.08;
	            camera_offset.x += (target_offset_x - camera_offset.x) * 0.1;
	            camera_offset.y = 0.0;

	            // --- CAMERA SHAKE ---
	            if shake_timer > 0.0 {
	                shake_timer -= dt;

	                let shake = vec2(
	                    rand::gen_range(-1.0, 1.0),
	                    rand::gen_range(-1.0, 1.0),
	                ) * shake_strength;

	                camera_offset += shake;
	            }

	            let was_flashing = player.is_flashing();

	            // --- GAME UPDATE ---
	            state.update(&mut player, dt);

	            // --- HIT → SHAKE ---
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
	    }

	    next_frame().await;
	}

}
