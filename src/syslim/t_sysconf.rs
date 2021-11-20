//listing 11-1 (page 216)
use std::os::raw::{c_int};
use libc::{exit, EXIT_SUCCESS, sysconf, _SC_ARG_MAX, _SC_LOGIN_NAME_MAX, _SC_OPEN_MAX, _SC_NGROUPS_MAX,
           _SC_PAGESIZE, _SC_RTSIG_MAX};

use rlpi::libc::{errno, set_errno};
use rlpi::error_functions::{err_exit};

fn sysconf_print(msg: &str, name: c_int) {
    // sysconf returns -1 if either an error occurred or the limit
    // cannot be determined.  Set errno to 0 before the call so these
    // cases can be distinguished (errno will be set if an error
    // occurred)
    set_errno(0);
    let lim = unsafe { sysconf(name) };
    if lim != -1 {
        //call succeeded, limit determined
        println!("{} {}", msg, lim);
    } else {
        let en = errno();
        if en == 0 {
            //call succeeded, limit indeterminate
            println!("{} (indeterminate)", msg);
        } else {
            err_exit(&format!("sysconf {}", msg));
        }
    }
}

pub fn main() {
    sysconf_print("_SC_ARG_MAX:        ", _SC_ARG_MAX);
    sysconf_print("_SC_LOGIN_NAME_MAX: ", _SC_LOGIN_NAME_MAX);
    sysconf_print("_SC_OPEN_MAX:       ", _SC_OPEN_MAX);
    sysconf_print("_SC_NGROUPS_MAX:    ", _SC_NGROUPS_MAX);
    sysconf_print("_SC_PAGESIZE:       ", _SC_PAGESIZE);
    sysconf_print("_SC_RTSIG_MAX:      ", _SC_RTSIG_MAX);

    unsafe { exit(EXIT_SUCCESS); }
}
