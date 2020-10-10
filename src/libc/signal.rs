use std::os::raw::{c_int};

// #define __SIGRTMIN 32          (bits/signum-generic.h)
// #define __SIGRTMAX __SIGRTMIN  (bits/signum-generic.h)
// #define _NGIG (__SIGRTMAX + 1) (bits/signum-generic.h)
// #define NSIG _NSIG             (signal.h)
pub const NSIG: c_int = 33;
