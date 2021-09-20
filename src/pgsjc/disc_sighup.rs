//listing 34-4 (page 712)
use std::os::raw::{c_int};
use std::{env, ptr};

use libc::{setbuf, getpid, tcgetpgrp, STDIN_FILENO, setpgid, sigaction, alarm, pause, sighandler_t, getpgrp, SIGHUP};

use rlpi::libc::stdio::{stdout};
use rlpi::error_functions::{usage_err, err_exit};
use rlpi::util::{fork_or_die, ForkResult};
use rlpi::signals::signal_functions::{sig_empty_set, str_signal};

extern "C" fn handler(sig: c_int) {
    println!("PID {}: caught signal {} ({})",
	     unsafe { getpid() },
	     sig,
	     str_signal(sig));
}

pub fn main() {
	let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
		usage_err(&format!("{} {{d|s}}... [ > sig.log 2>&1 ]", args[0]));
    }

    unsafe { setbuf(stdout, ptr::null_mut()); }

    let parent_pid = unsafe { getpid() };
    println!("PID of parent process is:          {}", parent_pid);
    println!("Foreground process id group ID is: {}", unsafe { tcgetpgrp(STDIN_FILENO) });

    // create child processes
    for arg in args[1..].iter() {
		match fork_or_die() {
			ForkResult::Child => {
				if let Some('d') = arg.chars().next() {
					// put child in new process group
					if unsafe { setpgid(0, 0) } == -1 {
					err_exit("setpgid");
					}
				}

				let sa = sigaction {
					sa_mask: sig_empty_set(),
					sa_flags: 0,
					sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
					sa_restorer: None
				};

				if unsafe { sigaction(SIGHUP, &sa, ptr::null_mut()) } == -1 {
					err_exit("sigaction");
				}

				// child exits loop
				break;
			},
			ForkResult::Parent(_child_pid) => {
			}
		}
    }

    // all processes fall through to here
    // ensure each process eventually terminates
    unsafe { alarm(60); }

    println!("PID={} PGID={}",
	     unsafe { getpid() },
	     unsafe { getpgrp() });

    // wait for signals
    loop {
	unsafe { pause(); }
    }
}
