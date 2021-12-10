use std::os::raw::{c_int, c_void, c_char};

use libc::{in6_addr, socklen_t};

#[link(name = "c")]
extern {
    pub fn htonl(host_long: u32) -> u32;
    pub fn htons(host_short: u16) -> u16;
    pub fn ntohl(net_long: u32) -> u32;
    pub fn ntohs(net_short: u16) -> u16;

    pub fn inet_ntop(af: c_int, src: *const c_void, dest: *mut c_char, size: socklen_t) -> *const c_char;
    pub fn inet_pton(af: c_int, src: *const c_char, dest: *mut c_void) -> c_int;

    pub static in6addr_any: in6_addr;
    pub static in6addr_loopback: in6_addr;
}

// NOTE: defined in netinet/in.h
pub const INET_ADDRSTRLN: usize = 16;
pub const INET6_ADDRSTRLN: usize = 46;