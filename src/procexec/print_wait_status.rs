//listing 26-2 (page 546)
use std::os::raw::{c_int};

use libc::{WIFEXITED, WEXITSTATUS, WIFSIGNALED, WTERMSIG, WIFSTOPPED, WSTOPSIG, WCOREDUMP,
           WIFCONTINUED};

use crate::signals::signal_functions::{str_signal};

enum ChildStatus {
    Exited(c_int),
    KilledBySignal { signal: c_int, core_dumped: bool },
    StoppedBySignal(c_int),
    Continued,
    Unknown
}

fn classify_status(status: c_int) -> ChildStatus {
    unsafe {
	if WIFEXITED(status) {
	    ChildStatus::Exited(WEXITSTATUS(status))
	} else if WIFSIGNALED(status) {
	    ChildStatus::KilledBySignal {
		signal: WTERMSIG(status),
		core_dumped: WCOREDUMP(status)
	    }
	} else if WIFSTOPPED(status) {
	    ChildStatus::StoppedBySignal(WSTOPSIG(status))
	} else if WIFCONTINUED(status) {
	    ChildStatus::Continued
	} else {
	    ChildStatus::Unknown
	}
    }
}

pub fn print_wait_status(msg: Option<&str>, status: c_int) {
    if let Some(msg) = msg {
	print!("{}", msg);
    }

    match classify_status(status) {
	ChildStatus::Exited(exit_code) => {
	    println!("child exited, status = {}", exit_code);
	},
	ChildStatus::KilledBySignal { signal, core_dumped } => {
	    println!("child killed by signal {} ({}){}",
		     signal,
		     str_signal(signal),
		     if core_dumped { " (core dumped)" } else { "" });
	},
	ChildStatus::StoppedBySignal(signal) => {
	    println!("child stopped by signal {} ({})",
		     signal,
		     str_signal(signal));
	},
	ChildStatus::Continued => {
	    println!("child continued");
	},
	ChildStatus::Unknown => {
	    println!("what happened to this child? (status = {})", status);
	}
    }
}
