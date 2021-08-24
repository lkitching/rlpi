//listing 44-7 (page 912)
use std::env;
use std::mem::{self, MaybeUninit};
use std::ffi::{CString};
use std::os::raw::{c_void};

use libc::{umask, mkfifo, getpid, S_IRUSR, S_IWUSR, S_IWGRP, EEXIST, open, atexit, unlink, exit, EXIT_SUCCESS, read, write, O_RDONLY, O_WRONLY};

extern crate rlpi;
use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::libc::{errno};
use rlpi::pipes::fifo_seqnum::{SERVER_FIFO, Request, Response, get_client_fifo};

extern "C" fn remove_fifo() {
    let client_fifo = get_client_fifo(unsafe { getpid() });
    let client_fifo_s = CString::new(client_fifo).expect("Failed to create CString");
    unsafe { unlink(client_fifo_s.as_ptr()); }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
	usage_err(&format!("{} [seq-len...]", args[0]));
    }

    let seq_len: usize = if args.len() > 1 { args[1].parse().expect("Invalid sequence length") } else { 1 };
    let client_fifo = get_client_fifo(unsafe { getpid() });

    // create client pipe before sending request
    unsafe {
	// clear so we get only the requested permissions
	umask(0);
	
	let client_fifo_s = CString::new(client_fifo.clone()).expect("Failed to create CString");
	let ret = mkfifo(client_fifo_s.as_ptr(), S_IRUSR | S_IWUSR | S_IWGRP);
	if ret == -1 && errno() != EEXIST {
	    err_exit(&format!("mkfifo {}", client_fifo));
	}	
    }

    // cleanup client pipe on exit
    if unsafe { atexit(remove_fifo) } != 0 {
	remove_fifo();
	err_exit("atexit");
    }
    
    let req = Request {
	pid: unsafe { getpid() },
	seq_len: seq_len
    };

    let server_fd = unsafe {
	let server_fifo_s = CString::new(SERVER_FIFO).expect("Failed to create CString");
	open(server_fifo_s.as_ptr(), O_WRONLY)
    };
    
    if server_fd == -1 {
	err_exit(&format!("open {}", SERVER_FIFO));
    }

    let bytes_written = unsafe { write(server_fd, &req as *const Request as *const c_void, mem::size_of::<Request>()) };
    if bytes_written != (mem::size_of::<Request>() as isize) {
	fatal("Can't write to server");
    }

    // open client pipe and display response
    let client_fd = {
	let client_fifo_s = CString::new(client_fifo.clone()).expect("Failed to create CString");
	unsafe { open(client_fifo_s.as_ptr(), O_RDONLY) }
    };
    if client_fd == -1 {
	err_exit(&format!("open {}", client_fifo));
    }

    let resp = unsafe {
	let mut resp: MaybeUninit::<Response> = MaybeUninit::uninit();
	let bytes_read = read(client_fd, resp.as_mut_ptr() as *mut c_void, mem::size_of::<Response>());
	if bytes_read != (mem::size_of::<Response>() as isize) {
	    fatal("Failed to read response from the server");
	}
	resp.assume_init()
    };

    println!("Client assigned: {}", resp.seq_num);
    unsafe { exit(EXIT_SUCCESS); }
}
