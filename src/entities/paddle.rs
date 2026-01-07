use raylib::prelude::*;

use crate::{PADDLE_HEIGHT, PADDLE_SPEED, PADDLE_WIDTH, PADDLE_Y, SCREEN_WIDTH};

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub target_x: f32,
}

impl Paddle {
    pub fn new() -> Self {
        let x = (SCREEN_WIDTH as f32 - PADDLE_WIDTH) / 2.0;
        Self {
            x,
            y: PADDLE_Y,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            target_x: x,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        // Keyboard input
        let mut move_dir = 0.0;

        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            move_dir -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            move_dir += 1.0;
        }

        if move_dir != 0.0 {
            // Keyboard movement
            self.target_x = self.x + move_dir * PADDLE_SPEED * dt;
        } else {
            // Mouse input (when no keyboard input)
            let mouse_x = rl.get_mouse_x() as f32;
            self.target_x = mouse_x - self.width / 2.0;
        }

        // Clamp target to screen bounds
        self.target_x = self.target_x.clamp(0.0, SCREEN_WIDTH as f32 - self.width);

        // Smooth lerp movement
        let lerp_speed = 15.0;
        self.x += (self.target_x - self.x) * lerp_speed * dt;
        self.x = self.x.clamp(0.0, SCREEN_WIDTH as f32 - self.width);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, ox: i32, oy: i32) {
        let x = self.x as i32 + ox;
        let y = self.y as i32 + oy;

        // Main paddle body
        d.draw_rectangle(
            x,
            y,
            self.width as i32,
            self.height as i32,
            Color::new(200, 200, 220, 255),
        );

        // Highlight on top
        d.draw_rectangle(
            x + 2,
            y + 2,
            self.width as i32 - 4,
            3,
            Color::new(255, 255, 255, 100),
        );

        // Shadow on bottom
        d.draw_rectangle(
            x + 2,
            y + self.height as i32 - 4,
            self.width as i32 - 4,
            2,
            Color::new(0, 0, 0, 50),
        );
    }

    pub fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }
}
