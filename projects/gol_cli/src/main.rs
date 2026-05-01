use libgol::{Cell, GameOfLife};

fn save_test() {
    let mut gol = GameOfLife::new(10, 10);
    gol.set_cell(0, 0, Cell::Used);
    gol.set_cell(5, 5, Cell::Used);
    gol.set_cell(3, 7, Cell::Used);
    println!("Hello, world!");
    println!("{}\n", gol);
    println!("Random fill:");
    gol.random_fill(0.7);
    println!("{}\n", gol);
    gol.save("/home/acorbi/gol-random.txt");
    println!("File saved.");
}

fn load_test() {
    let mut gol = GameOfLife::new(5, 3);
    gol.load("/home/acorbi/gol-random.txt");
    println!("File loaded.");
    println!("{}\n", gol);
}

fn main() {
    load_test();
}
