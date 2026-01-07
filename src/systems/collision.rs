use crate::entities::{Ball, Brick, Paddle};

/// Check collision between ball and paddle, applying reflection if hit.
/// Returns true if collision occurred.
pub fn check_ball_paddle_collision(ball: &mut Ball, paddle: &Paddle) -> bool {
    // Circle vs AABB collision
    let closest_x = ball.x.clamp(paddle.x, paddle.x + paddle.width);
    let closest_y = ball.y.clamp(paddle.y, paddle.y + paddle.height);

    let dist_x = ball.x - closest_x;
    let dist_y = ball.y - closest_y;
    let distance_sq = dist_x * dist_x + dist_y * dist_y;

    if distance_sq < ball.radius * ball.radius {
        // Collision detected - only bounce if ball is moving downward
        if ball.vy > 0.0 {
            // Calculate hit position relative to paddle center (-1 to 1)
            let hit_pos = (ball.x - paddle.center_x()) / (paddle.width / 2.0);
            let hit_pos = hit_pos.clamp(-1.0, 1.0);

            // Map to reflection angle (-60° to +60° from vertical)
            let max_angle = std::f32::consts::PI / 3.0; // 60 degrees
            let angle = hit_pos * max_angle;

            // Set new velocity
            ball.vx = angle.sin() * ball.speed;
            ball.vy = -angle.cos() * ball.speed;

            // Push ball out of paddle
            ball.y = paddle.y - ball.radius - 1.0;

            return true;
        }
    }

    false
}

/// Check collision between ball and brick.
/// Returns Some(score) if collision occurred and brick was damaged.
pub fn check_ball_brick_collision(ball: &mut Ball, brick: &mut Brick) -> Option<i32> {
    // Update brick flash timer
    brick.update(ball.speed * 0.0001); // rough dt approximation

    // Skip destroyed bricks
    if brick.health <= 0 && !brick.indestructible {
        return None;
    }

    // Circle vs AABB collision
    let closest_x = ball.x.clamp(brick.x, brick.x + brick.width);
    let closest_y = ball.y.clamp(brick.y, brick.y + brick.height);

    let dist_x = ball.x - closest_x;
    let dist_y = ball.y - closest_y;
    let distance_sq = dist_x * dist_x + dist_y * dist_y;

    if distance_sq < ball.radius * ball.radius {
        // Determine which face was hit by comparing overlaps
        let overlap_left = (ball.x + ball.radius) - brick.x;
        let overlap_right = (brick.x + brick.width) - (ball.x - ball.radius);
        let overlap_top = (ball.y + ball.radius) - brick.y;
        let overlap_bottom = (brick.y + brick.height) - (ball.y - ball.radius);

        let min_overlap_x = overlap_left.min(overlap_right);
        let min_overlap_y = overlap_top.min(overlap_bottom);

        if min_overlap_x < min_overlap_y {
            // Horizontal collision
            ball.vx = -ball.vx;
            // Push ball out
            if overlap_left < overlap_right {
                ball.x = brick.x - ball.radius;
            } else {
                ball.x = brick.x + brick.width + ball.radius;
            }
        } else {
            // Vertical collision
            ball.vy = -ball.vy;
            // Push ball out
            if overlap_top < overlap_bottom {
                ball.y = brick.y - ball.radius;
            } else {
                ball.y = brick.y + brick.height + ball.radius;
            }
        }

        // Damage brick and get score
        let score = brick.damage();

        // Increase ball speed on brick hit
        if score > 0 {
            ball.increase_speed();
        }

        return Some(score);
    }

    None
}
