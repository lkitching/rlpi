//based on listing 10-3 (page 197)

use libc::{setlocale, LC_ALL, exit, EXIT_SUCCESS, tm, memset, mktime};
use std::ffi::{CString, c_void};
use std::mem::{self, MaybeUninit};
use crate::error_functions::{usage_err, err_exit, fatal};
use crate::libc::time::{strptime};
use crate::util::{fmt_strftime};

pub fn main(args: &[String]) -> ! {
    if args.len() < 3 {
	usage_err(&format!("{} input-date-time in-format [out-format]", args[0]));
    }

    let lcstr = CString::new("").expect("Failed to create CString");
    if unsafe { setlocale(LC_ALL, lcstr.as_ptr()) }.is_null() {
	err_exit("setlocale");
    }

    let mut tm: MaybeUninit<tm> = unsafe { MaybeUninit::uninit() };
    unsafe {
	memset(tm.as_mut_ptr() as *mut c_void, 0, mem::size_of::<tm>());

	let in_format_cs = CString::new(args[1].as_str()).expect("Failed to create CString");
	let out_format_cs = CString::new(args[2].as_str()).expect("Failed to create CString");
	if unsafe { strptime(in_format_cs.as_ptr(), out_format_cs.as_ptr(), tm.as_mut_ptr()) }.is_null() {
	    fatal("strptime");
	}
    }

    let mut tm = unsafe { tm.assume_init() };

    // not set by strptime(); tells mktime() to determine if DST is in
    // effect
    tm.tm_isdst = -1;
    println!("calendar time (seconds since Epoch): {}", unsafe { mktime(&mut tm) });

    let ofmt = if args.len() > 3 {
	args[3].as_str()
    } else {
	"%H:%M:%S %A, %d %B %Y %Z"
    };

    let fr = fmt_strftime(ofmt, &tm);
    match fr {
	Ok(ref s) => {
	    println!("strftime() yields: {}", s);
	},
	Err(_) => {
	    fatal("strftime returned 0");
	}
    }
    

    unsafe { exit(EXIT_SUCCESS); }
}
