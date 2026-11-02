use glam::Vec2;

use crate::{
    ball::{create_ball, move_ball},
    r#match::*,
    player::*,
    team::*,
};

pub mod ball;
pub mod r#match;
pub mod pitch;
pub mod player;
pub mod team;
pub const DELTA_TIME: f32 = 0.1; //seconds
pub const FRICTION: f32 = 2.0;
pub const POSSESION_RADIUS: f32 = 1.0;
pub const PASS_SPEED: f32 = 22.0;
pub const SHOT_SPEED: f32 = 25.0;
//ptch
pub const PITCH_LENGTH: f32 = 105.0;
pub const PITCH_HEIGHT: f32 = 68.0;
pub const GOAL_LENGHT: f32 = 7.32;

//MATCH
pub const MATCHSPEED: f32 = 1.0;
pub const TIMESTEP: f32 = 1.0 / 60.0;

pub type PlayerId = u32;
pub type TeamId = u32;
pub fn lib() {
    let ball = &mut create_ball();
    let mut player_map = create_and_store_players();
    let mut teams = team::create_match_teams();
    let mut match_state = MatchState {
        match_phase: MatchPhase::FirstHalf,
        score: MatchScore { home: 0, away: 0 },
        play_state: PlayState::AwaitingKickOff,
        goal_scored_duration: 0.2,
        goal_scored_elapsed: 0.0,
        match_elapsed_time: 0.0,
        goal_presentation_started_at: None,
        replay_skipped: false,
        home_team_id: 1,
        away_team_id: 2,
        kickoff_team_id: 1,
        opening_kickoff_team_id: 1,
    };

    let mut tick = 0;

    while match_state.match_phase != MatchPhase::FullTime {
        println!("tick: {}", tick);

        // Only simulate gameplay while the match is actively playing.
        if matches!(
            match_state.match_phase,
            MatchPhase::FirstHalf | MatchPhase::SecondHalf
        ) && matches!(match_state.play_state, PlayState::Playing)
        {
            shooting_the_ball(ball, &mut player_map, &match_state.play_state);
            let previous_pos = Vec2::new(ball.position.x, ball.position.y);
            move_ball(ball, &match_state.play_state);
            let current_pos = Vec2::new(ball.position.x, ball.position.y);

            if let Some(match_event) = determine_goal_scoring_team_and_player(
                &previous_pos,
                &current_pos,
                &teams,
                ball,
                &mut player_map,
                &match_state.play_state,
            ) {
                handle_match_events_goal_scored(&mut match_state, &match_event);
            }

            tick_match_clock(&mut match_state);
        }
        // Handle kickoff setup when the game is paused waiting for a kickoff
        else if matches!(match_state.play_state, PlayState::AwaitingKickOff) {
            prepare_kickoff(ball, &mut match_state);

            start_kickoff(&mut match_state);
        }
        // Handle the half-time break transition
        else if matches!(match_state.match_phase, MatchPhase::HalfTime) {
            println!("--- HALF TIME ---");
            handle_half_times(&mut match_state, &mut teams);
        }

        // Handle goal presentation even when gameplay is paused.
        tick_goal_presentation_timer(&mut match_state);

        tick += 1;
    }
}
