//listing 22-5 (page 466)
use std::os::raw::{c_int, c_void};
use std::mem::{MaybeUninit};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{ptr, thread, time};

use libc::{SIGQUIT, exit, EXIT_SUCCESS, SIGINT, sigaddset, sigprocmask, SIG_BLOCK, sighandler_t, sigaction, EINTR,
           sigsuspend, SIG_SETMASK};

use rlpi::signals::signal_functions::{str_signal, print_sig_mask, sig_empty_set, print_pending_sigs};
use rlpi::libc::{errno};
use rlpi::libc::stdio::{stdout};
use rlpi::error_functions::{err_exit};

static GOT_SIGQUIT: AtomicBool = AtomicBool::new(false);

extern "C" fn handler(sig: c_int) {
    println!("Caught signal {} ({})", sig, str_signal(sig));

    if sig == SIGQUIT {
        GOT_SIGQUIT.store(true, Ordering::SeqCst);
    }
}

pub fn main() {
    print_sig_mask(unsafe { stdout }, "Initial signal mask is:").expect("Failed to print current sigmask");

    let mut block_mask = sig_empty_set();
    unsafe {
        sigaddset(&mut block_mask, SIGINT);
        sigaddset(&mut block_mask, SIGQUIT);
    }

    let mut orig_mask = MaybeUninit::uninit();
    if unsafe { sigprocmask(SIG_BLOCK, &block_mask, orig_mask.as_mut_ptr()) } == -1 {
        err_exit("sigprocmask - SIG_BLOCK");
    }
    let orig_mask = unsafe { orig_mask.assume_init() };

    let sa = sigaction {
        sa_mask: sig_empty_set(),
        sa_flags: 0,
        sa_sigaction: handler as extern fn(c_int) as *const c_void as sighandler_t,
        sa_restorer: None,
    };

    if unsafe { sigaction(SIGINT, &sa, ptr::null_mut()) } == -1 {
        err_exit("sigaction");
    }
    if unsafe { sigaction(SIGQUIT, &sa, ptr::null_mut()) } == -1 {
        err_exit("sigaction");
    }

    for i in 1.. {
        if GOT_SIGQUIT.load(Ordering::SeqCst) {
            break;
        }

        println!("=== LOOP {}", i);

        //simulate critical section by delaying a few seconds
        print_sig_mask(unsafe { stdout }, "starting critical section, signal mask is:").expect("Error printing signal mask");

        let period = time::Duration::from_secs(4);
        thread::sleep(period);

        print_pending_sigs(unsafe { stdout }, "Before sigsuspend() - pending signals:").expect("Error printing pending signals");

        let ret = unsafe { sigsuspend(&orig_mask) };
        if ret == -1 && errno() != EINTR {
            err_exit("sigsuspend");
        }
    }

    if unsafe { sigprocmask(SIG_SETMASK, &orig_mask, ptr::null_mut()) } == -1 {
        err_exit("sigprocmask - SIG_SETMASK");
    }

    print_sig_mask(unsafe { stdout }, "=== Exited loop\nRestored signal mask to:").expect("Error printing signal mask");

    unsafe { exit(EXIT_SUCCESS); }
}
