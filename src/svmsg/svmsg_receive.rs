//listing 46-3 (page 945)
use std::os::raw::{c_int, c_long};

use libc::{MSG_NOERROR, IPC_NOWAIT, MSG_EXCEPT, msgrcv};

use clap::{App, Arg, ArgMatches};
use std::mem::MaybeUninit;
use std::ffi::c_void;
use rlpi::error_functions::err_exit;
use rlpi::libc::sys::types::size_t;

const MAX_MTEXT: usize = 1024;

#[repr(C)]
struct Message {
    mtype: c_long,
    mtext: [u8; MAX_MTEXT]
}

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags: c_int = 0;
    if matches.is_present("noerror") { flags |= MSG_NOERROR; }
    if matches.is_present("nowait") { flags |= IPC_NOWAIT; }
    if matches.is_present("except") { flags |= MSG_EXCEPT; }
    flags
}

pub fn main() {
    let args = App::new("svmsg_receive")
        .arg(Arg::with_name("msqid")
            .index(1)
            .required(true)
            .help("Id of the message queue to receive from"))
        .arg(Arg::with_name("max-bytes")
            .index(2)
            .help("Maximum message size in bytes"))
        .arg(Arg::with_name("noerror")
            .short("e")
            .help("Use MSG_NOERROR flag"))
        .arg(Arg::with_name("message_type")
            .short("t")
            .value_name("type")
            .help("Select message of given type"))
        .arg(Arg::with_name("nowait")
            .short("n")
            .help("Use IPC_NOWAIT flag"))
        .arg(Arg::with_name("except")
            .short("x")
            .help("Use MSG_EXCEPT flag"));

    let matches = args.get_matches();
    let msqid: c_int = matches.value_of("msqid").unwrap().parse().expect("Invalid message queue id");
    let message_type: c_long = matches.value_of("message_type")
        .map(|t| t.parse().expect("Invalid message type"))
        .unwrap_or(0);
    let max_bytes: usize = matches.value_of("max-bytes")
        .map(|s| s.parse().expect("Invalid message size"))
        .unwrap_or(MAX_MTEXT);
    let flags = get_flags(&matches);

    let (message, message_len) = unsafe {
        let mut msg: MaybeUninit<Message> = MaybeUninit::uninit();
        let message_len = msgrcv(msqid, msg.as_mut_ptr() as *mut c_void, max_bytes, message_type, flags);

        if message_len == -1 {
            err_exit("msgrcv");
        }

        (msg.assume_init(), message_len as size_t)
    };

    print!("Received: type={}; length={}", message.mtype, message_len);

    if message_len > 0 {
        print!("; body={}", std::str::from_utf8(&message.mtext[0 .. message_len]).expect("Invalid string"));
    }
    println!();
}