// listing 57-4 (page 1169)
use std::mem::{self, MaybeUninit};
use std::os::raw::{c_void, c_char};

use libc::{socket, AF_UNIX, SOCK_STREAM, sockaddr_un, sockaddr, sa_family_t, socklen_t, connect, read,
           STDIN_FILENO, write, size_t, close, exit, EXIT_SUCCESS};

use rlpi::sockets::us_xfr_common::{BUF_SIZE, SV_SOCK_PATH};
use rlpi::error_functions::{err_exit, fatal};


pub fn main() {
    // create client socket
    let sfd = unsafe { socket(AF_UNIX, SOCK_STREAM, 0) };
    if sfd == -1 {
        err_exit("socket");
    }

    // construct server address and open connection
    unsafe {
        let mut addr = MaybeUninit::<sockaddr_un>::zeroed().assume_init();
        addr.sun_family = AF_UNIX as sa_family_t;
        let mut i = 0;
        for b in SV_SOCK_PATH.bytes() {
            addr.sun_path[i] = b as c_char;
            i += 1;
        }

        if connect(sfd, &addr as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t) == -1 {
            err_exit("connect");
        }
    }

    // copy stdin to socket
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    loop {
        let bytes_read = unsafe { read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, BUF_SIZE) };
        if bytes_read > 0 {
            let bytes_written = unsafe { write(sfd, buf.as_ptr() as *const c_void, bytes_read as size_t) };
            if bytes_written != bytes_read {
                fatal("partial/failed write");
            }
        } else if bytes_read == -1 {
            err_exit("read");
        } else {
            break;
        }
    }

    // NOTE: closes socket - server receives EOF
    unsafe { exit(EXIT_SUCCESS); }
}