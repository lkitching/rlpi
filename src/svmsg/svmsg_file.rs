use std::os::raw::{c_long, c_int, c_void};
use std::path::{Path};
use std::mem::MaybeUninit;
use std::str;

use libc::{PATH_MAX, key_t, size_t, msgrcv, EINTR, msgsnd};

use crate::libc::{errno};

pub const SERVER_KEY: key_t = 0x1aaaaaa1;

#[repr(C)]
pub struct RequestMessage {
    pub mtype: c_long,
    pub client_id: c_int,
    pub pathname: [u8; PATH_MAX as usize]
}

fn copy_str_to(s: &str, buf: &mut [u8]) {
    for (idx, b) in s.bytes().enumerate() {
        buf[idx] = b;
    }
}

impl RequestMessage {
    pub fn of_path(client_queue_id: c_int, path: &Path) -> Result<QueueData<RequestMessage>, String> {
        match path.canonicalize() {
            Ok(path) => {
                match path.to_str() {
                    Some(path_str) => {
                        let bytes = path_str.bytes();
                        if bytes.len() > PATH_MAX as usize {
                            Err("Request path too long".to_string())
                        } else {
                            let mut message = RequestMessage {
                                mtype: 1,
                                client_id: client_queue_id,
                                pathname: [0; PATH_MAX as usize]
                            };
                            copy_str_to(path_str, &mut message.pathname);
                            let message_len = offset_of!(RequestMessage, pathname) - offset_of!(RequestMessage, client_id) + path_str.bytes().len();
                            Ok(QueueData { message, message_len })
                        }
                    },
                    None => { Err("Path is invalid UTF-8".to_string()) }
                }
            },
            Err(_) => {
                Err("Failed to canonicalise path".to_string())
            }
        }
    }
}

pub fn get_req_msg_size() -> usize {
    span_of!(RequestMessage, client_id ..= pathname).len()
}

pub const RESP_MSG_SIZE: usize = 8192;

pub struct ResponseMessage {
    pub mtype: c_long,
    pub data: [u8; RESP_MSG_SIZE]
}

impl ResponseMessage {
    pub fn failure_response(message: &str) -> QueueData<ResponseMessage> {
        let mut resp = ResponseMessage {
            mtype: RESP_MT_FAILURE,
            data: [0; RESP_MSG_SIZE]
        };

        copy_str_to(message, &mut resp.data);
        QueueData { message: resp, message_len: message.bytes().len() }
    }
}

pub struct QueueData<T> {
    message_len: usize,
    message: T
}

impl <T> QueueData<T> {
    pub fn receive_message(queue_id: c_int, message_size: size_t) -> Result<Option<QueueData<T>>, String> {
        unsafe {
            let mut resp: MaybeUninit<T> = MaybeUninit::uninit();
            let msg_len = msgrcv(queue_id, resp.as_mut_ptr() as *mut c_void, message_size, 0, 0);

            if msg_len == -1 {
                if errno() == EINTR {
                    Ok(None)
                } else {
                    Err("Failed to receive message".to_owned())
                }
            } else {
                Ok(Some(QueueData { message_len: msg_len as usize, message: resp.assume_init() }))
            }
        }
    }

    pub fn send_message(&self, queue_id: c_int) -> Result<(), String> {
        let res = unsafe {
            msgsnd(queue_id, &self.message as *const T as *const c_void, self.message_len, 0)
        };

        if res == -1 {
            Err("Failed to send message".to_string())
        } else {
            Ok(())
        }
    }

    pub fn message(&self) -> &T {
        &self.message
    }
}

impl QueueData<ResponseMessage> {
    pub fn message_len(&self) -> usize {
        self.message_len
    }

    pub fn message_type(&self) -> c_long {
        self.message.mtype
    }

    pub fn get_string(&self) -> String {
        let bytes = &self.message.data[0 .. self.message_len];
        String::from_utf8(bytes.iter().map(|b| *b).collect()).expect("Invalid string")
    }
}

impl QueueData<RequestMessage> {
    fn path_bytes(&self) -> &[u8] {
        let byte_count = self.message_len + offset_of!(RequestMessage, client_id) - offset_of!(RequestMessage, pathname);
        &self.message.pathname[0 .. byte_count]
    }

    pub fn get_str(&self) -> &str {
        str::from_utf8(&self.path_bytes()).expect("Invalid string")
    }
}

// file couldn't be opened
pub const RESP_MT_FAILURE: c_long = 1;

// message contains file data
pub const RESP_MT_DATA: c_long = 2;

// file data complete
pub const RESP_MT_END: c_long = 3;