#[link(name = "util", vers = "1.0", author = "jbaldwin")];
#[crate_type = "lib"];

extern mod std;
extern mod extra;

use std::libc::funcs::c95::stdlib::*;	

pub fn sleep(ms: uint) {
	let iotask = extra::uv_iotask::spawn_iotask(std::task::task());
	extra::timer::sleep(&iotask, ms);
	extra::uv_iotask::exit(&iotask);

}

pub fn clear() {
	let cmd = "clear";
	unsafe {
		system(cmd.as_c_str(|x| { x }));
	}
}