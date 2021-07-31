//listing 34-5 (page 719)
extern crate rlpi;

use std::env;
use std::ptr;
use std::io::{self, Read, Write};
use std::os::raw::{c_int};

use libc::{getpid, getpgrp, tcgetpgrp, STDERR_FILENO, raise, SIGINT, SIGCONT, SIGTSTP, SIGSTOP, sigaction, SA_RESTART, sighandler_t,
           isatty, STDIN_FILENO, STDOUT_FILENO, getsid, getppid, pause};

use rlpi::signals::signal_functions::{sig_empty_set, str_signal};
use rlpi::error_functions::{err_exit};

static mut CMD_NUM: u8 = 0;

extern "C" fn handler(sig: c_int) {
    // unsafe: this handler uses non-async-signal-safe functions
    if unsafe { getpid() } == unsafe { getpgrp() } {
	eprintln!("Terminal FG process group: {}",
		  unsafe { tcgetpgrp(STDERR_FILENO) });
    }

    eprintln!("Process {} ({}) received signal {} ({})",
	      unsafe { getpid() },
	      unsafe { CMD_NUM },
	      sig,
	      str_signal(sig));

    // if signal is SIGTSTP it won't stop us. Raise SIGSTOP so this
    // process is actually stopped
    if sig == SIGTSTP {
	unsafe { raise(SIGSTOP); }
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    let sa = sigaction {
	sa_mask: sig_empty_set(),
	sa_flags: SA_RESTART,
	sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    for sig in vec![SIGINT, SIGTSTP, SIGCONT].into_iter() {
	if unsafe { sigaction(sig, &sa, ptr::null_mut()) } == -1 {
	    err_exit("sigaction");
	}
    }

    // if stdin has a terminal this is the first process in the
    // pipeline so print a heading and initialise message to be send
    // down the pipe
    let predecessor_pos = if unsafe { isatty(STDIN_FILENO) } == 1 {
	eprintln!("Terminal FG process group: {}",
		  unsafe { tcgetpgrp(STDIN_FILENO) });
	eprintln!("Command   PID   PPID   PGRP     SID");
	0
    } else {
	// not first in pipeline so read message from pipe
	let mut buf: [u8; 1] = [0; 1];
	let read = io::stdin().read(&mut buf).expect("Failed to read stdin");
	// let mut line = String::new();
	// io::stdin().read_line(&mut line);
	// let num: u8 = line.parse().expect("Failed to parse position");
	// unsafe { CMD_NUM = num; }
	// num
	buf[0]
    };

    let pos = predecessor_pos + 1;
    unsafe { CMD_NUM = pos; }

    eprintln!("{}    {} {} {} {}",
	      pos,
	      unsafe { getpid() },
	      unsafe { getppid() },
	      unsafe { getpgrp() },
	      unsafe { getsid(0) });

    // if this is not the last process in the pipeline write message
    // to the next process
    if unsafe { isatty(STDOUT_FILENO) } == 0 {
	let written = io::stdout().write(&vec![pos]).expect("Failed to write to stdout");
	io::stdout().flush();
    }

    // wait for signals
    loop {
	unsafe { pause(); }
    }
    
}
