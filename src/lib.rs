use std::env;

#[macro_use]
extern crate memoffset;

pub mod libc;
pub mod error_functions;
pub mod ename;
pub mod fileio;
pub mod proc;
pub mod util;
pub mod memalloc;
pub mod users_groups;
pub mod proccred;
pub mod time;
pub mod syslim;
pub mod sysinfo;
pub mod filebuff;
pub mod filesys;
pub mod files;
pub mod xattr;
pub mod dirs_links;
pub mod inotify;
pub mod signals;
pub mod timers;
pub mod procexec;
pub mod curr_time;
pub mod tty;
pub mod pty;
pub mod pgsjc;
pub mod procres;
pub mod daemons;
pub mod pipes;
pub mod svmsg;