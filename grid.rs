#[link(name = "grid", vers = "1.0", author = "jbaldwin")];
#[crate_type = "lib"];

extern mod std;
extern mod extra;

use std::path;
use std::io;
use std::iter;
//use std::task::spawn;

pub trait Printable {
	fn print(&self);
}

pub enum CellState {
	Alive,
	Dead
}

pub struct Cell {
	state: CellState
}

impl Cell {
	pub fn alive(&self, neighbours_alive: int) -> CellState {

		match self.state {
			Alive => match neighbours_alive {
				2 | 3 => Alive,	// Rule #2
				_ => Dead		// Rules #1, #3
			},
			Dead => match neighbours_alive {
				3 => Alive,		// Rule #4
				_ => Dead
			}
		}
	}
}

pub struct Grid {
	curr: ~[~[Cell]],
	next: ~[~[Cell]]
}

impl Grid {

	pub fn width(&self) -> int { self.curr.len() as int }
	pub fn height(&self) -> int { self.curr[0].len() as int }

	pub fn step(&mut self) {
		/*
		* Rules
		* 1) Any live cell with fewer than two live neighbours dies.
		* 2) Any live cell with two or three live neighbours lives.
		* 3) Any live cell with more than three live neighbours dies.
		* 4) Any dead cell with exactly three live neighbours becomes a live cell.
		*/

		static neighbours: &'static [&'static [int]] = &[
			&[-1, -1],
			&[ 0, -1],
			&[ 1, -1],
			&[-1,  0],
			&[ 1,  0],
			&[-1,  1],
			&[ 0,  1],
			&[ 1,  1]
		];

		for row in iter::range(0, self.width()) {
			for col in iter::range(0, self.height()) {
				let mut count = 0;

				for n in iter::range(0, neighbours.len() as int) {
					let x = neighbours[n][0] + row;
					let y = neighbours[n][1] + col;

					if self.in_bounds(x, y) {
						count +=  match self.curr[x][y].state {
							Dead => 0,
							Alive => 1,
						};
					}

					// stop counting, the cell is dead
					if count >= 4 { break; }
				}
				self.next[row][col].state = self.curr[row][col].alive(count);
			}
		}

		// set curr to next and next to curr for quick buffer swapping
		self.swap_buffers();
	}

	/**
	* Loads the input grid file.
	* File format is line delmited '0's and '1's.
	* 1 = The square is true|on in the game of life.
	* 0 = The square is false|off in the game of life.
	**/
	pub fn load(file: &path::Path) -> ~Grid {

		let read_result = io::file_reader(file);

		let lines = match read_result {
			Ok(file) => file.read_lines(),
			Err(e) => {
				fail!(fmt!("Error reading input grid file: %?", e))
			}
		};

		let mut g: ~Grid = ~Grid { curr: ~[], next: ~[] };

		for line in lines.iter() {
			let mut row1: ~[Cell] = ~[];
			let mut row2: ~[Cell] = ~[];
			for c in line.iter() {
				let cell: Cell = match c {
					'0' => Cell { state: Dead },
					'1' => Cell { state: Alive },
					_ => fail!(fmt!("Error parsing input grid file: %c is not valid, 0 or 1 only.", c))
				};
				row1.push(cell);
				row2.push(cell);
			}
			g.curr.push(row1);
			g.next.push(row2);
		}

		return g;
	}

	fn in_bounds(&self, x: int, y: int) -> bool {
		if x >= 0 && x < self.width() &&
		   y >= 0 && y < self.height() 
		   { true } else { false }
	}

	fn swap_buffers(&mut self) {
		unsafe {
			std::ptr::swap_ptr(
				std::ptr::to_mut_unsafe_ptr(&mut self.curr), 
				std::ptr::to_mut_unsafe_ptr(&mut self.next));
		}
	}
}

impl Printable for Grid {
	fn print(&self) {
		for row in self.curr.iter() {
			for cell in row.iter() {
				print(fmt!("%c", match cell.state {
					Dead => '-',
					Alive => '*'
				}));
			}
			println("");
		}
	}
}