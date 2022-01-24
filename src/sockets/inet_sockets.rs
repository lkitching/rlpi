// listing 59-9 (page 1228)
use std::os::raw::{c_int, c_void, c_char};
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::ffi::{CString, CStr};

use libc::{addrinfo, AF_UNSPEC, AI_PASSIVE, getaddrinfo, freeaddrinfo, connect, close, socket, setsockopt, socklen_t, SOL_SOCKET,
           SO_REUSEADDR, bind, SOCK_STREAM, listen, NI_MAXHOST, getnameinfo, sockaddr, NI_NUMERICSERV};

use crate::libc::netdb::NI_MAXSERV;

struct AddrInfoList {
    address_p: *mut addrinfo
}

struct AddrInfoListIter {
    address_p: *mut addrinfo
}

impl Iterator for AddrInfoListIter {
    type Item = addrinfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.address_p.is_null() {
            None
        } else {
            let addr = unsafe { *self.address_p };
            self.address_p = addr.ai_next;
            Some(addr)
        }
    }
}

impl IntoIterator for AddrInfoList {
    type Item = addrinfo;
    type IntoIter = AddrInfoListIter;

    fn into_iter(self) -> Self::IntoIter {
        AddrInfoListIter { address_p: self.address_p }
    }
}

impl Drop for AddrInfoList {
    fn drop(&mut self) {
        unsafe { freeaddrinfo(self.address_p) }
    }
}

pub fn inet_connect(host: &str, service: &str, socket_type: c_int) -> Result<c_int, String> {
    let addresses = unsafe {
        let mut hints = MaybeUninit::<addrinfo>::zeroed().assume_init();
        hints.ai_canonname = ptr::null_mut();
        hints.ai_addr = ptr::null_mut();
        hints.ai_next = ptr::null_mut();
        hints.ai_family = AF_UNSPEC;
        hints.ai_socktype = socket_type;

        let mut result: MaybeUninit<*mut addrinfo> = MaybeUninit::uninit();
        let host_s = CString::new(host).expect("Failed to create host CString");
        let service_s = CString::new(service).expect("Failed to create service CString");

        let s = getaddrinfo(host_s.as_ptr(), service_s.as_ptr(), &hints, result.as_mut_ptr());

        if s != 0 {
            return Err("Failed to resolve address".to_owned());
        }

        AddrInfoList { address_p: result.assume_init() }
    };

    // walk through list until we find an address that connects successfully
    for address in addresses {
        let sfd = unsafe { socket(address.ai_family, address.ai_socktype, address.ai_protocol) };
        if sfd == -1 {
            // try next address on error
            continue;
        }

        if unsafe { connect(sfd, address.ai_addr, address.ai_addrlen) } != -1 {
            return Ok(sfd);
        }

        // connect failed - close this socket and try next address
        unsafe { close(sfd); }
    }

    Err("Failed to connect to candidate address".to_owned())
}

pub struct PassiveSocket {
    socket_desc: c_int,
    address: addrinfo
}

fn bind_address(addresses: AddrInfoList, do_listen: bool) -> Result<PassiveSocket, String> {
    for address in addresses {
        let sfd = unsafe { socket(address.ai_family, address.ai_socktype, address.ai_protocol) };
        if sfd == -1 {
            continue;
        }

        if do_listen {
            let optval: c_int = 1;
            if unsafe { setsockopt(sfd, SOL_SOCKET, SO_REUSEADDR, &optval as *const c_int as *const c_void, mem::size_of::<c_int>() as socklen_t) } == -1 {
                unsafe { close(sfd); }
                return Err("Failed to set socket option".to_owned());
            }
        }

        if unsafe { bind(sfd, address.ai_addr, address.ai_addrlen) } == 0 {
            // bind succeeded
            return Ok(PassiveSocket { socket_desc: sfd, address });
        }
    }
    Err("Failed to bind any address".to_owned())
}
fn inet_passive_socket(service: &str, socket_type: c_int, do_listen: bool, backlog: c_int) -> Result<PassiveSocket, String> {
    let addresses = unsafe {
        let mut hints = MaybeUninit::<addrinfo>::zeroed().assume_init();
        hints.ai_canonname = ptr::null_mut();
        hints.ai_addr = ptr::null_mut();
        hints.ai_next = ptr::null_mut();
        hints.ai_socktype = socket_type;
        hints.ai_family = AF_UNSPEC;
        hints.ai_flags = AI_PASSIVE;

        let service_s = CString::new(service).expect("Failed to create CString");
        let mut result: MaybeUninit<*mut addrinfo> = MaybeUninit::uninit();
        let s = getaddrinfo(ptr::null(), service_s.as_ptr(), &hints, result.as_mut_ptr());

        if s != 0 {
            return Err("Failed to find address".to_owned());
        }

        AddrInfoList { address_p: result.assume_init() }
    };

    let sock = bind_address(addresses, do_listen)?;
    if do_listen {
        if unsafe { listen(sock.socket_desc, backlog) } == -1 {
            return Err("Failed to listen on socket".to_owned());
        }
    }

    Ok(sock)
}

pub fn inet_listen(service: &str, backlog: c_int) -> Result<PassiveSocket, String> {
    inet_passive_socket(service, SOCK_STREAM, true, backlog)
}

pub fn inet_bind(service: &str, socket_type: c_int) -> Result<PassiveSocket, String> {
    inet_passive_socket(service, socket_type, false, 0)
}

pub fn inet_address_str(addr: &sockaddr, addr_len: socklen_t) -> String {
    let mut host: [c_char; NI_MAXHOST as usize] = [0; NI_MAXHOST as usize];
    let mut service: [c_char; NI_MAXSERV as usize] = [0; NI_MAXSERV as usize];

    if unsafe { getnameinfo(addr, addr_len, host.as_mut_ptr(), NI_MAXHOST, service.as_mut_ptr(), NI_MAXSERV, NI_NUMERICSERV) } == 0 {
        let host = unsafe { CStr::from_ptr(host.as_ptr()) }.to_str().expect("Failed to read host");
        let service = unsafe { CStr::from_ptr(service.as_ptr()) }.to_str().expect("Failed to read service");
        format!("({},{})", host, service)
    } else {
        "(?UNKNOWN?)".to_owned()
    }
}