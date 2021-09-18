use std::os::raw::{c_int};
use std::mem::{MaybeUninit};

use libc::{termios, tcgetattr, tcsetattr, ICANON, ECHO, ISIG, ICRNL, TCSAFLUSH, VMIN, VTIME, BRKINT, IGNBRK, IGNCR, INLCR, INPCK, ISTRIP, PARMRK, OPOST,
           IXON, IEXTEN};

fn set_attrs(fd: c_int, t: termios) -> Result<(), String> {
    if unsafe { tcsetattr(fd, TCSAFLUSH, &t) } == -1 {
	Err(String::from("Failed to set terminal attributes"))
    } else {
	Ok(())
    }
}

pub fn get_attrs(fd: c_int) -> Result<termios, String> {
    unsafe {
	let mut t: MaybeUninit<termios> = MaybeUninit::uninit();
	if tcgetattr(fd, t.as_mut_ptr()) == -1 {
	    Err(String::from("Failed to read terminal attributes"))
	} else {
	    Ok(t.assume_init())
	}	
    }
}

/* Place terminal referred to by 'fd' in cbreak mode (non-canonical
 * mode with echoing turned off). This functions assumes the terminal
 * is currently in 'cooked' mode - this shouldn't be called if the
 * terminal is currently in raw mode since it does not undo all of the
 * changes made by the tty_set_raw() function below.  Returns the
 * previous settings on success or an error message on failure.
 */

pub fn tty_set_cbreak(fd: c_int) -> Result<termios, String> {
    let previous = get_attrs(fd)?;

    let mut t = previous;
    // turn off ICANON and ECHO
    // turn on ISIG
    // turn off ICRNL
    t.c_lflag &= !(ICANON | ECHO);
    t.c_lflag |= ISIG;
    t.c_iflag &= !ICRNL;

    // block reading a character at a time
    t.c_cc[VMIN] = 1;
    t.c_cc[VTIME] = 0;

    set_attrs(fd, t).map(|_| previous)
}

/* Place the terminal referred to by fd into raw mode (non-canonical
mode with all input and output processing disabled). Returns the
previous settings or an error message on failure */
pub fn tty_set_raw(fd: c_int) -> Result<termios, String> {
    let previous = get_attrs(fd)?;
    let mut t = previous;

    // disable signals, extended input processing and echo
    t.c_lflag &= !(ICANON | ISIG | IEXTEN | ECHO);

    // disable special handling of CR, NL and BREAK
    // no 8th-bit stripping or parity error handling
    // disable START/STOP output flow control
    t.c_iflag &= !(BRKINT | ICRNL | IGNBRK | IGNCR | INLCR | INPCK | ISTRIP | IXON | PARMRK);

    t.c_oflag &= !OPOST;

    // block reading a character at a time
    t.c_cc[VMIN] = 1;
    t.c_cc[VTIME] = 0;

    set_attrs(fd, t).map(|_| previous)
}
