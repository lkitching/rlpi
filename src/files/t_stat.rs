//listing 15-1 (page 284)
use std::mem::{MaybeUninit};
use std::ffi::CString;

use libc::{exit, EXIT_SUCCESS, stat, lstat, S_IFREG, S_IFDIR, S_IFCHR, S_IFBLK, S_IFLNK, S_IFIFO, S_IFSOCK, S_IFMT,
           major, minor, S_ISUID, S_ISGID, S_ISVTX};
use clap::{Arg, App};

use rlpi::error_functions::{err_exit};
use rlpi::util::{read_str};
use rlpi::libc::time::{ctime};
use rlpi::files::file_perms::{file_perm_str};

fn display_stat_info(sb: &stat) {
    print!("File type:\t\t");
    match sb.st_mode & S_IFMT {
        S_IFREG => { println!("regular file"); }
        S_IFDIR => { println!("directory"); }
        S_IFCHR => { println!("character device"); }
        S_IFBLK => { println!("block device"); }
        S_IFLNK => { println!("symbolic (soft) link"); }
        S_IFIFO => { println!("FIFO or pipe"); }
        S_IFSOCK => { println!("socket"); }
        _ => { println!("unknown file type"); }
    }

    println!("Device containing i-node: major={}\tminor={}", unsafe { major(sb.st_dev) }, unsafe { minor(sb.st_dev) });
    println!("I-node number: {}", sb.st_ino);
    println!("Mode:\t\t{} ({})", sb.st_mode, file_perm_str(sb.st_mode, true));

    if sb.st_mode & (S_ISUID | S_ISGID | S_ISVTX) > 0 {
        println!("special bits set:\t\t {}{}{}",
                 if sb.st_mode & S_ISUID > 0 { "set-UID " } else { "" },
                 if sb.st_mode & S_ISGID > 0 { "set-GID " } else { "" },
                 if sb.st_mode & S_ISVTX > 0 { "sticky" } else { "" });
    }

    println!("Number of (hard) links:\t{}", sb.st_nlink);
    println!("Ownership:\t\tUID={}\tGID={}", sb.st_uid, sb.st_gid);

    let is_chr_dev = sb.st_mode & S_IFMT == S_IFCHR;
    let is_blk_dev = sb.st_mode & S_IFMT == S_IFBLK;
    if is_chr_dev || is_blk_dev {
        println!("Device number (st_rdev):\tmajor={}; minor={}", unsafe { major(sb.st_rdev) }, unsafe { minor(sb.st_rdev) });
    }

    println!("File size:\t\t{} bytes", sb.st_size);
    println!("Optimal I/O block size:\t{} bytes", sb.st_blksize);
    println!("512B blocks allocated:\t{}", sb.st_blocks);
    println!("Last file access:\t{}", unsafe { read_str(ctime(&sb.st_atime)) });
    println!("Last file modification:\t{}", unsafe { read_str(ctime(&sb.st_mtime)) });
    println!("Last status change: {}", unsafe { read_str(ctime(&sb.st_ctime)) });
}

pub fn main() {
    let matches = App::new("t_stat")
        .about("Displays information about a file or symbolic link")
        .arg(Arg::with_name("path")
            .help("The path to display")
            .index(1))
        .arg(Arg::with_name("link")
            .help("Path is a symbolic link"))
        .get_matches();

    let stat_link = matches.is_present("link");
    let path = matches.value_of("path").unwrap();

    let mut sb = MaybeUninit::<stat>::uninit();
    let path_s = CString::new(path).expect("Failed to create CString");

    if stat_link {
        if unsafe { lstat(path_s.as_ptr(), sb.as_mut_ptr()) } == -1 {
            err_exit("lstat");
        }
    } else {
        if unsafe { stat(path_s.as_ptr(), sb.as_mut_ptr()) } == -1 {
            err_exit("stat");
        }
    }

    let sb = unsafe { sb.assume_init() };
    display_stat_info(&sb);

    unsafe { exit(EXIT_SUCCESS); }
}
