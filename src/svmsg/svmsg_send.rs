//listing 46-2 (page 941)
use std::os::raw::{c_int, c_void, c_long};
use std::process;

use clap::{App, Arg};
use libc::{msgsnd, IPC_NOWAIT};

extern crate rlpi;
use rlpi::error_functions::{err_exit};

const MAX_MTEXT: usize = 1024;

#[repr(C)]
struct Message {
    mtype: MessageType,
    mtext: [u8; MAX_MTEXT]
}

type MessageType = c_long;

fn build_message(message_type: MessageType, text: Option<&str>) -> Result<(Message, usize), String> {
    match text {
        None => { Ok((Message { mtype: message_type, mtext: [0; MAX_MTEXT] }, 0))},
        Some(msg) => {
            let buf = msg.as_bytes();

            if buf.len() > MAX_MTEXT {
                return Err(format!("msg-text too long (max: {} characters)", MAX_MTEXT));
            }
            let mut message = Message { mtype: message_type, mtext: [0; MAX_MTEXT]};
            for i in 0..buf.len() {
                message.mtext[i] = buf[i];
            }
            return Ok((message, buf.len()));
        }
    }
}

pub fn main() {
    let args = App::new("svmsg_send")
        .arg(Arg::with_name("msqid")
            .index(1)
            .required(true)
            .help("Id of the message queue to send to"))
        .arg(Arg::with_name("msg-type")
            .index(2)
            .required(true)
            .help("Message type to send"))
        .arg(Arg::with_name("msg-text")
            .index(3)
            .help("Message text to send"))
        .arg(Arg::with_name("nowait")
            .short("n")
            .help("Use the IPC_NOWAIT flag when sending"));

    let matches = args.get_matches();
    let msqid: c_int = matches.value_of("msqid").unwrap().parse().expect("Invalid message queue id");
    let message_type: MessageType = matches.value_of("msg-type").unwrap().parse().expect("Invalid message type");

    let flags = if matches.is_present("nowait") { IPC_NOWAIT } else { 0 };

    match build_message(message_type, matches.value_of("msg-text")) {
        Err(msg) => {
            eprintln!("Failed to build message: {}", msg);
            process::exit(1);
        },
        Ok((message, message_len)) => {
            if unsafe { msgsnd(msqid, &message as *const Message as *const c_void, message_len, flags) } == -1 {
                err_exit("msgsnd");
            }
        }
    }
}