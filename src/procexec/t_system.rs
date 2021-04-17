//listing 27-7 (page 581)
use std::io::{self, Write};
use std::ffi::{CString};

use libc::{exit, EXIT_SUCCESS, system, WIFEXITED, WEXITSTATUS};

use super::print_wait_status::{print_wait_status};
use crate::error_functions::{err_exit};

pub fn main(args: &[String]) -> ! {
    loop {
	print!("Command: ");
	io::stdout().flush().unwrap();
	
	let mut line = String::new();
	match io::stdin().read_line(&mut line) {
	    Ok(0) => {
		// EOF
		break;
	    },
	    Ok(_) => {
		let cs = CString::new(line).expect("Failed to create CString");
		let status = unsafe { system(cs.as_ptr()) };
		println!("system() returned: status={:#X} ({},{})",
			 status,
			 status >> 8,
			 status & 0xff);

		if status == -1 {
		    err_exit("system");
		} else {
		    if unsafe { WIFEXITED(status) } && unsafe { WEXITSTATUS(status) == 127 } {
			println!("(Probably) could not invoke shell");
		    } else {
			print_wait_status(None, status);
		    }
		}		
	    },
	    Err(_) => {
		println!("Failed to read from stdin");
	    }
	}
    }

    unsafe { exit(EXIT_SUCCESS); }
}
