// listing 31-2 (page 664)
use std::os::raw::{c_void};
use std::ptr;

use libc::{EPERM, EINVAL};
use rlpi::threads::thread_util::{or_die, create, join};
use rlpi::threads::strerror::{strerror};

extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    println!("Other thread about to call strerror()");
    println!("Other thread: str = {}", strerror(EPERM));
    ptr::null_mut()
}

pub fn main() {
    let s = strerror(EINVAL);
    println!("Main thread has called strerror()");

    let t = or_die(create(thread_func, ptr::null_mut()));
    or_die(join(t));

    println!("Main thread: str = {}", s);
}