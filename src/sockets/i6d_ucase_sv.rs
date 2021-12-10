// listing 59-3 (page 1208)
use std::mem::{self, MaybeUninit};
use std::os::raw::{c_char, c_void, c_int};

use libc::{AF_INET6, SOCK_DGRAM, sockaddr_in6, sa_family_t, in6_addr, bind, sockaddr, socklen_t, socket, recvfrom, sendto,
           toupper, size_t};

use rlpi::error_functions::{err_exit, fatal};
use rlpi::libc::inet::{htons, in6addr_any, INET6_ADDRSTRLN, inet_ntop};
use rlpi::sockets::i6d_ucase_common::{PORT_NUM, BUF_SIZE};
use std::ffi::CStr;

pub fn main() {
    let sfd = unsafe { socket(AF_INET6, SOCK_DGRAM, 0) };
    if sfd == -1 {
        err_exit("socket");
    }

    let sv_addr = {
        let mut sv_addr = unsafe { MaybeUninit::<sockaddr_in6>::uninit().assume_init() };
        sv_addr.sin6_family = AF_INET6 as sa_family_t;
        sv_addr.sin6_addr = unsafe { in6addr_any } as in6_addr;
        sv_addr.sin6_port = unsafe { htons(PORT_NUM) };
        sv_addr
    };

    if unsafe { bind(sfd, &sv_addr as *const sockaddr_in6 as *const sockaddr, mem::size_of::<sockaddr_in6>() as socklen_t) } == -1 {
        err_exit("bind");
    }

    // receive messages, convert to uppercase and return to client
    loop {
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

        let (bytes_read, client_addr) = unsafe {
            let mut client_addr = MaybeUninit::<sockaddr_in6>::uninit();
            let mut client_addr_len: socklen_t = mem::size_of::<sockaddr_in6>() as socklen_t;
            let bytes_read = recvfrom(sfd, buf.as_mut_ptr() as *mut c_void, BUF_SIZE, 0, client_addr.as_mut_ptr() as *mut sockaddr, &mut client_addr_len);

            if bytes_read == -1 {
                err_exit("recvfrom");
            }

            (bytes_read as size_t, client_addr.assume_init())
        };

        {
            let mut client_addr_str: [c_char; INET6_ADDRSTRLN] = [0; INET6_ADDRSTRLN];
            if unsafe { inet_ntop(AF_INET6, &client_addr.sin6_addr as *const in6_addr as *const c_void, client_addr_str.as_mut_ptr(), INET6_ADDRSTRLN as socklen_t) }.is_null() {
                println!("Couldn't convert client address to string");
            } else {
                let client_str = unsafe { CStr::from_ptr(client_addr_str.as_ptr()) };
                println!("Server received {} bytes from ({}, {})", bytes_read, client_str.to_str().expect("Invalid UTF8"), client_addr.sin6_port);
            }
        }

        for j in 0 .. bytes_read {
            buf[j] = unsafe { toupper(buf[j] as c_int) as u8 };
        }

        // send response to client
        let bytes_written = unsafe { sendto(sfd, buf.as_ptr() as *const c_void, bytes_read, 0, &client_addr as *const sockaddr_in6 as *const sockaddr, mem::size_of::<sockaddr_in6>() as socklen_t) };
        if bytes_written == -1 || bytes_written as size_t != bytes_read {
            //fatal("sendto");
            err_exit("sendto");
        }
    }
}