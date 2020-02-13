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
                let rnd = rng.gen_range(0, 6);
                let mut square_state: SquareState;
                match rnd {
                    0 => square_state = SquareState::Bomb,
                    _ => square_state = SquareState::Clear,
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
                for i in x_lower..x_upper {
                    for j in y_lower..y_upper {
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

    pub fn get_adjacent_unrevealed_squares(&mut self, x: usize, y: usize) -> Vec<Vec<usize>> {
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
        if x_upper > self.0[0].len() {
            x_upper = x + 1;
        }
        if y_upper > self.0.len() {
            y_upper = y + 1;
        }

        let mut adjacent_squares: Vec<Vec<usize>> = Vec::new();

        for i in x_lower..x_upper {
            for j in y_lower..y_upper {
                if self.0[i][j].revealed == false {
                    adjacent_squares.push(vec![i, j])
                }
            }
        }

        adjacent_squares
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
        match self.game_state {
            GameState::Playing => print!("🙂\n"),
            GameState::Lose => print!("🙁\n"),
            GameState::Win => print!("😎\n"),
        }

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

    fn reveal_clear_region(&mut self, x: usize, y: usize) {
        let mut a = self.grid.get_adjacent_unrevealed_squares(x, y);
        let mut count = a.len();
        let mut i = 0;
        while count > 0 {
            let coords = &a[i];
            match self.grid.0[coords[0]][coords[1]].state {
                SquareState::Bomb => (),
                SquareState::Count(_) => self.grid.0[coords[0]][coords[1]].revealed = true,
                SquareState::Clear => {
                    self.grid.0[coords[0]][coords[1]].revealed = true;
                    let adj = self
                        .grid
                        .get_adjacent_unrevealed_squares(coords[0], coords[1]);
                    count += &adj.len();
                    for c in adj {
                        a.push(c[..].to_vec());
                    }
                }
            }
            i += 1;
            count -= 1;
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
            if self.grid.0[x][y].state == SquareState::Bomb {
                self.game_state = GameState::Lose;
                self.reveal_board();
            }

            if self.grid.0[x][y].state == SquareState::Clear {
                self.reveal_clear_region(x, y);
            }

            // clear_screen();
            self.print();
        }
    }
}
