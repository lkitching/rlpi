use std::env;

mod libc;
mod error_functions;
mod ename;
mod fileio;
mod proc;
mod util;
mod memalloc;
mod users_groups;

// use crate::libc::{gnu_get_libc_version, read_char_ptr};
// use crate::fileio::{copy, seek_io, tee, t_readv, t_append};
//use crate::proc::{display_env, modify_env};
use crate::users_groups::ugid_functions;

fn main() {
    let args: Vec<String> = env::args().collect();
    ugid_functions::main(&args[..]);
}
