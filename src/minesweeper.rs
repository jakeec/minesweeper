use rand;
use rand::Rng;

#[derive(PartialEq)]
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
pub struct Grid(Vec<Row>);

impl Grid {
    fn new(dimensions: GridDimensions) -> Self {
        let mut grid: Vec<Row> = Vec::new();
        for x in 0..dimensions.x {
            let mut row: Row = Vec::new();
            for y in 0..dimensions.y {
                let mut rng = rand::thread_rng();
                let rnd = rng.gen_range(0, 2);
                let mut square_state: SquareState;
                match rnd {
                    0 => square_state = SquareState::Bomb,
                    1 => square_state = SquareState::Clear,
                    _ => panic!("never"),
                }
                row.push(Square::new(false, square_state));
            }

            grid.push(row);
        }

        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                let mut bomb_count = 0;
                let mut x_lower = match x {
                    0 => 0,
                    _ => x - 1,
                };
                let mut x_upper = x + 2;
                let mut y_lower = match y {
                    0 => 0,
                    _ => y - 1,
                };
                let mut y_upper = y + 2;
                if x_upper > dimensions.x {
                    x_upper = x + 1;
                }
                if y_upper > dimensions.y {
                    y_upper = y + 1;
                }
                println!(
                    "Square: ({},{}): ({},{}) to ({},{})",
                    x, y, x_lower, y_lower, x_upper, y_upper
                );
                for i in x_lower..x_upper {
                    for j in y_lower..y_upper {
                        println!("Checking ({},{})", i, j);
                        if grid[i][j].state == SquareState::Bomb {
                            bomb_count += 1;
                        }
                    }
                }
                if bomb_count > 0 && grid[x][y].state == SquareState::Clear {
                    grid[x][y].state = SquareState::Count(bomb_count);
                }
            }
        }

        Self(grid)
    }
}

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

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

impl Minesweeper {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            game_state: GameState::Playing,
            grid: match difficulty {
                Difficulty::Easy => Grid::new(GridDimensions::new(5, 5)),
                Difficulty::Medium => Grid::new(GridDimensions::new(7, 7)),
                Difficulty::Hard => Grid::new(GridDimensions::new(12, 12)),
                Difficulty::Custom(GridDimensions { x, y }) => Grid::new(GridDimensions::new(x, y)),
            },
        }
    }

    pub fn print(&self) {
        print!("🙂\n");
        for row in &self.grid.0 {
            for square in row {
                match square {
                    Square { revealed, state } => {
                        if *revealed {
                            match state {
                                SquareState::Bomb => print!("💣 "),
                                SquareState::Clear => print!("  "),
                                SquareState::Count(c) => print!("{} ", c),
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

    pub fn reveal_board(&mut self) {
        for row in &mut self.grid.0 {
            for square in row {
                square.revealed = true;
            }
        }
    }

    pub fn parse_move(&mut self, input: &str) {
        let dims: Vec<&str> = input.split(" ").collect();
        if dims[0] == "r\n" {
            self.reveal_board();
            clear_screen();
            self.print();
            return;
        }

        let x = match dims[0].replace("-", "").parse::<usize>() {
            Err(err) => return,
            Ok(x) => x,
        };
        let y = match dims[1].replace("-", "").replace("\n", "").parse::<usize>() {
            Err(err) => return,
            Ok(x) => x,
        };

        if x >= self.grid.0.len() || y >= self.grid.0[0].len() {
        } else {
            self.grid.0[x][y].revealed = true;
            clear_screen();
            self.print();
        }
    }
}
