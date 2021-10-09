//listing 52-2 (page 1069)
use std::os::raw::{c_int};
use std::ffi::{CString};

use clap::{App, Arg, ArgMatches};
use libc::{mq_open, mq_attr, O_RDWR, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR, mode_t};

use rlpi::error_functions::err_exit;
use std::num::ParseIntError;
use std::mem::MaybeUninit;
use std::borrow::BorrowMut;

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags = O_RDWR;
    if matches.is_present("create") { flags |= O_CREAT; }
    if matches.is_present("exclusive") { flags |= O_EXCL; }
    flags
}

fn get_permissions(matches: &ArgMatches) -> Result<mode_t, ParseIntError> {
    match matches.value_of("perms") {
        Some(ps) => {
            mode_t::from_str_radix(ps, 8) },
        None => { Ok(S_IRUSR | S_IWUSR) }
    }
}

pub fn main() {
    let args = App::new("svmsg_create")
        .version("1.0")
        .about("")
        .arg(Arg::with_name("create")
            .short("c")
            .help("Create queue (O_CREAT)"))
        .arg(Arg::with_name("exclusive")
            .short("x")
            .help("Create exclusively (O_EXCL)"))
        .arg(Arg::with_name("maxmsg")
            .short("m")
            .long("maxmsg")
            .value_name("MAXMSG")
            .help("Set the maximum number of messages in the queue"))
        .arg(Arg::with_name("msgsize")
            .short("s")
            .long("msgsize")
            .value_name("MSGSIZE")
            .help("Set the maximum size for messages in the queue"))
        .arg(Arg::with_name("name")
            .index(1)
            .required(true)
            .help("Name of the queue"))
        .arg(Arg::with_name("perms")
            .index(2)
            .required(false)
            .help("Octal permissions for the queue"));

    let matches = args.get_matches();
    let flags = get_flags(&matches);
    let perms = get_permissions(&matches).expect("Invalid permissions");

    unsafe {
        let mut attrp: MaybeUninit<mq_attr> = MaybeUninit::zeroed();
        let p = attrp.as_mut_ptr();
        (*p).mq_maxmsg = matches.value_of("maxmsg").map_or(50, |s| s.parse().expect("Invalid max messages"));
        (*p).mq_msgsize = matches.value_of("msgsize").map_or(2048, |s| s.parse().expect("Invalid message size"));

        let queue_name_s = CString::new(matches.value_of("name").unwrap()).expect("Failed to create CString");
        let mqd = mq_open(queue_name_s.as_ptr(), flags, perms, attrp.as_ptr());
        if mqd == -1 {
            err_exit("mq_open");
        }
    }
}