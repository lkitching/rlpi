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

use crate::time::calendar_time;

fn main() {
    let args: Vec<String> = env::args().collect();
    calendar_time::main(&args[..]);    
}
