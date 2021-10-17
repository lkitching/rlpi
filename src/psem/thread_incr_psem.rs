// listing 53-6 (page 1101)
use std::env;
use std::os::raw::{c_void};
use std::ptr;
use std::mem::MaybeUninit;

use libc::{sem_t, sem_init, pthread_create, pthread_join, pthread_t, sem_wait, sem_post};
use rlpi::error_functions::{err_exit, err_exit_en};

extern "C" fn thread_func(arg: *mut c_void) -> *mut c_void {
    let tdp = arg as *mut ThreadData;

    unsafe {
        let mut td = &mut(*tdp);

        for j in 0 .. unsafe { &*tdp }.num_loops {
            if unsafe { sem_wait(&mut td.sem) } == -1 {
                err_exit("sem_wait")
            }

            td.counter += 1;

            if unsafe { sem_post(&mut td.sem) } == -1 {
                err_exit("sem_post");
            }
        }
    }
    ptr::null_mut()
}

fn thread_join(thread_h: pthread_t) {
    let r = unsafe { pthread_join(thread_h, ptr::null_mut()) };
    if r != 0 {
        err_exit_en(r, "pthread_join");
    }
}

fn create_thread(td: &mut ThreadData) -> pthread_t {
    unsafe {
        let mut t: MaybeUninit<pthread_t> = MaybeUninit::uninit();
        let r = pthread_create(t.as_mut_ptr(), ptr::null(), thread_func, td as *mut ThreadData as *mut c_void);

        if r != 0 {
            err_exit_en(r, "pthread_create");
        }

        t.assume_init()
    }
}

struct ThreadData {
    counter: u64,
    sem: sem_t,
    num_loops: u32
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let loops = if args.len() > 1 { args[1].parse().expect("Invalid number of loops") } else { 1 };

    // initialise a thread-shared mutex with the value 1
    let sem = unsafe {
        let mut sem: MaybeUninit::<sem_t> = MaybeUninit::uninit();
        if sem_init(sem.as_mut_ptr(), 0, 1) == -1 {
            err_exit("sem_init");
        }
        sem.assume_init()
    };

    let mut td = ThreadData {
        counter: 0,
        sem: sem,
        num_loops: loops
    };

    // create two threads to increment counte
    let t1 = create_thread(&mut td);
    let t2= create_thread(&mut td);

    // wait for threads to terminate
    thread_join(t1);
    thread_join(t2);

    println!("glob = {}", td.counter);
}