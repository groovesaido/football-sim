use glam::Vec2;

use crate::{
    DELTA_TIME, MATCHSPEED, PITCH_HEIGHT, PITCH_LENGTH, PlayerId, TIMESTEP, TeamId,
    ball::{Ball, BallState},
    player::Player,
    team::{Team, switch_attacking_direction, switch_kickoffteam},
};
use std::{
    collections::HashMap,
    thread::sleep,
    time::{Duration, Instant},
};
pub enum MatchEvent {
    Goal {
        team_id: TeamId,
        player_id: Option<PlayerId>,
        own_goal: bool,
    },
}
pub struct MatchScore {
    pub home: u8,
    pub away: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchPhase {
    Starting,
    FirstHalf,
    HalfTime,
    SecondHalf,
    FullTime,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    Playing,
    GoalScored,
    AwaitingKickOff,
}
pub struct MatchState {
    pub match_phase: MatchPhase,
    pub score: MatchScore,
    pub play_state: PlayState,
    pub match_elapsed_time: f32,
    pub goal_scored_elapsed: f32,
    pub goal_scored_duration: f32,
    pub goal_presentation_started_at: Option<Instant>,
    pub replay_skipped: bool,
    pub home_team_id: TeamId,
    pub away_team_id: TeamId,
    pub kickoff_team_id: TeamId,
    pub opening_kickoff_team_id: TeamId,
}

pub fn handle_match_events_goal_scored(match_state: &mut MatchState, match_event: &MatchEvent) {
    if !matches!(match_state.play_state, PlayState::Playing) {
        return;
    }

    let MatchEvent::Goal { team_id, .. } = match_event;

    if *team_id == match_state.home_team_id {
        //add score
        match_state.score.home += 1;
        //assign kickoff to away team
        match_state.kickoff_team_id = match_state.away_team_id;
    } else if *team_id == match_state.away_team_id {
        //add score
        match_state.score.away += 1;
        //assign kickoff to home team
        match_state.kickoff_team_id = match_state.home_team_id;
    } else {
        return;
    }
    println!(
        "Home:{},Away:{}",
        match_state.score.home, match_state.score.away
    );
    match_state.play_state = PlayState::GoalScored;
}
pub fn tick_goal_presentation_timer(match_state: &mut MatchState) {
    //ticks a 60-sec skipable timer for goal replays and celebrations
    if !matches!(match_state.play_state, PlayState::GoalScored) {
        return;
    }
    // If skipped, end the presentation immediately.
    if match_state.replay_skipped {
        match_state.play_state = PlayState::AwaitingKickOff;
        match_state.goal_presentation_started_at = None;
        return;
    }
    //init timer
    let started_at = *match_state
        .goal_presentation_started_at
        .get_or_insert_with(Instant::now);
    //check time since timer started
    let elapsed = started_at.elapsed().as_secs_f32();

    if elapsed >= match_state.goal_scored_duration {
        match_state.play_state = PlayState::AwaitingKickOff;
        match_state.goal_presentation_started_at = None;
    }
}

pub fn prepare_kickoff(ball: &mut Ball, match_state: &mut MatchState) {
    if !matches!(match_state.play_state, PlayState::AwaitingKickOff) {
        return;
    }
    println!("ball reset,{},{}", ball.position.x, ball.position.y);
    //reset ball position and states
    ball.position.x = PITCH_LENGTH / 2.0;
    ball.position.y = PITCH_HEIGHT / 2.0;
    ball.velocity = Vec2::new(0.0, 0.0);
    ball.last_touched_by = None;
    ball.state = BallState::Free;
    println!("ball reset,{},{}", ball.position.x, ball.position.y)
}

pub fn start_kickoff(match_state: &mut MatchState) {
    if !matches!(match_state.play_state, PlayState::AwaitingKickOff) {
        return;
    }
    match_state.play_state = PlayState::Playing;
}

pub fn tick_match_clock(match_state: &mut MatchState) {
    if !matches!(match_state.play_state, PlayState::Playing) {
        return;
    }

    match match_state.match_phase {
        MatchPhase::FirstHalf => {
            match_state.match_elapsed_time += TIMESTEP * 22.5 * MATCHSPEED;
            println!("{}", match_state.match_elapsed_time);
            println!("Phase: {:?}", match_state.match_phase);
            if match_state.match_elapsed_time >= 2700.0 {
                match_state.match_elapsed_time = 2700.0;
                match_state.match_phase = MatchPhase::HalfTime;
            }
        }

        MatchPhase::SecondHalf => {
            match_state.match_elapsed_time += TIMESTEP * 22.5 * MATCHSPEED;
            println!("{}", match_state.match_elapsed_time);
            println!("Phase: {:?}", match_state.match_phase);
            if match_state.match_elapsed_time >= 5400.0 {
                match_state.match_elapsed_time = 5400.0;
                match_state.match_phase = MatchPhase::FullTime;
                match_state.play_state = PlayState::AwaitingKickOff;
            }
        }

        _ => return,
    }
}

pub fn handle_half_times(match_state: &mut MatchState, teams: &mut HashMap<TeamId, Team>) {
    if matches!(match_state.match_phase, MatchPhase::HalfTime) {
        println!("--- HALF TIME ---");
        switch_attacking_direction(teams);
        switch_kickoffteam(match_state);
        match_state.match_phase = MatchPhase::SecondHalf;
        match_state.play_state = PlayState::AwaitingKickOff;
    }
}
