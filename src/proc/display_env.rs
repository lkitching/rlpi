//based on listing 6.3 (page 127)
use rlpi::util::{display_env};
use libc::{exit, EXIT_SUCCESS};

pub fn main() {
    display_env();
    unsafe { exit(EXIT_SUCCESS); }
}
