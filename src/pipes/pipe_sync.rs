//listing 44-3 (page 897)
use std::{env, ptr, thread, time};
use std::os::raw::{c_int, c_void};

use libc::{setbuf, exit, _exit, EXIT_SUCCESS, pipe, close, getpid, read};

extern crate rlpi;
use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::curr_time::{curr_time};
use rlpi::libc::stdio::{stdout};
use rlpi::util::{ForkResult, try_fork};

struct Pipe {
    read_fd: c_int,
    write_fd: c_int
}

fn create_pipe() -> Result<Pipe, ()> {
    let mut pipe_fds: [c_int; 2] = [0; 2];
    if unsafe { pipe(pipe_fds.as_mut_ptr()) } == -1 {
	Err(())
    } else {
	Ok(Pipe { read_fd: pipe_fds[0], write_fd: pipe_fds[1] })
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
	usage_err(&format!("{} sleep-time...", args[0]));
    }

    unsafe { setbuf(stdout, ptr::null_mut()); }

    println!("{} Parent started", curr_time("%T"));

    let pipe = create_pipe().expect("Failed to create pipe");

    for j in (1 .. args.len()) {
	match try_fork() {
	    Err(_) => { err_exit(&format!("fork {}", j)); },
	    Ok(ForkResult::Parent(_)) => {
		// parent loops to create next child
	    },
	    Ok(ForkResult::Child) => {
		// read end is unused
		if unsafe { close(pipe.read_fd) } == -1 {
		    err_exit("close");
		}

		// child does some work before notifying parent
		let sleep_seconds: u8 = args[j].parse().expect("Invalid sleep period");
		let sleep_period = time::Duration::from_secs(sleep_seconds as u64);
		thread::sleep(sleep_period);

		println!("{}  Child {} (PID={}) closing pipe",
			 curr_time("%T"),
			 j,
			 unsafe { getpid() });

		if unsafe { close(pipe.write_fd) } == -1 {
		    err_exit("close");
		}

		// child does other things

		unsafe { _exit(EXIT_SUCCESS); }
	    }	    
	}
    }

    // parent continues here
    // close write end of pipe so EOF can be read
    if unsafe { close(pipe.write_fd) } == -1 {
	err_exit("close");
    }

    // parent may do other work
    
    // synchronise with children
    let mut dummy_buf: [u8; 1] = [0; 1];
    if unsafe { read(pipe.read_fd, dummy_buf.as_mut_ptr() as *mut c_void, 1) } != 0 {
	fatal("parent did not get EOF");
    }

    println!("parent continuing");

    unsafe { exit(EXIT_SUCCESS); }	
}
