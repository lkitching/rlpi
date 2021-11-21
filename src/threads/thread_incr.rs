// listing 30-1 (page 632)
use std::{env, ptr};
use std::os::raw::{c_int, c_void};
use std::mem::MaybeUninit;

use libc::{pthread_t, pthread_create, pthread_join};
use rlpi::error_functions::{err_exit_en};

static mut GLOB: usize = 0;

struct PThreadErr {
    err_no: c_int,
    source: String
}

fn or_die<T>(r: Result<T, PThreadErr>) -> T {
    match r {
        Ok(v) => { v },
        Err(e) => {
            err_exit_en(e.err_no, &e.source)
        }
    }
}

fn create(thread_func: extern "C" fn(*mut c_void) -> *mut c_void, data: *mut c_void) -> Result<pthread_t, PThreadErr> {
    unsafe {
        let mut t: MaybeUninit<pthread_t> = MaybeUninit::uninit();
        let s = pthread_create(t.as_mut_ptr(), ptr::null(), thread_func, data);
        if s != 0 {
            Err(PThreadErr { err_no: s, source: "pthread_create".to_string() })
        } else {
            Ok(t.assume_init())
        }
    }
}

fn join(t: pthread_t) -> Result<(), PThreadErr> {
    // TODO: get return value
    let s = unsafe { pthread_join(t, ptr::null_mut()) };
    if s != 0 {
        Err(PThreadErr { err_no: s, source: "pthread_join".to_string() })
    } else {
        Ok(())
    }
}

extern "C" fn thread_func(arg: *mut c_void) -> *mut c_void {
    let loops = unsafe { *(arg as *mut usize) };

    for _ in 0 .. loops {
        let mut loc = unsafe { GLOB };
        loc += 1;
        unsafe { GLOB = loc }
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