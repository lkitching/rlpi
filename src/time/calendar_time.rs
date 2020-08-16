//based on listing 10-2

use libc::{time, exit, EXIT_SUCCESS, timeval, gettimeofday, gmtime, localtime, mktime, tm};
use std::ptr;
use std::ffi::CStr;
use std::mem::{MaybeUninit};

use crate::error_functions::{err_exit};
use crate::libc::time::{asctime, ctime};

pub fn main(args: &[String]) -> ! {
    let mut t = unsafe { time(ptr::null_mut()) };
    println!("Seconds since the Epoch (1 Jan 1970): {}", t);

    let mut tv = MaybeUninit::<timeval>::uninit();
    if unsafe { gettimeofday(tv.as_mut_ptr(), ptr::null_mut()) } == -1 {
	err_exit("gettimeofday");
    }
    let tv = unsafe { tv.assume_init() };
    println!("  gettimeofday() returned {} secs, {} microsecs", tv.tv_sec, tv.tv_usec);

    let gmp = unsafe { gmtime(&t) };
    if gmp.is_null() {
	err_exit("gmtime");
    }    

    //save local copy since *gmp may be modified by asctime() or gmtime()
    let mut gm = unsafe { *gmp };
    
    println!("Broken down by gmtime():");
    println!("  year={} mon={} mday={} hour={} min={} sec={} wday={} yday={} isdst={}",
	     gm.tm_year,
	     gm.tm_mon,
	     gm.tm_mday,
	     gm.tm_hour,
	     gm.tm_min,
	     gm.tm_sec,
	     gm.tm_wday,
	     gm.tm_yday,
	     gm.tm_isdst);

    let locp = unsafe { localtime(&t) };
    if locp.is_null() {
	err_exit("localtime");
    }

    //save local copy
    let mut loc = unsafe { *locp };

    println!("Broken down by localtime():");
    println!("  year={} mon={} mday={} hour={} min={} sec={} wday={} yday={} isdst={}",
	     loc.tm_year,
	     loc.tm_mon,
	     loc.tm_mday,
	     loc.tm_hour,
	     loc.tm_min,
	     loc.tm_sec,
	     loc.tm_wday,
	     loc.tm_yday,
	     loc.tm_isdst);    

    let asctime_p = unsafe { asctime(&gm) };
    let asctime_str = unsafe { CStr::from_ptr(asctime_p) };
    println!("asctime() formats the gmtime() value as: {}", asctime_str.to_str().expect("Failed to convert CStr"));

    let ctime_p = unsafe { ctime(&t) };
    let ctime_str = unsafe { CStr::from_ptr(ctime_p) };
    println!("ctime() formats the time() value as: {}", ctime_str.to_str().expect("Failed to convert CStr"));
    
    println!("mktime() of gmtime() value: {} secs", unsafe { mktime(&mut gm as *mut tm) });
    println!("mktime() of localtime() value: {} secs", unsafe { mktime(&mut loc as *mut tm) });
	

    unsafe { exit(EXIT_SUCCESS); }
}
