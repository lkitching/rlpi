// listing 30-4 (page 649)
use std::os::raw::{c_void};
use std::{ptr, thread, env};
use std::time::Duration;

use libc::{PTHREAD_COND_INITIALIZER, PTHREAD_MUTEX_INITIALIZER, pthread_cond_t, pthread_mutex_t};

use rlpi::threads::thread_util::{or_die, mutex_lock, mutex_unlock, create, join, cond_signal, cond_wait};
use rlpi::error_functions::usage_err;

#[derive(PartialEq, Eq)]
enum ThreadState {
    Alive,
    Terminated,
    Joined
}

struct ThreadInfo {
    state: ThreadState,
    sleep_duration: Duration
}

static mut THREAD_DIED: pthread_cond_t = PTHREAD_COND_INITIALIZER;
static mut THREAD_MUTEX: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
static mut THREADS: Vec<ThreadInfo> = vec![];
static mut NUM_UNJOINED: usize = 0;

extern "C" fn thread_func(arg: *mut c_void) -> *mut c_void {
    let idx = unsafe { *(arg as *mut usize )};

    // simulate doing some work
    let sleep_duration = unsafe { &THREADS[idx] }.sleep_duration;
    thread::sleep(sleep_duration);

    println!("Thread {} terminating", idx);

    or_die(mutex_lock(unsafe { &mut THREAD_MUTEX }));
    unsafe {
        NUM_UNJOINED += 1;
        THREADS[idx].state = ThreadState::Terminated;
    }
    or_die(mutex_unlock(unsafe { &mut THREAD_MUTEX }));

    // signal unjoined threads available
    or_die(cond_signal(unsafe { &mut THREAD_DIED }));

    ptr::null_mut()
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        usage_err(&format!("{} nsecs ...", args[0]));
    }

    // create all threads
    let sleep_periods: Vec<u64> = args[1..].iter().map(|s| s.parse().expect("Invalid period")).collect();
    unsafe { THREADS = Vec::with_capacity(sleep_periods.len()) }
    let mut thread_ids = Vec::with_capacity(sleep_periods.len());
    let mut thread_indexes = Vec::with_capacity(sleep_periods.len());

    for (idx, &sleep_secs) in sleep_periods.iter().enumerate() {
        let info = ThreadInfo { state: ThreadState::Alive, sleep_duration: Duration::from_secs(sleep_secs) };
        unsafe { THREADS.push(info); }
        thread_indexes.push(idx);

        let idx_p = &mut thread_indexes[idx] as *mut usize as *mut c_void;
        let thread_id = or_die(create(thread_func, idx_p));
        thread_ids.push(thread_id);
    }

    let mut num_live = thread_ids.len();

    // join with terminated threads
    while num_live > 0 {
        or_die(mutex_lock(unsafe { &mut THREAD_MUTEX }));

        while unsafe { NUM_UNJOINED } == 0 {
            or_die(cond_wait(unsafe { &mut THREAD_DIED }, unsafe { &mut THREAD_MUTEX }));
        }

        for idx in 0 .. thread_ids.len() {
            if unsafe { &THREADS[idx] }.state == ThreadState::Terminated {
                let thread_id = thread_ids[idx];
                or_die(join(thread_id));

                unsafe { THREADS[idx].state = ThreadState::Joined; }
                num_live -= 1;
                unsafe { NUM_UNJOINED -= 1; }
            }
        }

        or_die(mutex_unlock(unsafe { &mut THREAD_MUTEX }));
    }
}