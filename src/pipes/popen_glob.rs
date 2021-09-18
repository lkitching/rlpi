//listing 44-5 (page 904)
use std::{io};
use std::io::{Write};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

use libc::{PATH_MAX, popen, pclose, fgets};
use regex::{Regex};

extern crate rlpi;
use rlpi::procexec::print_wait_status::{print_wait_status};

pub fn main() {
    let glob_regex = Regex::new(r"^[A-Za-z0-9_*?\[\]]+$").expect("Failed to create regex");

    loop {
		print!("pattern: ");
		io::stdout().flush().expect("Failed to flush stdout");

	let line = {
	    let mut s = String::new();
	    io::stdin().read_line(&mut s).expect("Failed to read stdin");
	    s
	};

	if line.is_empty() {
	    continue;
	}

	// ensure that the pattern only contains valid characters
	// i.e. letters, digits, underscore, dot and the shell
	// globbing characters
	let glob = line.trim();	
	if !glob_regex.is_match(&glob) {
	    println!("Bad pattern");
	    continue;
	}

	// build and execute command to glob
	let cmd = format!("/bin/ls -d {} 2> /dev/null", glob);

	let fp = {
	    let cmd_s = CString::new(cmd).expect("Failed to create CString");
	    let opt_s = CString::new("r").expect("Failed to create CString");
	    unsafe { popen(cmd_s.as_ptr(), opt_s.as_ptr()) }
	};

	if fp.is_null() {
	    println!("popen() failed");
	    continue;
	}

	// read resulting list of pathnames until EOF
	let mut file_count = 0;
	let mut pathname: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
	
	while ! unsafe { fgets(pathname.as_mut_ptr(), PATH_MAX, fp) }.is_null() {
	    let path_s = unsafe { CStr::from_ptr(pathname.as_ptr()) };
	    println!("{}", path_s.to_str().expect("Failed to read CStr"));
	    file_count += 1;
	}

	// close pipe, fetch and display termination status
	let status = unsafe { pclose(fp) };
	println!("    {} matching file{}", file_count, if file_count == 1 { "" } else { "s"});
	println!("    pclose() status == {}", status);

	if status != 1 {
	    print_wait_status(Some("\t"), status);
	}
    }
}
