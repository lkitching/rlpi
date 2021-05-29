//listing 27-9 (page 586)
use std::os::raw::{c_int, c_char};
use std::mem::{MaybeUninit};
use std::ptr;
use std::ffi::{CString};

use libc::{SIGCHLD, sigaddset, sigprocmask, SIG_IGN, SIG_DFL, sigaction, SIGINT, SIGQUIT, SIG_BLOCK, fork, EINTR,
           waitpid, SIG_SETMASK, pid_t, execl, _exit};

use crate::libc::{errno, set_errno};
use crate::signals::signal_functions::{sig_empty_set};

fn wait_for_child(child_pid: pid_t) -> Result<c_int, ()> {
    let mut status = 0;
    while unsafe { waitpid(child_pid, &mut status, 0) } == -1 {
	// NOTE: EINTR set if SIGCHLD or unblocked signal was caught
	if errno() != EINTR {
	    return Err(());
	}
    }
    Ok(status)
}

pub fn system(command: Option<&str>) -> Result<c_int, ()> {
    match command {
	None => {
	    system(Some(":"))
	},
	Some(command) => {
	    // block SIGCHLD
	    let mut block_mask = sig_empty_set();
	    let orig_mask = unsafe {
		let mut orig_mask = MaybeUninit::uninit();
		sigaddset(&mut block_mask, SIGCHLD);
		sigprocmask(SIG_BLOCK, &block_mask, orig_mask.as_mut_ptr());
		orig_mask.assume_init()
	    };

	    // ignore SIGINT and SIGQUIT
	    let sa_ignore = sigaction {
		sa_sigaction: SIG_IGN,
		sa_flags: 0,
		sa_mask: sig_empty_set(),
		sa_restorer: None
	    };

	    let sa_orig_int = unsafe {
		let mut h = MaybeUninit::uninit();
		sigaction(SIGINT, &sa_ignore, h.as_mut_ptr());
		h.assume_init()
	    };

	    let sa_orig_quit = unsafe {
		let mut h = MaybeUninit::uninit();
		sigaction(SIGQUIT, &sa_ignore, h.as_mut_ptr());
		h.assume_init()
	    };

	    let child_pid = unsafe { fork() };
	    let result = match child_pid {
		-1 => {
		    // fork() failed
		    Err(())
		},
		0 => {
		    // child
		    // exec command
		    let sa_default = sigaction {
			sa_sigaction: SIG_DFL,
			sa_flags: 0,
			sa_mask: sig_empty_set(),
			sa_restorer: None
		    };

		    if sa_orig_int.sa_sigaction != SIG_IGN {
			unsafe { sigaction(SIGINT, &sa_default, ptr::null_mut()); }
		    }

		    if sa_orig_quit.sa_sigaction != SIG_IGN {
			unsafe { sigaction(SIGINT, &sa_default, ptr::null_mut()); }
		    }

		    unsafe { sigprocmask(SIG_SETMASK, &orig_mask, ptr::null_mut()); }

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
		    // wait for child to terminate
		    wait_for_child(child_pid)
		}
	    };

	    // the following may change 'errno'
	    let saved_errno = errno();

	    unsafe {
		sigprocmask(SIG_SETMASK, &orig_mask, ptr::null_mut());
		sigaction(SIGINT, &sa_orig_int, ptr::null_mut());
		sigaction(SIGQUIT, &sa_orig_quit, ptr::null_mut());
	    }

	    set_errno(saved_errno);

	    result
	}
    }
}
