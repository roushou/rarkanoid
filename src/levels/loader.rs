use raylib::prelude::*;

use crate::entities::{Brick, BrickType};
use crate::{BRICK_HEIGHT, BRICK_OFFSET_LEFT, BRICK_OFFSET_TOP, BRICK_PADDING, BRICK_WIDTH};

// Predefined level layouts
const LEVEL_1: &str = r#"
0 0 1 1 1 1 1 1 0 0
0 1 1 1 1 1 1 1 1 0
1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1
0 1 1 1 1 1 1 1 1 0
0 0 1 1 1 1 1 1 0 0
"#;

const LEVEL_2: &str = r#"
9 0 0 0 0 0 0 0 0 9
0 2 2 1 1 1 1 2 2 0
0 2 1 1 1 1 1 1 2 0
0 1 1 2 2 2 2 1 1 0
0 1 1 1 1 1 1 1 1 0
0 2 2 1 1 1 1 2 2 0
9 0 0 0 0 0 0 0 0 9
"#;

const LEVEL_3: &str = r#"
1 1 9 1 1 1 1 9 1 1
2 2 9 2 2 2 2 9 2 2
1 1 9 1 1 1 1 9 1 1
2 2 9 2 2 2 2 9 2 2
1 1 9 1 1 1 1 9 1 1
2 2 9 2 2 2 2 9 2 2
1 1 9 1 1 1 1 9 1 1
"#;

// Color palette for bricks based on row
const BRICK_COLORS: [Color; 7] = [
    Color::new(255, 100, 100, 255), // Red
    Color::new(255, 180, 100, 255), // Orange
    Color::new(255, 255, 100, 255), // Yellow
    Color::new(100, 255, 100, 255), // Green
    Color::new(100, 200, 255, 255), // Cyan
    Color::new(150, 100, 255, 255), // Purple
    Color::new(255, 100, 200, 255), // Pink
];

const HARD_BRICK_COLOR: Color = Color::new(180, 180, 200, 255);
const INDESTRUCTIBLE_COLOR: Color = Color::new(120, 120, 140, 255);

pub fn load_level(level: usize) -> Vec<Brick> {
    let level_data = match level {
        1 => LEVEL_1,
        2 => LEVEL_2,
        3 => LEVEL_3,
        _ => return Vec::new(), // No more levels
    };

    parse_level(level_data)
}

fn parse_level(data: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();

    for (row, line) in data.trim().lines().enumerate() {
        for (col, value) in line.split_whitespace().enumerate() {
            let brick_type = match value {
                "0" => continue, // Empty space
                "1" => BrickType::Normal,
                "2" => BrickType::Hard,
                "9" => BrickType::Indestructible,
                _ => continue,
            };

            let x = BRICK_OFFSET_LEFT + col as f32 * (BRICK_WIDTH + BRICK_PADDING);
            let y = BRICK_OFFSET_TOP + row as f32 * (BRICK_HEIGHT + BRICK_PADDING);

            let color = match brick_type {
                BrickType::Normal => BRICK_COLORS[row % BRICK_COLORS.len()],
                BrickType::Hard => HARD_BRICK_COLOR,
                BrickType::Indestructible => INDESTRUCTIBLE_COLOR,
            };

            bricks.push(Brick::new(
                x,
                y,
                BRICK_WIDTH,
                BRICK_HEIGHT,
                brick_type,
                color,
            ));
        }
    }

    bricks
}
