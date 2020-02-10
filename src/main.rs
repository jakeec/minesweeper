mod minesweeper;
use minesweeper::{Difficulty, GameState, Minesweeper};
use std::io::Stdin;

fn main() {
    let stdin = std::io::stdin();
    println!("Select difficulty: ");
    let mut difficulty = Difficulty::Easy;
    let mut game = Minesweeper::new(difficulty);
    game.print();
    let mut input: String = String::from("");
    let mut stdout = std::io::stdout();
    while game.game_state == GameState::Playing {
        stdin.read_line(&mut input);
        game.parse_move(&input);
        input = String::from("");
    }
}
