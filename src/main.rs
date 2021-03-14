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
mod timers;
mod procexec;
mod curr_time;

use crate::procexec::{multi_sigchld};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    multi_sigchld::main(&args);
}
