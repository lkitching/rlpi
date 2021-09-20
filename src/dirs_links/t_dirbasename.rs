//listing 18-5 (page 371)
use std::ffi::{CString, CStr};
use std::env;

use libc::{exit, EXIT_SUCCESS};

use rlpi::libc::libgen::{dirname, basename};
use rlpi::util::{vec_u8_into_i8};

pub fn main() {
	let args: Vec<String> = env::args().collect();
    for path in args.iter().skip(1) {
		let path_s = CString::new(path.as_str()).expect("Failed to create CString");
		let path_s2 = path_s.clone();

		let dir: String = unsafe {
			let bytes = path_s.to_bytes_with_nul().iter().map(|b| *b).collect();
			let mut bytes = vec_u8_into_i8(bytes);
			let buf = dirname(bytes.as_mut_ptr());
			CStr::from_ptr(buf).to_str().expect("Failed to read CStr").to_owned()
		};

		let base: String = unsafe {
			let bytes = path_s2.to_bytes_with_nul().iter().map(|b| *b).collect();
			let mut bytes = vec_u8_into_i8(bytes);
			let buf = basename(bytes.as_mut_ptr());
			CStr::from_ptr(buf).to_str().expect("Failed to read CStr").to_owned()
		};

		println!("{} ==> {} + {}", path, dir, base);
    }

    unsafe { exit(EXIT_SUCCESS); }
}
