// listing 57-6 (page 1172)
use std::os::raw::{c_void, c_char, c_int};
use std::mem::{self, MaybeUninit};
use std::ffi::{CString, CStr};

use libc::{socket, remove, AF_UNIX, SOCK_DGRAM, ENOENT, sockaddr_un, sa_family_t, sockaddr, recvfrom, sendto, socklen_t, size_t, bind, toupper};

use rlpi::error_functions::{err_exit, fatal};
use rlpi::libc::errno;
use rlpi::sockets::ud_ucase_common::{SV_SOCK_PATH, BUF_SIZE};

pub fn main() {
    let sfd = unsafe { socket(AF_UNIX, SOCK_DGRAM, 0) };
    if sfd == -1 {
        err_exit("socket");
    }

    // construct well-known address and bind server socket to it
    unsafe {
        let path_s = CString::new(SV_SOCK_PATH).expect("Unable to create CString");
        if remove(path_s.as_ptr()) == -1 {
            if errno() != ENOENT {
                err_exit(&format!("remove-{}", SV_SOCK_PATH));
            }
        }

        let mut sv_addr = MaybeUninit::<sockaddr_un>::zeroed().assume_init();
        sv_addr.sun_family = AF_UNIX as sa_family_t;
        for (idx, b) in SV_SOCK_PATH.bytes().enumerate() {
            sv_addr.sun_path[idx] = b as c_char;
        }

        if bind(sfd, &sv_addr as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t) == -1 {
            err_exit("bind");
        }
    }

    // receive messages, convert to uppercase and return to the client
    loop {
        let mut len = mem::size_of::<sockaddr_un>() as socklen_t;
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

        let (num_bytes, cl_addr) = unsafe {
            let mut cl_addr = MaybeUninit::<sockaddr_un>::uninit();
            let num_bytes = recvfrom(sfd, buf.as_mut_ptr() as *mut c_void, BUF_SIZE, 0, cl_addr.as_mut_ptr() as *mut sockaddr, &mut len);

            if num_bytes == -1 {
                err_exit("recvfrom");
            }

            let cl_addr = cl_addr.assume_init();
            let client_path_s = CStr::from_ptr(cl_addr.sun_path.as_ptr());

            println!("Server received {} bytes from {}", num_bytes, client_path_s.to_str().expect("Invalid UTF-8"));

            (num_bytes as size_t, cl_addr)
        };

        for i in 0 .. num_bytes {
            buf[i] = unsafe { toupper(buf[i] as c_int) as u8 }
        }

        let result = unsafe { sendto(sfd, buf.as_ptr() as *const c_void, num_bytes, 0, &cl_addr as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t) };
        if result == -1 || result as size_t != num_bytes {
            fatal("sendto");
        }
    }
}