extern mod extra;

use std::os;
use std::path;
use std::io;
use std::int;
use extra::getopts::*;

mod util;


fn print_usage(program_name: &str, _opts: &[Opt]) {
	println(fmt!("Usage: %s [options]", program_name));
	println("  -i\t\t-> Path to input file");
	println("  -h | -help\t-> Print this help message");
}

/**
* Loads the input grid file.
* File format is line delmited '0's and '1's.
* 1 = The square is true|on in the game of life.
* 0 = The square is false|off in the game of life.
**/
fn load(file: &path::Path) -> ~[~[bool]] {

	let read_result = io::file_reader(file);

	let lines = match read_result {
		Ok(file) => file.read_lines(),
		Err(e) => {
			fail!(fmt!("Error reading input grid file: %?", e))
		}
	};

	let mut grid: ~[~[bool]] = ~[];

	for lines.iter().advance() |line: &~str| {
		let mut next: ~[bool] = ~[];
		for line.iter().advance() |c| {
			let b: bool = match c {
				'0' => false,
				'1' => true,
				_ => fail!(fmt!("Error parsing input grid file: %c is not valid, 0 or 1 only.", c))
			};
			next.push(b)
		}
		grid.push(next);
	}

	return grid;
}

fn print_grid(grid: &[~[bool]]) {
	for grid.iter().advance() |row| {
		for row.iter().advance() |b| {
			print(fmt!("%c", match b {
				&false => '-',
				&true => '*'
			}));
		}
		println("");
	}
}

fn copy_grid(src: &[~[bool]], dst: &mut[~[bool]]) {
	for int::range(0, src.len() as int) |i| {
		for int::range(0, src[i].len() as int) |j| {
			dst[i][j] = src[i][j];
		}
	}
}

fn alive(currently_alive: bool, neighbours_alive: int) -> bool {

	match currently_alive {
		true => match neighbours_alive {
			2 | 3 => true,	// Rule #2
			_ => false		// Rules #1, #3
		},
		false => match neighbours_alive {
			3 => true,		// Rule #4
			_ => false
		}
	}
}

fn in_bounds(x: int, y: int, width: int, height: int) -> bool {
	if x >= 0 && x < width && y >= 0 && y < height { true } else { false }
}

fn game_of_life_step(grid: &mut[~[bool]],  next: &mut[~[bool]]) {
	/*
	* Rules
	* 1) Any live cell with fewer than two live neighbours dies.
	* 2) Any live cell with two or three live neighbours lives.
	* 3) Any live cell with more than three live neighbours dies.
	* 4) Any dead cell with exactly three live neighbours becomes a live cell.
	*/

	let neighbours = [
		[-1, -1],
		[ 0, -1],
		[ 1, -1],
		[-1,  0],
		[ 1,  0],
		[-1,  1],
		[ 0,  1],
		[ 1,  1]
	];

	for int::range(0, grid.len() as int) |row| {
		for int::range(0, grid[row].len() as int) |col| {
			let mut count = 0;

			for int::range(0, neighbours.len() as int) |n| {
				let x = neighbours[n][0] + row;
				let y = neighbours[n][1] + col;

				if in_bounds(x, y, grid.len() as int, grid[row].len() as int) {
					count +=  match grid[x][y] {
						true => 1,
						false => 0
					};
				}

				// stop counting, the cell is dead
				if count >= 4 { break; }
			}
			next[row][col] = alive(grid[row][col], count);
		}
	}
}


fn main() {

	let args : ~[~str] = os::args();

	let opts = ~[
		reqopt("i"),
		optflag("h"),
		optflag("help")
	];

	let program_name: ~str = args[0].clone();

	let opt_matches = match getopts(args.tail(), opts) {
		Ok(m) => { m }
		Err(f) => { 
			print_usage(program_name, opts);
			println("");
			fail!(fail_str(f))
		}
	};

	if opt_present(&opt_matches, "h") || opt_present(&opt_matches, "help") {
		print_usage(program_name, opts);
		return;
	}

	let input: &str = opt_str(&opt_matches, "i");
	let input_path = ~path::Path(input);
	let mut grid: ~[~[bool]] = load(input_path);
	let mut next: ~[~[bool]] = grid.clone();

	util::clear();
	print_grid(grid);
	util::sleep(500);

	loop {
		util::clear();
		//let user = io::stdin();
		game_of_life_step(grid, next);
		print_grid(next);
		copy_grid(next, grid);
		//user.read_line();
		util::sleep(500);
	}
}
