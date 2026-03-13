use macroquad::prelude::*;
use macroquad::audio::*;

use crate::player::Player;
use crate::enemy::{Enemy, EnemyKind};
use crate::bullet::Bullet;
use crate::collision::aabb;
use crate::spawner::inimigos_para_fase;
use crate::explosion::Explosion;
use crate::enemy_bullet::EnemyBullet;

pub struct PlayingState {
    pub stage: u32,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub explosions: Vec<Explosion>,
    pub enemy_bullets: Vec<EnemyBullet>,

    shoot_timer: f32,
    waiting_next_stage: bool,
    stage_timer: f32,

    normal_enemy_texture: Texture2D,
    red_enemy_texture: Texture2D,
    miniboss_texture: Texture2D,
    boss_texture: Texture2D,

    explosion_texture: Texture2D,
    explosion_frames: Vec<Rect>,

    laser_sound: Sound,
    collide_sound: Sound,
}

const SHOOT_DELAY: f32 = 0.2;
const NEXT_STAGE_DURATION: f32 = 1.0;

impl PlayingState {
    pub async fn new(
        normal_enemy_texture: Texture2D,
        red_enemy_texture: Texture2D,
        miniboss_texture: Texture2D,
        boss_texture: Texture2D,
        explosion_texture: Texture2D,
        explosion_frames: Vec<Rect>,
    ) -> Self {
        let laser_sound = load_sound("audio/laser.wav").await.unwrap();
        let collide_sound = load_sound("audio/collide.wav").await.unwrap();

        let stage = 1;

        let enemies = inimigos_para_fase(
            stage,
            normal_enemy_texture.clone(),
            red_enemy_texture.clone(),
            miniboss_texture.clone(),
            boss_texture.clone(),

        );

        Self {
            stage,
            enemies,
            bullets: Vec::new(),
            explosions: Vec::new(),
            enemy_bullets: Vec::new(),

            shoot_timer: 0.0,
            waiting_next_stage: false,
            stage_timer: 0.0,

            normal_enemy_texture,
            red_enemy_texture,
            miniboss_texture,
            boss_texture,
            laser_sound,
            collide_sound,
            explosion_texture,
            explosion_frames,
        }
    }

    pub fn update(&mut self, player: &mut Player, dt: f32) {
        self.shoot_timer += dt;
        let speed_mult = 1.0 + (self.stage as f32 - 1.0) * 0.07;

        // INPUT DE TIRO
        if (is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Q))
            && self.shoot_timer >= SHOOT_DELAY
        {
            let origin = vec2(
                player.hitbox().x + player.hitbox().w / 2.0,
                player.hitbox().y,
            );

            self.bullets.push(Bullet::new(origin));
            self.shoot_timer = 0.0;
            play_sound(
                &self.laser_sound,
                PlaySoundParams {
                    volume: 0.4,
                    looped: false,
                },
            );
        }

        // UPDATE INIMIGOS
        let player_center_x =
            player.hitbox().x + player.hitbox().w / 2.0;

        for enemy in self.enemies.iter_mut() {
            enemy.update_with_speed_mult(speed_mult, player_center_x);
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.kind == EnemyKind::Boss {
                // chance simples de tiro (frame-based)
                if rand::gen_range(0.0, 1.0) < 0.02 {
                    let boss_center = enemy.pos + vec2(
                        enemy.hitbox().w / 2.0,
                        enemy.hitbox().h / 2.0,
                    );

                    // direção inicial aleatória
                    let dir = vec2(
                        rand::gen_range(-1.0, 1.0),
                        rand::gen_range(0.2, 1.0),
                    )
                    .normalize_or_zero();

                    self.enemy_bullets.push(
                        EnemyBullet::new(boss_center, dir * 5.0)
                    );
                }
            }
        }
        for bullet in self.enemy_bullets.iter_mut() {
            bullet.update();
        }

        self.enemy_bullets.retain(|b| !b.is_dead());

        // COLISÃO PLAYER × INIMIGO
        for enemy in self.enemies.iter_mut() {
            if aabb(player.hitbox(), enemy.hitbox()) {
                match enemy.kind {
                    EnemyKind::Normal | EnemyKind::Red => {
                        enemy.hp = 0;
                        player.hit(enemy.pos);
                    }
                    EnemyKind::MiniBoss | EnemyKind::Boss => {
                        player.hit(enemy.pos);
                    }
                }
                play_sound(
                    &self.collide_sound,
                    PlaySoundParams {
                        volume: 0.6,
                        looped: false,
                    },
                );
            }
        }

        // UPDATE TIROS
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
        self.bullets.retain(|b| !b.offscreen());

        // UPDATE EXPLOSOES
        for explosion in self.explosions.iter_mut() {
            explosion.update(dt);
        }

        self.explosions.retain(|e| !e.is_finished());

        // colisao balas
        let mut bullets_to_remove = Vec::new();

        for (bi, bullet) in self.bullets.iter().enumerate() {
            for enemy in self.enemies.iter_mut() {
                if aabb(bullet.hitbox(), enemy.hitbox()) {
                    enemy.hp -= 1;
                    bullets_to_remove.push(bi);

                    let center = vec2(
                        enemy.hitbox().x + enemy.hitbox().w / 2.0,
                        enemy.hitbox().y + enemy.hitbox().h / 2.0,
                    );

                    self.explosions.push(
                        Explosion::new(
                            center - vec2(32.0, 32.0),
                            self.explosion_texture.clone(),
                            self.explosion_frames.clone(),
                        )
                    );
                    play_sound(
                        &self.collide_sound,
                        PlaySoundParams {
                            volume: 0.6,
                            looped: false,
                        },
                    );
                    break;
                }
            }
        }

        //colisao com a bala do boss
        for bullet in self.enemy_bullets.iter() {
            if aabb(bullet.hitbox(), player.hitbox()) {
                player.hit(bullet.pos);
                play_sound(
                    &self.collide_sound,
                    PlaySoundParams {
                        volume: 0.6,
                        looped: false,
                    },
                );
            }
        }

        // Remover as balas que colidiram
        let mut i = 0;
        self.bullets.retain(|_| {
            let keep = !bullets_to_remove.contains(&i);
            i += 1;
            keep
        });

        // Remover inimigos mortos
        self.enemies.retain(|e| e.hp > 0);

        // TRANSIÇÃO DE FASE
        if self.enemies.is_empty() && !self.waiting_next_stage {
            self.waiting_next_stage = true;
            self.stage_timer = 0.0;
        }

        if self.waiting_next_stage {
            self.stage_timer += dt;
            if self.stage_timer >= NEXT_STAGE_DURATION {
                self.waiting_next_stage = false;
                self.stage += 1;

                self.enemies = inimigos_para_fase(
                    self.stage,
                    self.normal_enemy_texture.clone(),
                    self.red_enemy_texture.clone(),
                    self.miniboss_texture.clone(),
                    self.boss_texture.clone(),
                );
            }
        }
    }

    pub fn draw(&self, player: &Player, font: &Font) {
        player.draw();

        for enemy in self.enemies.iter() {
            enemy.draw();
        }

        for bullet in self.bullets.iter() {
            bullet.draw();
        }

        for bullet in self.enemy_bullets.iter() {
            bullet.draw();
        }

        for explosion in self.explosions.iter() {
            explosion.draw();
        }

        if self.waiting_next_stage {
            let t = self.stage_timer;

            let alpha = if t < 0.5 {
                t / 0.5
            } else if t > NEXT_STAGE_DURATION - 0.5 {
                (NEXT_STAGE_DURATION - t) / 0.5
            } else {
                1.0
            };

            let text = "PRÓXIMA FASE!";
            let font_size = 14.0;
            let text_dim = measure_text(text, None, font_size as u16, 1.0);

            let player_center_x =
                player.hitbox().x + player.hitbox().w / 2.0;

            let x = player_center_x - text_dim.width / 2.0 - 55.0;
            let y = player.hitbox().y - 20.0;

            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: font_size as u16,
                    color: Color::new(0.92, 0.6, 0.25, alpha),
                    ..Default::default()
                },
            );
        }
    }
}
