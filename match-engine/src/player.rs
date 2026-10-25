use crate::{DELTA_TIME, pitch::Pitch};
use glam::Vec2;
pub struct Player {
    pub id: u32,
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

pub fn lib() {
    let mut player = Player {
        id: 1,
        team_id: 1,
        position: Pitch { x: 30.0, y: 20.0 },
        speed: 5.0,
        targetpos: Pitch { x: 70.0, y: 60.0 },
    };
    let mut tick = 0;
    while glam::Vec2::distance(
        glam::Vec2::new(player.targetpos.x, player.targetpos.y),
        glam::Vec2::new(player.position.x, player.position.y),
    ) > 0.1
    {
        move_player(&mut player);
        tick += 1;
        println!(
            "tick:{},Player pos:{},{}",
            tick, player.position.x, player.position.y
        )
    }
}
