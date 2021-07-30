use std::os::raw::{c_int};
use std::ffi::{CStr};

use libc::{posix_openpt, O_RDWR, O_NOCTTY, grantpt, unlockpt, close, ptsname};

use crate::libc::{errno, set_errno};

fn bracket_close(fd: c_int) {
    let saved_errno = errno();
    unsafe { close(fd); }
    set_errno(saved_errno);
}

// TODO: implement Drop?
pub fn close_pty(pty: PtyInfo) {
    bracket_close(pty.master_fd);
}

pub struct PtyInfo {
    pub name: String,
    pub master_fd: c_int
}

pub fn pty_master_open() -> Result<PtyInfo, String> {
    // open pty master    
    let master_fd = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };
    if master_fd == -1 {
	return Err(String::from("Failed to open master"));
    }

    // grant access to slave pty
    if unsafe { grantpt(master_fd) } == -1 {
	bracket_close(master_fd);
	return Err(String::from("Failed to grant access to slave"));
    }

    // unlock slave pty
    if unsafe { unlockpt(master_fd) } == -1 {
	bracket_close(master_fd);
	return Err(String::from("Failed to unlock slave"));
    }

    let name = unsafe {
	let buf_p = ptsname(master_fd);
	if buf_p.is_null() {
	    bracket_close(master_fd);
	    return Err(String::from("Failed to get slave device name"));
	}
	let buf_cstr = CStr::from_ptr(buf_p);
	let buf_str = buf_cstr.to_str().expect("Failed to read CStr");
	String::from(buf_str)
    };

    Ok(PtyInfo { name: name, master_fd: master_fd })
}
