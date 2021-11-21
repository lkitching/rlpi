use std::os::raw::{c_int, c_void};
use std::ptr;
use std::mem::{MaybeUninit};

use libc::{pthread_t, pthread_create, pthread_join, pthread_mutex_lock, pthread_mutex_unlock, pthread_mutex_t};

use crate::error_functions::err_exit_en;

pub struct PThreadErr {
    err_no: c_int,
    source: String
}

pub fn or_die<T>(r: Result<T, PThreadErr>) -> T {
    match r {
        Ok(v) => { v },
        Err(e) => {
            err_exit_en(e.err_no, &e.source)
        }
    }
}

pub fn create(thread_func: extern "C" fn(*mut c_void) -> *mut c_void, data: *mut c_void) -> Result<pthread_t, PThreadErr> {
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

pub fn join(t: pthread_t) -> Result<(), PThreadErr> {
    // TODO: get return value
    let s = unsafe { pthread_join(t, ptr::null_mut()) };
    if s != 0 {
        Err(PThreadErr { err_no: s, source: "pthread_join".to_string() })
    } else {
        Ok(())
    }
}

pub fn mutex_lock(mutex: &mut pthread_mutex_t) -> Result<(), PThreadErr> {
    let s = unsafe { pthread_mutex_lock(mutex) };
    if s != 0 {
        Err(PThreadErr { err_no: s, source: "pthread_mutex_lock".to_string()} )
    } else {
        Ok(())
    }
}

pub fn mutex_unlock(mutex: &mut pthread_mutex_t) -> Result<(), PThreadErr> {
    let s = unsafe { pthread_mutex_unlock(mutex) };
    if s != 0 {
        Err(PThreadErr { err_no: s, source: "pthread_mutex_unlock".to_string() })
    } else {
        Ok(())
    }
}
