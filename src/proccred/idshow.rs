//based on listing 9.1 (page 182)
use std::mem::MaybeUninit;
use libc::{uid_t, exit, EXIT_SUCCESS, getresuid, gid_t, getresgid, setfsuid, setfsgid, getgroups, c_int};
use rlpi::error_functions::{err_exit};
use rlpi::users_groups::ugid_functions::{user_name_from_id, group_name_from_id};

fn get_uids() -> (uid_t, uid_t, uid_t) {
    let mut real = MaybeUninit::<uid_t>::uninit();
    let mut effective = MaybeUninit::<uid_t>::uninit();
    let mut saved = MaybeUninit::<uid_t>::uninit();

    unsafe {
        let r = getresuid(real.as_mut_ptr(), effective.as_mut_ptr(), saved.as_mut_ptr());
        if r == -1 {
            err_exit("getresuid");
        }
        (real.assume_init(), effective.assume_init(), saved.assume_init())
    }
}

fn get_gids() -> (gid_t, gid_t, gid_t) {
    let mut real = MaybeUninit::<uid_t>::uninit();
    let mut effective = MaybeUninit::<uid_t>::uninit();
    let mut saved = MaybeUninit::<uid_t>::uninit();

    unsafe {
        let r = getresgid(real.as_mut_ptr(), effective.as_mut_ptr(), saved.as_mut_ptr());
        if r == -1 {
            err_exit("getresgid");
        }
        (real.assume_init(), effective.assume_init(), saved.assume_init())
    }
}

fn get_groups() -> Vec<gid_t> {
    let num_groups = {
        let mut tmp = Vec::new();
        let num_groups = unsafe { getgroups(0, tmp.as_mut_ptr()) };

        if num_groups == -1 {
            panic!("Failed to get number of groups");
        } else {
            num_groups as usize
        }
    };

    let mut groups = Vec::with_capacity(num_groups);
    let r = unsafe { getgroups(num_groups as c_int, groups.as_mut_ptr()) };
    if r == -1 {
        panic!("Failed to get groups");
    } else if r as usize == num_groups {
        unsafe {
            groups.set_len(r as usize);
        }
        groups
    } else {
        panic!("Unexpected number of groups: expected {}, got {}", num_groups, r);
    }
}

pub fn main() {
    let (ruid, euid, suid) = get_uids();
    let (rgid, egid, sgid) = get_gids();

    let real_user_name = user_name_from_id(ruid);
    let eff_user_name = user_name_from_id(euid);
    let saved_user_name = user_name_from_id(suid);


    //attempts to change the file-systme IDs are always ignored
    //for unprivileged processor but the following calls return the
    //current file-system IDs
    let fsuid = unsafe { setfsuid(0) as uid_t };
    let fsgid = unsafe { setfsgid(0) as gid_t };

    let fs_user_name = user_name_from_id(fsuid);

    println!("UID: real={} ({}); eff={} ({}); saved={} ({}); fs={} ({});",
             real_user_name.as_deref().unwrap_or("???"),
             ruid,
             eff_user_name.as_deref().unwrap_or("???"),
             euid,
             saved_user_name.as_deref().unwrap_or("???"),
             suid,
             fs_user_name.as_deref().unwrap_or("???"),
             fsuid);

    let real_group_name = group_name_from_id(rgid);
    let eff_group_name = group_name_from_id(egid);
    let saved_group_name = group_name_from_id(sgid);
    let fs_group_name = group_name_from_id(fsgid);

    println!("GID: real={} ({}); eff={} ({}); saved={} ({}); fs={} ({});",
             real_group_name.as_deref().unwrap_or("???"),
             rgid,
             eff_group_name.as_deref().unwrap_or("???"),
             egid,
             saved_group_name.as_deref().unwrap_or("???"),
             sgid,
             fs_group_name.as_deref().unwrap_or("???"),
             fsgid);

    let groups = get_groups();

    print!("Supplementary groups ({}): ", groups.len());
    for group_id in groups.iter() {
        let group_name = group_name_from_id(*group_id);
        print!("{} ({}) ", group_name.as_deref().unwrap_or("???"), group_id);
    }
    println!();

    unsafe { exit(EXIT_SUCCESS); }
}
