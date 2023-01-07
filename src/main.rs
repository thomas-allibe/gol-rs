use std::{thread, time};
use termion::clear;

fn main() {
    const CELL_CHAR: char = '#';

    print!("{}", clear::All);

    // Get terminal size
    let (tcols, trows) = termion::terminal_size().unwrap();
    let (tcols, trows) = (tcols as usize, trows as usize);
    println!("Columns: {}, Lines: {}", tcols, trows);

    // Allocate cell matrix
    // true: is a cell
    // false: empty
    let mut cell_matrix: Vec<Vec<bool>> = Vec::with_capacity(trows);
    for _ in 0..trows {
        cell_matrix.push(vec![false; tcols]);
    }

    cell_matrix[3][0] = true;
    cell_matrix[3][1] = true;
    cell_matrix[3][2] = true;
    cell_matrix[3][3] = true;
    cell_matrix[3][4] = true;
    cell_matrix[3][5] = true;
    cell_matrix[3][6] = true;

    cell_matrix.iter().for_each(|l| {
        let mut buffer = String::new();
        l.iter().for_each(|c| {
            if *c {
                buffer.push(CELL_CHAR);
            } else {
                buffer.push(' ');
            }
        });
        println!("{}", buffer);
    });

    // let mut cell_matrix: CellMatrix::from();

    thread::sleep(time::Duration::from_secs(3));

    let mut game = GameOfLife::new(tcols, trows);
    game.run();
}

//
//   -----> x
//   |
//   |
//   v
//   y
//
struct Cell {
    x: usize,
    y: usize,
}

enum GolState {
    WaitingInput,
    Configuring(Vec<Cell>),
    Advancing,
    Waiting(f64),
}

struct GameOfLife {
    state: GolState,
    board: Vec<Vec<bool>>,
    ncols: usize,
    nrows: usize,
}

impl GameOfLife {
    fn new(rows: usize, cols: usize) -> Self {
        // Allocate a matrix full of false values
        let mut board: Vec<Vec<bool>> = Vec::with_capacity(rows);
        for _ in 0..rows {
            board.push(vec![false; cols]);
        }

        GameOfLife {
            state: GolState::WaitingInput,
            board: board,
            ncols: cols,
            nrows: rows,
        }
    }

    fn run(&mut self) -> &mut Self {
        match &self.state {
            GolState::WaitingInput => {
                println!("WaitInput");
            }
            GolState::Configuring(cells) => {
                println!("Setup");
                cells.iter().for_each(|c| {
                    if c.x > self.ncols || c.y > self.nrows {
                        // return error
                    }
                    self.board[c.y][c.x] = true
                });
            }
            GolState::Advancing => {
                println!("Step");
            }
            GolState::Waiting(time) => {
                println!("Pause: {}", time);
            }
        }
        self
    }
}
