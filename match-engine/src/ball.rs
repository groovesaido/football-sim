use std::collections::HashMap;

use glam::Vec2;

use crate::{DELTA_TIME, FRICTION, PlayerId, r#match::PlayState, pitch::Pitch, player::Player};

#[derive(PartialEq, Eq)]
pub enum BallState {
    Free,
    Possesed(PlayerId),
    InFlight,
    OutOfPlay,
}
pub struct Ball {
    pub position: Pitch,
    pub velocity: Vec2,
    pub state: BallState,
    pub last_touched_by: Option<PlayerId>,
}

pub fn create_ball() -> Ball {
    let ball = Ball {
        position: Pitch { x: 70.0, y: 60.0 },
        velocity: Vec2 { x: 20.0, y: 10.0 },
        state: BallState::Possesed(3),
        last_touched_by: None,
    };
    ball
}

pub fn update_ball_pos_to_possesing_player(
    ball: &mut Ball,
    player_map: &HashMap<PlayerId, Player>,
    play_state: &PlayState,
) {
    // check if we are in playing state
    if !matches!(play_state, PlayState::Playing) {
        return;
    }
    if let BallState::Possesed(id) = ball.state {
        if let Some(player) = player_map.get(&id) {
            println!("Found player ID: {}", id);

            ball.position.x = player.position.x;
            ball.position.y = player.position.y;

            println!("Ball pos: {}, {}", ball.position.x, ball.position.y);
            ball.last_touched_by = Some(id);
        } else {
            println!("Player not found");
        }
    }
}

pub fn move_ball(ball: &mut Ball, play_state: &PlayState) {
    // check if we are in playing state
    if !matches!(play_state, PlayState::Playing) {
        return;
    }
    if ball.velocity.x == 0.0 && ball.velocity.y == 0.0 {
        return;
    }
    if matches!(ball.state, BallState::Free | BallState::InFlight) {
        //friction
        let initial_speed = ball.velocity.length();
        let deceleration_per_tick = FRICTION * DELTA_TIME;
        let new_speed = (initial_speed - deceleration_per_tick).max(0.0);
        let norm_dir = ball.velocity.normalize_or_zero();
        let new_velocity = norm_dir * new_speed;
        ball.velocity = new_velocity;
        ball.position.x += new_velocity.x * DELTA_TIME;
        ball.position.y += new_velocity.y * DELTA_TIME;
        println!(
            "Ball moving at vel:{},{} position:{},{}",
            ball.velocity.x, ball.velocity.y, ball.position.x, ball.position.y
        );
    }
}
