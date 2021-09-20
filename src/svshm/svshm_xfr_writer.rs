//listing 48-2 (page 1003)
use std::{mem, ptr};

use libc::{semget, IPC_CREAT, shmget, STDIN_FILENO, read, IPC_RMID, shmat, shmdt, shmctl, semctl};

use rlpi::svsem::binary_sems::{BinarySempahores};

use rlpi::svshm::svshm_xfr::{SEM_KEY, SHM_KEY, OBJ_PERMS, READ_SEM, WRITE_SEM, ShmSeg, BUF_SIZE};
use rlpi::error_functions::err_exit;
use std::os::raw::{c_ushort, c_void};

pub fn main() {
    let sem_id = unsafe { semget(SEM_KEY, 2, IPC_CREAT | OBJ_PERMS) };
    if sem_id == -1 {
        err_exit("semget");
    }

    let sems = BinarySempahores::default();
    sems.init_sem_available(sem_id, WRITE_SEM).expect("Failed to initialise write semaphore");
    sems.init_sem_in_use(sem_id, READ_SEM).expect("Failed to initialise read semaphore");

    let shm_id = unsafe { shmget(SHM_KEY, mem::size_of::<ShmSeg>(), IPC_CREAT | OBJ_PERMS) };
    if shm_id == -1 {
        err_exit("shmget");
    }

    let shm_p = unsafe { shmat(shm_id, ptr::null(), 0) } as *mut ShmSeg;
    if (shm_p as isize) == -1 {
        err_exit("shmat");
    }

    // transfer blocks of data from stdin to shared memory
    let mut transfer_count: u32 = 0;
    let mut bytes = 0;

    loop {
        if let Err(_) = sems.reserve_sem(sem_id, WRITE_SEM as c_ushort) {
            err_exit("reserve_sem");
        }

        let bytes_read = unsafe { read(STDIN_FILENO, ((*shm_p).buf).as_mut_ptr() as *mut c_void, BUF_SIZE) };
        if bytes_read == -1 {
            err_exit("read");
        }
        unsafe { &mut (*shm_p) }.count = bytes_read as usize;

        // unblock reader
        if let Err(_) = sems.release_sem(sem_id, READ_SEM as c_ushort) {
            err_exit("release_sem");
        }

        // check if EOF has been reached
        // NOTE: This is checked after releasing the reader so it can observe the empty segment
        if unsafe { &*shm_p }.count == 0 {
            break;
        }

        transfer_count += 1;
        bytes += bytes_read as usize;
    }

    // wait until the reader has signalled it has completed
    if let Err(_) = sems.reserve_sem(sem_id, WRITE_SEM as c_ushort) {
        err_exit("reserve_sem");
    }

    // delete semaphore set and shared memory segment
    if unsafe { semctl(sem_id, 0, IPC_RMID) } == -1 {
        err_exit("semctl");
    }

    if unsafe { shmdt(shm_p as *const c_void) } == -1 {
        err_exit("shmdt");
    }

    if unsafe { shmctl(shm_id, IPC_RMID, ptr::null_mut()) } == -1 {
        err_exit("shmctl");
    }

    eprintln!("Sent {} bytes in {} transfers", bytes, transfer_count);
}
