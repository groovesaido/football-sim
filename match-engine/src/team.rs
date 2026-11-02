use std::collections::HashMap;

use glam::Vec2;

use crate::{
    PlayerId, TeamId,
    ball::Ball,
    r#match::{MatchEvent, MatchState, PlayState},
    player::{Player, goal_detection},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackingDirection {
    Left,
    Right,
}
pub struct Team {
    pub id: TeamId,
    pub attacking_direction: AttackingDirection,
}

pub fn create_match_teams() -> HashMap<TeamId, Team> {
    let team_a = Team {
        id: 1,
        attacking_direction: AttackingDirection::Left,
    };
    let team_b = Team {
        id: 2,
        attacking_direction: AttackingDirection::Right,
    };
    let mut teams: HashMap<TeamId, Team> = HashMap::new();
    teams.insert(team_a.id, team_a);
    teams.insert(team_b.id, team_b);
    teams
}

pub fn determine_goal_scoring_team_and_player(
    previous_pos: &Vec2,
    current_pos: &Vec2,
    teams: &HashMap<TeamId, Team>,
    ball: &Ball,
    players: &HashMap<PlayerId, Player>,
    play_state: &PlayState,
) -> Option<MatchEvent> {
    // 1. Detect if a goal happened and which side it went into
    let goal_side = goal_detection(previous_pos, current_pos, play_state)?;

    // 2. Find the team whose attacking direction matches that goal side
    let scoring_team = teams
        .values()
        .find(|team| team.attacking_direction == goal_side)?;

    // 3. Determine the player responsible
    if let Some(id) = ball.last_touched_by {
        if let Some(scoring_player) = players.get(&id) {
            let players_team = scoring_player.team_id;

            if players_team == scoring_team.id {
                // Regular goal
                return Some(MatchEvent::Goal {
                    team_id: scoring_team.id,
                    player_id: Some(scoring_player.id),
                    own_goal: false,
                });
            } else {
                println!(
                    "Goal!!! Team {:?} scored!,by Player {:?}",
                    scoring_team.id, scoring_player.id
                );
                // Own goal: The scoring team gets the point, but credited as an own goal
                return Some(MatchEvent::Goal {
                    team_id: scoring_team.id,
                    player_id: Some(scoring_player.id), //credit the defender
                    own_goal: true,
                });
            }
        }
    }

    // Fallback: A goal happened, but we don't know who touched it last
    Some(MatchEvent::Goal {
        team_id: scoring_team.id,
        player_id: None,
        own_goal: false,
    })
}

pub fn switch_attacking_direction(teams: &mut HashMap<TeamId, Team>) {
    for team in teams.values_mut() {
        team.attacking_direction = match team.attacking_direction {
            AttackingDirection::Left => AttackingDirection::Right,
            AttackingDirection::Right => AttackingDirection::Left,
        };
    }
}
pub fn switch_kickoffteam(match_state: &mut MatchState) {
    match_state.opening_kickoff_team_id =
        if match_state.opening_kickoff_team_id == match_state.away_team_id {
            match_state.home_team_id
        } else {
            match_state.away_team_id
        };
}
