// listing 53-1 (page 1092)
use std::env;
use std::ffi::{CString};
use std::os::raw::{c_int, c_uint};

use clap::{App, Arg, ArgMatches};
use libc::{sem_open, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR, mode_t, SEM_FAILED};
use rlpi::error_functions::err_exit;

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags = 0;
    if matches.is_present("create") { flags |= O_CREAT; }
    if matches.is_present("exclusive") { flags |= O_EXCL; }
    flags
}

pub fn main() {
    let args = App::new("psem_create")
        .about("CLI interface to sem_open")
        .version("1.0")
        .arg(Arg::with_name("create")
            .short("c")
            .required(false)
            .help("Create semaphore (O_CREAT)"))
        .arg(Arg::with_name("exclusive")
            .short("x")
            .required(false)
            .help("Create exclusively (O_EXCL)"))
        .arg(Arg::with_name("name")
            .index(1)
            .required(true))
        .arg(Arg::with_name("perms")
            .index(2)
            .required(false))
        .arg(Arg::with_name("value")
            .index(3)
            .required(false));

    let matches = args.get_matches();
    let flags = get_flags(&matches);
    let perms = matches.value_of("perms").map_or(S_IRUSR | S_IWUSR, |p| mode_t::from_str_radix(p, 8).expect("Invalid octal permissions"));
    let value: c_uint = matches.value_of("value").map_or(0, |v| v.parse().expect("Invalid unsigned value"));

    let sem  =unsafe {
        let name = matches.value_of("name").unwrap();
        let name_s = CString::new(name).expect("Failed to create CString");
        sem_open(name_s.as_ptr(), flags, perms, value)
    };
    if sem == SEM_FAILED {
        err_exit("sem_open");
    }
}