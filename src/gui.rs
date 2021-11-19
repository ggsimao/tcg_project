use super::entities::{Board, Card, CardData, Monster, MonsterData};
use rltk::{Rltk, RGB};
use specs::prelude::*;
use std::any::Any;
use std::io;
use std::io::*;

const ENEMY_BOARD: u8 = 1;
const PLAYER_BOARD: u8 = 22;
const HIGHLIGHTED_CARD: u8 = 44;
const PLAYER_HAND: u8 = 64;
const CARD_WIDTH: u8 = 15;
const HIGHLIGHTED_TEXT: u8 = 65;

pub fn draw_filled_board(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    for (vert_pos, (entity, board)) in (&entities, &boards).join().enumerate() {
        let mut hori_pos: u8 = 0;
        for slot in board.field() {
            match slot {
                Some(monster) => {
                    draw_monster(ctx, monster, vert_pos as u8, hori_pos)
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
    hori_pos: u8
) {
    assert!(vert_pos <= 2);
    assert!(hori_pos <= 4);
    assert!(vert_pos != 2 || hori_pos == 3);
    let calculated_vert_post = match vert_pos {
        0 => ENEMY_BOARD + 2,
        1 => PLAYER_BOARD + 1,
        2 => HIGHLIGHTED_CARD + 1,
        _ => 0,
    };
    if let Some(monster_data) = monster.data().as_monster() {
        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster.data().name(),
        );
        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post + 2,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "HEALTH: ",
        );
        ctx.print_color(
            9 + CARD_WIDTH * hori_pos,
            calculated_vert_post + 2,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster.health(),
        );
        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post + 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "MAX HEALTH: ",
        );
        ctx.print_color(
            13 + CARD_WIDTH * hori_pos,
            calculated_vert_post + 3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster_data.base_health(),
        );
        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post + 4,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "DAMAGE: ",
        );
        ctx.print_color(
            9 + CARD_WIDTH * hori_pos,
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
        HIGHLIGHTED_CARD,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "CARD: ",
    );
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "┌───────────────┐",
    );
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "│               │",
    );
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "├───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            32,
            HIGHLIGHTED_CARD + 4 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│               │",
        );
    }
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 19,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "└───────────────┘",
    );
}

pub fn draw_empty_board(ecs: &World, ctx: &mut Rltk, id: u8) {
    let y = match id {
        0 => ENEMY_BOARD,
        _ => PLAYER_BOARD,
    };
    let cards_remaining_y = match id {
        0 => 0,
        _ => HIGHLIGHTED_CARD - 1,
    };
    ctx.print_color(
        0,
        cards_remaining_y,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "   CARDS REMAINING",
    );
    ctx.print_color(
        0,
        y,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "┌───────────────┬───────────────┬───────────────┬───────────────┬───────────────┐",
    );
    ctx.print_color(
        0,
        y + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "│               │               │               │               │               │",
    );
    ctx.print_color(
        0,
        y + 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "├───────────────┼───────────────┼───────────────┼───────────────┼───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            0,
            y + 3 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│               │               │               │               │               │",
        );
    }
    ctx.print_color(
        0,
        y + 18,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘",
    );

    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();
    const WIDTH: i32 = 16;

    for (_, board) in (&entities, &boards).join().filter(|x| x.1.id() == 0) {
        let highlighted = board.highlighted();
        if id == highlighted.0 {
            ctx.print_color(
                WIDTH * highlighted.1 + 8,
                y + 19,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "A",
            );
        }
    }

}

pub fn display_hand(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    for (entity, board) in (&entities, &boards).join().filter(|x| x.1.id() == 0) {
        let mut printed_now = 0;
        let mut printed_next = 0;
        let hand = board.hand();
        for index in 0..hand.len() {
            let card = &hand[index];
            let mut card_name = card.name();
            if board.highlighted().0 == 2 {
                if (index as i32) == board.highlighted().1 {
                    card_name.insert_str(0, "> ");
                    card_name.push_str(" <");
                }
            }
            ctx.print_color(
                printed_now,
                PLAYER_HAND,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                &card_name,
            );
            printed_next += card_name.len() as u32 + 2;
            if printed_next > 80 {
                break;
            }
            printed_now = printed_next;
        }
    }
}
