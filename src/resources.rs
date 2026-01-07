use std::ffi::CString;

pub struct SoundEffects {
    paddle_hit: Option<raylib::ffi::Sound>,
    brick_hit: Option<raylib::ffi::Sound>,
    brick_destroy: Option<raylib::ffi::Sound>,
    lose_life: Option<raylib::ffi::Sound>,
    level_complete: Option<raylib::ffi::Sound>,
    audio_initialized: bool,
}

impl SoundEffects {
    pub fn new() -> Self {
        // Initialize audio device
        unsafe {
            raylib::ffi::InitAudioDevice();
        }

        Self {
            paddle_hit: Self::load_sound("assets/sounds/paddle_hit.wav"),
            brick_hit: Self::load_sound("assets/sounds/brick_hit.wav"),
            brick_destroy: Self::load_sound("assets/sounds/brick_destroy.wav"),
            lose_life: Self::load_sound("assets/sounds/lose_life.wav"),
            level_complete: Self::load_sound("assets/sounds/level_complete.wav"),
            audio_initialized: true,
        }
    }

    fn load_sound(path: &str) -> Option<raylib::ffi::Sound> {
        if std::path::Path::new(path).exists() {
            let c_path = CString::new(path).ok()?;
            unsafe {
                let sound = raylib::ffi::LoadSound(c_path.as_ptr());
                if sound.frameCount > 0 {
                    Some(sound)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn play_paddle_hit(&self) {
        if let Some(sound) = self.paddle_hit {
            unsafe {
                raylib::ffi::PlaySound(sound);
            }
        }
    }

    pub fn play_brick_hit(&self) {
        if let Some(sound) = self.brick_hit {
            unsafe {
                raylib::ffi::PlaySound(sound);
            }
        }
    }

    pub fn play_brick_destroy(&self) {
        if let Some(sound) = self.brick_destroy {
            unsafe {
                raylib::ffi::PlaySound(sound);
            }
        }
    }

    pub fn play_lose_life(&self) {
        if let Some(sound) = self.lose_life {
            unsafe {
                raylib::ffi::PlaySound(sound);
            }
        }
    }

    pub fn play_level_complete(&self) {
        if let Some(sound) = self.level_complete {
            unsafe {
                raylib::ffi::PlaySound(sound);
            }
        }
    }
}

impl Drop for SoundEffects {
    fn drop(&mut self) {
        unsafe {
            if let Some(sound) = self.paddle_hit {
                raylib::ffi::UnloadSound(sound);
            }
            if let Some(sound) = self.brick_hit {
                raylib::ffi::UnloadSound(sound);
            }
            if let Some(sound) = self.brick_destroy {
                raylib::ffi::UnloadSound(sound);
            }
            if let Some(sound) = self.lose_life {
                raylib::ffi::UnloadSound(sound);
            }
            if let Some(sound) = self.level_complete {
                raylib::ffi::UnloadSound(sound);
            }
            if self.audio_initialized {
                raylib::ffi::CloseAudioDevice();
            }
        }
    }
}
