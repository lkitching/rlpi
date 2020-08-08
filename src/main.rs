use std::env;

mod libc;
mod error_functions;
mod ename;
mod fileio;
mod proc;
mod util;
mod memalloc;
mod users_groups;
mod proccred;

use crate::proccred::idshow;

fn main() {
    let args: Vec<String> = env::args().collect();
    idshow::main(&args[..]);
}
