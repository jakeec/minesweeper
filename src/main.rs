mod minesweeper;
use minesweeper::{GameState, Minesweeper};
use std::io::Stdin;

fn main() {
    let mut game = Minesweeper::new();
    game.print();
    let mut input: String = String::from("");
    while game.game_state == GameState::Playing {
        let stdin = std::io::stdin();
        stdin.read_line(&mut input);
        game.parse_move(&input);
        input = String::from("");
    }
}
