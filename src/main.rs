use std::env;

mod libc;
mod error_functions;
mod ename;
mod fileio;
mod proc;
mod util;

// use crate::libc::{gnu_get_libc_version, read_char_ptr};
// use crate::fileio::{copy, seek_io, tee, t_readv, t_append};
use crate::proc::{display_env, modify_env};

fn main() {
    let args: Vec<String> = env::args().collect();
    modify_env::main(&args[..]);
}
