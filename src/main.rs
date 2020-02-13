mod minesweeper;
use getch::Getch;
use minesweeper::{Difficulty, GameState, GridDimensions, Minesweeper};
use std::io::Stdin;

fn parse_difficulty_input(difficulty: &str) -> Option<Difficulty> {
    match difficulty.replace("\n", "").parse::<usize>() {
        Err(err) => None,
        Ok(diff) => match diff {
            1 => Some(Difficulty::Easy),
            2 => Some(Difficulty::Medium),
            3 => Some(Difficulty::Hard),
            4 => Some(Difficulty::Custom(GridDimensions::new(4, 4))),
            _ => None,
        },
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input: String = String::from("");
    println!("Select difficulty: ");
    println!("\t 1: Easy");
    println!("\t 2: Normal");
    println!("\t 3: Hard");
    println!("\t 4: Custom");
    let mut difficulty = None;
    while difficulty == None {
        stdin.read_line(&mut input);
        println!("{}", input);
        difficulty = parse_difficulty_input(&input);
        println!("{:?}", difficulty);
        input = String::from("");
    }
    let mut game = Minesweeper::new(difficulty.unwrap());

    game.print();
    // h = 104
    // j = 106
    // k = 107
    // l = 108
    // space = 32
    // f = 102
    while game.game_state == GameState::Playing {
        let get_key = &[Getch::new().getch().unwrap()];
        game.parse_arrow_input(get_key);
        // stdin.read_line(&mut input);
        // game.parse_move(&input);
        // input = String::from("");
    }
}
