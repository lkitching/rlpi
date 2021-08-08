//listing 36-3 (page 758)
use std::env;
use std::ops::{AddAssign};

use libc::{RLIMIT_NPROC, RLIM_INFINITY, _exit, EXIT_SUCCESS, rlimit, setrlimit, rlim_t};

extern crate rlpi;
use rlpi::util::{fork_or_die, ForkResult};
use rlpi::error_functions::{usage_err, err_exit};
use rlpi::procres::print_rlimit::{print_rlimit};

fn parse_limit(limit_arg: Option<&str>) -> rlim_t {
    match limit_arg {
	None => { 0 },
	Some("i") => { RLIM_INFINITY },
	Some(s) => { s.parse().expect("Expected numeric value") }
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 || args[1] == "--help" {
	usage_err(&format!("{} soft-limit [hard-limit]", args[0]));
    }

    print_rlimit("Initial maximum process limits: ", RLIMIT_NPROC);

    // set new process limits (hard == soft if not specified)
    let rl = rlimit {
	rlim_cur: parse_limit(args.get(1).map(|s| s.as_str())),
	rlim_max: parse_limit(args.get(2).map(|s| s.as_str()))
    };

    if unsafe { setrlimit(RLIMIT_NPROC, &rl) } == -1 {
	err_exit("setrlimit");
    }

    print_rlimit("New maximum process limits: ", RLIMIT_NPROC);

    // create as many children as possible
    let mut j = 1;
    loop {
	match fork_or_die() {
	    ForkResult::Parent(child_pid) => {
		// display message about each new child and let the
		// resulting zombies accumulate
		println!("Child {} (PID={}) started", j, child_pid);
		j += 1;
	    },
	    ForkResult::Child => {
		unsafe { _exit(EXIT_SUCCESS); }
	    }
	}
    }
}
