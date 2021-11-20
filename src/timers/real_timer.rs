//listing 23-1 (page 482)
use std::{env, ptr};
use std::mem::{MaybeUninit};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering, AtomicI64};
use std::os::raw::{c_int};

use libc::{timeval, gettimeofday, itimerval, exit, EXIT_SUCCESS, ITIMER_REAL, sigaction, sighandler_t, SIGALRM};

use rlpi::util::{numeric_arg_or};
use rlpi::libc::time::{clock, CLOCKS_PER_SEC};
use rlpi::libc::sys::time::{getitimer, setitimer};
use rlpi::error_functions::{err_exit, usage_err};
use rlpi::signals::signal_functions::{sig_empty_set};

//number of calls to the display_times function
const CALL_NUM: AtomicUsize = AtomicUsize::new(0);
const GOT_ALARM: AtomicBool = AtomicBool::new(false);

//fields of global start timeval
//TODO: replace with mutable global
const START_SEC: AtomicI64 = AtomicI64::new(0);
const START_USEC: AtomicI64 = AtomicI64::new(0);

fn set_start_time(start: &timeval) {
    START_SEC.store(start.tv_sec, Ordering::SeqCst);
    START_USEC.store(start.tv_usec, Ordering::SeqCst);
}

fn get_start_time() -> timeval {
    timeval {
		tv_sec: START_SEC.load(Ordering::SeqCst),
		tv_usec: START_USEC.load(Ordering::SeqCst)
    }
}

fn display_times(msg: &str, include_timer: bool) {
    let call_num = CALL_NUM.fetch_add(1, Ordering::SeqCst);

    if call_num == 0 {
		let mut start: MaybeUninit<timeval> = MaybeUninit::uninit();
		if unsafe { gettimeofday(start.as_mut_ptr(), ptr::null_mut()) } == -1 {
			err_exit("gettimeofday");
		} else {
			let start = unsafe { start.assume_init() };
			set_start_time(&start);
		}
    }

    if call_num % 20 == 0 {
		println!("        Elapsed\tValue\tInterval");
    }

    let mut current: MaybeUninit::<timeval> = MaybeUninit::uninit();
    if unsafe { gettimeofday(current.as_mut_ptr(), ptr::null_mut()) } == -1 {
		err_exit("gettimeofday");
    }
    let current = unsafe { current.assume_init() };
    
    let start = get_start_time();
    let elapsed = (current.tv_sec - start.tv_sec) as f64 + ((current.tv_usec - start.tv_usec) as f64 / 1000000.0);
    print!("{} {}", msg, elapsed);

    if include_timer {
		let mut itv: MaybeUninit::<itimerval> = MaybeUninit::uninit();
		if unsafe { getitimer(ITIMER_REAL, itv.as_mut_ptr()) } == -1 {
			err_exit("getitimer");
		}
		let itv = unsafe { itv.assume_init() };
		print!("\t{}\t{}",
			   (itv.it_value.tv_sec + itv.it_value.tv_usec) as f64 / 1000000.0,
			   (itv.it_interval.tv_sec + itv.it_interval.tv_usec) as f64 / 1000000.0);
    }
    
    println!();
    //NOTE: CALL_NUM incremented by fetch_add above
}

extern "C" fn sigalarm_handler(_sig: c_int) {
    println!("BEFORE: {}", GOT_ALARM.load(Ordering::SeqCst));
    GOT_ALARM.store(true, Ordering::SeqCst);
    println!("AFTER: {}", GOT_ALARM.load(Ordering::SeqCst));
}

pub fn main() {
	let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
		usage_err(&format!("{} [secs [usecs [int-secs [int-usecs]]]]", args[0]));
    }

    let sa = sigaction {
		sa_sigaction: sigalarm_handler as extern "C" fn(c_int) as sighandler_t,
		sa_mask: sig_empty_set(),
		sa_flags: 0,
		sa_restorer: None
    };

    if unsafe { sigaction(SIGALRM, &sa, ptr::null_mut()) } == -1 {
		err_exit("sigaction");
    }

    //set timer from the command-line arguments
    let itv = itimerval {
		it_value: timeval {
			tv_sec: numeric_arg_or(&args, 1, 2),
			tv_usec: numeric_arg_or(&args, 2, 0)
		},
		it_interval: timeval {
			tv_sec: numeric_arg_or(&args, 3, 0),
			tv_usec: numeric_arg_or(&args, 4, 0)
		}
    };

    //exit after 3 signals or on first signal if interval is 0
    let max_sigs = if itv.it_interval.tv_sec == 0 && itv.it_interval.tv_usec == 0 {
		1
    } else {
		3
    };

    display_times("START:", false);
    if unsafe { setitimer(ITIMER_REAL, &itv, ptr::null_mut()) } == -1 {
		err_exit("setitimer");
    }

    let mut prev_clock = unsafe { clock() };
    let mut sig_count = 0;

    loop {
		//inner loop consumes at least 0.5 seconds CPU time
		while ((unsafe { clock() } - prev_clock) * 10 / CLOCKS_PER_SEC) < 5 {
			if GOT_ALARM.load(Ordering::SeqCst) {
			GOT_ALARM.store(false, Ordering::SeqCst);
			display_times("ALARM:", true);

			sig_count = sig_count + 1;
			if sig_count >= max_sigs {
				println!("That's all folks!");
				unsafe { exit(EXIT_SUCCESS) };
			}
			}
		}

		prev_clock = unsafe { clock() };
		display_times("Main: ", true);
    }
}
