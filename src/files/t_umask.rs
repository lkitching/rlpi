//listing 15-5 (page 302)
use std::ffi::{CString};
use std::mem::{MaybeUninit};

use libc::{exit, EXIT_SUCCESS, S_IRGRP, S_IWGRP, S_IXGRP, S_IWOTH, S_IXOTH, open, O_RDWR, O_CREAT, O_EXCL,
           mkdir, umask, S_IRUSR, S_IWUSR, S_IRWXU, S_IRWXG, S_IRWXO, unlink, stat, rmdir};

use rlpi::error_functions::{err_exit, err_msg};
use rlpi::files::file_perms::{file_perm_str};

pub fn main() {
    let umask_setting = S_IWGRP | S_IXGRP | S_IWOTH | S_IXOTH;

    let file_perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP;

    unsafe { umask(umask_setting); }

    let my_file = "myfile";
    let my_file_s = CString::new(my_file).expect("Failed to create CString");

    let fd = unsafe { open(my_file_s.as_ptr(), O_RDWR | O_CREAT | O_EXCL, file_perms) };
    if fd == -1 {
        err_exit(&format!("open-{}", my_file));
    }

    let my_dir = "mydir";
    let my_dir_s = CString::new(my_dir).expect("Failed to create CString");
    let dir_perms = S_IRWXU | S_IRWXG | S_IRWXO;

    if unsafe { mkdir(my_dir_s.as_ptr(), dir_perms) } == -1 {
        err_exit(&format!("mkdir-{}", my_dir));
    }

    let u = unsafe { umask(0) };    //retrieves (and clears) umask value

    let mut sb = MaybeUninit::uninit();
    if unsafe { stat(my_file_s.as_ptr(), sb.as_mut_ptr()) } == -1 {
        err_exit(&format!("stat-{}", my_file));
    }

    let mut sb = unsafe { sb.assume_init() };

    println!("Requested file perms:\t{}", file_perm_str(file_perms, false));
    println!("Process umask:\t{}", file_perm_str(u, false));
    println!("Actual file perms:\t{}", file_perm_str(sb.st_mode, false));

    if unsafe { stat(my_dir_s.as_ptr(), &mut sb) } == -1 {
        err_exit(&format!("stat-{}", my_dir));
    }

    println!("Requested dir. perms:\t{}", file_perm_str(dir_perms, false));
    println!("Process umask:\t{}", file_perm_str(u, false));
    println!("Actual dir. perms:\t{}", file_perm_str(sb.st_mode, false));

    if unsafe { unlink(my_file_s.as_ptr()) } == -1 {
        err_msg(&format!("unlink-{}", my_file));
    }

    if unsafe { rmdir(my_dir_s.as_ptr()) } == -1 {
        err_msg(&format!("rmdir-{}", my_dir));
    }

    unsafe { exit(EXIT_SUCCESS); }
}
