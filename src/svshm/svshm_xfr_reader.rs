// listing 48-3 (page 1005)
use std::{mem, ptr};

use libc::{semget, IPC_CREAT, shmget, write, STDOUT_FILENO, ssize_t, shmat, shmdt};

use rlpi::svshm::svshm_xfr::{SEM_KEY, OBJ_PERMS, WRITE_SEM, READ_SEM, ShmSeg, SHM_KEY};
use rlpi::error_functions::{err_exit, fatal};
use rlpi::svsem::binary_sems::BinarySempahores;
use std::os::raw::{c_ushort, c_void};

pub fn main() {
    // get ids for semaphore set and shared memory created by writer
    let sem_id = unsafe { semget(SEM_KEY, 0, 0) };

    if sem_id == -1 {
        err_exit("semget");
    }

    let shm_id = unsafe { shmget(SHM_KEY, 0, 0) };
    if shm_id == -1 {
        err_exit("shmget");
    }

    let shm_p = unsafe { shmat(shm_id, ptr::null(), 0) } as *mut ShmSeg;
    if shm_p as isize == -1 {
        err_exit("shmat");
    }

    let sems = BinarySempahores::default();

    // read from shared memory and output to stdout
    let mut num_transfers = 0;
    let mut total_bytes = 0;

    loop {
        // wait for writer to transfer
        sems.reserve_sem(sem_id, READ_SEM as c_ushort).expect("Failed to wait for read semaphore");

        let chunk_size = unsafe { &*shm_p }.count;

        if chunk_size == 0 {
            // writer encountered EOF
            break;
        }

        total_bytes += chunk_size;
        let chunk_bytes = &(unsafe {&*shm_p }.buf)[0 .. chunk_size];
        let bytes_written = unsafe { write(STDOUT_FILENO, chunk_bytes.as_ptr() as *const c_void, chunk_size) };
        if bytes_written != chunk_size as ssize_t {
            fatal("partial/failed write");
        }

        sems.release_sem(sem_id, WRITE_SEM as c_ushort).expect("Failed to release write semaphore");
        num_transfers += 1;
    }

    if unsafe { shmdt(shm_p as *const c_void) } == -1 {
        err_exit("shmdt");
    }

    // signal writer to clean up
    sems.release_sem(sem_id, WRITE_SEM as c_ushort).expect("Failed to release write semaphore");

    eprintln!("Received {} bytes ({} transfers)", total_bytes, num_transfers);

}