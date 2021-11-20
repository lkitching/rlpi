//listing 7.1 (page 142)
use std::fmt::{Display};
use std::str::{FromStr};
use libc::{exit, EXIT_SUCCESS, sbrk, malloc, free};
use crate::error_functions::{usage_err, err_exit, cmd_line_err};

fn parse_gt<T: FromStr + Display + PartialOrd>(s: &str, min_exclusive: T, arg_name: &str) -> T {
    match T::from_str(s) {
        Ok(r) => {
            if r > min_exclusive {
                r
            } else {
                err_exit(&format!("{} must be greater than {}", arg_name, min_exclusive));
            }
        }
        Err(_) => {
            err_exit(&format!("{} is not a valid number: {}", arg_name, s));
        }
    }
}

fn parse_gt_or<T: FromStr + Display + PartialOrd>(args: &[String],
                                                  index: usize,
                                                  min_exclusive: T,
                                                  arg_name: &str,
                                                  default: T) -> T {
    if index < args.len() {
        parse_gt(args[index].as_str(), min_exclusive, arg_name)
    } else {
        default
    }
}

pub fn main(args: &[String]) -> ! {
    if args.len() < 3 {
        usage_err(&format!("{} num-allocs block-size [step] [min] [max]\n", args[0]));
    }

    let num_allocs = parse_gt(args[1].as_str(), 0, "num-allocs");
    let block_size = parse_gt(args[2].as_str(), 0, "block-size");

    let free_step = parse_gt_or(args, 3, 0, "step", 1);
    let free_min = parse_gt_or(args, 4, 0, "min", 1);
    let free_max = parse_gt_or(args, 5, 0, "max", num_allocs);

    if free_max > num_allocs {
        cmd_line_err("free-max > num-allocs");
    }

    println!("Initial program break:\t\t\t{:p}", unsafe { sbrk(0) });

    println!("Allocating {} * {} bytes", num_allocs, block_size);

    let mut pointers = Vec::new();

    for _i in 0..num_allocs {
        let p = unsafe { malloc(block_size) };
        if p.is_null() {
            err_exit("malloc");
        } else {
            pointers.push(p);
        }
    }

    println!("Program break is now:\t\t\t{:p}", unsafe { sbrk(0) });
    println!("Freeing blocks from {} to {} in steps of {}", free_min, free_max, free_step);

    for i in (free_min..free_max).step_by(free_step) {
        unsafe { free(pointers[i]); }
    }

    println!("After free(), program break is:\t\t{:p}", unsafe { sbrk(0) });

    unsafe { exit(EXIT_SUCCESS); }
}
