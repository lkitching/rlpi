use std::ffi::CString;
use libc::{sem_t, sem_open, SEM_FAILED};

pub fn open_existing(name: &str) -> Result<*mut sem_t, String> {
    let name_s = CString::new(name).map_err(|_| "Failed to create CString".to_string())?;
    let sem = unsafe { sem_open(name_s.as_ptr(), 0) };
    if sem == SEM_FAILED {
        Err("Failed to open semaphore".to_string())
    } else {
        Ok(sem)
    }
}