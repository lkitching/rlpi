use std::env;
use std::os::raw::{c_int};
use std::io;
use std::io::{stdout, stderr, Write};

use crate::ename;
use crate::libc::{strerror, read_char_ptr,abort,_exit,exit};

fn str_error(errnum: c_int) -> String {
    let chars = unsafe { strerror(errnum) };
    read_char_ptr(chars)
}

//see https://www.gnu.org/software/libc/manual/html_node/Exit-Status.html
enum ExitStatus {
    Success = 0,
    Failure = 1
}

fn should_dump_core() -> bool {
    match env::var("EF_DUMPCORE") {
	Ok(s) => { s.len() > 0 }
	_ => { false }
    }
}

pub fn terminate(use_exit3: bool) -> ! {
    if should_dump_core() {
	unsafe { abort() }
    } else if use_exit3 {
	unsafe { exit(ExitStatus::Failure as c_int) }
    } else {
	unsafe { _exit(ExitStatus::Failure as c_int) }
    }
}

pub fn output_error(use_err: bool, err: c_int, flush_stdout: bool, msg: &str) {    
    let err_text = if use_err {
	let name_str = ename::error_name(err);
	format!(" [{} {}]", name_str, str_error(err))
    } else {
	":".to_owned()
    };

    let err_msg = format!("ERROR{} {}", err_text, msg);

    if flush_stdout {
	//NOTE: C version uses fflush
	io::stdout().flush().expect("failed to flush stdout");
    } 

    //NOTE: C version uses fflush
    eprintln!("{}", err_msg);
    io::stderr().flush().expect("failed to flush stderr");
}
