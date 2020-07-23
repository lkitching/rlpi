use std::mem::ManuallyDrop;
use std::os::raw::{c_int, c_void};
use std::ffi::CString;

use crate::libc::{exit, ExitStatus};
use crate::error_functions::{usage_err, err_exit, cmd_line_err};
use crate::libc::sys::types::{off_t, size_t};
use crate::libc::ctype::{isprint};
use crate::libc::fcntl::{open, O_RDWR, O_CREAT};
use crate::libc::sys::stat::{S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH};
use crate::libc::unistd::{read, write, lseek, SEEK_SET};

fn can_print(b: u8) -> bool {
    let i = unsafe { isprint(b as c_int) };
    i != 0
}

pub fn main(args: &[String]) -> ! {
    if args.len() < 3 {
	usage_err(&format!("{} file {{r<length>|R<length>|w<string>|s<offset>}}+\n", args[0]));
    }

    let src_path = args[1].as_str();
    let csrc_path = CString::new(src_path).expect("Failed to create CString");

    let fd = unsafe { open(csrc_path.as_ptr(), O_RDWR | O_CREAT, S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH) };
    if fd == -1 {
	err_exit("open");
    }

    for cmd in args.iter().skip(2) {
	let mut chars = cmd.chars();
	match chars.next() {
	    Some('r') | Some('R') => {
		let offset: off_t = cmd[1..].parse().expect("Failed to parse offset");
		let mut buf = Vec::with_capacity(offset as usize);
		let num_read = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, offset as size_t) };

		if num_read == 0 {
		    println!("{}: end-of-file", cmd);
		} else {
		    print!("{}: ", cmd);
		    unsafe {
			let p = buf.as_mut_ptr();
			let buf = ManuallyDrop::new(buf);
			let read_buf = Vec::from_raw_parts(p, num_read as usize, offset as usize);

			//if command begins with 'r' display characters otherwise it starts with
			//'R' so display the bytes			
			let display_chars = cmd.chars().next().unwrap() == 'r';
			
			for b in read_buf.iter() {
			    if display_chars {
				if can_print(*b) {
				    print!("{}", char::from(*b));
				} else {
				    print!("?");
				}
			    } else {
				//display hex
				print!("{:02x} ", b);
			    }
			}

			println!("");
		    }
		}
	    },
	    Some('w') => {
		//write string at current offset
		let to_write = CString::new(&cmd[1..]).expect("Failed to create CString");
		let bytes_to_write = to_write.as_bytes();	//NOTE: don't write the null terminator!
		let num_written = unsafe { write(fd, bytes_to_write.as_ptr() as *const c_void, bytes_to_write.len()) };

		if num_written == -1 {
		    err_exit("write");
		}
		println!("{}: wrote {} bytes", cmd, num_written);
	    },
	    Some('s') => {
		//change the file offset
		let offset: off_t = cmd[1..].parse().expect("Failed to parse offset");
		let loc = unsafe { lseek(fd, offset, SEEK_SET) };
		if loc == -1 {
		    err_exit("lseek");
		}
		println!("{}: seek succeeded", cmd);
	    },
	    _ => {
		cmd_line_err(&format!("Argument must start with [rRws]: {}\n", cmd));
	    }
	}
    }

    unsafe { exit(ExitStatus::Success as c_int) };
}
