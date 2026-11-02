use std::collections::HashMap;

use crate::{
    DELTA_TIME, GOAL_LENGHT, PASS_SPEED, PITCH_HEIGHT, PITCH_LENGTH, POSSESION_RADIUS, PlayerId,
    SHOT_SPEED,
    ball::{Ball, BallState, update_ball_pos_to_possesing_player},
    r#match::{MatchState, PlayState},
    pitch::Pitch,
    team::{AttackingDirection, Team},
};
use glam::Vec2;
pub struct Player {
    pub id: PlayerId,
    pub team_id: u32,
    pub position: Pitch,
    pub speed: f32,
    pub targetpos: Pitch,
}

pub fn move_player(player: &mut Player) {
    let cur_position = Vec2::new(player.position.x, player.position.y);
    let target_pos = Vec2::new(player.targetpos.x, player.targetpos.y);
    let dist = glam::Vec2::distance(target_pos, cur_position);
    //clamp movement
    if dist < 0.1 {
        player.position = Pitch {
            x: target_pos.x,
            y: target_pos.y,
        };
        return;
    }
    let mut dir = target_pos - cur_position;

    if let Some(normalized_dir) = dir.try_normalize() {
        dir = normalized_dir;
    }

    let step = player.speed * DELTA_TIME;
    if step >= dist {
        player.position = Pitch {
            x: target_pos.x,
            y: target_pos.y,
        };
        return;
    } else {
        player.position = Pitch {
            x: cur_position.x + step * dir.x,
            y: cur_position.y + step * dir.y,
        };
    }
}

pub fn create_and_store_players() -> HashMap<u32, Player> {
    let player1 = Player {
        id: 1,
        team_id: 1,
        position: Pitch { x: 30.0, y: 20.0 },
        speed: 5.0,
        targetpos: Pitch { x: 70.0, y: 60.0 },
    };
    let player2 = Player {
        id: 2,
        team_id: 1,
        position: Pitch { x: 80.0, y: 60.0 },
        speed: 4.9,
        targetpos: Pitch { x: 180.0, y: 115.0 },
    };
    let player3 = Player {
        id: 3,
        team_id: 1,
        position: Pitch { x: 70.0, y: 52.0 },
        speed: 5.5,
        targetpos: Pitch { x: 6.0, y: 8.0 },
    };
    let mut players = HashMap::new();
    players.insert(player1.id, player1);
    players.insert(player2.id, player2);
    players.insert(player3.id, player3);
    players
}

pub fn ball_possesion(ball: &mut Ball, players: &HashMap<PlayerId, Player>) {
    let mut min_distance = POSSESION_RADIUS;
    let mut closest_player: Option<PlayerId> = None;
    let ball_pos = Vec2::new(ball.position.x, ball.position.y);
    if matches!(ball.state, BallState::Free | BallState::InFlight) {
        for (id, player) in players {
            let player_pos = Vec2::new(player.position.x, player.position.y);
            let distance_between = player_pos.distance(ball_pos);
            if distance_between <= POSSESION_RADIUS && distance_between < min_distance {
                min_distance = distance_between;
                closest_player = Some(*id);
            }
        }
    }
    if let Some(id) = closest_player {
        ball.state = BallState::Possesed(id);
        ball.velocity = Vec2 { x: 0.0, y: 0.0 };
        println!(
            "pass received,posessed by playerid{} at pos:{},{}",
            id, ball.position.x, ball.position.y
        );
        ball.last_touched_by = Some(id);
    }
}

pub fn passing_the_ball(
    ball: &mut Ball,
    player_map: &HashMap<PlayerId, Player>,
    tar_player: &Player,
    play_state: &PlayState,
) {
    // check if we are in playing state
    if !matches!(play_state, PlayState::Playing) {
        return;
    }
    if let BallState::Possesed(_) = ball.state {
        update_ball_pos_to_possesing_player(ball, player_map, play_state);

        let ball_pos = Vec2::new(ball.position.x, ball.position.y);
        let tar_pos = Vec2::new(tar_player.position.x, tar_player.position.y);

        let direction = tar_pos - ball_pos;

        // Reject a pass if the receiver is too close.
        if direction.length() < 0.001 {
            return;
        }

        let norm_dir = direction.normalize_or_zero();
        let velocity = norm_dir * PASS_SPEED;

        ball.velocity = velocity;
        println!("Ball passed");
        ball.state = BallState::InFlight;
        println!("bal vel:{:?} ", velocity);
    }
}

pub fn shooting_the_ball(
    ball: &mut Ball,
    player_map: &HashMap<PlayerId, Player>,
    play_state: &PlayState,
) {
    // check if we are in playing state
    if !matches!(play_state, PlayState::Playing) {
        return;
    }
    if let BallState::Possesed(_) = ball.state {
        update_ball_pos_to_possesing_player(ball, player_map, play_state);
        let ball_pos = Vec2::new(ball.position.x, ball.position.y);
        //attacking right goal
        let goal_target_pos = Vec2::new(PITCH_LENGTH, PITCH_HEIGHT / 2.0);
        //attacking lef
        //let goal_target_pos = Vec2::new(0.0, 34.0);
        let dir = goal_target_pos - ball_pos;
        if dir.length() < 0.001 {
            return;
        }
        let nor_dir = dir.normalize_or_zero();
        let velocity = nor_dir * SHOT_SPEED;
        ball.velocity = velocity;
        ball.state = BallState::InFlight;
        println!("Ball shot at vel:{:?}  ball pos:{:?}", velocity, ball_pos)
    }
}

pub fn goal_detection(
    previous_pos: &Vec2,
    current_pos: &Vec2,
    play_state: &PlayState,
) -> Option<AttackingDirection> {
    // check if we are in playing state
    if !matches!(play_state, PlayState::Playing) {
        return None;
    }
    // 1. Check for a goal on the Right side (AttackingDirection::Right)
    let right_crossed = previous_pos.x < PITCH_LENGTH && current_pos.x >= PITCH_LENGTH;
    if right_crossed {
        let t = (PITCH_LENGTH - previous_pos.x) / (current_pos.x - previous_pos.x);
        let ycross = previous_pos.y + t * (current_pos.y - previous_pos.y);

        let goal_min = PITCH_HEIGHT / 2.0 - GOAL_LENGHT / 2.0;
        let goal_max = PITCH_HEIGHT / 2.0 + GOAL_LENGHT / 2.0;

        if ycross > goal_min && ycross < goal_max {
            return Some(AttackingDirection::Right);
        }
    }

    // 2. Check for a goal on the Left side (AttackingDirection::Left)
    let left_crossed = previous_pos.x > 0.0 && current_pos.x <= 0.0;
    if left_crossed {
        let t = (0.0 - previous_pos.x) / (current_pos.x - previous_pos.x);
        let ycross = previous_pos.y + t * (current_pos.y - previous_pos.y);

        let goal_min = PITCH_HEIGHT / 2.0 - GOAL_LENGHT / 2.0;
        let goal_max = PITCH_HEIGHT / 2.0 + GOAL_LENGHT / 2.0;

        if ycross > goal_min && ycross < goal_max {
            return Some(AttackingDirection::Left);
        }
    }

    // No goal detected on either side
    None
}
