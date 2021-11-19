use crate::entities::CardHolder;

use super::entities::{Board, Card, CardData, Monster, MonsterData};
use rltk::{Rltk, RGB};
use specs::prelude::*;
use std::any::Any;
use std::io;
use std::io::*;
use std::str::FromStr;

const ENEMY_RESOURCES: u8 = 0;
const ENEMY_HAND: u8 = ENEMY_RESOURCES + 1;
const ENEMY_BOARD: u8 = ENEMY_HAND + 2;
const FIELD_HEIGHT: u8 = 7;
const PLAYER_BOARD: u8 = ENEMY_BOARD + FIELD_HEIGHT;
const HIGHLIGHTED_CARD: u8 = PLAYER_BOARD + FIELD_HEIGHT + 1;
const HIGHLIGHTED_HEIGHT: u8 = FIELD_HEIGHT;
const PLAYER_HAND: u8 = HIGHLIGHTED_CARD + HIGHLIGHTED_HEIGHT;
const CARD_WIDTH: u8 = 16;
const PLAYER_RESOURCES: u8 = PLAYER_HAND + 1;
const HIGHLIGHTED_TEXT: u8 = PLAYER_RESOURCES + 1;

const FIELD_SLOTS: u8 = 5;

const MAX_HIGHLIGHTED_Y: u8 = 3;
const PLAYER_HAND_HIGHLIGHT_INDEX: u8 = MAX_HIGHLIGHTED_Y;

pub fn draw_filled_board(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    for (vert_pos, (entity, board)) in (&entities, &boards).join().enumerate() {
        let mut hori_pos: u8 = 0;
        for slot in board.field() {
            match slot {
                Some(monster) => {
                    draw_monster(ctx, &monster, vert_pos as u8, hori_pos)
                }
                None => {}
            }
            hori_pos += 1;
        }
    }
}

pub fn draw_monster(
    ctx: &mut Rltk,
    monster: &Monster,
    vert_pos: u8,
    hori_pos: u8
) {
    assert!(vert_pos <= 2);
    assert!(hori_pos <= 4);
    assert!(vert_pos != 2 || hori_pos == 2);
    if let Some(monster_data) = monster.data().as_monster() {
        let calculated_vert_post = match vert_pos {
            0 => ENEMY_BOARD + 2,
            1 => PLAYER_BOARD + 1,
            2 => HIGHLIGHTED_CARD + 1,
            _ => 0,
        };
        let mut offset = 0;

        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post + offset,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            monster.data().name(),
        );
        offset += 2;

        if vert_pos == 2 {
            ctx.print_color(
                1 + CARD_WIDTH * hori_pos,
                calculated_vert_post + offset,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                monster.cost(),
            );
            offset += 1;
        }

        let health_string = format!("{}/{}", monster.health(), monster_data.base_health());
        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post + offset,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
            health_string,
        );
        offset += 1;

        let damage_string = format!("{}, {}", monster.damage(), monster_data.attack_type().name());
        ctx.print_color(
            1 + CARD_WIDTH * hori_pos,
            calculated_vert_post + offset,
            RGB::named(monster_data.attack_type().color()),
            RGB::named(rltk::BLACK),
            damage_string,
        );
    } else {
        panic!("Expected monster card!");
    }
}

pub fn draw_template_highlighted_card(ctx: &mut Rltk) {
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "┌───────────────┐",
    );
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "│               │",
    );
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "├───────────────┤",
    );
    for i in 0..3 {
        ctx.print_color(
            32,
            HIGHLIGHTED_CARD + 3 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│               │",
        );
    }
    ctx.print_color(
        32,
        HIGHLIGHTED_CARD + 6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "└───────────────┘",
    );
}

pub fn draw_highlighted_card(ctx: &mut Rltk, board: &Board) {
    let highlighted = board.highlighted();

    let hand = board.hand();
    match &hand[highlighted.1 as usize] {
        CardHolder::MonsterCard(m) => draw_monster(ctx, m, 2, 2),
        _ => {}
    }
}

pub fn draw_empty_board(ecs: &World, ctx: &mut Rltk, id: u8) {
    let y = match id {
        1 => ENEMY_BOARD,
        0 => PLAYER_BOARD,
        _ => 0,
    };
    let cards_remaining_y = match id {
        1 => ENEMY_BOARD - 1,
        0 => PLAYER_BOARD + FIELD_HEIGHT,
        _ => 0,
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
    for i in 0..2 {
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
        y + 5,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "└───────────────┴───────────────┴───────────────┴───────────────┴───────────────┘",
    );

    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();
    const WIDTH: i32 = 16;

    for (_, board) in (&entities, &boards).join().filter(|x| x.1.id() == 0) {
        let highlighted = board.highlighted();
        match highlighted.0 {
            1 => {
                if id == 1 {
                    ctx.print_color(
                        WIDTH * highlighted.1 + 8,
                        y + 6,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::BLACK),
                        "A",
                    );
                }
            },
            2 => {
                if id == 0 {
                    ctx.print_color(
                        WIDTH * highlighted.1 + 8,
                        y + 6,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::BLACK),
                        "A",
                    );
                }
            },
            _ => {},
        }
    }

}

pub fn display_hand(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    for (entity, board) in (&entities, &boards).join() {
        let mut printed_now = 0;
        let mut printed_next = 0;
        let hand = board.hand();
        let id = board.id();
        let y = match id {
            1 => ENEMY_HAND,
            0 => PLAYER_HAND,
            _ => 0,
        };
        
        for index in 0..hand.len() {
            let card = &hand[index];
            let mut card_name: String;
            if id == 0 || !card.hidden() {
                card_name = card.name();
            } else {
                card_name = format!("Card");
            }
            if board.id() == 0 && board.highlighted().0 == PLAYER_HAND_HIGHLIGHT_INDEX {
                if (index as i32) == board.highlighted().1 {
                    card_name.insert_str(0, "> ");
                    card_name.push_str(" <");
                }
                draw_template_highlighted_card(ctx);
                draw_highlighted_card(ctx, board);
            }
            if index > 0 {
                card_name.insert_str(0, " | ");
            }
            ctx.print_color(
                printed_now,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                &card_name,
            );
            printed_next += card_name.len() as u32;
            if printed_next > 80 {
                break;
            }
            printed_now = printed_next;
        }
    }
}
