//listing 62-2
use std::io::{self, BufRead, Write};
use std::mem::{MaybeUninit};

use libc::{exit, EXIT_SUCCESS, tcgetattr, tcsetattr, STDIN_FILENO, TCSANOW, TCSAFLUSH, ECHO, termios};

use crate::error_functions::{err_exit};

pub fn main(args: &[String]) -> ! {
    // retrieve current terminal settings and turn off echoing
    let mut tp: MaybeUninit<termios> = MaybeUninit::uninit();
    if unsafe { tcgetattr(STDIN_FILENO, tp.as_mut_ptr()) } == -1 {
	err_exit("tcgetattr");
    }
    let mut tp = unsafe { tp.assume_init() };

    // copy so settings can be restored
    let save = tp;

    // turn off ECHO flag
    tp.c_lflag = tp.c_lflag & !ECHO;

    if unsafe { tcsetattr(STDIN_FILENO, TCSAFLUSH, &tp) } == -1 {
	err_exit("tcsetattr");
    }

    // read some input and display it back to the user
    print!("Enter text: ");
    io::stdout().flush().unwrap();

    // read
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).expect("Failed to read stdin");
    println!("\nRead: {}", line);

    // restore original terminal settings
    if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &save) } == -1 {
	err_exit("tcsetattr");
    }
    
    
    unsafe { exit(EXIT_SUCCESS); }
}
