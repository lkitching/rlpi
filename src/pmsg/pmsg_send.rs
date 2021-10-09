// listing 52-4 (page 1074)
use std::os::raw::{c_int, c_char};
use std::ffi::{CString};

use clap::{App, Arg, ArgMatches};

use libc::{O_WRONLY, O_NONBLOCK, mq_open, mq_send};
use rlpi::error_functions::err_exit;

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags = O_WRONLY;
    if matches.is_present("nonblock") { flags |= O_NONBLOCK; }
    flags
}

pub fn main() {
    let args = App::new("pmsg_send")
        .version("0.1")
        .about("Sends a message to a posix message queue")
        .arg(Arg::with_name("nonblock")
            .short("n")
            .help("Use O_NONBLOCK flag")
            .required(false))
        .arg(Arg::with_name("name")
            .index(1)
            .help("The name of the queue to send to")
            .required(true))
        .arg(Arg::with_name("message")
            .index(2)
            .help("The message to send")
            .required(true))
        .arg(Arg::with_name("priority")
            .index(3)
            .required(false));

    let matches = args.get_matches();
    let flags = get_flags(&matches);

    let mq_desc = unsafe {
        let name_s = CString::new(matches.value_of("name").unwrap()).expect("Failed to create CString");
        mq_open(name_s.as_ptr(), flags)
    };
    if mq_desc == -1 {
        err_exit("mq_open");
    }

    let priority = matches.value_of("priority").map_or(0, |s| s.parse().expect("Invalid priority"));
    let message = matches.value_of("message").unwrap();

    if unsafe { mq_send(mq_desc, message.as_ptr() as *const c_char, message.as_bytes().len(), priority) } == -1 {
        err_exit("mq_send");
    }
}