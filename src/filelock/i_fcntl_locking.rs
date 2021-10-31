// listing 55-2 (page 1130)
use std::{env, process};
use std::io::{self, Write, BufRead};
use std::ffi::{CString};
use std::os::raw::{c_short, c_int};

use libc::{open, O_RDWR, getpid, pid_t, flock, off_t, F_GETLK, F_SETLK, F_SETLKW, F_UNLCK, F_RDLCK, F_WRLCK, EAGAIN, EDEADLK, EACCES, fcntl,
           SEEK_SET, SEEK_CUR, SEEK_END};
use rlpi::error_functions::{usage_err, err_exit, err_msg};
use std::mem::MaybeUninit;
use rlpi::libc::errno;

fn display_cmd_fmt() {
    println!();
    println!("    Format: cmd lock start length [whence]");
    println!();
    println!();
    println!("   'cmd' is 'g' (GETLK), 's' (SETLK), or 'w' (SETLKW)");
    println!("   'lock' is 'r' (READ), 'w' (WRITE), or 'u' (UNLOCK)");
    println!("   'start' and 'length' specify byte range to lock");
    println!("   'whence' is 's' (SEEK_SET, default), 'c' (SEEK_CUR), or 'e' (SEEK_END)");
    println!();
}

fn write_prompt(pid: pid_t) {
    //prompt for locking command
    print!("PID={}> ", pid);
    io::stdout().flush();
}

struct LockInfo {
    cmd_type: c_int,
    lock_type: c_short,
    whence: c_short,
    start: off_t,
    len: off_t
}

enum Command {
    Help,
    Empty,
    Lock(LockInfo)
}

fn parse_command_type(cmd_type: &str) -> Result<c_int, String> {
    match cmd_type {
        "g" => { Ok(F_GETLK) },
        "s" => { Ok(F_SETLK) },
        "w" => { Ok(F_SETLKW) },
        _ => { Err("Command type must be one of g, s or w".to_string()) }
    }
}

fn parse_lock_type(lock_type: &str) -> Result<c_short, String> {
    match lock_type {
        "r" => { Ok(F_RDLCK as c_short) },
        "w" => { Ok(F_WRLCK as c_short) },
        "u" => { Ok(F_UNLCK as c_short) },
        _ => { Err("Lock type must be one of r, w, or u".to_string()) }
    }
}

fn parse_whence(whence: Option<&str>) -> Result<c_short, String> {
    match whence {
        None => { Ok(SEEK_SET as c_short) },
        Some("c") => { Ok(SEEK_CUR as c_short) },
        Some("e") => { Ok(SEEK_END as c_short) },
        _ => { Err("whence must be c or e".to_string()) }
    }
}

fn parse_command(cmd: &str) -> Result<Command, String> {
    match cmd.chars().next() {
        None => { Ok(Command::Empty) },
        Some('?') => { Ok(Command::Help) },
        _ => {
            let cmd_elements: Vec<&str> = cmd.split(" ").collect();
            if cmd_elements.len() < 4 {
                Err("Invalid command: command lock start length [whence] required".to_string())
            } else {
                let cmd_type = parse_command_type(cmd_elements[0])?;
                let lock_type = parse_lock_type(cmd_elements[1])?;
                let start = cmd_elements[2].parse().map_err(|_| "Invalid start".to_string())?;
                let len: off_t = cmd_elements[3].parse().map_err(|_| "Invalid length".to_string())?;
                let whence = parse_whence(cmd_elements.get(4).map(|s| *s))?;
                Ok(Command::Lock(LockInfo { cmd_type, lock_type, start, len, whence }))
            }
        }
    }
}

fn exec_command(fd: c_int, info: &LockInfo) -> (c_int, flock) {
    unsafe {
        let mut fl: MaybeUninit<flock> = MaybeUninit::uninit();
        (&mut *fl.as_mut_ptr()).l_type = info.lock_type;
        (&mut *fl.as_mut_ptr()).l_whence = info.whence;
        (&mut *fl.as_mut_ptr()).l_start = info.start;
        (&mut *fl.as_mut_ptr()).l_len = info.len;

        let status = fcntl(fd, info.cmd_type, fl.as_mut_ptr());
        (status, fl.assume_init())
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} file", args[0]));
    }

    let fd = unsafe {
        let file_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        open(file_s.as_ptr(), O_RDWR)
    };

    if fd == -1 {
        err_exit(&format!("open ({})", args[1]));
    }

    println!("Enter ? for help");
    let pid = unsafe { getpid() };

     loop {
         write_prompt(pid);
         let line = {
             let mut line_buf = String::new();
             let bytes_read = io::stdin().lock().read_line(&mut line_buf);
             match bytes_read {
                 Err(_) => {
                     eprintln!("Failed to read from stdin");
                     process::exit(1);
                 },
                 Ok(0) => {
                     // EOF
                     break;
                 }
                 Ok(_) => {
                     line_buf.trim().to_string()
                 }
             }
         };

         match parse_command(&line) {
             Err(msg) => {
                 println!("Invalid command: {}", msg);
                 continue;
             },
             Ok(Command::Empty) => {
                 // skip empty commands
                 continue;
             }
             Ok(Command::Help) => {
                 display_cmd_fmt();
                 continue;
             },
             Ok(Command::Lock(info)) => {
                 let (status, fl) = exec_command(fd, &info);

                 // check outcome of request
                 if info.cmd_type == F_GETLK {
                     if status == -1 {
                         err_msg("fcntl - F_GETLK");
                     } else {
                         if info.lock_type as c_int == F_UNLCK {
                             println!("[PID={}] Lock can be placed", pid);
                         } else {
                             // locked by someone else
                             println!("[PID={}] Denied by {} lock on {}: {} (held by PID {})",
                                 pid,
                                 if info.lock_type as c_int == F_RDLCK { "READ" } else { "WRITE" },
                                 info.start,
                                 info.len,
                                 fl.l_pid
                             );
                         }
                     }
                 } else {
                     // F_SETLK, F_SETLKW
                     if status == 0 {
                         println!("[PID={}] {}", pid, if info.lock_type as c_int == F_UNLCK { "unlocked" } else { "got lock" });
                     } else {
                         let e = errno();
                         if e == EAGAIN || e == EACCES {
                             println!("[PID={}] failed (incompatible lock)", pid);
                         } else if e == EDEADLK {
                             println!("[PID={}] failed (deadlock)", pid);
                         } else {
                             err_msg("fcntl - F_SETLK(W)");
                         }
                     }
                 }
             }
         }
    }
}