//listing 10-5 (page 209)
use std::env;
use std::os::raw::{c_long};
use libc::{sysconf, _SC_CLK_TCK, exit, EXIT_SUCCESS, getppid, tms, times};
use std::mem::MaybeUninit;

use rlpi::libc::time::{clock, CLOCKS_PER_SEC};
use rlpi::error_functions::{err_exit};

fn display_process_times(msg: &str, clock_ticks: c_long) {
    println!("{}", msg);

    let clock_time = unsafe { clock() };
    if clock_time == -1 {
        err_exit("clock");
    }

    println!("        clock() returns: {} clocks-per-sec ({} secs)",
             clock_time,
             clock_time as f64 / CLOCKS_PER_SEC as f64);

    let mut t = MaybeUninit::<tms>::uninit();
    let r = unsafe { times(t.as_mut_ptr()) };
    if r == -1 {
        err_exit("times");
    }

    let t = unsafe { t.assume_init() };
    println!("        times() yields: user CPU={}, system CPU: {}",
             t.tms_utime as f64 / clock_ticks as f64,
             t.tms_stime as f64 / clock_ticks as f64);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let clock_ticks = unsafe { sysconf(_SC_CLK_TCK) };
    println!("sysconf(_SC_CLK_TCK)={}", clock_ticks);

    display_process_times("At program start:", clock_ticks);

    let num_calls = if args.len() > 1 {
        args[1].parse().expect("Invalid number")
    } else {
        100000000
    };

    for _ in 0..num_calls {
        unsafe { getppid() };
    }

    display_process_times("After getppid() loop:", clock_ticks);

    unsafe { exit(EXIT_SUCCESS); }
}
