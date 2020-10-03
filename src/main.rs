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
mod time;
mod syslim;
mod sysinfo;
mod filebuff;
mod filesys;
mod files;

use crate::files::{t_chown};

fn main() {
    let args: Vec<String> = env::args().collect();
    t_chown::main(&args[..]);    
}
