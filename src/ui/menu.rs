use raylib::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub enum MenuType {
    Title,
    Pause,
    LevelComplete(usize),
    GameOver(i32),
}

pub fn draw_menu(d: &mut RaylibDrawHandle, menu_type: MenuType) {
    match menu_type {
        MenuType::Title => draw_title_screen(d),
        MenuType::Pause => draw_pause_screen(d),
        MenuType::LevelComplete(level) => draw_level_complete(d, level),
        MenuType::GameOver(score) => draw_game_over(d, score),
    }
}

fn draw_title_screen(d: &mut RaylibDrawHandle) {
    // Darken background
    d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 200));

    // Title
    let title = "RARKANOID";
    let title_size = 60;
    let title_width = d.measure_text(title, title_size);
    d.draw_text(
        title,
        (SCREEN_WIDTH - title_width) / 2,
        SCREEN_HEIGHT / 2 - 80,
        title_size,
        Color::WHITE,
    );

    // Subtitle
    let subtitle = "A Breakout Clone";
    let subtitle_size = 20;
    let subtitle_width = d.measure_text(subtitle, subtitle_size);
    d.draw_text(
        subtitle,
        (SCREEN_WIDTH - subtitle_width) / 2,
        SCREEN_HEIGHT / 2 - 10,
        subtitle_size,
        Color::new(200, 200, 200, 255),
    );

    // Instructions
    draw_blinking_text(d, "Press SPACE or CLICK to Start", SCREEN_HEIGHT / 2 + 60);

    // Controls
    let controls = "Controls: A/D or Arrow Keys | Mouse";
    let controls_size = 16;
    let controls_width = d.measure_text(controls, controls_size);
    d.draw_text(
        controls,
        (SCREEN_WIDTH - controls_width) / 2,
        SCREEN_HEIGHT - 40,
        controls_size,
        Color::new(150, 150, 150, 255),
    );
}

fn draw_pause_screen(d: &mut RaylibDrawHandle) {
    // Semi-transparent overlay
    d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 150));

    let text = "PAUSED";
    let size = 50;
    let width = d.measure_text(text, size);
    d.draw_text(
        text,
        (SCREEN_WIDTH - width) / 2,
        SCREEN_HEIGHT / 2 - 40,
        size,
        Color::WHITE,
    );

    draw_blinking_text(d, "Press ESC or P to Resume", SCREEN_HEIGHT / 2 + 30);
}

fn draw_level_complete(d: &mut RaylibDrawHandle, level: usize) {
    // Semi-transparent overlay
    d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 150));

    let text = format!("LEVEL {} COMPLETE!", level);
    let size = 40;
    let width = d.measure_text(&text, size);
    d.draw_text(
        &text,
        (SCREEN_WIDTH - width) / 2,
        SCREEN_HEIGHT / 2 - 40,
        size,
        Color::new(100, 255, 100, 255),
    );

    draw_blinking_text(d, "Press SPACE to Continue", SCREEN_HEIGHT / 2 + 30);
}

fn draw_game_over(d: &mut RaylibDrawHandle, score: i32) {
    // Darken background
    d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 220));

    let text = "GAME OVER";
    let size = 50;
    let width = d.measure_text(text, size);
    d.draw_text(
        text,
        (SCREEN_WIDTH - width) / 2,
        SCREEN_HEIGHT / 2 - 60,
        size,
        Color::new(255, 100, 100, 255),
    );

    let score_text = format!("Final Score: {}", score);
    let score_size = 30;
    let score_width = d.measure_text(&score_text, score_size);
    d.draw_text(
        &score_text,
        (SCREEN_WIDTH - score_width) / 2,
        SCREEN_HEIGHT / 2,
        score_size,
        Color::WHITE,
    );

    draw_blinking_text(d, "Press SPACE to Return to Menu", SCREEN_HEIGHT / 2 + 60);
}

fn draw_blinking_text(d: &mut RaylibDrawHandle, text: &str, y: i32) {
    let time = unsafe { raylib::ffi::GetTime() as f32 };
    let alpha = ((time * 3.0).sin() * 0.5 + 0.5) * 255.0;

    let size = 20;
    let width = d.measure_text(text, size);
    d.draw_text(
        text,
        (SCREEN_WIDTH - width) / 2,
        y,
        size,
        Color::new(255, 255, 255, alpha as u8),
    );
}
