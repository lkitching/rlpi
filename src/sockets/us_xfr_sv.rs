// listing 57-3 (page 1168)
use std::ffi::CString;
use std::mem::{self, MaybeUninit};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use libc::{AF_UNIX, SOCK_STREAM, socket, remove, ENOENT, sockaddr_un, sa_family_t, bind, sockaddr, socklen_t, listen,
           accept, read, write, STDOUT_FILENO, size_t, close};
use rlpi::error_functions::{err_exit, fatal, err_msg};
use rlpi::libc::errno;
use rlpi::sockets::us_xfr_common::{BUF_SIZE, SV_SOCK_PATH};

const BACKLOG: c_int = 5;

pub fn main() {
    let sfd = unsafe {
        socket(AF_UNIX, SOCK_STREAM, 0)
    };

    if sfd == -1 {
        err_exit("socket");
    }

    unsafe {
        let path_s = CString::new(SV_SOCK_PATH).expect("Failed to create CString");
        if remove(path_s.as_ptr()) == -1 {
            if errno() != ENOENT {
                err_exit(&format!("remove-{}", SV_SOCK_PATH));
            }
        }
    }

    unsafe {
        let mut addr = MaybeUninit::<sockaddr_un>::zeroed().assume_init();
        addr.sun_family = AF_UNIX as sa_family_t;
        let mut i = 0;
        for b in SV_SOCK_PATH.bytes() {
            addr.sun_path[i] = b as c_char;
            i += 1;
        }

        // bind socket to address
        if bind(sfd, &addr as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t) == -1 {
            err_exit("bind");
        }

        // listen
        if listen(sfd, BACKLOG) == -1 {
            err_exit("listen");
        }
    }

    loop {
        // accept a connection
        // the connection is returned on a new socket 'cfd'
        // the listening socket remains open and can be used to accept further connections
        let cfd = unsafe { accept(sfd, ptr::null_mut(), ptr::null_mut()) };
        if cfd == -1 {
            err_exit("accept");
        }

        // transfer data from connected socket to sdtout until EOF
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        loop {
            let bytes_read = unsafe { read(cfd, buf.as_mut_ptr() as *mut c_void, BUF_SIZE) };
            if bytes_read > 0 {
                let bytes_written = unsafe { write(STDOUT_FILENO, buf.as_ptr() as *const c_void, bytes_read as size_t) };
                if bytes_read != bytes_written {
                    fatal("partial/failed write")
                }
            } else {
                break;
            }
        }

        if unsafe { close(cfd) } == -1 {
            err_msg("close");
        }
    }
}