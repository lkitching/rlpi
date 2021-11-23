// listing 32-1
use std::{ptr, thread};
use std::os::raw::c_void;
use std::time::Duration;

use rlpi::pthread::{PTHREAD_CANCELED};
use rlpi::threads::thread_util::{create, join, or_die, cancel};

extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    for j in 0 .. {
        println!("Loop {}", j);
        thread::sleep(Duration::from_secs(1));
    }

    // NOTE: not reached
    ptr::null_mut()
}

pub fn main() {
    let t = or_die(create(thread_func, ptr::null_mut()));

    // allow new thread to run for a while
    thread::sleep(Duration::from_secs(3));

    or_die(cancel(t));
    let result = or_die(join(t));
    if result == unsafe { PTHREAD_CANCELED } {
        println!("Thread was cancelled");
    } else {
        println!("Thread was not cancelled (should not happen!)");
    }
}