// listing 55-1 (page 1121)
use std::{env, thread};
use std::ffi::{CString};

use libc::{LOCK_SH, LOCK_EX, LOCK_NB, O_RDONLY, open, EWOULDBLOCK, LOCK_UN, flock, getpid};
use rlpi::libc::{errno};
use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::curr_time::curr_time;
use std::time::Duration;

fn fail_usage(prog_name: &str) -> ! {
    let msg = format!(
        "{} file lock [sleep-time]\n\t'lock' is 's' (shared) or 'x' (exclusive)\n\t\toptionally followed by 'n' (non-blocking)\n\t'secs' specified time to hold lock",
        prog_name);

    usage_err(&msg)
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] == "--help" {
        fail_usage(args[0].as_str());
    }

    let mut lock_chars = args[2].chars();
    let (mut lock_mode, lock_name) = match lock_chars.next() {
        Some('s') => { (LOCK_SH, "LOCK_SH") },
        Some('x') => { (LOCK_EX, "LOCK_EX") },
        _ => { fail_usage(args[0].as_str()) }
    };

    match lock_chars.next() {
        Some('n') => { lock_mode |= LOCK_NB; },
        Some(c) => {
            eprintln!("Invalid lock mode modifier '{}'. Only 'n' is permitted", c);
            fail_usage(args[0].as_str());
        },
        None => { }
    }

    let sleep_seconds = args.get(3).map_or(10, |s| s.parse().expect("Invalid sleep period"));

    let fd = unsafe {
        let file_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        open(file_s.as_ptr(), O_RDONLY)
    };
    if fd == -1 {
        err_exit("open");
    }

    let pid = unsafe { getpid() };
    println!("PID {}: requesting {} at {}", pid, lock_name, curr_time("%T"));

    if unsafe { flock(fd, lock_mode) } == -1 {
        if errno() == EWOULDBLOCK {
            fatal(&format!("PID {}: already locked", pid))
        } else {
            err_exit(&format!("flock (PID={})", pid));
        }
    }

    println!("PID {}: granted {} at {}", pid, lock_name, curr_time("%T"));
    thread::sleep(Duration::from_secs(sleep_seconds));

    println!("PID {}: releasing {} at {}", pid, lock_name, curr_time("%T"));

    if unsafe { flock(fd, LOCK_UN) } == -1 {
        err_exit("flock");
    }
}