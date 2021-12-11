// listing 59-6 (page 1221)
use std::{env, ptr};
use std::mem::{self, MaybeUninit};
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void, c_char};

use libc::{signal, SIGPIPE, SIG_IGN, SIG_ERR, addrinfo, getaddrinfo, setsockopt, socklen_t, SOL_SOCKET, SO_REUSEADDR, close,
           SOCK_STREAM, AF_UNSPEC, AI_PASSIVE, AI_NUMERICSERV, socket, bind, listen, freeaddrinfo, accept, sockaddr_storage,
           NI_MAXHOST, getnameinfo, sockaddr, write, read};

use rlpi::error_functions::{usage_err, err_exit, fatal, err_msg};
use rlpi::sockets::is_seqnum_common::{PORT_NUM, INT_LEN, read_line};
use rlpi::libc::netdb::{NI_MAXSERV};

const BACKLOG: c_int = 50;

fn try_accept<T>(server_socket_fd: c_int) -> Result<(c_int, T), ()> {
    unsafe {
        let mut client_addr = MaybeUninit::<T>::uninit();
        let mut addr_len = mem::size_of::<T>() as socklen_t;
        let client_fd = accept(server_socket_fd, client_addr.as_mut_ptr() as *mut sockaddr, &mut addr_len);

        if client_fd == -1 {
            Err(())
        } else {
            Ok((client_fd, client_addr.assume_init()))
        }
    }
}

fn find_address(mut address_p:  *mut addrinfo) -> Option<c_int> {
    // walk through the returned list to find an address structure that can be used to bind a socket
    while !address_p.is_null() {
        let addr = unsafe { *address_p };

        let lfd = unsafe { socket(addr.ai_family, addr.ai_socktype, addr.ai_protocol) };
        if lfd == -1 {
            continue;
        }

        let opt_val: c_int = 1;
        if unsafe { setsockopt(lfd, SOL_SOCKET, SO_REUSEADDR, &opt_val as *const c_int as *const c_void, mem::size_of::<c_int>() as socklen_t) } == -1 {
            err_exit("setsockopt");
        }

        if unsafe { bind(lfd, addr.ai_addr, addr.ai_addrlen) } == 0 {
            // success
            return Some(lfd);
        }

        // bind() failed - close socket and try next address
        unsafe { close(lfd); }

        address_p = addr.ai_next;
    }

    None
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        usage_err(&format!("{} [init-seq-num]", args[0]));
    }

    let mut seq_num = args.get(1).map_or(0, |s| s.parse().expect("Invalid sequence number"));

    // ignore SIGPIPE signal
    if unsafe { signal(SIGPIPE, SIG_IGN) } == SIG_ERR {
        err_exit("signal");
    }

    // call getaddrinfo to obtain a list of addresses to try binding to
    let hints = {
        let mut hints = unsafe { MaybeUninit::<addrinfo>::zeroed().assume_init() };
        hints.ai_canonname = ptr::null_mut();
        hints.ai_addr = ptr::null_mut();
        hints.ai_next = ptr::null_mut();
        hints.ai_socktype = SOCK_STREAM;
        hints.ai_family = AF_UNSPEC;  //allow IPv4 or IPv6
        hints.ai_flags = AI_PASSIVE | AI_NUMERICSERV;  //wildcard IP address, service name is numeric
        hints
    };

    let mut address_p: *mut addrinfo = ptr::null_mut();

    if unsafe {
        let port_s = CString::new(PORT_NUM).expect("Failed to create CString");
        getaddrinfo(ptr::null(), port_s.as_ptr(), &hints, &mut address_p)
    } != 0 {
        err_exit("getaddrinfo");
    }

    match find_address(address_p) {
        None => {
            fatal("Could not bind socket to any address");
        },
        Some(lfd) => {
            if unsafe { listen(lfd, BACKLOG) } == -1 {
                err_exit("listen");
            }

            unsafe { freeaddrinfo(address_p) }

            // handle clients iteratively
            loop {
                // accept a client connection and obtain client address
                match try_accept::<sockaddr_storage>(lfd) {
                    Ok((cfd, client_addr)) => {
                        {
                            let mut host: [c_char; NI_MAXHOST as usize] = [0; NI_MAXHOST as usize];
                            let mut service: [c_char; NI_MAXSERV as usize] = [0; NI_MAXSERV as usize];

                            if unsafe { getnameinfo(
                                &client_addr as *const sockaddr_storage as *const sockaddr,
                                mem::size_of::<sockaddr_storage>() as socklen_t,
                                host.as_mut_ptr(),
                                NI_MAXHOST,
                                service.as_mut_ptr(),
                                NI_MAXSERV,
                                0) } == 0 {
                                let host_s = unsafe { CStr::from_ptr(host.as_ptr()) };
                                let service_s = unsafe { CStr::from_ptr(service.as_ptr()) };
                                println!("Connection from ({}, {})", host_s.to_str().expect("invalid host utf8"), service_s.to_str().expect("Invalid service utf8"))
                            } else {
                                println!("Connection from (?UNKNOWN?)")
                            }
                        }

                        // read client request and send back sequence number
                        let req: Result<usize, String> = read_line(cfd, INT_LEN).and_then(|rs| rs.parse().map_err(|_| "Invalid increment".to_owned()));
                        match req {
                            Err(_) => {
                                // bad request
                                unsafe { close(cfd); }
                            },
                            Ok(req_len) => {
                                let resp_str = format!("{}\n", seq_num);
                                let resp_bytes = resp_str.as_bytes();
                                let bytes_written = unsafe { write(cfd, resp_bytes.as_ptr() as *const c_void, resp_bytes.len()) };
                                if bytes_written < 0 || bytes_written as usize != resp_bytes.len() {
                                    eprintln!("Error on write");
                                }

                                seq_num += req_len;

                                // close connection
                                if unsafe { close(cfd) } == -1 {
                                    err_msg("close");
                                }
                            }
                        }
                    },
                    Err(_) => {
                        err_msg("accept");
                        continue;
                    }
                }
            }
        }
    }
}