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

use crate::sysinfo::{t_uname};

fn main() {
    let args: Vec<String> = env::args().collect();
    t_uname::main(&args[..]);    
}
