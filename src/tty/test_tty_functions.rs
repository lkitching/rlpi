//listing 62-4 (page 1313)
use std::os::raw::{c_int, c_char, c_uchar, c_void};
use std::mem::{MaybeUninit};
use std::ptr;

use libc::{exit, _exit, EXIT_SUCCESS, termios, STDIN_FILENO, TCSAFLUSH, tcgetattr, tcsetattr, SA_RESTART, sigaction, sighandler_t, SIGQUIT, SIG_IGN,
           setbuf, isalpha, putchar, tolower, iscntrl, read, SIGINT, SIGTERM, signal, raise, sigaddset, sigprocmask, SIGTSTP, SIG_DFL, SIG_ERR,
           SIG_UNBLOCK, SIG_SETMASK};

use crate::libc::{errno, set_errno};
use crate::libc::stdio::{stdout};
use crate::error_functions::{err_exit, err_msg};
use crate::signals::signal_functions::{sig_empty_set};
use super::tty_functions::{tty_set_cbreak, tty_set_raw, get_attrs};

// terminal settings defined by user
static mut USER_TERMIOS: Option<termios> = None;

extern "C" fn handler(sig: c_int) {
    unsafe {
	// TODO: lock?
	if let Some(user_termios) = USER_TERMIOS {
	    if tcsetattr(STDIN_FILENO, TCSAFLUSH, &user_termios) == -1 {
		err_exit("tcsetattr");
	    }
	}
	_exit(EXIT_SUCCESS);
    }
}

// handler for SIGTSTP
extern "C" fn tstp_handler(sig: c_int) {
    let saved_errno = errno();

    // save current terminal settings, restore terminal to state at time of program startup
    let our_termios = unsafe {
	let mut t: MaybeUninit<termios> = MaybeUninit::uninit();
	if tcgetattr(STDIN_FILENO, t.as_mut_ptr()) == -1 {
	    err_exit("tcgetattr");
	}
	t.assume_init()
    };

    if let Some(user_termios) = unsafe { USER_TERMIOS } {
	if unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &user_termios) } == -1 {
	    err_exit("tsetattr");
	}
    }

    // set the disposition of SIGTSTP to the default, raise the signal
    // again and then unblock it so we actually stop
    if unsafe { signal(SIGTSTP, SIG_DFL) } == SIG_ERR {
	err_exit("signal");
    }
    unsafe { raise(SIGTSTP); }

    let prev_mask = unsafe {
	let mut tstp_mask = sig_empty_set();
	sigaddset(&mut tstp_mask, SIGTSTP);

	let mut prev_mask = MaybeUninit::uninit();
	if sigprocmask(SIG_UNBLOCK, &tstp_mask, prev_mask.as_mut_ptr()) == -1 {
	    err_exit("sigprocmask");
	}

	prev_mask.assume_init()
    };

    // execution resumes here after SIGCONT
    // re-block SIGTSTP
    if unsafe { sigprocmask(SIG_SETMASK, &prev_mask, ptr::null_mut()) } == -1 {
	err_exit("sigprocmask");
    }

    // re-establish handler
    let sa = sigaction {
	sa_sigaction: tstp_handler as extern "C" fn(c_int) as sighandler_t,
	sa_mask: sig_empty_set(),
	sa_flags: SA_RESTART,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGTSTP, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    // the user could have changed the terminal settings while we were
    // stopped, save the setings so they can be restored later
    match get_attrs(STDIN_FILENO) {
	Err(msg) => {
	    err_exit(&format!("Cannot save terminal settings: {}", msg));
	},
	Ok(termios) => {
	    unsafe {
		USER_TERMIOS = Some(termios);
	    }
	}
    }

    // restore our terminal settings
    if unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &our_termios) } == -1 {
	err_exit("tcsetattr");
    }

    set_errno(saved_errno);
}

fn set_if_not_ignored(signum: c_int, act: &sigaction) {
    let mut prev: MaybeUninit<sigaction> = MaybeUninit::uninit();
    if unsafe { sigaction(SIGQUIT, ptr::null(), prev.as_mut_ptr()) } == -1 {
	err_exit("sigaction");
    }
    let prev = unsafe { prev.assume_init() };

    if prev.sa_sigaction != SIG_IGN {
	if unsafe { sigaction(signum, &*act, ptr::null_mut()) } == -1 {
	    err_exit("sigaction");
	}
    }
}

pub fn main(args: &[String]) -> ! {

    let sa = sigaction {
	sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
	sa_mask: sig_empty_set(),
	sa_flags: SA_RESTART,
	sa_restorer: None
    };

    if args.len() > 1 {
	// use cbreak mode
	match tty_set_cbreak(STDIN_FILENO) {
	    Err(msg) => {
		err_exit(&format!("Cannot set cbreak mode: {}", msg));
	    },
	    Ok(previous) => {
		unsafe {
		    //TODO: lock?
		    USER_TERMIOS = Some(previous);
		}
	    }
	}

	// terminal special characters can generate signals in cbreak mode. Catch them so we can adjust the terminal mode.
	// only establish handlers if the signals are not being ignored.
	set_if_not_ignored(SIGQUIT, &sa);
	set_if_not_ignored(SIGINT, &sa);

	let tstp_sa = sigaction {
	    sa_sigaction: tstp_handler as extern "C" fn(c_int) as sighandler_t,
	    sa_mask: sig_empty_set(),
	    sa_flags: SA_RESTART,
	    sa_restorer: None
	};
	
	set_if_not_ignored(SIGINT, &tstp_sa);	
    } else {
	// use raw mode
	match tty_set_raw(STDIN_FILENO) {
	    Err(msg) => {
		err_exit(&format!("Cannot set raw mode: {}", msg));
	    },
	    Ok(previous) => {
		unsafe {
		    // TODO: lock
		    USER_TERMIOS = Some(previous);
		}
	    }
	}
    }

    if unsafe { sigaction(SIGTERM, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    // disable stdout buffering
    unsafe { setbuf(stdout, ptr::null_mut()); }

    // read and echo stdin
    loop {
	let mut ch: c_char = 0;
	let n = unsafe { read(STDIN_FILENO, &mut ch as *mut c_char as *mut c_void, 1) };

	if n == -1 {
	    err_msg("read");
	    break;
	}

	// can occur after terminal disconnect
	if n == 0 {
	    break;
	}

	if unsafe { isalpha(ch as c_int) } != 0 {
	    unsafe { putchar(tolower(ch as c_int)); }
	} else if ch as u8 as char == '\n' || ch as u8 as char == '\r' {
	    unsafe { putchar(ch as c_int); }
	} else if unsafe { iscntrl(ch as c_int) != 0 } {
	    unsafe {
		putchar('^' as c_int);
		putchar((ch ^ 64) as c_int);
	    }	    
	} else {
	    unsafe { putchar('*' as c_int); }
	}

	if ch as u8 as char == 'q' {
	    break;
	}
    }

    unsafe {
	// TODO: lock?
	if let Some(user_termios) = USER_TERMIOS {
	    if tcsetattr(STDIN_FILENO, TCSAFLUSH, &user_termios) == -1 {
		err_exit("tcsetattr");
	    }
	}
	exit(EXIT_SUCCESS);
    }
}
