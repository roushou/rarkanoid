use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BrickType {
    Normal,
    Hard,
    Indestructible,
}

pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub brick_type: BrickType,
    pub health: i32,
    pub max_health: i32,
    pub color: Color,
    pub indestructible: bool,
    pub flash_timer: f32,
}

impl Brick {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        brick_type: BrickType,
        color: Color,
    ) -> Self {
        let (health, indestructible) = match brick_type {
            BrickType::Normal => (1, false),
            BrickType::Hard => (2, false),
            BrickType::Indestructible => (999, true),
        };

        Self {
            x,
            y,
            width,
            height,
            brick_type,
            health,
            max_health: health,
            color,
            indestructible,
            flash_timer: 0.0,
        }
    }

    pub fn damage(&mut self) -> i32 {
        if self.indestructible {
            self.flash_timer = 0.1;
            return 0;
        }

        self.health -= 1;
        self.flash_timer = 0.1;

        if self.health <= 0 {
            100 // Score for destroying brick
        } else {
            25 // Score for damaging brick
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, ox: i32, oy: i32) {
        let x = self.x as i32 + ox;
        let y = self.y as i32 + oy;
        let w = self.width as i32;
        let h = self.height as i32;

        // Determine draw color (flash white on hit)
        let draw_color = if self.flash_timer > 0.0 {
            Color::WHITE
        } else {
            self.color
        };

        // Main brick body
        d.draw_rectangle(x, y, w, h, draw_color);

        // Border/outline
        d.draw_rectangle_lines(x, y, w, h, Color::new(0, 0, 0, 100));

        // Highlight on top-left
        d.draw_line(
            x + 2,
            y + 2,
            x + w - 2,
            y + 2,
            Color::new(255, 255, 255, 80),
        );
        d.draw_line(
            x + 2,
            y + 2,
            x + 2,
            y + h - 2,
            Color::new(255, 255, 255, 80),
        );

        // Shadow on bottom-right
        d.draw_line(
            x + 2,
            y + h - 2,
            x + w - 2,
            y + h - 2,
            Color::new(0, 0, 0, 80),
        );
        d.draw_line(
            x + w - 2,
            y + 2,
            x + w - 2,
            y + h - 2,
            Color::new(0, 0, 0, 80),
        );

        // Draw cracks for damaged hard bricks
        if self.brick_type == BrickType::Hard && self.health < self.max_health {
            self.draw_cracks(d, x, y, w, h);
        }

        // Draw metal appearance for indestructible
        if self.indestructible {
            // Diagonal lines pattern
            for i in 0..((w + h) / 8) {
                let start_x = x + i * 8;
                let start_y = y;
                let end_x = start_x - h;
                let end_y = y + h;
                d.draw_line(
                    start_x.max(x).min(x + w),
                    start_y,
                    end_x.max(x).min(x + w),
                    end_y.min(y + h),
                    Color::new(255, 255, 255, 30),
                );
            }
        }
    }

    fn draw_cracks(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32) {
        let crack_color = Color::new(0, 0, 0, 150);

        // Draw some crack lines
        d.draw_line(x + w / 3, y + 2, x + w / 2, y + h / 2, crack_color);
        d.draw_line(x + w / 2, y + h / 2, x + w * 2 / 3, y + h - 2, crack_color);
        d.draw_line(x + w / 2, y + h / 2, x + w - 5, y + h / 3, crack_color);
    }

    pub fn update(&mut self, dt: f32) {
        if self.flash_timer > 0.0 {
            self.flash_timer -= dt;
        }
    }
}
