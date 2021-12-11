use std::os::raw::{c_int, c_void};
use libc::{read};

// port number for server
pub static PORT_NUM: &str = "50000";

// size of string able to hold largest integer (including terminating \n)
pub const INT_LEN: usize = 30;

pub fn read_line(fd: c_int, max_line: usize) -> Result<String, String> {
    let mut buf: [u8; 1024] = [0; 1024];
    let mut total_bytes = 0;
    let mut result = String::new();

    while total_bytes < max_line {
        let bytes_read = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        if bytes_read == -1 {
            return Err("Failed to read".to_owned());
        }

        for i in 0 .. bytes_read as usize {
            let c = buf[i] as char;
            if c == '\n' {
                return Ok(result)
            } else {
                result.push(c);
                total_bytes += 1;
                if total_bytes >= max_line {
                    return Ok(result);
                }
            }
        }
    }
    Ok(result)
}
