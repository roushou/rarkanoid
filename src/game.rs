use raylib::prelude::*;

use crate::entities::{Ball, Brick, Paddle};
use crate::levels::load_level;
use crate::resources::SoundEffects;
use crate::systems::{
    Camera, ParticleSystem, check_ball_brick_collision, check_ball_paddle_collision,
};
use crate::ui::{MenuType, draw_hud, draw_menu};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, STARTING_LIVES};

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    LevelComplete,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub paddle: Paddle,
    pub ball: Ball,
    pub bricks: Vec<Brick>,
    pub score: i32,
    pub lives: i32,
    pub level: usize,
    pub combo: i32,
    pub particles: ParticleSystem,
    pub camera: Camera,
    pub sounds: SoundEffects,
}

impl Game {
    pub fn new() -> Self {
        let paddle = Paddle::new();
        let ball = Ball::new_attached(&paddle);

        Self {
            state: GameState::Menu,
            paddle,
            ball,
            bricks: Vec::new(),
            score: 0,
            lives: STARTING_LIVES,
            level: 1,
            combo: 0,
            particles: ParticleSystem::new(500),
            camera: Camera::new(),
            sounds: SoundEffects::new(),
        }
    }

    pub fn start_game(&mut self) {
        self.score = 0;
        self.lives = STARTING_LIVES;
        self.level = 1;
        self.combo = 0;
        self.load_current_level();
        self.reset_ball();
        self.state = GameState::Playing;
    }

    pub fn load_current_level(&mut self) {
        self.bricks = load_level(self.level);
    }

    pub fn reset_ball(&mut self) {
        self.ball = Ball::new_attached(&self.paddle);
        self.combo = 0;
    }

    pub fn next_level(&mut self) {
        self.level += 1;
        self.load_current_level();
        if self.bricks.is_empty() {
            // No more levels, player wins
            self.state = GameState::GameOver;
        } else {
            self.reset_ball();
            self.state = GameState::Playing;
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        match self.state {
            GameState::Menu => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                    || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    self.start_game();
                }
            }
            GameState::Playing => {
                self.update_playing(rl, dt);
            }
            GameState::Paused => {
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                    || rl.is_key_pressed(KeyboardKey::KEY_P)
                {
                    self.state = GameState::Playing;
                }
            }
            GameState::LevelComplete => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                    || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    self.next_level();
                }
            }
            GameState::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                    || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    self.state = GameState::Menu;
                }
            }
        }

        // Always update particles and camera shake
        self.particles.update(dt);
        self.camera.update(dt);
    }

    fn update_playing(&mut self, rl: &mut RaylibHandle, dt: f32) {
        // Check for pause
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) || rl.is_key_pressed(KeyboardKey::KEY_P) {
            self.state = GameState::Paused;
            return;
        }

        // Update paddle
        self.paddle.update(rl, dt);

        // Update ball
        if self.ball.attached {
            // Ball follows paddle
            self.ball.x = self.paddle.x + self.paddle.width / 2.0;

            // Launch on space or click
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.ball.launch();
            }
        } else {
            self.ball.update(dt);

            // Wall collisions
            self.ball.handle_wall_collision();

            // Check if ball fell below screen
            if self.ball.y > SCREEN_HEIGHT as f32 + self.ball.radius {
                self.lives -= 1;
                self.combo = 0;
                self.sounds.play_lose_life();
                if self.lives <= 0 {
                    self.state = GameState::GameOver;
                } else {
                    self.reset_ball();
                }
                return;
            }

            // Paddle collision
            if check_ball_paddle_collision(&mut self.ball, &self.paddle) {
                self.combo = 0;
                self.sounds.play_paddle_hit();
            }

            // Brick collisions
            let mut bricks_to_remove = Vec::new();
            for (i, brick) in self.bricks.iter_mut().enumerate() {
                if let Some(score) = check_ball_brick_collision(&mut self.ball, brick) {
                    self.combo += 1;
                    self.score += score * self.combo;

                    // Screen shake based on combo
                    let shake_intensity = (4.0 + self.combo as f32 * 1.0).min(12.0);
                    self.camera.shake(shake_intensity, 0.2);

                    if brick.health <= 0 {
                        // Big explosion for destroyed bricks
                        self.particles.spawn_explosion(
                            brick.x + brick.width / 2.0,
                            brick.y + brick.height / 2.0,
                            brick.color,
                            30,
                        );
                        bricks_to_remove.push(i);
                        self.sounds.play_brick_destroy();
                    } else {
                        // Small burst for damaged bricks
                        self.particles.spawn_explosion(
                            brick.x + brick.width / 2.0,
                            brick.y + brick.height / 2.0,
                            brick.color,
                            10,
                        );
                        self.sounds.play_brick_hit();
                    }
                }
            }

            // Remove destroyed bricks (in reverse order to preserve indices)
            for i in bricks_to_remove.into_iter().rev() {
                self.bricks.remove(i);
            }

            // Check level completion
            if self.bricks.iter().all(|b| b.indestructible) {
                self.state = GameState::LevelComplete;
                self.sounds.play_level_complete();
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let offset = self.camera.get_offset();

        // Draw background gradient
        self.draw_background(d);

        // Apply camera shake offset for game elements
        let ox = offset.x as i32;
        let oy = offset.y as i32;

        match self.state {
            GameState::Menu => {
                draw_menu(d, MenuType::Title);
            }
            GameState::Playing | GameState::Paused => {
                // Draw game elements with shake offset
                self.draw_game_elements(d, ox, oy);

                if self.state == GameState::Paused {
                    draw_menu(d, MenuType::Pause);
                }
            }
            GameState::LevelComplete => {
                self.draw_game_elements(d, ox, oy);
                draw_menu(d, MenuType::LevelComplete(self.level));
            }
            GameState::GameOver => {
                draw_menu(d, MenuType::GameOver(self.score));
            }
        }

        // Draw HUD (no shake)
        if self.state == GameState::Playing || self.state == GameState::Paused {
            draw_hud(d, self.score, self.lives, self.level, self.combo);
        }

        // Draw particles (with shake)
        self.particles.draw(d, ox, oy);
    }

    fn draw_background(&self, d: &mut RaylibDrawHandle) {
        // Simple gradient background
        for y in 0..SCREEN_HEIGHT {
            let t = y as f32 / SCREEN_HEIGHT as f32;
            let r = (20.0 + t * 10.0) as u8;
            let g = (20.0 + t * 15.0) as u8;
            let b = (30.0 + t * 20.0) as u8;
            d.draw_line(0, y, SCREEN_WIDTH, y, Color::new(r, g, b, 255));
        }
    }

    fn draw_game_elements(&self, d: &mut RaylibDrawHandle, ox: i32, oy: i32) {
        // Draw bricks
        for brick in &self.bricks {
            brick.draw(d, ox, oy);
        }

        // Draw paddle
        self.paddle.draw(d, ox, oy);

        // Draw ball
        self.ball.draw(d, ox, oy);
    }
}
