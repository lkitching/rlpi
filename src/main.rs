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

use crate::signals::{sig_sender, sig_receiver};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
	eprintln!("Usage: {} cmd args...", args[0]);
	std::process::exit(-1);
    } else {
	let cmd = args.remove(1);
	match cmd.as_str() {
	    "send" => { sig_sender::main(&args); },
	    "receive" => { sig_receiver::main(&args)  },
	    cmd => {
		eprintln!("Invalid command '{}', expected send or receive", cmd);
		std::process::exit(-1);
	    }
	}
    }    
}
