use super::entities::{Board, Monster};
use rltk::{Rltk, RGB};
use specs::prelude::*;
use std::io;
use std::io::*;

pub fn show_board(my_board: Board, enemy_board: Board) -> String {
    let mut return_string: String = enemy_board.count_deck_size().to_string();
    return_string.push_str(" CARDS REMAINING\n");
    return_string.push_str(&enemy_board.format_field()[..]);
    return_string.push_str("\n");
    return_string.push_str(&my_board.format_field()[..]);
    return_string.push_str("\n");
    return_string.push_str(&my_board.format_hand()[..]);
    return_string.push_str("\n");
    return_string.push_str(&my_board.count_deck_size().to_string()[..]);
    return_string.push_str(" CARDS REMAINING\n");

    return_string
}

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
    assert!(vert_pos >= 0 && vert_pos <= 2);
    assert!(hori_pos >= 0 && hori_pos <= 4);
    assert!(vert_pos != 2 || hori_pos == 3);
    let calculated_vert_post = match vert_pos {
        0 => 2,
        1 => 22,
        2 => 43,
        _ => 0,
    };
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
        monster.data().base_health(),
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
    // TODO: REST OF CARD
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
