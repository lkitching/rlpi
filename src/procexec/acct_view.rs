//listing 28-1 (page 592)
use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_void};
use std::convert::{TryFrom};

use libc::{exit, EXIT_SUCCESS, open, O_RDONLY, read, time_t, localtime, sysconf, _SC_CLK_TCK, uid_t};

use crate::libc::sys::acct::{acct, comp_t, AFORK, ASU, ACORE, AXSIG};
use crate::error_functions::{usage_err, err_exit};
use crate::users_groups::ugid_functions::{user_name_from_id};
use crate::util::{fmt_strftime};

fn compt_to_i64(ct: comp_t) -> i64 {
    const EXP_SIZE: comp_t = 3;
    const MANTISSA_SIZE: comp_t = 13;
    const MANTISSA_MASK: comp_t = (1 << MANTISSA_SIZE) - 1;
    
    let mantissa = (ct & MANTISSA_MASK) as i64;
    let exp = (ct >> MANTISSA_SIZE) & ((1 << EXP_SIZE) - 1);
    mantissa << (exp as i64 * 3)
}

fn read_acct(fd: c_int) -> Result<Option<acct>, ()> {
    unsafe {
	let mut acct = mem::MaybeUninit::<acct>::uninit();
	let num_read = read(fd, acct.as_mut_ptr() as *mut c_void, mem::size_of::<acct>());
	if num_read < 0 {
	    // read failed
	    Err(())
	}
	else if num_read == 0 {
	    Ok(None)
	} else if num_read as usize == mem::size_of::<acct>() {
	    Ok(Some(acct.assume_init()))
	} else {
	    // not an error but book treats it as one
	    Err(())
	}	
    }
}

pub fn main(args: &[String]) -> ! {
    if args.len() != 2 || args.len() == 2 && args[1] == "--help" {
	usage_err(&format!("{} file", args[0]));
    }

    let acct_file = unsafe {
	let path_s = CString::new(args[1].as_str()).expect("Failed to create CString");
	open(path_s.as_ptr(), O_RDONLY)
    };

    if acct_file == -1 {
	err_exit("open");
    }

    println!("command  flags     term.   user      start time        CPU    elapsed");
    println!("                  status                               time   time");
    
    loop {
	match read_acct(acct_file) {
	    Err(_) => {
		err_exit("read");
	    },
	    Ok(None) => {
		break;
	    },
	    Ok(Some(ac)) => {
		unsafe {
		    let cs = CStr::from_ptr(ac.ac_comm.as_ptr());
		    print!("{} ", cs.to_str().expect("Failed to convert CStr"));
		}
		print!("{}{}{}{}",
		       if ac.ac_flag & AFORK == AFORK { "F" } else { "-" },
		       if ac.ac_flag & ASU == ASU { "S" } else { "-" },
		       if ac.ac_flag & AXSIG == AXSIG { "X" } else { "-" },
		       if ac.ac_flag & ACORE == ACORE { "C" } else { "-" });
		
		print!(" {}  ", ac.ac_exitcode);

		let user_name = user_name_from_id(ac.ac_uid as uid_t);
		print!("{} ", user_name.as_ref().map(|s| s.as_str()).unwrap_or("???"));

		match time_t::try_from(ac.ac_btime) {
		    Err(_) => {
			print!("???Unknown time???  ");
		    },
		    Ok(t) => {
			let loc = unsafe { localtime(&t) };
			if loc.is_null() {
			    print!("???Unknown time???  ");
			} else {
			    print!("{} ", fmt_strftime("%Y-%m-%d %T ", & unsafe { *loc }).expect("Failed to format time"));
			}
		    }
		}

		let clk_tck = unsafe { sysconf(_SC_CLK_TCK) };

		print!("{:5.2} {:7.2} ",
		       compt_to_i64(ac.ac_utime) + compt_to_i64(ac.ac_stime) / clk_tck,
		       compt_to_i64(ac.ac_etime) + clk_tck);

		println!("");
	    }

	}
    }    

    unsafe { exit(EXIT_SUCCESS); }
}
