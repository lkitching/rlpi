//listing 16-1 (page 317)
use std::os::raw::{c_char, c_void};
use std::ffi::{CString, CStr};

use libc::{exit, EXIT_SUCCESS, listxattr, getxattr, strlen};
use clap::{Arg, App};

use crate::error_functions::{err_exit};

pub fn main(args: &[String]) -> ! {
    let matches = App::new("View file extended attributes")
	.version("1.0")
	.about("Displays extended attribute data for files")
	.arg(Arg::with_name("display_hex")
	     .short("x")
	     .required(false)
	     .takes_value(false)
	     .help("Display values as hexadecimal"))
	.arg(Arg::with_name("files")	     
	     .required(true)
	     .multiple(true))
	.get_matches();

    let hex = matches.is_present("display_hex");
    let files: Vec<&str> = matches.values_of("files").expect("No file argument").collect();

    let mut names_buf: [c_char; 10000] = [0; 10000];
    let mut value_buf: [c_char; 10000] = [0; 10000];

    for file in files {
	let file_s = CString::new(file).expect("Failed to create CString");
	let list_len = unsafe { listxattr(file_s.as_ptr(), names_buf.as_mut_ptr(), 10000) };
	if list_len == -1 {
	    err_exit("listxattr");
	}

	println!("{}", file);

	//loop through all EA names and display name + value
	let mut name_p = names_buf.as_mut_ptr();
	for i in 0..list_len {
	    let name_s = unsafe { CStr::from_ptr(name_p).to_str().expect("Failed to read CStr") };
	    print!("name = {}; ", name_s);

	    let value_len = unsafe { getxattr(file_s.as_ptr(), name_p, value_buf.as_mut_ptr() as *mut c_void, 10000) };
	    if value_len == -1 {
		print!("coudn't get value");
	    } else {		
		if hex {
		    for j in 0..(value_len as usize) {
			print!("{:x?}", value_buf[j]);
		    }
		} else {
		    let value_s = unsafe { CStr::from_ptr(value_buf.as_ptr()).to_str().expect("Failed to read value CStr") };		
		    print!("{}", value_s);
		}

		println!("");
	    }

	    let name_len = unsafe { strlen(name_p) };
	    name_p = unsafe { name_p.add(name_len + 1) };
	}
    }

    unsafe { exit(EXIT_SUCCESS); }
}
