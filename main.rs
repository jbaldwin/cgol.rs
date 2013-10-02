extern mod extra;

use std::os;
use std::path;
use extra::getopts::*;
use grid::{Grid, Printable};
use util::console;
use util::thread;

mod util;
mod grid;

fn print_usage(program_name: &str, _opts: &[Opt]) {
	println!("Usage: {:s} [options]", program_name);
	println("  -i\t\t-> Path to input file");
	println("  -h | -help\t-> Print this help message");
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
			fail!(f.to_err_msg());
		}
	};

	if opt_matches.opt_present("h") ||
	   opt_matches.opt_present("help") {
		print_usage(program_name, opts);
		return;
	}

	let input: &str = match opt_matches.opt_str("i") {
		Some(s) => s,
		None => fail!("Option -i not provided a value.")
	};

	let input_path = ~path::Path(input);
	let mut g: ~Grid = Grid::load(input_path);

	console::clear();
	g.print();
	thread::sleep(500);

	loop {
		console::clear();
		g.step();
		g.print();
		thread::sleep(500);
	}
}
