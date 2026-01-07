use raylib::prelude::*;

use crate::SCREEN_WIDTH;

pub fn draw_hud(d: &mut RaylibDrawHandle, score: i32, lives: i32, level: usize, combo: i32) {
    let font_size = 20;

    // Score (left)
    d.draw_text(
        &format!("SCORE: {}", score),
        10,
        10,
        font_size,
        Color::WHITE,
    );

    // Lives (center)
    let lives_text = format!("LIVES: {}", lives);
    let lives_width = d.measure_text(&lives_text, font_size);
    d.draw_text(
        &lives_text,
        (SCREEN_WIDTH - lives_width) / 2,
        10,
        font_size,
        Color::WHITE,
    );

    // Level (right)
    let level_text = format!("LEVEL: {}", level);
    let level_width = d.measure_text(&level_text, font_size);
    d.draw_text(
        &level_text,
        SCREEN_WIDTH - level_width - 10,
        10,
        font_size,
        Color::WHITE,
    );

    // Combo indicator (if active)
    if combo > 1 {
        let combo_text = format!("COMBO x{}", combo);
        let combo_width = d.measure_text(&combo_text, font_size + 4);
        let alpha = ((combo as f32 * 30.0).min(255.0)) as u8;
        d.draw_text(
            &combo_text,
            (SCREEN_WIDTH - combo_width) / 2,
            35,
            font_size + 4,
            Color::new(255, 255, 100, alpha),
        );
    }
}
