// listing 52-6 (page 1079)
use std::{env, ptr};
use std::ffi::CString;
use std::os::raw::{c_int};
use std::mem::MaybeUninit;

use libc::{mq_open, O_RDONLY, O_NONBLOCK, mq_attr, mq_getattr, SIGUSR1, sigaddset, sigprocmask, SIG_BLOCK,
           sigaction, sighandler_t, sigevent, sigsuspend, SIGEV_SIGNAL, mq_receive, EAGAIN};

use rlpi::error_functions::{usage_err, err_exit};
use rlpi::signals::signal_functions::sig_empty_set;
use rlpi::libc::{errno};
use rlpi::libc::mqueue::{mq_notify};

const NOTIFY_SIG: c_int = SIGUSR1;

extern "C" fn handler(_sig: c_int) {
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} mq-name", args[0]))
    }

    let mq_desc = unsafe {
        let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        mq_open(name_s.as_ptr(), O_RDONLY | O_NONBLOCK)
    };

    if mq_desc == -1 {
        err_exit("mq_open");
    }

    let attr = unsafe {
        let mut attr: MaybeUninit<mq_attr> = MaybeUninit::uninit();
        if mq_getattr(mq_desc, attr.as_mut_ptr()) == -1 {
            err_exit("mq_getattr");
        }
        attr.assume_init()
    };

    let mut buf = Vec::with_capacity(attr.mq_msgsize as usize);
    let mut block_mask = sig_empty_set();
    unsafe { sigaddset(&mut block_mask, NOTIFY_SIG); }
    if unsafe { sigprocmask(SIG_BLOCK, &block_mask, ptr::null_mut()) } == -1 {
        err_exit("sigprocmask");
    }

    let sa = sigaction {
        sa_mask: sig_empty_set(),
        sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
        sa_flags: 0,
        sa_restorer: None
    };

    if unsafe { sigaction(NOTIFY_SIG, &sa, ptr::null_mut()) } == -1 {
        err_exit("sigaction");
    }

    let sev = unsafe {
        let mut sev: MaybeUninit<sigevent> = MaybeUninit::uninit();
        let p = sev.as_mut_ptr();
        (*p).sigev_notify = SIGEV_SIGNAL;
        (*p).sigev_signo = NOTIFY_SIG;
        if mq_notify(mq_desc, p) == -1 {
            err_exit("mq_notify");
        }
        sev.assume_init()
    };

    let empty_mask = sig_empty_set();
    loop {
        // wait for notification signal
        unsafe { sigsuspend(&empty_mask); }

        if unsafe { mq_notify(mq_desc, &sev) } == -1 {
            err_exit("mq_notify");
        }

        // drain all messages from the queue
        loop {
            let bytes_read = unsafe { mq_receive(mq_desc, buf.as_mut_ptr(), attr.mq_msgsize as usize, ptr::null_mut()) };
            if bytes_read >= 0 {
                println!("Read {} bytes", bytes_read);
            } else {
                break;
            }
        }

        if errno() != EAGAIN {
            // unexpected error
            err_exit("mq_receive");
        }
    }
}