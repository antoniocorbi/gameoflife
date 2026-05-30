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

// User interface zoom
pub const UI_ZOOM: f32 = 2.0;

// Thread sleep in microseconds

pub const SLEEP: u64 = 40000;

// World dimensions
pub const NCOLS: f32 = 20.0;
pub const NROWS: f32 = 20.0;
pub const WR_MIN: [f32; 2] = [0.0, 0.0];
pub const WR_MAX: [f32; 2] = [NROWS, NCOLS];

// Canvas dimensions
pub const CANVAS_W: f32 = 1024.0;
pub const CANVAS_H: f32 = 768.0;
pub const BG_COLOR_BLUE: &str = "#143c64";
pub const BG_COLOR_W95: &str = "#008080"; // W95
pub const BG_COLOR_WINE: &str = "#9B4F62"; // Wine
pub const BG_COLOR: &str = BG_COLOR_W95;
pub const FG_COLOR_TOMATO: &str = "#FF6347"; // Blanco Lino
pub const FG_COLOR_CLEAR: &str = "#FDFBF7"; // Blanco Lino
pub const FG_COLOR: &str = FG_COLOR_TOMATO; // Blanco Lino
pub const LINE_COLOR: &str = "#00E5FF"; // Cian Eléctrico

// Delta time for status bar update.
pub const DELTA_TIME: f64 = 3.0;

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub const LOREM_IPSUM_LONG: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Curabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam various, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris. Integer in mauris eu nibh euismod gravida. Duis ac tellus et risus vulputate vehicula. Donec lobortis risus a elit. Etiam tempor. Ut ullamcorper, ligula eu tempor congue, eros est euismod turpis, id tincidunt sapien risus a quam. Maecenas fermentum consequat mi. Donec fermentum. Pellentesque malesuada nulla a mi. Duis sapien sem, aliquet nec, commodo eget, consequat quis, neque. Aliquam faucibus, elit ut dictum aliquet, felis nisl adipiscing sapien, sed malesuada diam lacus eget erat. Cras mollis scelerisque nunc. Nullam arcu. Aliquam consequat. Curabitur augue lorem, dapibus quis, laoreet et, pretium ac, nisi. Aenean magna nisl, mollis quis, molestie eu, feugiat in, orci. In hac habitasse platea dictumst.";
