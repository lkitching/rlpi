//listing 23-8 (page 510)
use std::mem::{self, MaybeUninit};
use std::os::raw::c_void;
use std::result::Result;
use std::{env, ptr};

use libc::{
    clock_gettime, exit, itimerspec, read, ssize_t, timerfd_create, timerfd_settime, timespec,
    CLOCK_MONOTONIC, CLOCK_REALTIME, EXIT_SUCCESS,
};

use rlpi::error_functions::{err_exit, usage_err};

fn timespec_from_str(s: &str) -> Result<timespec, &'static str> {
    let components: Vec<&str> = s.split("/").collect();

    if components.len() < 1 || components.len() > 2 {
        Err("Expected seconds[/nanoseconds]")
    } else {
        let seconds = components[0].parse().map_err(|_| "Invalid seconds")?;
        let nanos = if components.len() == 2 {
            components[1].parse().map_err(|_| "Invalid nanoseconds")?
        } else {
            0
        };

        Ok(timespec {
            tv_sec: seconds,
            tv_nsec: nanos,
        })
    }
}

fn itimerspec_from_str(s: &str) -> Result<itimerspec, &'static str> {
    let times: Vec<&str> = s.split(":").collect();

    if times.len() < 1 || times.len() > 2 {
        Err("Expected timespec[:timespec]")
    } else {
        let value = timespec_from_str(times[0])?;
        let interval = if times.len() == 2 {
            timespec_from_str(times[1])?
        } else {
            timespec {
                tv_sec: 0,
                tv_nsec: 0,
            }
        };

        Ok(itimerspec {
            it_value: value,
            it_interval: interval,
        })
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || (args.len() > 1 && args[1].as_str() == "--help") {
        usage_err(&format!(
            "{} secs[/nsecs][:int-secs[/int-nsecs]] [max-exp]",
            args[0]
        ));
    }

    let ts = itimerspec_from_str(args[1].as_str()).expect("Invalid timerspec");
    let max_exp = if args.len() > 2 {
        args[2].parse().expect("Invalid int for max-exp")
    } else {
        1
    };

    let fd = unsafe { timerfd_create(CLOCK_REALTIME, 0) };
    if fd == -1 {
        err_exit("timerfd_create");
    }

    if unsafe { timerfd_settime(fd, 0, &ts, ptr::null_mut()) } == -1 {
        err_exit("timerfd_settime");
    }

    let mut start: MaybeUninit<timespec> = MaybeUninit::uninit();

    if unsafe { clock_gettime(CLOCK_MONOTONIC, start.as_mut_ptr()) } == -1 {
        err_exit("clock_gettime");
    }
    let start = unsafe { start.assume_init() };

    let mut total_exp = 0;
    while total_exp < max_exp {
        /* Read number of expirations on the timer and then display
         * the time elapsed since the timer was started followed by
         * the number of expirations read and total expirations so far */

        let mut num_exp: u64 = 0;
        let s = unsafe {
            read(
                fd,
                &mut num_exp as *mut u64 as *mut c_void,
                mem::size_of::<u64>(),
            )
        };
        if s != mem::size_of::<u64>() as ssize_t {
            err_exit("read");
        }

        total_exp = total_exp + num_exp;

        let mut now: MaybeUninit<timespec> = MaybeUninit::uninit();
        if unsafe { clock_gettime(CLOCK_MONOTONIC, now.as_mut_ptr()) } == -1 {
            err_exit("clock_gettime");
        }
        let now = unsafe { now.assume_init() };

        let mut secs = now.tv_sec - start.tv_sec;
        let mut nanosecs = now.tv_nsec - start.tv_nsec;

        if nanosecs < 0 {
            secs = secs - 1;
            nanosecs = nanosecs + 1000000000;
        }

        println!(
            "{}.{}: expirations read: {}, total={}",
            secs,
            (nanosecs + 500000) / 1000000,
            num_exp,
            total_exp
        );
    }

    unsafe { exit(EXIT_SUCCESS) };
}
