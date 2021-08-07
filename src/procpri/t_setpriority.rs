//listing 35-1 (page 736)
use std::env;
use std::collections::{HashMap};

use libc::{__priority_which_t, getpriority, setpriority, exit, EXIT_SUCCESS, PRIO_PROCESS, PRIO_PGRP, PRIO_USER};

extern crate rlpi;
use rlpi::error_functions::{usage_err, err_exit};
use rlpi::libc::{errno, set_errno};

enum Which {
    Process,
    User,
    Group
}

impl Which {
    fn key(&self) -> __priority_which_t {
	match *self {
	    Self::Process => { PRIO_PROCESS },
	    Self::Group => { PRIO_PGRP },
	    Self::User => { PRIO_USER }
	}
    }
    
    fn description(&self) -> &'static str {
	match *self {
	    Self::Process => { "process" },
	    Self::Group => { "process group" },
	    Self::User => { "processes for user" }
	}
    }

    fn opt(&self) -> char {
	match *self {
	    Self::Process => { 'p' },
	    Self::Group => { 'g' },
	    Self::User => { 'u' }
	}
    }
}

fn usage(args: &[String], whiches: &[Which]) -> ! {
    usage_err(&format!("{} {{{}}} who priority\n    set priority of: {}\n",
		       args[0],
		       whiches.iter().map(|w| w.opt().to_string()).collect::<Vec<String>>().join("|"),
		       whiches.iter().map(|w| format!("{}={}", w.opt(), w.description())).collect::<Vec<String>>().join("; ")));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let whiches = [Which::Process, Which::Group, Which::User];
    
    if args.len() != 4 {
	usage(&args, &whiches);
    }

    let which_opt = args[1].chars().next().unwrap();
    
    match whiches.iter().find(|&w| w.opt() == which_opt) {
	None => { usage(&args, &whiches); },
	Some(ref which) => {
	    let who = args[2].parse().expect("Invalid id");
	    let prio = args[3].parse().expect("Invalid priority");

	    if unsafe { setpriority(which.key(), who, prio) } == -1 {
		err_exit("getpriority");
	    }

	    // retrieve nice value to check change
	    // NOTE: successful call to getpriority can return -1
	    // set errno to 0 and check it after call to getpriority
	    // to see if an error occurred
	    set_errno(0);
	    let prio = unsafe { getpriority(which.key(), who) };

	    if prio == -1 && errno() != 0 {
		err_exit("getpriority");
	    }

	    println!("Nice value = {}", prio);

	    unsafe { exit(EXIT_SUCCESS); }
	}
    }
}
