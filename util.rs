#[link(name = "util", vers = "1.0", author = "jbaldwin")];
#[crate_type = "lib"];

pub mod thread {
	extern mod std;
	extern mod extra;

	pub fn sleep(ms: uint) {
		let iotask = extra::uv_iotask::spawn_iotask(std::task::task());
		extra::timer::sleep(&iotask, ms);
		extra::uv_iotask::exit(&iotask);

	}
}

pub mod console {
	use std::libc::funcs::c95::stdlib::*;	

	pub fn clear() {
		unsafe {
			system("clear".as_c_str(|x| { x }));
		}
	}
}