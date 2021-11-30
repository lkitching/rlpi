// listing 57-7 (page 1173)
use std::{env, mem, ptr};
use std::mem::MaybeUninit;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

use libc::{AF_UNIX, SOCK_DGRAM, sockaddr_un, bind, sa_family_t, getpid, sockaddr, socklen_t, sendto, recvfrom, size_t, socket, remove};

use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::sockets::ud_ucase_common::{SV_SOCK_PATH, BUF_SIZE};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        usage_err(&format!("{} msg ...", args[0]));
    }

    // create client socket and bind to unique pathname based on PID
    let sfd = unsafe { socket(AF_UNIX, SOCK_DGRAM, 0) };
    if sfd == -1 {
        err_exit("socket");
    }

    let socket_path = format!("/tmp/ud_ucase_cl.{}", unsafe { getpid() });

    unsafe {
        let mut cl_addr = MaybeUninit::<sockaddr_un>::zeroed().assume_init();
        cl_addr.sun_family = AF_UNIX as sa_family_t;

        for (idx, b) in socket_path.bytes().enumerate() {
            cl_addr.sun_path[idx] = b as c_char;
        }

        if bind(sfd, &cl_addr as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t) == -1 {
            err_exit("bind");
        }
    }

    // construct address of server
    let sv_addr = {
        let mut sv_addr = unsafe { MaybeUninit::<sockaddr_un>::zeroed().assume_init() };
        sv_addr.sun_family = AF_UNIX as sa_family_t;
        for (idx, b) in SV_SOCK_PATH.bytes().enumerate() {
            sv_addr.sun_path[idx] = b as c_char;
        }
        sv_addr
    };

    // send messages to server and echo responses on stdout
    for msg in args[1 .. ].iter() {
        // NOTE: maybe be longer than BUF_SIZE
        let msg_bytes = msg.as_bytes();
        let msg_len = msg_bytes.len();

        let bytes_sent = unsafe { sendto(sfd, msg_bytes.as_ptr() as *const c_void, msg_len, 0, &sv_addr as *const sockaddr_un as *const sockaddr, mem::size_of::<sockaddr_un>() as socklen_t) };
        if bytes_sent == -1 || bytes_sent as size_t != msg_len {
            fatal("send_to");
        }

        // get response from server
        let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
        let bytes_received = unsafe { recvfrom(sfd, buf.as_mut_ptr() as *mut c_void, BUF_SIZE, 0, ptr::null_mut(), ptr::null_mut()) };
        if bytes_received == -1 {
            err_exit("recvfrom");
        }

        let response_msg = String::from_utf8(buf[0 .. bytes_received as usize].iter().map(|b| *b).collect()).expect("Invalid UTF-8");
        println!("Response {}: {}", bytes_received, response_msg);
    }

    // remove client socket file
    unsafe {
        let path_s = CString::new(socket_path).expect("Failed to create CString");
        remove(path_s.as_ptr());
    }
}