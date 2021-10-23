// listing 54-1 (page 1110)
use std::os::raw::{c_int};
use std::ffi::{CString};
use std::ptr;

use clap::{App, Arg, ArgMatches};
use libc::{O_RDWR, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR, mmap, ftruncate, shm_open, mode_t, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, size_t, off_t};
use rlpi::error_functions::err_exit;

fn get_flags(matches: &ArgMatches) -> c_int {
    let mut flags = O_RDWR;
    if matches.is_present("create") { flags |= O_CREAT }
    if matches.is_present("exclusive") { flags |= O_EXCL }
    flags
}

pub fn main() {
    let args = App::new("pshm_create")
        .about("Creates a POSIX shared memory mapping")
        .version("1.0")
        .arg(Arg::with_name("create")
            .required(false)
            .short("c")
            .help("Create shared memory (O_CREAT)"))
        .arg(Arg::with_name("exclusive")
            .required(false)
            .short("x")
            .help("Create exclusively (O_EXCL)"))
        .arg(Arg::with_name("name")
            .required(true)
            .index(1)
            .help("Name of the region to open"))
        .arg(Arg::with_name("size")
            .required(true)
            .index(2)
            .help("Size of the region"))
        .arg(Arg::with_name("perms")
            .required(false)
            .index(3)
            .help("Octal permissions for the region"));

    let matches = args.get_matches();
    let size: size_t = matches.value_of("size").unwrap().parse().expect("Invalid size");
    let flags = get_flags(&matches);
    let perms = matches.value_of("perms").map_or(S_IRUSR | S_IWUSR, |p| mode_t::from_str_radix(p, 8).expect("Invalid octal permissions"));

    // create shared memory object and set its size
    let fd = unsafe {
        let name_s = CString::new(matches.value_of("name").unwrap()).expect("Failed to create CString");
        shm_open(name_s.as_ptr(), flags, perms)
    };

    if fd == -1 {
        err_exit("shm_open");
    }

    if unsafe { ftruncate(fd, size as off_t) } == -1 {
        err_exit("ftruncate");
    }

    // map shared memory object
    let addr = unsafe { mmap(ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }
}