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

    for g in 0..10 {
        println!("Evolve:");
        gol.compute_next_gen();
        println!("{}\n----------------", gol);
    }
    // println!("Evolve:");
    // gol.compute_next_gen();
    // println!("{}\n", gol);
}

fn random_test() {
    let mut gol = GameOfLife::new(10, 10);
    gol.random_fill(0.3);
    println!("Initial Pop.:\n{}\n", gol);
    for g in 0..10 {
        println!("Evolve:");
        gol.compute_next_gen();
        println!("{}\n----------------", gol);
    }
}

fn wiki_test() {
    let mut gol = GameOfLife::new(20, 20);

    gol.set_cell(2, 2, Cell::Used);
    gol.set_cell(3, 2, Cell::Used);
    gol.set_cell(3, 3, Cell::Used);
    gol.set_cell(3, 4, Cell::Used);
    gol.set_cell(4, 3, Cell::Used);
    println!("Initial Pop.:\n{}\n", gol);
    println!("Evolve:");
    for g in 0..40 {
        gol.compute_next_gen();
        println!("{}", gol);
    }
}

fn glider_test() {
    let mut gol = GameOfLife::new(20, 20);

    gol.set_cell(2, 2, Cell::Used);
    gol.set_cell(3, 3, Cell::Used);
    gol.set_cell(1, 4, Cell::Used);
    gol.set_cell(2, 4, Cell::Used);
    gol.set_cell(3, 4, Cell::Used);
    println!("Initial Pop.:\n{}\n", gol);
    println!("Evolve:");
    for g in 0..50 {
        gol.compute_next_gen();
        println!("{}", gol);
    }
}

fn main() {
    glider_test();
}
