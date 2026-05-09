// Copyright (C) 2026  Antonio-M. Corbi Bellot
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

pub enum Figure {
    // WxH
    Blinker,    // 1x3
    Toad,       // 4x2
    Lighthouse, // 4x4
    Pulsar,     // 13x13
    PentaDec,   // 5x12
    Glider,     // 3x3
    SShip1,     // 5x4
    SShip2,     // 6x4
    SShip3,     // 7x4
    Block,      // 2x2
    Hive,       // 4x3
    Pan,        // 4x4
    Boat,       // 3x3
    Bath,       // 3x3
}

// -- Traits: -------------------------------------------------------------
pub trait FigureExt {
    fn insert_figure(&mut self, f: Figure, x: usize, y: usize);
    fn blinker(&mut self, x: usize, y: usize);
    fn toad(&mut self, x: usize, y: usize);
    fn lighthouse(&mut self, x: usize, y: usize);
    fn pulsar(&mut self, x: usize, y: usize);
    fn penta_dec(&mut self, x: usize, y: usize);
    fn glider(&mut self, x: usize, y: usize);
    fn sship1(&mut self, x: usize, y: usize);
    fn sship2(&mut self, x: usize, y: usize);
    fn sship3(&mut self, x: usize, y: usize);
    fn block(&mut self, x: usize, y: usize);
    fn hive(&mut self, x: usize, y: usize);
    fn pan(&mut self, x: usize, y: usize);
    fn boat(&mut self, x: usize, y: usize);
    fn bath(&mut self, x: usize, y: usize);
}
