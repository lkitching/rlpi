//listing  37-2 (page 770)
use std::ops::{BitAnd};
use std::ffi::{CString};

use libc::{_exit, EXIT_SUCCESS, umask, setsid, chdir, close, open, O_RDWR, STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO, dup2};

use crate::util::{try_fork, ForkResult};

type daemon_flags = u16;
pub const BD_NO_CHDIR: daemon_flags = 0o1;
pub const BD_NO_CLOSE_FILES: daemon_flags = 0o2;
pub const BD_NO_REOPEN_STD_FDS: daemon_flags = 0o4;
pub const BD_NO_UMASK0: daemon_flags = 0o10;
pub const BD_MAX_CLOSE: daemon_flags = 8192;

pub fn is_set<T: Eq + Copy + BitAnd<Output = T>>(value: T, flag: T) -> bool {
    return value & flag == flag
}

pub fn become_daemon(flags: daemon_flags) -> Result<(), String> {
    // become background process
    if let ForkResult::Parent(_) = try_fork()? {
	unsafe { _exit(EXIT_SUCCESS); }
    }

    // child continues here
    // become leader of new session
    // this removes any controlling terminal
    if unsafe { setsid() } == -1 {
	return Err("Failed to create session".to_string());
    }

    // ensure we are not session leader
    // this prevents future calls to open etc. acquiring a controlling terminal
    if let ForkResult::Parent(_) = try_fork()? {
	unsafe { _exit(EXIT_SUCCESS); }
    }

    if ! is_set(flags, BD_NO_UMASK0) {
	// clear file mode creation mask
	unsafe { umask(0); }
    }

    if ! is_set(flags, BD_NO_CHDIR) {
	// change to root directory
	let dir_s = CString::new("/").expect("Failed to create CString");
	if unsafe { chdir(dir_s.as_ptr()) } == -1 {
	    return Err("Failed to change root directory".to_string());
	}
    }

    if ! is_set(flags, BD_NO_CLOSE_FILES) {
	// re-open standard file descriptors to /dev/null
	let path_s = CString::new("/dev/null").expect("Failed to create CString");
	
	unsafe {
	    close(STDIN_FILENO);
	    
	    let fd = open(path_s.as_ptr(), O_RDWR);
	    if fd != STDIN_FILENO {
		return Err("Unexpected opened file descriptor".to_string());
	    }

	    // duplicate stdin (on /dev/null) to stdout and stderr
	    if dup2(STDIN_FILENO, STDOUT_FILENO) != STDOUT_FILENO {
		return Err("Failed to duplicate stdin to stdout".to_string());
	    }

	    if dup2(STDIN_FILENO, STDERR_FILENO) != STDERR_FILENO {
		return Err("Failed to duplicate stdin to stderr".to_string());
	    }
	}	
    }

    if ! is_set(flags, BD_NO_REOPEN_STD_FDS) {
    }

    Ok(())
}
