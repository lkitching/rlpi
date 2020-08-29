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

use crate::filebuff::{direct_read};

fn main() {
    let args: Vec<String> = env::args().collect();
    direct_read::main(&args[..]);    
}
