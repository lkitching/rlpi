use std::env;

mod libc;
mod error_functions;
mod ename;
mod fileio;
mod proc;
mod util;
mod memalloc;

// use crate::libc::{gnu_get_libc_version, read_char_ptr};
// use crate::fileio::{copy, seek_io, tee, t_readv, t_append};
//use crate::proc::{display_env, modify_env};
use crate::memalloc::{free_and_sbrk};

fn main() {
    let args: Vec<String> = env::args().collect();
    free_and_sbrk::main(&args[..]);
}
