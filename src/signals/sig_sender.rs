//listing 20-6 (page 412)
use std::env;
use libc::{exit, EXIT_SUCCESS, kill};

use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        usage_err(&format!("{} pid num-sigs sig-num [sig-num-2]\n", args[0]));
    }

    let pid = args[1].parse().expect("Invalid PID");
    let num_sigs = args[2].parse().expect("Invalid count");
    let sig = args[3].parse().expect("Invalid signal");
    let sig2 = args.get(4).map(|s| s.parse().expect("Invalid signal"));

    //send signals to receiver
    println!("{}: sending signal {} to process {} {} times", args[0], sig, pid, num_sigs);

    for _j in 0..num_sigs {
        if unsafe { kill(pid, sig) } == -1 {
            err_exit("kill");
        }
    }

    //send second signal if specified
    if let Some(sig) = sig2 {
        if unsafe { kill(pid, sig) } == -1 {
            err_exit("kill");
        }
    }

    println!("{}: exiting", args[0]);
    unsafe { exit(EXIT_SUCCESS); }
}
