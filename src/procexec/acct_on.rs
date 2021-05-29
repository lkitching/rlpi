//listing 28-1 (page 592)
use std::ffi::{CString};
use std::ptr;

use libc::{exit, EXIT_SUCCESS, acct};

use crate::error_functions::{usage_err, err_exit};

pub fn main(args: &[String]) -> ! {
    if args.len() > 2 || (args.len() > 1 && args[1].as_str() == "--help") {
	usage_err(&format!("{} [file]\n", args[0]));
    }

    let acct_file = args.get(1);
    let status = match acct_file {
	None => {
	    unsafe { acct(ptr::null()) }
	},
	Some(file) => {
	    let cs = CString::new(file.as_str()).expect("Failed to create CString");
	    unsafe { acct(cs.as_ptr()) }
	}
    };

    if status == -1 {
	err_exit("acct");
    }

    let state = if acct_file.is_some() { "enabled" } else { "disabled" };
    println!("Process accounting {}", state);

    unsafe { exit(EXIT_SUCCESS); }
}
