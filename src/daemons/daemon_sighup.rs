//listing 37-3 (page 774)
use std::env;
use std::ptr;
use std::os::raw::{c_int, c_uint};
use std::ops::{AddAssign};
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, BufWriter, Result};

use libc::{sigaction, sighandler_t, SA_RESTART, SIGHUP, sleep};
use chrono::prelude::{Local};

extern crate rlpi;
use rlpi::signals::signal_functions::{sig_empty_set};
use rlpi::daemons::become_daemon::{become_daemon};
use rlpi::error_functions::{err_exit};

extern "C" fn sighup_handler(sig: c_int) {
    unsafe { HUP_RECEIVED = true; }
}

static mut HUP_RECEIVED: bool = false;

pub fn main() {
    let args: Vec<String> = env::args().collect();    
    let log_path = "/tmp/ds.log";    
    let config_file = "/tmp/ds.conf";

    // time to sleep between messages
    const SLEEP_TIME: c_uint = 15;

    let sa = sigaction {
	sa_mask: sig_empty_set(),
	sa_flags: SA_RESTART,
	sa_sigaction: sighup_handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    // install handler for SIGHUP
    if unsafe { sigaction(SIGHUP, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    if let Err(msg) = become_daemon(0) {
	err_exit("become_daemon");
    }

    let mut unslept = SLEEP_TIME;
    let mut count = 1;
    let mut log_writer = log_open(&log_path);
    
    loop {
	// NOTE: returns 0 if interrupted
	unslept = unsafe { sleep(unslept) };

	if unsafe { HUP_RECEIVED } {
	    log_writer = log_open(&log_path);
	    read_config_file(&config_file, &mut log_writer);

	    // wait for next SIGHUP
	    unsafe { HUP_RECEIVED = false; }
	}

	// on completed interval
	if unslept == 0 {
	    count += 1;
	    log_message(&mut log_writer, &format!("Main: {}", count));

	    // reset interval
	    unslept = SLEEP_TIME;
	}
    }
}

fn log_open(log_file: &str) -> BufWriter<File> {
    let f = OpenOptions::new().write(true).create(true).open(log_file).expect("Failed to create log file");
    let mut writer = BufWriter::new(f);
    log_message(&mut writer, "Opened log file");
    writer
}

fn read_config_file(config_file: &str, log_writer: &mut BufWriter<File>) {
    let config = match File::open(config_file) {
	Ok(mut f) => {
	    let mut s = String::new();
	    f.read_to_string(&mut s).expect("Failed to read");
	    s
	},
	Err(_) => { String::new() }
    };
    log_message(log_writer, &format!("Read config file: {}", config));
}

fn log_message(writer: &mut BufWriter<File>, message: &str) {
    let time = Local::now();
    let m = format!("{}: {}\n",
		    time.format("%Y-%m-%d %H:%M:%S"),
		    message);    
    writer.write(m.as_bytes()).expect("Failed to write message");
    writer.flush().expect("Failed to flush");
}
