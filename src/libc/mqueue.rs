use std::os::raw::{c_int};
use libc::{mqd_t, sigevent};

#[link(name = "rt")]
extern {
    pub fn mq_notify(mqdes: mqd_t, sevp: *const sigevent) -> c_int;
}