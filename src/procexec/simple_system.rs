//listing 27-8 (page 582)
use std::os::raw::{c_int, c_char};
use std::ffi::{CString};
use std::ptr;

use libc::{fork, _exit, waitpid, execl};

pub fn system(command: &str) -> Result<c_int, ()> {
    let child_pid = unsafe { fork() };
    match child_pid {
	-1 => {
	    Err(())
	},
	0 => {	    
	    // child
	    let sh_cmd = CString::new("/bin/sh").expect("Failed to create CString");
	    let arg1 = CString::new("sh").expect("Failed to create CString");
	    let arg2 = CString::new("-c").expect("Failed to craete CString");
	    let arg3 = CString::new(command).expect("Failed to create CString");
	    unsafe { execl(sh_cmd.as_ptr(),
			   arg1.as_ptr(),
			   arg2.as_ptr(),
			   arg3.as_ptr(),
			   ptr::null::<c_char>()); }

	    // failed exec
	    unsafe { _exit(127); }
	},
	_ => {
	    // parent
	    let mut status = 0;
	    if unsafe { waitpid(child_pid, &mut status, 0) } == -1 {
		Err(())
	    } else {
		Ok(status)
	    }
	}
    }
}
