// listing 47-10 (page 990)
use std::os::raw::{c_int, c_short, c_ushort};

use libc::{sembuf, semctl, semop, EINTR};

use crate::libc::{errno};
use crate::libc::sys::sem::{semun, SETVAL, SEM_UNDO};

pub struct BinarySempahores {
    pub use_sem_undo: bool,
    pub retry_on_eintr: bool
}

fn set_value(sem_id: c_int, sem_num: c_int, value: c_int) -> Result<(), ()> {
    let arg = semun { val: value };
    if unsafe { semctl(sem_id, sem_num, SETVAL, arg) } == 0 {
        Ok(())
    } else {
        Err(())
    }
}

impl BinarySempahores {
    pub fn default() -> Self {
        BinarySempahores {
            use_sem_undo: false,
            retry_on_eintr: true
        }
    }

    pub fn init_sem_available(&self, sem_id: c_int, sem_num: c_int) -> Result<(), ()> {
        set_value(sem_id, sem_num, 1)
    }

    pub fn init_sem_in_use(&self, sem_id: c_int, sem_num: c_int) -> Result<(), ()> {
        set_value(sem_id, sem_num, 0)
    }

    fn create_op(&self, sem_num: c_ushort, op: c_short) -> sembuf {
        let flags = if self.use_sem_undo { SEM_UNDO } else { 0 };

        sembuf {
            sem_num: sem_num,
            sem_op: op,
            sem_flg: flags
        }
    }

    pub fn reserve_sem(&self, sem_id: c_int, sem_num: c_ushort) -> Result<(), ()> {
        let mut sops = self.create_op(sem_num, -1);

        while unsafe { semop(sem_id, &mut sops, 1) } == -1 {
            let should_retry = self.retry_on_eintr && errno() == EINTR;
            if ! should_retry {
                return Err(());
            }
        }

        Ok(())
    }
    pub fn release_sem(&self, sem_id: c_int, sem_num: c_ushort) -> Result<(), ()> {
        let mut sops = self.create_op(sem_num, 1);
        if unsafe { semop(sem_id, &mut sops, 1) } == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}