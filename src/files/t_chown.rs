use std::ffi::{CString};
use libc::{exit, EXIT_SUCCESS, EXIT_FAILURE, chown};

use crate::error_functions::{usage_err, fatal};
use crate::users_groups::ugid_functions::{user_id_from_name, group_id_from_name};

pub fn main(args: &[String]) -> ! {
    if args.len() < 3 {
	usage_err(&format!("{} owner group [file ...]\n. Specify owner or group as '-' to leave unchanged", args[0]));
    }

    let uid = match user_id_from_name(args[1].as_str()) {
	Some(id) => id,
	None => { fatal(&format!("No such user ({})", &args[1])); }
    };

    let gid = match group_id_from_name(args[2].as_str()) {
	Some(id) => { id },
	None => {
	    fatal(&format!("No such group ({})", &args[2]));
	}
    };

    let mut had_error = false;
    for file in &args[2..] {
	let file_s = CString::new(file.as_str()).expect("Failed to create CString");
	if unsafe { chown(file_s.as_ptr(), uid, gid) } == -1 {
	    eprintln!("chown: {}", file);
	    had_error = true;
	}
    }

    unsafe { exit(if had_error { EXIT_FAILURE } else { EXIT_SUCCESS }); }
}
