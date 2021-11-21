// listing 30-2 (page 636)
use std::os::raw::{c_void};
use std::{env, ptr};

use libc::{PTHREAD_MUTEX_INITIALIZER, pthread_mutex_t};
use rlpi::threads::thread_util::{create, join, or_die, mutex_lock, mutex_unlock};

static mut GLOB: usize = 0;
static mut MUTEX: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

extern "C" fn thread_func(arg: *mut c_void) -> *mut c_void {
    let loops = unsafe { *(arg as *mut usize) };
    for _ in 0 .. loops {
        or_die(mutex_lock(unsafe { &mut MUTEX }));
        let mut loc = unsafe { GLOB };
        loc += 1;
        unsafe { GLOB = loc; }

        or_die(mutex_unlock(unsafe { &mut MUTEX }))
    }
    ptr::null_mut()
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut loops: usize = args.get(1).map_or(10000000, |s| s.parse().expect("Invalid number of iterations"));

    let t1 = or_die(create(thread_func, &mut loops as *mut usize as *mut c_void));
    let t2 = or_die(create(thread_func, &mut loops as *mut usize as *mut c_void));

    or_die(join(t1));
    or_die(join(t2));

    println!("glob = {}", unsafe { GLOB })
}