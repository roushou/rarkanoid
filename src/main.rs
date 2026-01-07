mod entities;
mod game;
mod levels;
mod resources;
mod systems;
mod ui;

use game::Game;
use raylib::prelude::*;

// Window
pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

// Paddle
pub const PADDLE_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 15.0;
pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_Y: f32 = 550.0;

// Ball
pub const BALL_RADIUS: f32 = 8.0;
pub const BALL_INITIAL_SPEED: f32 = 400.0;
pub const BALL_MAX_SPEED: f32 = 600.0;
pub const BALL_SPEED_INCREMENT: f32 = 5.0;

// Bricks
pub const BRICK_WIDTH: f32 = 70.0;
pub const BRICK_HEIGHT: f32 = 25.0;
pub const BRICK_PADDING: f32 = 5.0;
pub const BRICK_OFFSET_TOP: f32 = 60.0;
pub const BRICK_OFFSET_LEFT: f32 = 35.0;

// Gameplay
pub const STARTING_LIVES: i32 = 3;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rarkanoid")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut game = Game::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        game.update(&mut rl, dt);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(20, 20, 30, 255));

        game.draw(&mut d);
    }
}
