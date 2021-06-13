//listing 28-3 (page 601)
use std::ptr;
use std::ffi::{CString};
use std::os::raw::{c_void, c_int, c_char};

use libc::{exit, EXIT_SUCCESS, open, O_RDWR, CLONE_FILES, size_t, malloc, SIGUSR1, SIGCHLD, SIG_IGN, SIG_ERR,
          waitpid, __WCLONE, EBADF, close, signal, clone, write};

use crate::libc::{errno};
use crate::error_functions::{err_exit, str_error};

extern "C" fn child_func(arg: *mut c_void) -> c_int {
    let fd = unsafe { *(arg as *mut c_int) };
    if unsafe { close(fd) } == -1 {
	err_exit("close");
    }
    0
}

pub fn main(args: &[String]) -> ! {
    let fd = unsafe {
	let path_s = CString::new("/dev/null").expect("Failed to create CString");
	open(path_s.as_ptr(), O_RDWR)
    };

    if fd == -1 {
	err_exit("open");
    }

    // if args.len() > 1 child shares file descriptor table with parent
    let flags = if args.len() > 1 { CLONE_FILES } else { 0 };

    // allocate stack for child
    const STACK_SIZE: size_t = 65536;
    let stack = unsafe { malloc(STACK_SIZE) };
    if stack.is_null() {
	err_exit("malloc");
    }

    // NOTE: stack grows downward on x86
    let stack_top = unsafe { stack.add(STACK_SIZE) };

    // ignore CHILD_SIG in case it is a signal which terminates the
    // process by default but don't ignore SIGCHLD which is ignored by
    // default since that would prevent the creation of a zombie
    // process
    const CHILD_SIG: c_int = SIGUSR1;
    if CHILD_SIG != 0 && CHILD_SIG != SIGCHLD {
	if unsafe { signal(CHILD_SIG, SIG_IGN) } == SIG_ERR {
	    err_exit("signal");
	}
    }

    // create child
    // child continues execution in child_func
    {
	let mut fd = fd;
	if unsafe { clone(child_func, stack_top, flags | CHILD_SIG, &mut fd as *mut c_int as *mut c_void) } == -1 {
	    err_exit("clone");
	}
    }

    // parent continues here - wait for child. __WCLONE is required
    // for child notifying with signal other than SIGCHLD
    let wait_opts = if CHILD_SIG != SIGCHLD { __WCLONE } else { 0 };
    if unsafe { waitpid(-1, ptr::null_mut(), wait_opts) } == -1 {
	err_exit("waitpid");
    }

    println!("child has terminated");

    // did close() of file descriptor in child affect parent?

    let buf: [c_char; 1] = [ 120 ]; // 'x'
    let s = unsafe { write(fd, buf.as_ptr() as *const c_void, buf.len()) };
    if s == -1 {
	let en = errno();
	if en == EBADF {
	    println!("file descriptor {} has been closed", fd);
	} else {
	    println!("write() on file descriptor {} failed unexpectedly ({})",
		     fd,
		     str_error(en));
	}
    } else {
	println!("write() on file descriptor {} succeeded", fd);
    }    
    
    unsafe { exit(EXIT_SUCCESS); }
}
