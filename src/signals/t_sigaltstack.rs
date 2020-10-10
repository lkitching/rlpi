//listing 21-3 (page 436)
use std::ptr;
use std::os::raw::{c_int, c_void};

use libc::{malloc, SIGSTKSZ, sigaltstack, stack_t, sigaction, sighandler_t, SA_ONSTACK, SIGSEGV,
           _exit, EXIT_FAILURE};

use crate::error_functions::{err_exit};
use super::signal_functions::{sig_empty_set, str_signal};

extern "C" fn handler(sig: c_int) {
    let x: u8 = 0;

    // UNSAFE: This handler uses non-async-safe functions
    println!("Caught signal {} ({})", sig, str_signal(sig));
    println!("Top of handler stack near {:#x}", &x as *const u8 as usize);

    // can't return after SIGSEGV
    unsafe { _exit(EXIT_FAILURE); }
}

fn overflow_stack(call_num: u32) {
    let a: [u8; 100000000] = [0; 100000000];
    println!("Call {} - top of stack near {:#x}", call_num, &a as *const [u8; 100000000] as usize);
    overflow_stack(call_num + 1);
}

pub fn main(args: &[String]) {
    let j: i32 = 0;
    println!("Top of standard stack is near {:#x}", &j as *const i32 as usize);

    // allocate alternate stack and inform kernel
    let alt_stack = unsafe { malloc(SIGSTKSZ) };
    if alt_stack.is_null() {
	err_exit("malloc");
    }

    let mut sigstack = stack_t {
	ss_sp: alt_stack,
	ss_size: SIGSTKSZ,
	ss_flags: 0
    };

    if unsafe { sigaltstack(&sigstack, ptr::null_mut()) } == -1 {
	err_exit("sigaltstack");
    }

    println!("Alternate stack is at {:#x}", alt_stack as usize);

    //establish handler for SIGSEGV
    let sa = sigaction {
	sa_sigaction: handler as extern fn(c_int) as *const c_void as sighandler_t,
	sa_mask: sig_empty_set(),
	sa_flags: SA_ONSTACK,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGSEGV, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    overflow_stack(1);
}
