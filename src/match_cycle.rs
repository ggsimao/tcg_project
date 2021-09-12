use super::entities::{Board, DamageType, MagicSchool, MonsterData};
use DamageType::*;
use MagicSchool::*;

pub fn pre_play(player: &mut Board) {
    player.draw_card();
}

pub fn in_play(player: &mut Board) {}

pub fn post_play() {}
