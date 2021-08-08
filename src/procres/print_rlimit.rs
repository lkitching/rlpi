use std::fmt::{Display, Formatter, Error};
use std::convert::{From, Into};
use std::mem::{MaybeUninit};
use std::os::raw::{c_int};

use libc::{rlimit, getrlimit, RLIM_INFINITY, RLIM_SAVED_CUR, rlim_t, __rlimit_resource_t};

enum RLimit {
    Infinity,
    Unrepresentable,
    Limit(rlim_t)
}

impl Display for RLimit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	match *self {
	    Self::Infinity => { write!(f, "inifinity") },
	    Self::Unrepresentable => { write!(f, "unrepresentable") },
	    Self::Limit(i) => { write!(f, "{}", i) }
	}
    }
}

impl From<rlim_t> for RLimit {
    fn from(l: rlim_t) -> Self {
	if l == RLIM_INFINITY { Self::Infinity }
	else if l == RLIM_SAVED_CUR { Self::Unrepresentable }
	else { Self::Limit(l) }
    }
}

//listing 36-2 (page 757)
pub fn print_rlimit(msg: &str, resource: __rlimit_resource_t) -> Result<(), ()> {
    let rlim = unsafe {
	let mut rlim: MaybeUninit<rlimit> = MaybeUninit::uninit();
	if getrlimit(resource, rlim.as_mut_ptr()) == -1 {
	    return Err(());
	}
	rlim.assume_init()
    };

    let soft_limit = RLimit::from(rlim.rlim_cur);
    let hard_limit = RLimit::from(rlim.rlim_max);

    println!("{} soft={}; hard={}", msg, soft_limit, hard_limit);

    Ok(())
}
