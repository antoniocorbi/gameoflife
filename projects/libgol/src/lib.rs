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

// -- Types: --------------------------------------------------------------
// type Cell = bool;

type Neighbor = (usize, usize);

#[derive(Clone, Copy)]
pub enum Cell {
    Used,
    Unused,
}

type Matrix = Vec<Vec<Cell>>;

pub struct GameOfLife {
    curr_gen: Matrix,
    next_gen: Matrix,
}

// -- Impl: ---------------------------------------------------------------

impl GameOfLife {
    fn new(nrows: usize, ncols: usize) -> Self {
        let curr_gen = vec![vec![Cell::Unused; ncols]; nrows];
        let next_gen = vec![vec![Cell::Unused; ncols]; nrows];

        GameOfLife { curr_gen, next_gen }
    }

    pub fn nrows(&self) -> usize {
        self.curr_gen.len()
    }

    pub fn ncols(&self) -> usize {
        self.curr_gen[0].len()
    }

    pub fn set_cell(&mut self, x: usize, y: usize, status: Cell) {
        assert!(x < self.ncols() && y < self.nrows());
        self.curr_gen[y][x] = status;
    }

    pub fn cell(&mut self, x: usize, y: usize) -> Cell {
        assert!(x < self.ncols() && y < self.nrows());
        self.curr_gen[y][x]
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<Neighbor> {
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
                    n.push((ix, iy));
                }
            }
        }

        // dbg!(&n);

        n
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
}
