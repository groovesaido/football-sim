use crate::pitch::Pitch;

pub struct Player {
    id: u32,
    team_id: u32,
    position: Pitch,
    speed: f32,
    targetpos: Pitch,
}
