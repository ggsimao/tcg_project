use super::entities::{Board, Card, CardData, Monster, MonsterData};
use rltk::{Rltk, RGB};
use specs::prelude::*;
use std::any::Any;
use std::io;
use std::io::*;

pub fn draw_filled_board(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    for (vert_pos, (entity, board)) in (&entities, &boards).join().enumerate() {
        let mut hori_pos: u8 = 0;
        let is_highlighted =
            (vert_pos as u8) == board.highlighted().0 && (hori_pos as i32) == board.highlighted().1;
        for slot in board.field() {
            match slot {
                Some(monster) => {
                    draw_monster(ctx, monster, vert_pos as u8, hori_pos, is_highlighted)
                }
                None => {}
            }
            hori_pos += 1;
        }
    }
}

pub fn draw_monster(
    ctx: &mut Rltk,
    monster: Monster,
    vert_pos: u8,
    hori_pos: u8,
    is_highlighted: bool,
) {
    assert!(vert_pos <= 2);
    assert!(hori_pos <= 4);
    assert!(vert_pos != 2 || hori_pos == 3);
    let calculated_vert_post = match vert_pos {
        0 => 2,
        1 => 23,
        2 => 44,
        _ => 0,
    };
    if let Some(monster_data) = monster.data().as_monster() {
        ctx.print_color(
            1 + 15 * hori_pos,
            calculated_vert_post,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster.data().name(),
        );
        ctx.print_color(
            1 + 15 * hori_pos,
            calculated_vert_post + 2,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "HEALTH: ",
        );
        ctx.print_color(
            9 + 15 * hori_pos,
            calculated_vert_post + 2,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster.health(),
        );
        ctx.print_color(
            1 + 15 * hori_pos,
            calculated_vert_post + 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "MAX HEALTH: ",
        );
        ctx.print_color(
            13 + 15 * hori_pos,
            calculated_vert_post + 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster_data.base_health(),
        );
        ctx.print_color(
            1 + 15 * hori_pos,
            calculated_vert_post + 4,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "DAMAGE: ",
        );
        ctx.print_color(
            9 + 15 * hori_pos,
            calculated_vert_post + 4,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster.damage(),
        );
        if is_highlighted {
            ctx.print_color(
                7 + 15 * hori_pos,
                calculated_vert_post + 18,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "ΛΛΛ",
            );
        }
    } else {
        panic!("Expected monster card!");
    }
}

pub fn draw_template_highlighted_card(ctx: &mut Rltk) {
    ctx.print_color(
        0,
        43,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "CARD: ",
    );
    ctx.print_color(
        0,
        44,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                ┌───────────────┐",
    );
    ctx.print_color(
        0,
        45,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                │               │",
    );
    ctx.print_color(
        0,
        46,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                ├───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            0,
            47 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "                                │               │",
        );
    }
    ctx.print_color(
        0,
        62,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                └───────────────┘",
    );
}

pub fn draw_empty_board(ctx: &mut Rltk, id: u8) {
    ctx.print_color(
        0,
        id * 42,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "   CARDS REMAINING",
    );
    ctx.print_color(
        0,
        (id * 21) + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "┌───────────────┬───────────────┬───────────────┬───────────────┬───────────────┐",
    );
    ctx.print_color(
        0,
        (id * 21) + 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "│               │               │               │               │               │",
    );
    ctx.print_color(
        0,
        (id * 21) + 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            0,
            (id * 21) + 4 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│               │               │               │               │               │",
        );
    }
    ctx.print_color(
        0,
        (id * 21) + 19,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘",
    );
}

pub fn display_hand(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    
    for (entity, board) in (&entities, &boards)
    .join()
    .filter(|x| x.1.id() == 0)
    {
        let mut printed = 0;
        let hand = board.hand();
        for index in 0..hand.len() {
            let card = &hand[index];
            let mut card_name = card.name();
            if board.highlighted().0 == 0 {
                if (index as i32) == board.highlighted().1 {
                    card_name.insert_str(0, "> ");
                    printed += 4;
                    card_name.push_str(" <");
                }
            }
            let printed_end = printed + &card_name.len();
            if printed_end > 80 {
                break;
            }
            ctx.print_color(
                printed,
                63,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                &card_name,
            );
            printed = printed_end + 1;
        }
    }
}
