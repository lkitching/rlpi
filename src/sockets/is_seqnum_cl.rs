// listing 59-7 (page 1224)
use std::{env, ptr};
use std::ffi::{CString};
use std::os::raw::{c_int, c_void};
use std::mem::MaybeUninit;

use libc::{addrinfo, getaddrinfo, AF_UNSPEC, SOCK_STREAM, AI_NUMERICSERV, socket, connect, write, size_t};

use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::sockets::is_seqnum_common::{PORT_NUM, read_line, INT_LEN};
use rlpi::libc::unistd::close;

fn find_address(mut address_p: *mut addrinfo) -> Option<c_int> {
    while !address_p.is_null() {
        let addr = unsafe { *address_p };

        let cfd = unsafe { socket(addr.ai_family, addr.ai_socktype, addr.ai_protocol) };
        if cfd == -1 {
            // try next address
            continue;
        }

        if unsafe { connect(cfd, addr.ai_addr, addr.ai_addrlen) } != -1 {
            // success
            return Some(cfd);
        }

        // connect failed - close this socket and try next address
        unsafe { close(cfd); }

        address_p = addr.ai_next;
    }

    None
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        usage_err(&format!("{} server-host [sequence-len]", args[0]));
    }

    // obtain a list of addresses to try connecting to
    let mut address_p: *mut addrinfo = ptr::null_mut();
    unsafe {
        let mut hints = MaybeUninit::<addrinfo>::zeroed().assume_init();
        hints.ai_canonname = ptr::null_mut();
        hints.ai_addr = ptr::null_mut();
        hints.ai_next = ptr::null_mut();
        hints.ai_family = AF_UNSPEC;  //allow IPv4 or IPv6
        hints.ai_socktype = SOCK_STREAM;
        hints.ai_flags = AI_NUMERICSERV;

        let host_s = CString::new(args[1].as_str()).expect("Failed to create host CString");
        let port_s = CString::new(PORT_NUM).expect("Failed to create port CString");
        if getaddrinfo(host_s.as_ptr(), port_s.as_ptr(), &hints, &mut address_p) != 0 {
            err_exit("getaddrinfo");
        }
    }

    match find_address(address_p) {
        None => {
            fatal("Could not connect socket to any address");
        },
        Some(cfd) => {
            let seq_num_s = if args.len() > 2 { args[2].as_str() } else { "1" };
            let req_buf = format!("{}\n", seq_num_s).into_bytes();
            let bytes_written = unsafe { write(cfd, req_buf.as_ptr() as *const c_void, req_buf.len()) };
            if bytes_written < 0 || bytes_written as size_t != req_buf.len() {
                fatal("Partial/failed write");
            }

            // read and display sequence number returned by server
            match read_line(cfd, INT_LEN) {
                Ok(line) => {
                    println!("Sequence number: {}", line);
                },
                Err(msg) => {
                    fatal(&format!("Failed to read response from server: {}", msg));
                }
            }
        }
    }
}