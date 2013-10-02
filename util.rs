#[link(name = "util", vers = "1.0", author = "jbaldwin")];
#[crate_type = "lib"];

pub mod thread {
	extern mod std;

	pub fn sleep(ms: u64) {
		std::rt::io::timer::sleep(ms);
	}
}

pub mod console {
	extern mod std;

	use std::libc::funcs::c95::stdlib::*;

	#[fixed_stack_segment]
	pub fn clear() {
		unsafe {
			system("clear".to_c_str().unwrap());
		}
	}
}