use libc::{exit, EXIT_FAILURE, EXIT_SUCCESS, MS_BIND, MS_DIRSYNC, MS_MANDLOCK, MS_MOVE, MS_NOATIME,
           MS_NODEV, MS_NODIRATIME, MS_NOEXEC, MS_NOSUID, MS_RDONLY, MS_REC, MS_REMOUNT, MS_SYNCHRONOUS, mount};
use clap::{Arg, App};
use std::ptr;
use std::ffi::{c_void, CString};

use crate::error_functions::{err_exit};

fn usage_error(prog_name: &str) {
    eprintln!("Usage: {} [options] source target", prog_name);
    eprintln!("\t-t fstype\t[e.g., 'ext2' or 'reiserfs']");
    eprintln!("\t-o data\t[file system-dependent options]");
    eprintln!("\t\t\te.g. 'bsdgroups' for ext2]");
    eprintln!("\t-f mountflags\tcan include any of:");
    eprintln!("\t\tb - MS_BIND\t\tcreate a bind mount");
    eprintln!("\t\td - MS_DIRSYNC\t\tsynchronous directory updates");
    eprintln!("\t\tl - MS_MANDLOCK\t\tpermit mandatory locking");
    eprintln!("\t\tm - MS_MOVE\t\tatomically move subtree");
    eprintln!("\t\tA - MS_NOATIME\t\tdon't update atime (last access time)");
    eprintln!("\t\tV - MS_NODEV\t\tdon't permit device access");
    eprintln!("\t\tD - MS_NODIRATIME\t\tdon't update atime on directories");
    eprintln!("\t\tE - MS_NOEXEC\t\tdon't allow executables");
    eprintln!("\t\tS - MS_NOSUID\t\tdisable set-user/group-ID programs");
    eprintln!("\t\tr - MS_RDONLY\t\tread-only mount");
    eprintln!("\t\tc - MS_REC\t\trecursive mount");
    eprintln!("\t\tR - MS_REMOUNTt\\tremount");
    eprintln!("\t\ts - MS_SYNCHRONOUS\t\tmake writes synchronous");

    unsafe { exit(EXIT_FAILURE); }
}

pub fn main(args: &[String]) -> ! {
    let matches = App::new("t_mount")
	.arg(Arg::with_name("fstype")
	     .short("t")
	     .takes_value(true)
	     .help("[e.g. 'ext2' or 'reiserfs']"))
	.arg(Arg::with_name("options")
	     .short("o")
	     .takes_value(true)
	     .help("[file system-dependent options"))
	.arg(Arg::with_name("mountflags")
	     .short("f")
	     .takes_value(true)
	     .help("mount flags"))
	.arg(Arg::with_name("source")
	     .index(1)
	     .required(true))
	.arg(Arg::with_name("target")
	     .index(2)
	     .required(true))
	.get_matches();

    let fstype = matches.value_of("fstype");
    let options = matches.value_of("options");
    let mut flags = 0;

    if let Some(flags_str) = matches.value_of("mountflags") {
	for c in flags_str.chars() {
	    match c {
		'b' => { flags |= MS_BIND; },
		'd' => { flags |= MS_DIRSYNC; },
		'l' => { flags |= MS_MANDLOCK; },
		'm' => { flags |= MS_MOVE; },
		'A' => { flags |= MS_NOATIME; },
		'V' => { flags |= MS_NODEV; },
		'D' => { flags |= MS_NODIRATIME; },
		'E' => { flags |= MS_NOEXEC; },
		'S' => { flags |= MS_NOSUID; },
		'r' => { flags |= MS_RDONLY; },
		'c' => { flags |= MS_REC; },
		'R' => { flags |= MS_REMOUNT; },
		's' => { flags |= MS_SYNCHRONOUS; },
		_ => { usage_error(&args[0]); }
	    }
	}
    }

    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();

    unsafe {
	let source_s = CString::new(source).expect("Failed to create CString");
	let target_s = CString::new(target).expect("Failed to create CString");
	let fstype_s = fstype.map(|t| CString::new(t).expect("Failed to create CString"));
	let options_s = options.map(|t| CString::new(t).expect("Failed to create CString"));

	let fstype_p = fstype_s.map(|cs| cs.as_ptr()).unwrap_or(ptr::null());
	let options_p = options_s.map(|cs| cs.as_ptr()).unwrap_or(ptr::null());
	let r = mount(source_s.as_ptr(), target_s.as_ptr(), fstype_p, flags, options_p as *const c_void);
	if r == -1 {
	    err_exit("mount");
	} else {
	    exit(EXIT_SUCCESS);
	}
    }
}
