pub enum SquareState {
    Clear,
    Bomb,
    Count(usize),
}

pub struct Square {
    revealed: bool,
    state: SquareState,
}

impl Square {
    fn new(revealed: bool, state: SquareState) -> Self {
        Self {
            revealed: revealed,
            state: state,
        }
    }
}

pub type Row = Vec<Square>;
pub type Grid = Vec<Row>;

#[derive(PartialEq)]
pub enum GameState {
    Win,
    Lose,
    Playing,
}

pub struct Minesweeper {
    pub game_state: GameState,
    pub grid: Grid,
}

impl Minesweeper {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Playing,
            grid: vec![
                vec![
                    Square::new(false, SquareState::Bomb),
                    Square::new(false, SquareState::Clear),
                    Square::new(false, SquareState::Clear),
                ],
                vec![
                    Square::new(false, SquareState::Bomb),
                    Square::new(false, SquareState::Clear),
                    Square::new(false, SquareState::Clear),
                ],
                vec![
                    Square::new(false, SquareState::Bomb),
                    Square::new(false, SquareState::Clear),
                    Square::new(false, SquareState::Clear),
                ],
            ],
        }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for square in row {
                match square {
                    Square { revealed, state } => {
                        if *revealed {
                            match state {
                                SquareState::Bomb => print!("💣 "),
                                SquareState::Clear => print!("  "),
                                SquareState::Count(c) => print!("{}", c),
                            }
                        } else {
                            print!("⬜ ");
                        }
                    }
                }
            }

            print!("\n");
        }
    }

    pub fn parse_move(&mut self, input: &str) {
        let dims: Vec<&str> = input.split(" ").collect();
        let x = dims[0].parse::<usize>().unwrap();
        let y = dims[1].replace("\n", "").parse::<usize>().unwrap();

        self.grid[x][y].revealed = true;
        self.print();
    }
}
