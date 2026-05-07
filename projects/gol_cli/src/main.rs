use libgol::{Cell, FigureExt, GameOfLife};

const CLS: &str = "\x1B[2J";
const MOVE11: &str = "\x1B[1;1H";
const HIDECRSR: &str = "\x1B[?25l";
const SHOWCRSR: &str = "\x1B[?25h";

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
    use std::io::Write;
    const MILLIS: u64 = 45;
    const GENERATIONS: u64 = 140;
    const NROWS: usize = 40;
    const NCOLS: usize = 80;
    const X: usize = 20;
    const Y: usize = 0;

    let stdout = std::io::stdout();
    let frame_duration = std::time::Duration::from_millis(MILLIS);

    let mut gol = GameOfLife::new(NROWS, NCOLS);
    gol.set_visuals('o', '·');

    gol.set_cell(X, Y, Cell::Used);
    gol.set_cell(X + 1, Y + 1, Cell::Used);
    gol.set_cell(X - 1, Y + 2, Cell::Used);
    gol.set_cell(X, Y + 2, Cell::Used);
    gol.set_cell(X + 1, Y + 2, Cell::Used);

    // println!("Initial Pop.:\n{}\n", gol);
    // println!("Evolve:");

    print!("{}{}", HIDECRSR, CLS);
    for _ in 0..GENERATIONS {
        gol.compute_next_gen();
        print!("{}{}", MOVE11, gol);

        // 3. Forzamos la salida y esperamos
        //stdout.flush().unwrap();
        std::thread::sleep(frame_duration);
    }
    print!("{}", SHOWCRSR);
}

fn figure_test() {
    use libgol::Figure;
    const MILLIS: u64 = 90;
    const GENERATIONS: u64 = 140;
    let frame_duration = std::time::Duration::from_millis(MILLIS);

    let mut gol = GameOfLife::new(20, 50);

    // gol.insert_figure(Figure::Block, 2, 2);
    // //gol.insert_figure(Figure::Block, 12, 10);
    // gol.insert_figure(Figure::Block, 20, 2);
    // gol.insert_figure(Figure::Block, 18, 14);
    //
    gol.insert_figure(Figure::Blinker, 20, 5);
    gol.insert_figure(Figure::Blinker, 25, 9);
    // gol.insert_figure(Figure::Blinker, 10, 14);
    // gol.insert_figure(Figure::Blinker, 19, 17);
    // gol.insert_figure(Figure::Blinker, 11, 20);
    //
    // gol.insert_figure(Figure::Toad, 7, 9);
    // gol.insert_figure(Figure::Lighthouse, 4, 12);
    //gol.insert_figure(Figure::Pulsar, 2, 2);
    gol.insert_figure(Figure::PentaDec, 2, 2);

    println!("{}", gol);

    // print!("{}{}", HIDECRSR, CLS);
    // for _ in 0..GENERATIONS {
    //     gol.compute_next_gen();
    //     print!("{}{}", MOVE11, gol);
    //
    //     // 3. Forzamos la salida y esperamos
    //     //stdout.flush().unwrap();
    //     std::thread::sleep(frame_duration);
    // }
    // print!("{}", SHOWCRSR);
}

fn main() {
    figure_test();
}
