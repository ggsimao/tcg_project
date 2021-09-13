use super::entities::{Board, Monster};

pub fn show_board(my_board: Board, enemy_board: Board) {
    let mut his_deck: String = enemy_board.count_deck_size().to_string();
    his_deck.push_str(" CARD REMAINING\n");
    let mut his_field: String = enemy_board.format_field();
    his_field.push_str("\n");
    let mut my_field: String = my_board.format_field();
    my_field.push_str("\n");
    let mut my_hand: String = my_board.format_hand();
    my_hand.push_str("\n");
    let mut my_deck: String = my_board.count_deck_size().to_string();
    my_deck.push_str(" CARD REMAINING\n");
}
