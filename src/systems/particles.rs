use raylib::prelude::*;

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    lifetime: f32,
    max_lifetime: f32,
    size: f32,
    active: bool,
}

impl Particle {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: Color::WHITE,
            lifetime: 0.0,
            max_lifetime: 1.0,
            size: 4.0,
            active: false,
        }
    }

    fn spawn(&mut self, x: f32, y: f32, vx: f32, vy: f32, color: Color, lifetime: f32, size: f32) {
        self.x = x;
        self.y = y;
        self.vx = vx;
        self.vy = vy;
        self.color = color;
        self.lifetime = lifetime;
        self.max_lifetime = lifetime;
        self.size = size;
        self.active = true;
    }

    fn update(&mut self, dt: f32) {
        if !self.active {
            return;
        }

        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Gravity
        self.vy += 200.0 * dt;

        // Friction
        self.vx *= 0.99;

        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            self.active = false;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, ox: i32, oy: i32) {
        if !self.active {
            return;
        }

        let t = self.lifetime / self.max_lifetime;
        let alpha = (t * 255.0) as u8;
        // Keep size at least 50% of original
        let size = self.size * (0.5 + t * 0.5);
        let color = Color::new(self.color.r, self.color.g, self.color.b, alpha);

        d.draw_rectangle(
            (self.x - size / 2.0) as i32 + ox,
            (self.y - size / 2.0) as i32 + oy,
            size.max(2.0) as i32,
            size.max(2.0) as i32,
            color,
        );
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new(capacity: usize) -> Self {
        let mut particles = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            particles.push(Particle::new());
        }
        Self { particles }
    }

    pub fn spawn_explosion(&mut self, x: f32, y: f32, color: Color, count: usize) {
        let mut spawned = 0;

        for particle in &mut self.particles {
            if !particle.active {
                let angle = rand_f32() * std::f32::consts::PI * 2.0;
                let speed = 100.0 + rand_f32() * 200.0;
                let vx = angle.cos() * speed;
                let vy = angle.sin() * speed - 100.0; // Bias upward

                let lifetime = 0.5 + rand_f32() * 0.5;
                let size = 6.0 + rand_f32() * 8.0;

                // Vary the color slightly, keep it bright
                let r = (color.r as i32 + (rand_f32() * 60.0 - 30.0) as i32).clamp(50, 255) as u8;
                let g = (color.g as i32 + (rand_f32() * 60.0 - 30.0) as i32).clamp(50, 255) as u8;
                let b = (color.b as i32 + (rand_f32() * 60.0 - 30.0) as i32).clamp(50, 255) as u8;
                let particle_color = Color::new(r, g, b, 255);

                particle.spawn(x, y, vx, vy, particle_color, lifetime, size);

                spawned += 1;
                if spawned >= count {
                    break;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, ox: i32, oy: i32) {
        for particle in &self.particles {
            particle.draw(d, ox, oy);
        }
    }
}

fn rand_f32() -> f32 {
    unsafe { raylib::ffi::GetRandomValue(0, 10000) as f32 / 10000.0 }
}
