use std::os::raw::{c_char};

const ACCT_COMM: usize = 16;

#[allow(non_camel_case_types)]
pub type comp_t = u16;

// enum
//   {
//     AFORK = 0x01,		/* Has executed fork, but no exec.  */
//     ASU = 0x02,			/* Used super-user privileges.  */
//     ACORE = 0x08,		/* Dumped core.  */
//     AXSIG = 0x10		/* Killed by a signal.  */
//   };

pub const AFORK: c_char = 0x01;
pub const ASU: c_char = 0x02;
pub const ACORE: c_char = 0x08;
pub const AXSIG: c_char = 0x10;

#[repr(C)]
pub struct acct {
    pub ac_flag: c_char,
    pub ac_uid: u16,
    pub ac_gid: u16,
    pub ac_tty: u16,
    pub ac_btime: u32,
    pub ac_utime: comp_t,
    pub ac_stime: comp_t,
    pub ac_etime: comp_t,
    pub ac_mem: comp_t,
    pub ac_io: comp_t,
    pub ac_rw: comp_t,
    pub ac_minflt: comp_t,
    pub ac_majflt: comp_t,
    pub ac_swaps: comp_t,
    pub ac_exitcode: u32,
    pub ac_comm: [c_char; ACCT_COMM + 1],
    pub ac_pad: [c_char; 10]
}
