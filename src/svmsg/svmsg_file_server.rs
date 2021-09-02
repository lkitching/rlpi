// listing 46-8 (page 957)
use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};
use std::mem;

use libc::{msgget, IPC_CREAT, IPC_EXCL, S_IRUSR, S_IWUSR, S_IWGRP, sigaction, sighandler_t, SA_RESTART, SIGCHLD,
           msgctl, msgrcv, EINTR, _exit, EXIT_SUCCESS, EXIT_FAILURE, exit, IPC_RMID, O_RDONLY, open, msgsnd, read, size_t,
           waitpid, WNOHANG};

use rlpi::svmsg::svmsg_file::*;
use rlpi::error_functions::{err_exit, err_msg};
use rlpi::signals::signal_functions::sig_empty_set;
use std::mem::MaybeUninit;
use rlpi::libc::{errno, set_errno};
use rlpi::util::{ForkResult, try_fork};

extern "C" fn grim_reaper(sig: c_int) {
    // waitpid() might change errno
    let saved_errno = errno();

    loop {
        // wait for any child process that have exited since the signal was raised
        let child_pid = unsafe { waitpid(-1, ptr::null_mut(), WNOHANG) };
        if child_pid <= 0 {
            break;
        }
    }

    set_errno(saved_errno);
}

fn serve_request(req: &QueueData<RequestMessage>) {
    let client_queue_id = req.message().client_id;

    let fd = unsafe {
        println!("[server] Fetching file {}", req.get_str());
        let cs = CString::new(req.get_str()).expect("Failed to create CString");
        open(cs.as_ptr(), O_RDONLY)
    };
    if fd == -1 {
        // open failed, send error text
        let data = ResponseMessage::failure_response("Couldn't open");

        unsafe {
            data.send_message(client_queue_id);
            exit(EXIT_FAILURE);
        }
    }

    // transmit file contents in messages with type RESP_MT_DATA
    // don't diagnose read() and msgsend() errors since we can't notify the client
    let mut resp = ResponseMessage {
        mtype: RESP_MT_DATA,
        data: [0; RESP_MSG_SIZE]
    };
    loop {
        let bytes_read = unsafe { read(fd, resp.data.as_mut_ptr() as *mut c_void, RESP_MSG_SIZE) };
        if bytes_read <= 0 { break; }

        if unsafe { msgsnd(client_queue_id, &resp as *const ResponseMessage as *const c_void, bytes_read as size_t, 0) } == -1 {
            break;
        }
    }

    // send a message of type RESP_MT_END to signify EOF
    let resp = ResponseMessage {
        mtype: RESP_MT_END,
        data: [0; RESP_MSG_SIZE]
    };
    unsafe { msgsnd(client_queue_id, &resp as *const ResponseMessage as *const c_void, 0, 0); }
}

pub fn main() {
    // create server message queue
    let server_id = unsafe {
        let flags = IPC_CREAT | IPC_EXCL | S_IRUSR as c_int | S_IWUSR as c_int | S_IWGRP as c_int;
        msgget(SERVER_KEY, flags) };

    if server_id == -1 {
        err_exit("msgget");
    }

    // establish SIGCHLD handler to reap terminated children
    let sa = sigaction {
        sa_mask: sig_empty_set(),
        sa_flags: SA_RESTART,
        sa_sigaction: grim_reaper as extern "C" fn(c_int) as sighandler_t,
        sa_restorer: None
    };

    if unsafe { sigaction(SIGCHLD, &sa, ptr::null_mut()) } == -1 {
        err_exit("sigaction");
    }

    // read requests and handle each one in a separate child process
    // TODO: can return value from loop?
    loop {
        let message_size = get_req_msg_size();
        match QueueData::<RequestMessage>::receive_message(server_id, message_size) {
            Err(msg) => {
                err_msg(msg.as_str());
                break;
            },
            Ok(None) => {
                continue;
            },
            Ok(Some(data)) => {
                match try_fork() {
                    Err(msg) => {
                        err_msg(&format!("fork: {}", msg));
                        break;
                    },
                    Ok(ForkResult::Parent(_)) => {
                        // parent loops to receive next client request
                    },
                    Ok(ForkResult::Child) => {
                        serve_request(&data);
                        unsafe { _exit(EXIT_SUCCESS); }
                    }
                }
            }
        }
    }

    // if msgrcv() or fork() fails, remove server message queue and exit
    if unsafe { msgctl(server_id, IPC_RMID, ptr::null_mut()) } == -1 {
        err_exit("msgctl");
    }
    unsafe { exit(EXIT_SUCCESS); }
}