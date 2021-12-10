// listing 59-4 (page 1209)
use std::{env, ptr};
use std::os::raw::{c_void};
use std::mem::{self, MaybeUninit};

use libc::{socket, AF_INET6, SOCK_DGRAM, sendto, recvfrom, sockaddr, sockaddr_in6, socklen_t, size_t, sa_family_t, in6_addr};

use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::sockets::i6d_ucase_common::{BUF_SIZE, PORT_NUM};
use rlpi::libc::inet::{htons, inet_pton};
use std::ffi::CString;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] == "--help" {
        usage_err(&format!("{} host-address msg ...", args[0]));
    }

    // create client socket
    let sfd = unsafe { socket(AF_INET6, SOCK_DGRAM, 0) };
    if sfd == -1 {
        err_exit("socket");
    }

    let server_addr = unsafe {
        let mut addr = MaybeUninit::<sockaddr_in6>::zeroed().assume_init();
        addr.sin6_family = AF_INET6 as sa_family_t;
        addr.sin6_port =  htons(PORT_NUM);

        let addr_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        if inet_pton(AF_INET6, addr_s.as_ptr(), &mut addr.sin6_addr as *mut in6_addr as *mut c_void) <= 0 {
            fatal(&format!("inet_pton failed for address '{}'", args[1]));
        }
        addr
    };

    // send messages to sever and echo responses on stdout
    for msg in args[2..].iter() {
        let bytes = msg.as_bytes();
        let msg_len = bytes.len();

        let bytes_written = unsafe { sendto(sfd, bytes.as_ptr() as *const c_void, msg_len, 0, &server_addr as *const sockaddr_in6 as *const sockaddr, mem::size_of::<sockaddr_in6>() as socklen_t) };
        if bytes_written == -1 || bytes_written as size_t != msg_len {
            fatal("sendto");
        }

        // read response from server
        let mut resp: [u8; BUF_SIZE] = [0; BUF_SIZE];
        let bytes_read = unsafe { recvfrom(sfd, resp.as_mut_ptr() as *mut c_void, BUF_SIZE, 0, ptr::null_mut(), ptr::null_mut()) };
        if bytes_read == -1 {
            err_exit("recvfrom");
        }

        let response_msg = String::from_utf8(resp[0 .. bytes_read as usize].iter().map(|b| *b).collect()).expect("Invalid UTF-8");
        println!("Response: {}: {}", bytes_read, response_msg);
    }
}