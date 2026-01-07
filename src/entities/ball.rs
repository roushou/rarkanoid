use raylib::prelude::*;

use crate::entities::Paddle;
use crate::{BALL_INITIAL_SPEED, BALL_MAX_SPEED, BALL_RADIUS, BALL_SPEED_INCREMENT, SCREEN_WIDTH};

const TRAIL_LENGTH: usize = 10;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub speed: f32,
    pub attached: bool,
    trail: [(f32, f32); TRAIL_LENGTH],
    trail_index: usize,
    trail_timer: f32,
}

impl Ball {
    pub fn new_attached(paddle: &Paddle) -> Self {
        Self {
            x: paddle.center_x(),
            y: paddle.y - BALL_RADIUS - 2.0,
            vx: 0.0,
            vy: 0.0,
            radius: BALL_RADIUS,
            speed: BALL_INITIAL_SPEED,
            attached: true,
            trail: [(0.0, 0.0); TRAIL_LENGTH],
            trail_index: 0,
            trail_timer: 0.0,
        }
    }

    pub fn launch(&mut self) {
        if self.attached {
            self.attached = false;
            // Launch at slight angle upward
            let angle = -std::f32::consts::PI / 2.0 + (rand_f32() - 0.5) * 0.5;
            self.vx = angle.sin() * self.speed;
            self.vy = angle.cos() * -self.speed;
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update trail
        self.trail_timer += dt;
        if self.trail_timer >= 0.016 {
            self.trail_timer = 0.0;
            self.trail[self.trail_index] = (self.x, self.y);
            self.trail_index = (self.trail_index + 1) % TRAIL_LENGTH;
        }

        // Move ball
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    pub fn handle_wall_collision(&mut self) {
        // Left wall
        if self.x - self.radius < 0.0 {
            self.x = self.radius;
            self.vx = self.vx.abs();
        }

        // Right wall
        if self.x + self.radius > SCREEN_WIDTH as f32 {
            self.x = SCREEN_WIDTH as f32 - self.radius;
            self.vx = -self.vx.abs();
        }

        // Top wall
        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.vy = self.vy.abs();
        }
    }

    pub fn increase_speed(&mut self) {
        self.speed = (self.speed + BALL_SPEED_INCREMENT).min(BALL_MAX_SPEED);
        // Normalize and rescale velocity
        let len = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if len > 0.0 {
            self.vx = self.vx / len * self.speed;
            self.vy = self.vy / len * self.speed;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, ox: i32, oy: i32) {
        // Draw trail (oldest to newest so newer ones draw on top)
        for i in 0..TRAIL_LENGTH {
            // Calculate index from oldest to newest
            let idx = (self.trail_index + i) % TRAIL_LENGTH;
            let (tx, ty) = self.trail[idx];

            // Skip uninitialized positions
            if tx == 0.0 && ty == 0.0 {
                continue;
            }

            // i=0 is oldest, i=TRAIL_LENGTH-1 is newest
            let t = i as f32 / TRAIL_LENGTH as f32;
            let alpha = (t * 150.0) as u8;
            let size = self.radius * (0.3 + t * 0.7);

            d.draw_circle(
                tx as i32 + ox,
                ty as i32 + oy,
                size,
                Color::new(200, 220, 255, alpha),
            );
        }

        // Draw main ball
        let x = self.x as i32 + ox;
        let y = self.y as i32 + oy;

        // Glow effect
        d.draw_circle(x, y, self.radius + 3.0, Color::new(200, 220, 255, 40));
        d.draw_circle(x, y, self.radius + 1.5, Color::new(255, 255, 255, 60));

        // Main ball
        d.draw_circle(x, y, self.radius, Color::WHITE);

        // Highlight
        d.draw_circle(
            x - (self.radius * 0.3) as i32,
            y - (self.radius * 0.3) as i32,
            self.radius * 0.3,
            Color::new(255, 255, 255, 180),
        );
    }
}

// Simple random function using raylib
fn rand_f32() -> f32 {
    unsafe { raylib::ffi::GetRandomValue(0, 1000) as f32 / 1000.0 }
}
