//listing 19-1 (page 382)
use std::mem;
use std::ffi::{CString};
use std::os::raw::{c_void};

use libc::{exit, EXIT_SUCCESS, inotify_init, IN_ALL_EVENTS, inotify_add_watch, inotify_event, FILENAME_MAX, read,
           IN_ACCESS, IN_ATTRIB, IN_CLOSE_NOWRITE, IN_CLOSE_WRITE, IN_CREATE, IN_DELETE, IN_DELETE_SELF, IN_IGNORED,
           IN_ISDIR, IN_MODIFY, IN_MOVE_SELF, IN_MOVED_FROM, IN_MOVED_TO, IN_OPEN, IN_Q_OVERFLOW, IN_UNMOUNT};

use crate::error_functions::{usage_err, err_exit, fatal};

fn is_set(flags: u32, flag: u32) -> bool {
    flags & flag == flag
}

fn display_inotify_event(i: &inotify_event) {
    print!("    wd = {}; ", i.wd);
    if i.cookie > 0 {
	print!("cookie ={}; ", i.cookie);
    }
    print!("mask = ");

    if is_set(i.mask, IN_ACCESS) { print!("IN_ACCESS "); }
    if is_set(i.mask, IN_ATTRIB) { print!("IN_ATTRIB "); }
    if is_set(i.mask, IN_CLOSE_NOWRITE) { print!("IN_CLOSE_NOWRITE "); }
    if is_set(i.mask, IN_CLOSE_WRITE) { print!("IN_CLOSE_WRITE "); }
    if is_set(i.mask, IN_CREATE) { print!("IN_CREATE "); }
    if is_set(i.mask, IN_DELETE) { print!("IN_DELETE "); }
    if is_set(i.mask, IN_DELETE_SELF) { print!("IN_DELETE_SELF "); }
    if is_set(i.mask, IN_IGNORED) { print!("IN_IGNORED "); }
    if is_set(i.mask, IN_ISDIR) { print!("IN_ISDIR "); }
    if is_set(i.mask, IN_MODIFY) { print!("IN_MODIFY "); }
    if is_set(i.mask, IN_MOVE_SELF) { print!("IN_MOVE_SELF "); }
    if is_set(i.mask, IN_MOVED_FROM) { print!("IN_MOVED_FROM "); }
    if is_set(i.mask, IN_MOVED_TO) { print!("IN_MOVED_TO "); }
    if is_set(i.mask, IN_OPEN) { print!("IN_OPEN "); }
    if is_set(i.mask, IN_Q_OVERFLOW) { print!("IN_Q_OVERFLOW"); }
    if is_set(i.mask, IN_UNMOUNT) { print!("IN_UNMOUNT "); }
    println!("");

    // if i.len > 0 {
    // 	let name_s = unsafe { CStr::from_ptr(i.name).as_str().expect("Could not read CStr") };
    // 	println!("        name = {}", name_s);
    // }
}

pub fn main(args: &[String]) -> ! {
    if args.len() < 2 {
	usage_err(&format!("{} pathname...\n", args[0]));
    }

    let inotify_fd = unsafe { inotify_init() };
    if inotify_fd == -1 {
	err_exit("inotify_init");
    }

    for path in args.iter().skip(1) {
	let path_s = unsafe { CString::new(path.as_str()).expect("Failed to create CString") };
	let wd = unsafe { inotify_add_watch(inotify_fd, path_s.as_ptr(), IN_ALL_EVENTS) };

	if wd == -1 {
	    err_exit("inotify_add_watch");
	}

	println!("Watching {} using descriptor {}", path, wd);
    }

    const BUF_LEN: usize = 10 * (mem::size_of::<inotify_event>() + (FILENAME_MAX as usize) + 1);
    let mut buf: [u8; BUF_LEN] = [0; BUF_LEN];
    loop {
	let num_read = unsafe { read(inotify_fd, buf.as_mut_ptr() as *mut c_void, BUF_LEN) };
	if num_read == 0 {
	    fatal("read() from inotify fd returned 0");
	}

	if num_read == -1 {
	    err_exit("read");
	}

	println!("Read {} bytes from inotify fd", num_read);

	//process all the events in buf returned by read()
	let mut p = buf.as_mut_ptr();
	for i in 0..num_read {
	    let event_p = p as *const inotify_event;
	    display_inotify_event(& unsafe { *event_p });

	    p = unsafe { p.add(mem::size_of::<inotify_event>() + (*event_p).len as usize) };
	}
	
    }

    unsafe { exit(EXIT_SUCCESS); }
}
