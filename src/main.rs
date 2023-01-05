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
}

struct CellMatrix {
    matrix: Vec<Vec<bool>>,
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

impl CellMatrix {
    fn init(rows: usize, cols: usize, cells: Vec<Cell>) -> Self {
        // Allocate a matrix full of false values
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(rows);
        for _ in 0..rows {
            matrix.push(vec![false; cols]);
        }

        cells.iter().for_each(|c| {
            if c.x > cols || c.y > rows {
                // return error
            }
            matrix[c.y][c.x] = true
        });

        Self {
            matrix: vec![vec![true, false]],
        }
    }

    fn draw(&self) -> () {}
}
