use std::os::raw::{c_int, c_void};
use std::ptr;
use std::mem::{MaybeUninit};

use libc::{pthread_t, pthread_create, pthread_join, pthread_mutex_lock, pthread_mutex_unlock, pthread_mutex_t,
           pthread_cond_signal, pthread_cond_t, pthread_cond_wait};

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

fn to_result<T, F: FnOnce() -> T>(result: c_int, on_success: F, source_name: &str) -> Result<T, PThreadErr> {
    if result != 0 {
        Err(PThreadErr { err_no: result, source: source_name.to_string() })
    } else {
        Ok(on_success())
    }
}

pub fn cond_signal(signal: &mut pthread_cond_t) -> Result<(), PThreadErr> {
    let s = unsafe { pthread_cond_signal(signal) };
    to_result(s, || (), "pthread_cond_signal")
}

pub fn cond_wait(cond: &mut pthread_cond_t, lock: &mut pthread_mutex_t) -> Result<(), PThreadErr> {
    let s = unsafe { pthread_cond_wait(cond, lock) };
    to_result(s, || (), "pthread_cond_wait")
}