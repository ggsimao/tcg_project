use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use super::entities::Board;

pub fn change_highlight(y: i32, x: i32, ecs: &mut World) {
    let entities = ecs.entities();
    let mut boards = ecs.write_storage::<Board>();

    for (entity, board) in (&entities, &mut boards).join().filter(|x| x.1.id() == 0) {
        let old_highlighted = board.highlighted();
        let mut new_highlighted: (u8, i32) = (0, 0);
        new_highlighted.0 = min(max(old_highlighted.0 as i32 + y, 0) as u8, 2);
        new_highlighted.1 = min(max(old_highlighted.1 + x, 0), board.hand().len() as i32 - 1);

        println!("{}, {}", new_highlighted.0, new_highlighted.1);

        board.change_highlighted(new_highlighted);
    }
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
