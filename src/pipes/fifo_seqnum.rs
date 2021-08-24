use libc::{pid_t};

pub const SERVER_FIFO: &'static str = "/tmp/seqnum_sv";

#[repr(C)]
pub struct Request {
    pub pid: pid_t,
    pub seq_len: usize
}

#[repr(C)]
pub struct Response {
    pub seq_num: usize
}

pub fn get_client_fifo(client_pid: pid_t) -> String {
    format!("/tmp/seqnum_cl.{}", client_pid)
}
