use std::{ptr, time, thread};
use std::mem::{MaybeUninit};

use libc::{exit, EXIT_SUCCESS, getpid, sigfillset, sigprocmask, SIG_SETMASK, sigwaitinfo, SIGINT,
           SIGTERM};

use crate::error_functions::{usage_err, err_exit};
use crate::signals::signal_functions::{str_signal, SI_QUEUE, SI_USER};

pub fn main(args: &[String]) -> ! {
    let prog_name = args[0].as_str();
    if args.len() > 1 {
	usage_err(&format!("{} [delay-secs]\n", prog_name));
    }

    println!("{}: PID is {}", prog_name, unsafe { getpid() });

    //block all signals except SIGKILL and SIGSTOP
    let all_sigs = {
	let mut set = MaybeUninit::uninit();
	unsafe {
	    sigfillset(set.as_mut_ptr());
	    set.assume_init()
	}
    };

    if unsafe { sigprocmask(SIG_SETMASK, &all_sigs, ptr::null_mut()) } == -1 {
	err_exit("sigprocmask");
    }

    println!("{}: signals blocked", prog_name);

    if args.len() > 1 {
	//delay so signals can be sent to us
	let delay_seconds = args[1].parse().expect("Expected delay period");
	let period = time::Duration::from_secs(delay_seconds);
	println!("{}: about to delay {} seconds", prog_name, delay_seconds);
	thread::sleep(period);
    }

    loop {
	let mut si = MaybeUninit::uninit();
	let sig = unsafe { sigwaitinfo(&all_sigs, si.as_mut_ptr()) };
	if sig == -1 {
	    err_exit("sigwaitinfo");
	}

	if sig == SIGINT || sig == SIGTERM {
	    unsafe { exit(EXIT_SUCCESS); }
	}

	let si = unsafe { si.assume_init() };

	let si_value = if si.si_code == SI_USER {
	    "SI_USER"
	} else if si.si_code == SI_QUEUE {
	    "SI_QUEUE"
	} else {
	    "other"
	};
	
	println!("got signal: {} ({})", sig, str_signal(sig));
	println!("    si_signo={}, si_code={} ({})",
		 si.si_signo,
		 si.si_code,
		 si_value);

	// println!("    si_pid={}, si_uid={}",
	// 	 unsafe { si.si_pid() },
	// 	 unsafe { si.si_uid() });
    }
}
