extern mod extra;
use std::os;
use std::path;
use std::io;
use extra::getopts::*;

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

fn print_grid(grid: ~[~[bool]]) {
	for grid.iter().advance() |row| {
		for row.iter().advance() |b| {
			print(fmt!("%c", match b {
				&false => '0',
				&true => '1'
			}));
		}
		println("");
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
	let grid: ~[~[bool]] = load(input_path);
	print_grid(grid);
}