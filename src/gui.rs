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

pub fn draw(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let boards = ecs.read_storage::<Board>();

    for (entity, board) in (&entities, &boards).join() {
        let gotten_id = board.id();
        if gotten_id == 0 {
            let mut my_board_string: String = String::new();
            my_board_string.push_str(&board.format_field()[..]);
            my_board_string.push_str("\n");
            my_board_string.push_str(&board.format_hand()[..]);
            my_board_string.push_str("\n");
            ctx.print_color(
                0,
                gotten_id * 39,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                &board.count_deck_size(),
            );
        }
    }
}

pub fn draw_monster(ctx: &mut Rltk, monster: Monster, vert_pos: u8, hori_pos: u8) {}

pub fn draw_template_highlighted_card(ctx: &mut Rltk) {
    ctx.print_color(
        0,
        40,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "CARD: ",
    );
    ctx.print_color(
        0,
        41,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                ┌───────────────┐",
    );
    ctx.print_color(
        0,
        42,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                │               │",
    );
    ctx.print_color(
        0,
        43,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                ├───────────────┤",
    );
    for i in 0..15 {
        ctx.print_color(
            0,
            44 + i,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "                                │               │",
        );
    }
    ctx.print_color(
        0,
        59,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        "                                └───────────────┘",
    );
}

pub fn draw_empty_board(ctx: &mut Rltk, id: u8) {
    ctx.print_color(
        0,
        id * 39,
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
