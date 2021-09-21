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
        let mut hori_pos = 0;
        for slot in board.field() {
            match slot {
                Some(monster) => draw_monster(ctx, monster, vert_pos as u8, hori_pos),
                None => {}
            }
            hori_pos += 1;
        }
    }
}

pub fn draw_monster(ctx: &mut Rltk, monster: Monster, vert_pos: u8, hori_pos: u8) {
    assert!(vert_pos <= 2);
    assert!(hori_pos <= 4);
    assert!(vert_pos != 2 || hori_pos == 3);
    let calculated_vert_post = match vert_pos {
        0 => 2,
        1 => 22,
        2 => 43,
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
    } else {
        panic!("Expected monster card!");
    }
}

pub fn draw_template_highlighted_card(ctx: &mut Rltk) {
    ctx.print_color(
        0,
        41,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "CARD: ",
    );
    ctx.print_color(
        0,
        42,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                ┌───────────────┐",
    );
    ctx.print_color(
        0,
        43,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                │               │",
    );
    ctx.print_color(
        0,
        44,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                ├───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            0,
            45 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "                                │               │",
        );
    }
    ctx.print_color(
        0,
        60,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                └───────────────┘",
    );
}

pub fn draw_empty_board(ctx: &mut Rltk, id: u8) {
    ctx.print_color(
        0,
        id * 40,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "   CARDS REMAINING",
    );
    ctx.print_color(
        0,
        (id * 20) + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "┌───────────────┬───────────────┬───────────────┬───────────────┬───────────────┐",
    );
    ctx.print_color(
        0,
        (id * 20) + 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "│               │               │               │               │               │",
    );
    ctx.print_color(
        0,
        (id * 20) + 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            0,
            (id * 20) + 4 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│               │               │               │               │               │",
        );
    }
    ctx.print_color(
        0,
        (id * 20) + 19,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘",
    );
}

pub fn display_hand(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.fetch::<Entities>();
    let boards = ecs.read_storage::<Board>();

    for (index, (entity, board)) in (&entities, &boards)
        .join()
        .filter(|x| x.1.id() == 0)
        .enumerate()
    {
        let mut printed = 0;
        for card in board.hand() {
            let mut card_name = card.data().name();
            if (index as i32) == board.highlighted() {
                card_name.insert_str(0, "> ");
                printed += 4;
                card_name.push_str(" <");
            }
            let printed_end = printed + &card_name.len();
            if printed_end > 80 {
                break;
            }
            ctx.print_color(
                printed,
                61,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                &card_name,
            );
            printed = printed_end + 1;
        }
    }
}
