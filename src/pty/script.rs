//listing 64-3
use std::mem::{MaybeUninit};
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::env;
use std::ffi::{CString};

use libc::{termios, tcgetattr, tcsetattr, STDIN_FILENO, STDOUT_FILENO, winsize, TIOCGWINSZ, execlp, ioctl, open, read, write, FD_ZERO, FD_SET, FD_ISSET,
	   O_WRONLY, O_CREAT, O_TRUNC, S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH, atexit, fd_set, select, exit, EXIT_SUCCESS, TCSANOW};

use rlpi::pty::pty_fork::{pty_fork};
use rlpi::tty::tty_functions::{tty_set_raw};
use rlpi::error_functions::{err_exit, fatal};

const BUF_SIZE: usize = 256;
static mut TTY_ORIG: Option<termios> = None;

extern "C" fn tty_reset() {
    if let Some(settings) = unsafe { TTY_ORIG } {
		if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &settings) } == -1 {
			err_exit("tcsetattr");
		}
    }
}

pub fn main() {
	let args: Vec<String> = env::args().collect();

    let tty = unsafe {
		let mut tty: MaybeUninit<termios> = MaybeUninit::uninit();
		if tcgetattr(STDIN_FILENO, tty.as_mut_ptr()) == -1 {
			err_exit("tcgetattr");
		}
		tty.assume_init()
    };

    unsafe { TTY_ORIG = Some(tty); }

    let ws = unsafe {
		let mut ws: MaybeUninit<winsize> = MaybeUninit::uninit();
		if ioctl(STDIN_FILENO, TIOCGWINSZ, ws.as_mut_ptr()) < 0 {
			err_exit("ioctl-TIOCSWINSZ");
		}
		ws.assume_init()
    };

    match pty_fork(Some(tty), Some(ws)) {
		Err(msg) => {
			err_exit(&format!("pty_fork failed: {}", msg));
		},
		Ok(None) => {
			// child
			// TODO: use var_os?
			let shell_cmd = env::var("SHELL").unwrap_or("/bin/sh".to_string());
			unsafe {
				let cmd_s = CString::new(shell_cmd).expect("Failed to create CString");
				execlp(cmd_s.as_ptr(), cmd_s.as_ptr(), ptr::null() as *const c_char);
			}
			// something went wrong if we get here
			err_exit("execlp");
		},
		Ok(Some(fork_info)) => {
			// parent - relay data between terminal and pty master
			let master_fd = fork_info.pty.master_fd;
			let filename = args.get(1).map(|s| s.as_str()).unwrap_or("typescript");
			let script_fd = unsafe {
				let filename_s = CString::new(filename).expect("Failed to create CString");
				let mode = O_WRONLY | O_CREAT | O_TRUNC;
				let flags = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
				open(filename_s.as_ptr(), mode, flags)
			};

			if script_fd == -1 {
				err_exit("open typescript");
			}

			match tty_set_raw(STDIN_FILENO) {
				Err(_msg) => {
					err_exit("tty_set_raw");
				},
				Ok(previous) => {
					unsafe { TTY_ORIG = Some(previous); }
				}
			}

			// reset terminal settings on exit
			if unsafe { atexit(tty_reset) } != 0 {
				err_exit("atexit");
			}

			loop {
				let mut in_fds = unsafe {
					// monitor stdin and the master terminal for new data
					let mut fds: MaybeUninit<fd_set> = MaybeUninit::uninit();
					FD_ZERO(fds.as_mut_ptr());
					FD_SET(STDIN_FILENO, fds.as_mut_ptr());
					FD_SET(master_fd, fds.as_mut_ptr());
					fds.assume_init()
				};

				if unsafe { select(master_fd + 1, &mut in_fds, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) } == -1 {
					err_exit("select");
				}

				if unsafe { FD_ISSET(STDIN_FILENO, &mut in_fds) } {
					// data on stdin so read and copy to pty

					let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
					let num_read = unsafe { read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, BUF_SIZE) };
					if num_read <= 0 {
					unsafe { exit(EXIT_SUCCESS); }
					}

					if unsafe { write(master_fd, buf.as_mut_ptr() as *mut c_void, num_read as usize) } != num_read {
					fatal("partial/failed write (master_fd)");
					}
				}

				if unsafe { FD_ISSET(master_fd, &mut in_fds) } {
					// data written to pty output
					// copy to stdout and the script file
					let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
					let num_read = unsafe { read(master_fd, buf.as_mut_ptr() as *mut c_void, BUF_SIZE) };

					if num_read <= 0 {
					unsafe { exit(EXIT_SUCCESS); }
					}

					if unsafe { write(STDOUT_FILENO, buf.as_mut_ptr() as *mut c_void, num_read as usize) } != num_read {
					fatal("partial/failed write (STDOUT_FILENO)");
					}

					if unsafe { write(script_fd, buf.as_mut_ptr() as *mut c_void, num_read as usize) } != num_read {
					fatal("partial/failed write (script_fd)");
					}
				}
			}
		}
    }
}
