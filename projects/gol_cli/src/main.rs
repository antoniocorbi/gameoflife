use libgol::{Cell, GameOfLife};

fn main() {
    let mut gol = GameOfLife::new(10, 10);
    gol.set_cell(0, 0, Cell::Used);
    gol.set_cell(5, 5, Cell::Used);
    gol.set_cell(3, 7, Cell::Used);
    println!("Hello, world!");
    println!("{}", gol);
}
