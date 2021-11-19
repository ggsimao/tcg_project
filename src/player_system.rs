use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use super::entities::Board;

pub fn change_highlight(y: i32, x: i32, ecs: &mut World) {
    let entities = ecs.entities();
    let mut boards = ecs.write_storage::<Board>();
    let mut joined_storage = (&entities, &mut boards).join();
    let (_, mut player_board) = joined_storage.nth(0).expect("No player");
    let (_, mut enemy_board) = joined_storage.nth(0).expect("No enemy");

    for (_, b) in joined_storage {
        if b.id() == 0 {
            player_board = b;
        } else if b.id() == 1 {
            enemy_board = b;
        }
    }

    
    let old_highlighted = player_board.highlighted();
    let mut new_highlighted: (u8, i32) = (0, 0);
    new_highlighted.0 = min(max(old_highlighted.0 as i32 + y, 0) as u8, 3);
    new_highlighted.1 = match new_highlighted.0 {
        0 => min(max(old_highlighted.1 + x, 0), enemy_board.hand().len() as i32 - 1),
        1 | 2 => min(max(old_highlighted.1 + x, 0), 4),
        3 => min(max(old_highlighted.1 + x, 0), player_board.hand().len() as i32 - 1),
        _ => 0,
    };

    println!("{}, {}", new_highlighted.0, new_highlighted.1);

    player_board.change_highlighted(new_highlighted);
}

pub fn select_highligted(ecs: &World) {}

pub fn player_input(ecs: &mut World, ctx: &mut Rltk) {
    match ctx.key {
        None => return, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => change_highlight(0, -1, ecs),

            VirtualKeyCode::Right => change_highlight(0, 1, ecs),

            VirtualKeyCode::Up => change_highlight(-1, 0, ecs),

            VirtualKeyCode::Down => change_highlight(1, 0, ecs),

            VirtualKeyCode::Return => select_highligted(ecs),

            _ => return,
        },
    }
}
