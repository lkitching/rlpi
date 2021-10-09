//listing 50-2 (page 1052)
use std::{ptr, env};
use std::os::raw::{c_void, c_uchar};

use libc::{sysconf, _SC_PAGE_SIZE, size_t, mincore, mmap, MAP_FAILED, mlock, PROT_READ, MAP_SHARED, MAP_ANONYMOUS};
use rlpi::error_functions::{err_exit, usage_err};

fn display_mincore(addr: *mut c_void, page_size: size_t, length: size_t) {
    let num_pages = (length + page_size - 1) / page_size;

    let mut vec: Vec<c_uchar> = Vec::with_capacity(num_pages);

    unsafe {
        if mincore(addr, length, vec.as_mut_ptr()) == -1 {
            err_exit("mincore");
        }
        vec.set_len(num_pages);
    }

    for j in 0..num_pages {
        if j % 64 == 0 {
            print!("{}{:p}: ", if j == 0 { "" } else { "\n" }, unsafe { addr.add(j * page_size) });
        }
        print!("{}", if vec[j] & 1 == 1 { '*' } else { '.' });
    }
    println!();
}
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 || args[1] == "--help" {
        usage_err(&format!("{} num-pages lock-page-step lock-page-len", args[0]));
    }

    let page_size = unsafe { sysconf(_SC_PAGE_SIZE) };
    if page_size == -1 {
        err_exit("sysconf(_SC_PAGE_SIZE)");
    }
    let page_size = page_size as size_t;

    let len = args[1].parse::<size_t>().expect("Invalid page size") * page_size;
    let step_size = args[2].parse::<size_t>().expect("Invalid step size") * page_size;
    let lock_len = args[3].parse::<size_t>().expect("INvalid lock page length") * page_size;

    let addr = unsafe { mmap(ptr::null_mut(), len, PROT_READ, MAP_SHARED | MAP_ANONYMOUS, -1, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }

    println!("Allocated {} bytes starting at {:p}", len, addr);
    println!("Before mlock():");
    display_mincore(addr, page_size, len);

    // lock pages specified on the command-line into to memory
    for j in (0 .. len + 1).step_by(step_size) {
        if unsafe { mlock(addr.add(j), lock_len) } == -1 {
            err_exit("mlock");
        }
    }

    println!("After mlock:");
    display_mincore(addr, page_size, len);
}