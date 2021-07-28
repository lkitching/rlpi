// listing 62-1 (page 1301)
use std::mem::{MaybeUninit};

use libc::{exit, EXIT_SUCCESS, _PC_VDISABLE, fpathconf, tcgetattr, tcsetattr, termios, TCSAFLUSH, cc_t, VINTR};

use crate::error_functions::{usage_err, err_exit};
use crate::libc::unistd::{STDIN_FILENO};

pub fn main(args: &[String]) -> ! {
    if args.len() > 1 && args[1] == "--help" {
	usage_err(&format!("{} [intr-char]", args[0]));
    }

    // determine new INTR setting from the command-line
    let intr_char = if args.len() == 1 {
	let c = unsafe { fpathconf(STDIN_FILENO, _PC_VDISABLE) };
	if c == -1 {
	    err_exit("Couldn't determine VDISABLE");
	}
	c as u8
    } else {
	// try parse arg as decimal, then hex or octal number
	// otherwise arg is expected to be a single character to set
	let s = args[1].as_str();
	s.parse::<cc_t>()
	    .or_else(|_| cc_t::from_str_radix(s, 16))
	    .or_else(|_| cc_t::from_str_radix(s, 8))
	    .unwrap_or(s.as_bytes()[0])	    
    };

    // fetch current terminal settings, modify INTR character and push
    // changes back to the terminal driver
    let mut tp: MaybeUninit<termios> = unsafe { MaybeUninit::uninit() };
    if unsafe { tcgetattr(STDIN_FILENO, tp.as_mut_ptr()) } == -1 {
	err_exit("tcgetattr");
    }
    let mut tp = unsafe { tp.assume_init() };
    tp.c_cc[VINTR] = intr_char;

    if unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &tp) } == -1 {
	err_exit("tcsetattr");
    }

    unsafe { exit(EXIT_SUCCESS); }    
}
