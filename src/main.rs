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

use crate::dirs_links::{list_files};

fn main() {
    let args: Vec<String> = env::args().collect();
    list_files::main(&args[..]);
}
