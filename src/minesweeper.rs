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

#[derive(PartialEq, Debug)]
pub struct GridDimensions {
    x: usize,
    y: usize,
}

impl GridDimensions {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Custom(GridDimensions),
}

fn create_grid(dimensions: GridDimensions) -> Grid {
    let mut grid: Grid = Vec::new();
    for x in 0..dimensions.x {
        let mut row: Row = Vec::new();
        for y in 0..dimensions.y {
            row.push(Square::new(false, SquareState::Clear));
        }

        grid.push(row);
    }

    grid
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            game_state: GameState::Playing,
            grid: match difficulty {
                Difficulty::Easy => create_grid(GridDimensions::new(5, 5)),
                Difficulty::Medium => create_grid(GridDimensions::new(7, 7)),
                Difficulty::Hard => create_grid(GridDimensions::new(12, 12)),
                Difficulty::Custom(GridDimensions { x, y }) => {
                    create_grid(GridDimensions::new(x, y))
                }
            },
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
        let x = match dims[0].replace("-", "").parse::<usize>() {
            Err(err) => return,
            Ok(x) => x,
        };
        let y = match dims[1].replace("-", "").replace("\n", "").parse::<usize>() {
            Err(err) => return,
            Ok(x) => x,
        };

        if x >= self.grid.len() || y >= self.grid[0].len() {
        } else {
            self.grid[x][y].revealed = true;
            clear_screen();
            self.print();
        }
    }
}
