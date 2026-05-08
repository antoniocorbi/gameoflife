// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
#![allow(unused)]

// -- Modules: ------------------------------------------------------------

mod figure;

// -- Uses: ---------------------------------------------------------------
use std::fmt;
// The prelude import enables methods we use below, specifically
// Rng::random, Rng::sample, SliceRandom::shuffle and IndexedRandom::choose.
use rand::prelude::*;
// For file saving/loading
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub use crate::figure::*;

// -- Constants: ----------------------------------------------------------
const USED_CHAR: char = '@';
const UNUSED_CHAR: char = '_';

// -- Types: --------------------------------------------------------------
// type Cell = bool;

type Neighbor = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Used,
    Unused,
}

type Matrix = Vec<Vec<Cell>>;

pub struct GameOfLife {
    curr_gen: Matrix,
    next_gen: Matrix,
    used_char: char,
    unused_char: char,
}

// -- Impl: ---------------------------------------------------------------

impl GameOfLife {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let curr_gen = vec![vec![Cell::Unused; ncols]; nrows];
        let next_gen = vec![vec![Cell::Unused; ncols]; nrows];
        let used_char = USED_CHAR;
        let unused_char = UNUSED_CHAR;

        GameOfLife {
            curr_gen,
            next_gen,
            used_char,
            unused_char,
        }
    }

    pub fn set_visuals(&mut self, used: char, unused: char) {
        self.used_char = used;
        self.unused_char = unused;
    }

    pub fn nrows(&self) -> usize {
        self.curr_gen.len()
    }

    pub fn ncols(&self) -> usize {
        self.curr_gen[0].len()
    }

    fn resize_next_gen(&mut self) {
        self.next_gen.clear();
        self.next_gen.shrink_to_fit();
        self.next_gen = vec![vec![Cell::Unused; self.ncols()]; self.nrows()];
    }

    pub fn set_cell(&mut self, x: usize, y: usize, status: Cell) {
        assert!(x < self.ncols() && y < self.nrows());
        self.curr_gen[y][x] = status;
    }

    pub fn cell(&mut self, x: usize, y: usize) -> Cell {
        assert!(x < self.ncols() && y < self.nrows());
        self.curr_gen[y][x]
    }

    pub fn random_fill(&mut self, p: f64) {
        assert!(p >= 0.0 && p <= 1.0); // Probability 0%..100%

        let mut rng = rand::rng(); // Get an RNG:

        for y in 0..self.nrows() {
            for x in 0..self.ncols() {
                if rng.random_bool(p) {
                    self.set_cell(x, y, Cell::Used);
                } else {
                    self.set_cell(x, y, Cell::Unused);
                }
            }
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<Neighbor> {
        assert!(x < self.ncols() && y < self.nrows());

        let mut n = vec![];
        let min_x = if x > 0 { x - 1 } else { 0 };
        let min_y = if y > 0 { y - 1 } else { 0 };
        let max_x = if x == self.ncols() - 1 {
            self.ncols() - 1
        } else {
            x + 1
        };
        let max_y = if y == self.nrows() - 1 {
            self.nrows() - 1
        } else {
            y + 1
        };

        // dbg!(min_x);
        // dbg!(min_y);
        // dbg!(max_x);
        // dbg!(max_y);

        for ix in min_x..=max_x {
            for iy in min_y..=max_y {
                if ix != x || iy != y {
                    if self.curr_gen[iy][ix] == Cell::Used {
                        n.push((ix, iy));
                    }
                }
            }
        }

        // dbg!(&n);

        n
    }

    pub fn num_neighbors(&self, x: usize, y: usize) -> usize {
        assert!(x < self.ncols() && y < self.nrows());

        let mut n: usize = 0;
        let min_x = if x > 0 { x - 1 } else { 0 };
        let min_y = if y > 0 { y - 1 } else { 0 };
        let max_x = if x == self.ncols() - 1 {
            self.ncols() - 1
        } else {
            x + 1
        };
        let max_y = if y == self.nrows() - 1 {
            self.nrows() - 1
        } else {
            y + 1
        };

        // if x == 0 && y == 0 {
        //     dbg!(min_x);
        //     dbg!(min_y);
        //     dbg!(max_x);
        //     dbg!(max_y);
        // }

        for ix in min_x..=max_x {
            for iy in min_y..=max_y {
                if ix != x || iy != y {
                    if self.curr_gen[iy][ix] == Cell::Used {
                        n += 1;
                    }
                }
            }
        }

        // dbg!(&n);

        n
    }

    /// Cada fila de la matriz será una línea en el archivo.
    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        let unused = ' '; // self.UNUSED_CHAR

        for row in &self.curr_gen {
            let line: String = row
                .iter()
                .map(|cell| {
                    if *cell == Cell::Used {
                        self.used_char
                    } else {
                        // UNUSED_CHAR
                        unused
                    }
                })
                .collect();
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    /// Lee un archivo y lo convierte en una Matrix.
    /// Cualquier carácter que no sea un espacio en blanco se interpreta como Cell::Used.
    pub fn load(&mut self, path: &str) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut matrix = Vec::new();
        let unused = ' '; // self.UNUSED_CHAR

        for line in reader.lines() {
            let line = line?;
            let row: Vec<Cell> = line
                .chars()
                .map(|c| {
                    if c == unused {
                        Cell::Unused
                    } else {
                        Cell::Used
                    }
                })
                .collect();
            matrix.push(row);
        }

        self.curr_gen = matrix;
        self.resize_next_gen();

        Ok(())
    }

    // - Nace: Si una célula muerta tiene exactamente 3 células vecinas vivas "nace" (es decir, al turno siguiente estará viva).
    //
    // - Muere: una célula viva puede morir por uno de 2 casos:
    //     Sobrepoblación: si tiene más de tres vecinos alrededor.
    //     Aislamiento: si tiene solo un vecino alrededor o ninguno.
    //
    // - Vive: una célula se mantiene viva si tiene 2 o 3 vecinos a su alrededor.
    pub fn compute_next_gen(&mut self) {
        for y in 0..self.nrows() {
            for x in 0..self.ncols() {
                let nn = self.num_neighbors(x, y);
                // if x == 7 && y == 0 {
                //     println!("0_7nn = {nn}");
                // }
                match self.curr_gen[y][x] {
                    Cell::Used => {
                        if nn == 2 || nn == 3 {
                            self.next_gen[y][x] = Cell::Used;
                        } else if nn > 3 || nn < 2 {
                            self.next_gen[y][x] = Cell::Unused;
                        }
                    }
                    Cell::Unused => {
                        if nn == 3 {
                            // New cell on next_gen
                            self.next_gen[y][x] = Cell::Used;
                        }
                    }
                }
            }
        }
        self.curr_gen = self.next_gen.clone();
    }
}

impl FigureExt for GameOfLife {
    fn insert_figure(&mut self, f: Figure, x: usize, y: usize) {
        match f {
            Figure::Block => self.block(x, y),
            Figure::Hive => self.hive(x, y),
            Figure::Pan => self.pan(x, y),
            Figure::Boat => self.boat(x, y),
            Figure::Bath => self.bath(x, y),
            Figure::Blinker => self.blinker(x, y),
            Figure::Toad => self.toad(x, y),
            Figure::Lighthouse => self.lighthouse(x, y),
            Figure::Pulsar => self.pulsar(x, y),
            Figure::PentaDec => self.penta_dec(x, y),
            Figure::Glider => self.glider(x, y),
            Figure::SShip1 => self.sship1(x, y),
            Figure::SShip2 => self.sship2(x, y),
            Figure::SShip3 => self.sship3(x, y),
        }
    }

    fn hive(&mut self, x: usize, y: usize) {
        let w = 4; // width
        let h = 3; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
        }
    }

    fn pan(&mut self, x: usize, y: usize) {
        let w = 4; // width
        let h = 4; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 3
            y += 1;
            self.set_cell(x + 2, y, Cell::Used);
        }
    }

    fn boat(&mut self, x: usize, y: usize) {
        let w = 3; // width
        let h = 3; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
        }
    }

    fn bath(&mut self, x: usize, y: usize) {
        let w = 3; // width
        let h = 3; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 1, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
        }
    }

    fn sship1(&mut self, x: usize, y: usize) {
        let w = 5; // width
        let h = 4; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 3
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
        }
    }

    fn sship2(&mut self, x: usize, y: usize) {
        let w = 6; // width
        let h = 4; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            // Line 3
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
        }
    }

    fn sship3(&mut self, x: usize, y: usize) {
        let w = 7; // width
        let h = 4; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 6, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            // Line 3
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
        }
    }

    fn glider(&mut self, x: usize, y: usize) {
        let w = 3; // width
        let h = 3; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 2, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
        }
    }

    fn penta_dec(&mut self, x: usize, y: usize) {
        let w = 5; // width
        let h = 12; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 2, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            // Line 9
            y += 7;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            // Line 4
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            // Line 5
            y += 1;
            self.set_cell(x + 2, y, Cell::Used);
        } else {
            eprintln!("Unable to insert Pulsar.");
        }
    }

    fn pulsar(&mut self, x: usize, y: usize) {
        let w = 13; // width
        let h = 13; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            // Line 0
            let mut y = y;
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 10, y, Cell::Used);
            // Line 1
            y += 1;
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 8, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            // Line 2
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 7, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 12, y, Cell::Used);
            // Line 3
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 7, y, Cell::Used);
            self.set_cell(x + 8, y, Cell::Used);
            self.set_cell(x + 10, y, Cell::Used);
            self.set_cell(x + 11, y, Cell::Used);
            self.set_cell(x + 12, y, Cell::Used);
            // Line 4
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 7, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 11, y, Cell::Used);
            // Line 5
            y += 1;
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 8, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 10, y, Cell::Used);
            // Line 6: Blank
            // Line 7
            y += 2;
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 8, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 10, y, Cell::Used);
            // Line 8
            y += 1;
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 7, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 11, y, Cell::Used);
            // Line 9
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 7, y, Cell::Used);
            self.set_cell(x + 8, y, Cell::Used);
            self.set_cell(x + 10, y, Cell::Used);
            self.set_cell(x + 11, y, Cell::Used);
            self.set_cell(x + 12, y, Cell::Used);
            // Line 10
            y += 1;
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 5, y, Cell::Used);
            self.set_cell(x + 7, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 12, y, Cell::Used);
            // Line 11
            y += 1;
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 4, y, Cell::Used);
            self.set_cell(x + 8, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            // Line 12
            y += 1;
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x + 9, y, Cell::Used);
            self.set_cell(x + 10, y, Cell::Used);
        } else {
            eprintln!("Unable to insert Pulsar.");
        }
    }

    fn block(&mut self, x: usize, y: usize) {
        let w = 2; // width
        let h = 2; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x, y + 1, Cell::Used);
            self.set_cell(x + 1, y + 1, Cell::Used);
        }
        // else {
        //     eprintln!("Unable to insert Block.");
        // }
    }

    fn blinker(&mut self, x: usize, y: usize) {
        let w = 1; // width
        let h = 2; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x, y + 1, Cell::Used);
            self.set_cell(x, y + 2, Cell::Used);
        }
    }

    fn toad(&mut self, x: usize, y: usize) {
        let w = 4; // width
        let h = 2; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x + 2, y, Cell::Used);
            self.set_cell(x + 3, y, Cell::Used);
            self.set_cell(x, y + 1, Cell::Used);
            self.set_cell(x + 1, y + 1, Cell::Used);
            self.set_cell(x + 2, y + 1, Cell::Used);
        }
    }

    fn lighthouse(&mut self, x: usize, y: usize) {
        let w = 4; // width
        let h = 4; // height

        let nr = self.nrows();
        let nc = self.ncols();

        if x + (w - 1) < nc && y + (h - 1) < nr {
            // If it fits -> place it
            self.set_cell(x, y, Cell::Used);
            self.set_cell(x + 1, y, Cell::Used);
            self.set_cell(x, y + 1, Cell::Used);
            self.set_cell(x + 1, y + 1, Cell::Used);
            self.set_cell(x + 2, y + 2, Cell::Used);
            self.set_cell(x + 3, y + 2, Cell::Used);
            self.set_cell(x + 2, y + 2, Cell::Used);
            self.set_cell(x + 2, y + 3, Cell::Used);
            self.set_cell(x + 3, y + 3, Cell::Used);
        }
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Escribimos en el 'buffer' f la representación deseada
        self.curr_gen.iter().for_each(|v| {
            v.iter().for_each(|cell| {
                write!(
                    f,
                    "{}",
                    if *cell == Cell::Used {
                        self.used_char
                    } else {
                        self.unused_char
                    }
                );
            });
            writeln!(f);
        });
        Ok(())
    }
}

// -- Tests: --------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nrows() {
        let gol = GameOfLife::new(20, 30);

        assert_eq!(gol.nrows(), 20)
    }

    #[test]
    fn test_ncols() {
        let gol = GameOfLife::new(20, 30);

        assert_eq!(gol.ncols(), 30)
    }

    #[test]
    fn test_num_neighbors00() {
        let gol = GameOfLife::new(20, 30);
        let n = gol.num_neighbors(0, 0);

        assert_eq!(n, 3);
    }

    #[test]
    fn test_neighbors00_len() {
        let gol = GameOfLife::new(20, 30);
        let nb = gol.neighbors(0, 0);

        assert_eq!(nb.len(), 3);
    }

    #[test]
    fn test_neighbors00_items() {
        let gol = GameOfLife::new(20, 30);
        let nb = gol.neighbors(0, 0);
        let expected = vec![(0, 1), (1, 0), (1, 1)];

        assert_eq!(nb, expected);
    }

    #[test]
    fn test_num_neighbors11() {
        let gol = GameOfLife::new(20, 30);
        let n = gol.num_neighbors(1, 1);

        assert_eq!(n, 8);
    }

    #[test]
    fn test_neighbors11_len() {
        let gol = GameOfLife::new(20, 30);
        let nb = gol.neighbors(1, 1);

        assert_eq!(nb.len(), 8);
    }

    #[test]
    fn test_neighbors11_items() {
        let gol = GameOfLife::new(20, 30);
        let nb = gol.neighbors(1, 1);
        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];

        assert_eq!(nb, expected);
    }

    #[test]
    fn test_num_neighbors52() {
        let gol = GameOfLife::new(5, 6);
        let n = gol.num_neighbors(5, 2);

        assert_eq!(n, 5);
    }

    #[test]
    fn test_neighbors52_len() {
        let gol = GameOfLife::new(5, 6);
        let nb = gol.neighbors(5, 2);

        assert_eq!(nb.len(), 5);
    }

    #[test]
    fn test_neighbors52_items() {
        let gol = GameOfLife::new(5, 6);
        let nb = gol.neighbors(5, 2);
        let expected = vec![(4, 1), (4, 2), (4, 3), (5, 1), (5, 3)];

        assert_eq!(nb, expected);
    }

    #[test]
    fn test_neighbors50_items() {
        let gol = GameOfLife::new(5, 6);
        let nb = gol.neighbors(5, 0);
        let expected = vec![(4, 0), (4, 1), (5, 1)];

        assert_eq!(nb, expected);
    }

    #[test]
    fn test_num_neighbors50() {
        let gol = GameOfLife::new(5, 6);
        let n = gol.num_neighbors(5, 0);

        assert_eq!(n, 3);
    }

    #[test]
    fn test_neighbors04_items() {
        let gol = GameOfLife::new(5, 6);
        let nb = gol.neighbors(0, 4);
        let expected = vec![(0, 3), (1, 3), (1, 4)];

        assert_eq!(nb, expected);
    }

    #[test]
    fn test_num_neighbors04() {
        let gol = GameOfLife::new(5, 6);
        let n = gol.num_neighbors(0, 4);

        assert_eq!(n, 3);
    }

    #[test]
    fn test_neighbors54_items() {
        let gol = GameOfLife::new(5, 6);
        let nb = gol.neighbors(5, 4);
        let expected = vec![(4, 3), (4, 4), (5, 3)];

        assert_eq!(nb, expected);
    }

    #[test]
    fn test_num_neighbors54() {
        let gol = GameOfLife::new(5, 6);
        let n = gol.num_neighbors(5, 4);

        assert_eq!(n, 3);
    }
}
