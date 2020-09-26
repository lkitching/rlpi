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

use crate::filesys::{t_mount};

fn main() {
    let args: Vec<String> = env::args().collect();
    t_mount::main(&args[..]);    
}
