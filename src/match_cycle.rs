use super::entities::{Board, DamageType, MagicSchool};
use specs::prelude::*;

pub fn pre_game(ecs: &World) {
    let entities = ecs.entities();
    let mut boards = ecs.write_storage::<Board>();

    
    for (entity, board) in (&entities, &mut boards)
    .join()
    {
        for i in 0..6 {
            board.draw_card();
        }
    }
}

pub fn pre_play(player: &mut Board) {
    player.draw_card();
}

pub fn in_play(player: &mut Board) {}

pub fn post_play() {}
