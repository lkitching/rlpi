// listing 52-5 (page 1076)
use std::os::raw::{c_int, c_char};
use std::ffi::{CString};
use std::mem::MaybeUninit;

use clap::{App, Arg, ArgMatches};
use libc::{O_RDONLY, O_NONBLOCK, mq_open, mq_getattr, mq_attr, mq_receive, size_t};
use rlpi::error_functions::err_exit;

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags = O_RDONLY;
    if matches.is_present("nonblock") { flags |= O_NONBLOCK; }
    flags
}

pub fn main() {
    let args = App::new("pmsg_receive")
        .version("1.0")
        .about("Receives a message from a named posix message queue")
        .arg(Arg::with_name("nonblock")
            .short("n")
            .help("Use O_NONBLOCK flag")
            .required(false))
        .arg(Arg::with_name("name")
            .index(1)
            .help("Name of the message queue to receive from")
            .required(true));

    let matches = args.get_matches();
    let flags = get_flags(&matches);

    let mq_desc = unsafe {
        let name_s = CString::new(matches.value_of("name").unwrap()).expect("Failed to create CString");
        mq_open(name_s.as_ptr(), flags)
    };

    if mq_desc == -1 {
        err_exit("mq_open");
    }

    let attr = unsafe {
        let mut attr: MaybeUninit<mq_attr> = MaybeUninit::uninit();
        if mq_getattr(mq_desc, attr.as_mut_ptr()) == -1 {
            err_exit("mq_getattr");
        }
        attr.assume_init()
    };

    let mut buf: Vec<u8> = Vec::with_capacity(attr.mq_msgsize as usize);
    let mut priority = 0;
    let message_len = unsafe { mq_receive(mq_desc, buf.as_mut_ptr() as *mut c_char, attr.mq_msgsize as size_t, &mut priority) };
    if message_len == -1 {
        err_exit("mq_receive");
    }

    unsafe { buf.set_len(message_len as usize); }

    println!("Read {} bytes; priority = {}", message_len, priority);
    let s = String::from_utf8(buf).expect("Failed to create String");
    println!("{}", s);
}