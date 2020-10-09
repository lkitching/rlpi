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
mod xattr;
mod dirs_links;
mod inotify;
mod signals;

use crate::signals::{ouch};

fn main() {
    let args: Vec<String> = env::args().collect();
    ouch::main(&args[..]);
}
