//use termion::color;

fn main() {
    // Get terminal size
    let (tcols, trows) = termion::terminal_size().unwrap();
    println!("Columns: {}, Lines: {}", tcols, trows);
}
