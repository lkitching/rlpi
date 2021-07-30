use std::ffi::{CString};

use libc::{pid_t, fork, termios, winsize, ioctl, close, setsid, tcsetattr, TCSANOW, TIOCSWINSZ, dup2, STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO, open, O_RDWR};

use super::pty_master_open::{pty_master_open, PtyInfo, close_pty};
use crate::error_functions::{err_exit};

pub struct ForkInfo {
    pub child_pid: pid_t,
    pub pty: PtyInfo
}

pub fn pty_fork(slave_termios: Option<termios>, slave_ws: Option<winsize>) -> Result<Option<ForkInfo>, String> {
    let info = pty_master_open()?;

    let child_pid = unsafe { fork() };

    if child_pid == -1 {
	// fork failed
	close_pty(info);
	return Err(String::from("fork() failed"));	
    } else if child_pid != 0 {
	// parent
	return Ok(Some(ForkInfo { child_pid: child_pid, pty: info }));
    } else {
	// child
	// start a new session
	if unsafe { setsid() } == -1 {
	    err_exit("pty_fork:setsid");
	}

	// not needed in child
	unsafe { close(info.master_fd); }

	// open the slave terminal - this will become the controlling
	// terminal for the child
	let slave_fd = unsafe {
	    let cs = CString::new(info.name).expect("Failed to create CString");
	    open(cs.as_ptr(), O_RDWR)
	};

	if slave_fd == -1 {
	    err_exit("pty_fork:open-slave");
	}

	// acquire controlling tty on BSD

	// set slave tty attributes
	if let Some(termios) = slave_termios {
	    if unsafe { tcsetattr(slave_fd, TCSANOW, &termios) } == -1 {
		err_exit("pty_fork:tcsetattr");
	    }
	}

	// set slave window size
	if let Some(ws) = slave_ws {
	    if unsafe { ioctl(slave_fd, TIOCSWINSZ, &ws) } == -1 {
		err_exit("pty_fork:ioctl-TIOCSWINSZ");
	    }
	}

	// duplicate pty slave to be child's stdin, stdout and stderr
	if unsafe { dup2(slave_fd, STDIN_FILENO) } != STDIN_FILENO {
	    err_exit("pty_fork:dup2-STDIN_FILENO");
	}

	if unsafe { dup2(slave_fd, STDOUT_FILENO) } != STDOUT_FILENO {
	    err_exit("pty_fork:dup2-STDOUT_FILENO");
	}

	if unsafe { dup2(slave_fd, STDERR_FILENO) } != STDERR_FILENO {
	    err_exit("pty_fork:dup2-STDERR_FILENO");
	}

	// no longer need the slave fd
	// safety check
	if slave_fd > STDERR_FILENO {
	    unsafe { close(slave_fd); }
	}

	Ok(None)	
    }
}
