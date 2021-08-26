//listing 46-1 (page 938)
use std::process;
use clap::{App, Arg, ArgMatches};
use std::os::raw::c_int;
use std::ops::{BitOrAssign};

use libc::{IPC_CREAT, IPC_EXCL, IPC_PRIVATE, S_IRUSR, S_IWUSR, mode_t, key_t, ftok, msgget};
use std::ffi::CString;
use rlpi::error_functions::err_exit;

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags = 0;
    if matches.is_present("create") { flags |= IPC_CREAT; }
    if matches.is_present("exclusive") { flags |= IPC_EXCL; }
    flags
}

fn get_permissions(matches: &ArgMatches) -> mode_t {
    match matches.value_of("perms") {
        None => { S_IRUSR | S_IWUSR },
        Some(s) => {
            match mode_t::from_str_radix(s, 8) {
                Ok(mode) => mode,
                Err(_) => {
                    eprintln!("Invalid octal permissions");
                    eprintln!("{}", matches.usage());
                    process::exit(1)
                }
            }
        }
    }
}

fn get_key(matches: &ArgMatches) -> key_t {
    match (matches.value_of("ftok"), matches.value_of("key"), matches.is_present("private")) {
        (Some(pathname), None, false) => {
            let cs = CString::new(pathname).expect("Failed to create CString");
            let key = unsafe { ftok(cs.as_ptr(), 1) };
            if key == -1 {
                eprintln!("Failed to generate key with ftok()");
                process::exit(1)
            } else {
                key
            }
        },
        (None, Some(key_str), false) => {
            match key_str.parse() {
                Ok(k) => k,
                Err(_) => {
                    eprintln!("-k option requires a numeric argument");
                    process::exit(1)
                }
            }
        },
        (None, None, true) => {
            IPC_PRIVATE
        },
        _ => {
            eprintln!("Exactly one of the options -f, -k or -p must be supplied");
            process::exit(1)
        }
    }
}

pub fn main() {
    let args = App::new("svmsg_create")
        .version("1.0")
        .about("")
        .arg(Arg::with_name("create")
            .short("c")
            .help("Use IPC_CREAT flag"))
        .arg(Arg::with_name("exclusive")
            .short("x")
            .help("Use IP_EXCL flag"))
        .arg(Arg::with_name("ftok")
            .short("f")
            .value_name("pathname")
            .help("Generate key using ftok()"))
        .arg(Arg::with_name("key")
            .short("k")
            .value_name("key")
            .help("Use 'key' as key"))
        .arg(Arg::with_name("private")
            .short("p")
            .help("Use IPC_PRIVATE key"))
        .arg(Arg::with_name("perms")
            .index(1)
            .required(false)
            .help("Octal permissions for the queue"));

    let matches = args.get_matches();

    let flags = get_flags(&matches);
    let key = get_key(&matches);
    let permissions = get_permissions(&matches);

    let queue_id = unsafe { msgget(key, flags | (permissions as c_int)) };
    if queue_id == -1 {
        err_exit("Failed to create message queue");
    } else {
        println!("{}", queue_id);
    }

}