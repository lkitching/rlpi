use std::iter::{Iterator};
use std::ffi::{CStr, CString};
use libc::{uid_t, gid_t, passwd, getpwuid, getpwnam, getgrgid, getgrnam, getpwent, exit, EXIT_SUCCESS};
use crate::error_functions::{usage_err};

fn read_user_name(p: *mut passwd) -> String {
    let cs = unsafe { CStr::from_ptr((*p).pw_name).to_str().expect("Cannot create str") };
    cs.to_owned()
}

pub fn user_name_from_id(uid: uid_t) -> Option<String> {
    let p = unsafe { getpwuid(uid) };
    if p.is_null() {
	None
    } else {
	let cs = unsafe { CStr::from_ptr((*p).pw_name).to_str().expect("Cannot create str") };
	Some(cs.to_owned())
    }
}

pub fn user_id_from_name(name: &str) -> Option<uid_t> {
    if let Ok(id) = name.parse::<uid_t>() {
	return Some(id)
    }

    let cs = CString::new(name).expect("Failed to create CString");
    let pwd = unsafe { getpwnam(cs.as_ptr()) };
    if pwd.is_null() {
	None
    } else {
	let id = unsafe { (*pwd).pw_uid };
	Some(id)
    }
}

pub fn group_name_from_id(gid: gid_t) -> Option<String> {
    let gp = unsafe { getgrgid(gid) };
    if gp.is_null() {
	None
    } else {
	let cs = unsafe { CStr::from_ptr((*gp).gr_name).to_str().expect("Failed to create str") };
	Some(cs.to_owned())
    }
}

pub fn group_id_from_name(name: &str) -> Option<gid_t> {
    if let Ok(id) = name.parse::<gid_t>() {
	return Some(id);
    }

    let cs = CString::new(name).expect("Failed to create CString");
    let grp = unsafe { getgrnam(cs.as_ptr()) };

    if grp.is_null() {
	None
    } else {
	let id = unsafe { (*grp).gr_gid };
	Some(id)
    }
}

struct PWIterator {
    ended: bool
}

impl Iterator for PWIterator {
    type Item = *mut passwd;
    fn next(&mut self) -> Option<Self::Item> {
	if self.ended { return None; }

	let p = unsafe { getpwent() };
	if p.is_null() {
	    self.ended = true;
	    None
	} else {
	    Some(p)
	}	    
    }
}

pub fn main(args: &[String]) -> ! {
    let mut i = PWIterator { ended: false };
    for p in i {	
	unsafe {
	    println!("id: {}", (*p).pw_uid);
	    println!("name: {}", read_user_name(p));
	}
    }
    unsafe { exit(EXIT_SUCCESS) };
}
