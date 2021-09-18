// listing 47-8 (page 982)
use std::env;
use std::str::FromStr;
use regex::Regex;
use std::os::raw::{c_ushort, c_short, c_int};
use std::process;

use libc::{IPC_NOWAIT, sembuf, getpid, semop};
use rlpi::libc::sys::sem::{SEM_UNDO};
use rlpi::curr_time::curr_time;
use rlpi::error_functions::err_exit;

enum OpType {
    Zero, Plus(c_short), Minus(c_short)
}

struct Op {
    sem_num: c_ushort,
    op_type: OpType,
    flags: c_short
}

fn parse_flags(flags_str: &str) -> c_short {
    let mut flags: c_short = 0;
    for c in flags_str.chars() {
        match c {
            'n' => { flags |= IPC_NOWAIT as c_short },
            'u' => { flags |= SEM_UNDO },
            _ => { }
        }
    }
    flags
}

impl Op {
    fn to_sembuf(&self) -> sembuf {
        sembuf {
            sem_num: self.sem_num,
            sem_op: match self.op_type {
                OpType::Zero => { 0 },
                OpType:: Plus(v) => { v },
                OpType::Minus(v) => { -v }
            },
            sem_flg: self.flags
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let zre = Regex::new(r"^(\d+)=0(n?)$").unwrap();
        let vre = Regex::new(r"^(\d+)([+-])(\d+)([nu]{0,2})$").expect("Invalid regex");

        if let Some(captures) = zre.captures(s) {
            let sem_num: c_ushort = captures[1].parse().expect("Invalid semaphore number");
            let flags = parse_flags(&captures[2]);
            Ok(Op {sem_num, op_type: OpType::Zero, flags})
        } else if let Some(captures) = vre.captures(s) {
            let sem_num: c_ushort = captures[1].parse().expect("Invalid semaphore number");
            let op_s = &captures[2];
            let value = captures[3].parse().expect("Invalid value");

            let op_type = match op_s {
                "+" => { OpType::Plus(value) },
                "-" => { OpType::Minus(value) },
                _ => { panic!("should never happen?!") }
            };

            let flags = parse_flags(&captures[4]);
            Ok(Op {sem_num, op_type, flags})
        } else {
            Err("Failed to parse".to_string())
        }
    }
}

struct OpGroup {
    ops: Vec<Op>
}

impl FromStr for OpGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op_strs = s.split(",");
        let mut ops: Vec<Op> = Vec::new();
        for op_str in op_strs {
            let op = op_str.parse()?;
            ops.push(op);
        }
        Ok(OpGroup { ops })
    }
}

impl OpGroup {
    fn get_sembufs(&self) -> Vec<sembuf> {
        self.ops.iter().map(|o| o.to_sembuf()).collect()
    }
}

fn usage_error(prog_name: &str) -> ! {
    eprintln!("Usage: {} semid op[,op[ ...] ...", prog_name);
    eprintln!("'op' is either: <sem#>{{+|-}}<value>[n][u]");
    eprintln!("            or: <sem#>=0[n]");
    eprintln!("    \"n\" means include IPC_NOWAIT in 'op'");
    eprintln!("    \"u\" means include SEM_UNDO in 'op'");
    eprintln!("The operations in each argument are performed in a single semop() call");
    eprintln!("e.g.: {} 12345 0+1,1-2un", prog_name);
    eprintln!("      {} 12345 0=0n 1+1,2-1u 1=0", prog_name);
    process::exit(1);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        usage_error(args[0].as_str());
    }

    let sem_id: c_int = args[1].parse().expect("Invalid semaphore id");
    let ops: Vec<OpGroup> = args[2 ..].iter().map(|s| s.parse().expect("Invalid operation")).collect();

    let pid = unsafe { getpid() };

    for (index, group) in ops.iter().enumerate() {
        let mut bufs = group.get_sembufs();

        println!("{}, {}: about to semop()", pid, curr_time("%T"));

        if unsafe { semop(sem_id, bufs.as_mut_ptr(), bufs.len()) } == -1 {
            err_exit(&format!("semop (PID={})", pid));
        }

        println!("{}, {}: semop() completed [{}]", pid, curr_time("%T"), index);
    }
}