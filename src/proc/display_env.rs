//based on listing 6.3 (page 127)
use crate::util::{display_env};
use libc::{exit, EXIT_SUCCESS};

pub fn main(args: &[String]) -> ! {
    display_env();
    unsafe { exit(EXIT_SUCCESS); }
}
