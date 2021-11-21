//listing 29-1 (page 626)
use std::os::raw::{c_void, c_char};
use std::ffi::{CString, CStr};
use std::mem::{MaybeUninit};
use std::ptr;

use libc::{pthread_create, pthread_join, pthread_t};
use rlpi::error_functions::{err_exit_en};

extern "C" fn thread_func(arg: *mut c_void) -> *mut c_void {
    let msg_s = unsafe { CStr::from_ptr(arg as *mut c_char) };
    let msg = msg_s.to_str().expect("Invalid utf8");
    println!("{}", msg);
    msg.chars().count() as *mut c_void
}

pub fn main() {
    let msg = "Hello world";
    let msg_s = CString::new(msg).expect("Failed to create CString");

    let t = unsafe {
        let mut t: MaybeUninit<pthread_t> = MaybeUninit::uninit();
        let s = pthread_create(t.as_mut_ptr(), ptr::null(), thread_func, msg_s.as_ptr() as *mut c_void);

        if s != 0 {
            err_exit_en(s, "pthread_create")
        }
        t.assume_init()
    };

    println!("Message from main()");
    let len_p = unsafe {
        let mut res: MaybeUninit<*mut c_void> = MaybeUninit::uninit();
        let s = pthread_join(t, res.as_mut_ptr());

        if s != 0 {
            err_exit_en(s, "pthread_join");
        }

        res.assume_init()
    };

    println!("Thread returned {}", len_p as usize);
}