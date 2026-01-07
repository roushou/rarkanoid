use raylib::prelude::*;

pub struct Camera {
    shake_intensity: f32,
    shake_duration: f32,
    shake_timer: f32,
    offset: Vector2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            shake_intensity: 0.0,
            shake_duration: 0.0,
            shake_timer: 0.0,
            offset: Vector2::zero(),
        }
    }

    pub fn shake(&mut self, intensity: f32, duration: f32) {
        // Only override if new shake is stronger
        if intensity > self.shake_intensity {
            self.shake_intensity = intensity;
            self.shake_duration = duration;
            self.shake_timer = duration;
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.shake_timer > 0.0 {
            self.shake_timer -= dt;

            // Calculate current intensity (decays over time)
            let t = self.shake_timer / self.shake_duration;
            let current_intensity = self.shake_intensity * t;

            // Random offset
            self.offset.x = (rand_f32() - 0.5) * 2.0 * current_intensity;
            self.offset.y = (rand_f32() - 0.5) * 2.0 * current_intensity;
        } else {
            self.shake_intensity = 0.0;
            self.offset = Vector2::zero();
        }
    }

    pub fn get_offset(&self) -> Vector2 {
        self.offset
    }
}

fn rand_f32() -> f32 {
    unsafe { raylib::ffi::GetRandomValue(0, 10000) as f32 / 10000.0 }
}
