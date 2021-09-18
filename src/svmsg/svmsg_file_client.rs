// listing 46-9 (page 960)
use std::env;
use std::ptr;
use std::path::{Path};

use libc::{msgget, S_IRUSR, S_IWUSR, S_IWGRP, atexit, msgctl, IPC_RMID, exit, EXIT_SUCCESS, EXIT_FAILURE,
           IPC_PRIVATE};

use rlpi::svmsg::svmsg_file::*;
use rlpi::error_functions::{usage_err, err_exit};
use std::os::raw::c_int;

static mut CLIENT_QUEUE_ID: c_int = -1;

fn remove_queue_by_id(id: c_int) {
    if unsafe { msgctl(id, IPC_RMID, ptr::null_mut()) } == -1 {
        err_exit("msgctl");
    }
}

extern "C" fn remove_queue() {
    let client_id = unsafe { CLIENT_QUEUE_ID };
    if client_id != -1 {
        remove_queue_by_id(client_id);
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} pathname", args[0]));
    }

    let request_path = Path::new(args[1].as_str());

    // get server queue identifier and create queue for response
    let server_id = unsafe { msgget(SERVER_KEY, S_IWUSR as c_int) };
    if server_id == -1 {
        err_exit("msgget - server message queue");
    }

    let client_id = unsafe { msgget(IPC_PRIVATE, (S_IRUSR | S_IWUSR | S_IWGRP) as c_int) };
    if client_id == -1 {
        err_exit("msgget - client message queue");
    }
    unsafe {
        CLIENT_QUEUE_ID = client_id;
        atexit(remove_queue);
    }

    // send request message for specified file
    let request_data = RequestMessage::of_path(client_id,request_path).expect("Failed to create request message");
    request_data.send_message(server_id).expect("Failed to send message");

    // get first response
    let mut data: QueueData<ResponseMessage> = QueueData::receive_message(client_id, get_req_msg_size())
        .expect("Failed to receive message")
        .expect("Failed to receive message");

    if data.message_type() == RESP_MT_FAILURE {
        remove_queue_by_id(client_id);
        unsafe { exit(EXIT_FAILURE); }
    }

    let mut total_bytes = data.message_len();
    let mut num_messages = 1;
    while data.message_type() == RESP_MT_DATA {
        data = QueueData::receive_message(client_id,get_req_msg_size())
            .expect("Failed to receive message")
            .expect("Failed to receive message");
        total_bytes += data.message_len();
        num_messages += 1;
    }

    println!("Received {} bytes ({} messages)", total_bytes, num_messages);

    unsafe { exit(EXIT_SUCCESS); }
}