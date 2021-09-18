//listing 44-7 (page 912)
use std::ffi::{CString};
use std::mem::{self, MaybeUninit};
use std::os::raw::{c_void};

use libc::{umask, mkfifo, S_IRUSR, S_IWUSR, S_IWGRP, open, O_RDONLY, O_WRONLY, signal, SIGPIPE, SIG_IGN, SIG_ERR, read, write,
		   close};

extern crate rlpi;
use rlpi::error_functions::{err_exit, err_msg};
use rlpi::pipes::fifo_seqnum::{SERVER_FIFO, Request, Response, get_client_fifo};

pub fn main() {
    // create named pipe and open it for reading
    let server_fifo_s = CString::new(SERVER_FIFO).expect("Failed to create CString");
    unsafe {
	// make sure we get the exact permissions we request
	umask(0);

	
	if mkfifo(server_fifo_s.as_ptr(), S_IRUSR | S_IWUSR | S_IWGRP) == -1 {
	    err_exit(&format!("mkfifo {}", SERVER_FIFO));
	}
    }

    let server_fd = unsafe { open(server_fifo_s.as_ptr(), O_RDONLY) };
    if server_fd == -1 {
	err_exit(&format!("open {}", SERVER_FIFO));
    }

    // open extra write descriptor so we don't receive EOF
    let dummy_fd = unsafe { open(server_fifo_s.as_ptr(), O_WRONLY) };
    if dummy_fd == -1 {
	err_exit(&format!("open {}", SERVER_FIFO));
    }

    // ignore SIGPIPE signal
    // calls to write should return EPIPE instead of raising signal
    if unsafe { signal(SIGPIPE, SIG_IGN) } == SIG_ERR {
	err_exit("signal");
    }

    // main loop
    let mut seq_num = 0;
    loop {
	let req = unsafe {
	    let mut req: MaybeUninit<Request> = MaybeUninit::uninit();
	    let bytes_read = read(server_fd, req.as_mut_ptr() as *mut c_void, mem::size_of::<Request>());
	    if bytes_read != (mem::size_of::<Request>() as isize) {
		eprintln!("Error reading request - discarding");
		continue;
	    }
	    req.assume_init()
	};	

	// open client FIFO
	let client_fifo = get_client_fifo(req.pid);
	let client_fifo_s = CString::new(client_fifo.clone()).expect("Failed to create CString");
	let client_fd = unsafe { open(client_fifo_s.as_ptr(), O_WRONLY) };
	if client_fd == -1 {
	    eprintln!("open failed for client FIFO: {}", client_fifo);
	    continue;
	}

	// send response and close
	let resp = Response { seq_num: seq_num };
	let bytes_written = unsafe { write(client_fd, &resp as *const Response as *const c_void, mem::size_of::<Response>()) };
	if bytes_written != (mem::size_of::<Response>() as isize) {
	    eprintln!("Error writing to client FIFO {}. Wrote {} bytes, expected {}",
		      client_fifo,
		      bytes_written,
		      mem::size_of::<Response>());
	}
	if unsafe { close(client_fd) } == -1 {
	    err_msg("close");
	}

	// update our sequence number
	seq_num += req.seq_len;
    }    
}
