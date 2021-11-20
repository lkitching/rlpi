//listing 12-2
use libc::{exit, EXIT_SUCCESS, uname, utsname};
use std::mem::MaybeUninit;

use rlpi::util::{read_str};
use rlpi::error_functions::{err_exit};

pub fn main() {
    let mut uts = MaybeUninit::<utsname>::uninit();
    if unsafe { uname(uts.as_mut_ptr()) } == -1 {
        err_exit("uname");
    }

    let uts = unsafe { uts.assume_init() };

    println!("Node name: {}", read_str(uts.nodename.as_ptr()));
    println!("System name: {}", read_str(uts.sysname.as_ptr()));
    println!("Release: {}", read_str(uts.release.as_ptr()));
    println!("Version: {}", read_str(uts.version.as_ptr()));
    println!("Machine: {}", read_str(uts.machine.as_ptr()));

    unsafe { exit(EXIT_SUCCESS); }
}
