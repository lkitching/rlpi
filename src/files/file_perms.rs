//listing 15-4 (page 296)
use libc::{mode_t, S_IRUSR, S_IWUSR, S_IXUSR, S_IRGRP, S_IWGRP, S_IXGRP, S_IROTH, S_IWOTH, S_IXOTH,
           S_ISUID, S_ISGID, S_ISVTX};

fn is_set(flags: mode_t, flag: mode_t) -> bool {
    flags & flag == flag
}

pub fn file_perm_str(perm: mode_t, show_special: bool) -> String {
    format!("{}{}{}{}{}{}{}{}{}",
	    if is_set(perm, S_IRUSR) { 'r' } else { '-' },
	    if is_set(perm, S_IWUSR) { 'w' } else { '-' },
	    if is_set(perm, S_IXUSR) {
		if is_set(perm, S_ISUID) && show_special { 's' } else { 'x' }
	    }
	    else {
		if is_set(perm, S_ISUID) && show_special { 'S' } else { '-' }
	    },
	    if is_set(perm, S_IRGRP) { 'r' } else { '-' },
	    if is_set(perm, S_IWGRP) { 'w' } else { '-' },
	    if is_set(perm, S_IXGRP) {
		if is_set(perm, S_ISGID) && show_special { 's' } else { 'x' }
	    } else {
		if is_set(perm, S_ISGID) && show_special { 'S' } else { '-' }
	    },
	    if is_set(perm, S_IROTH) { 'r' } else { '-' },
	    if is_set(perm, S_IWOTH) { 'w' } else { '-' },
	    if is_set(perm, S_IXOTH) {
		if is_set(perm, S_ISVTX) && show_special { 't' } else { 'x' }
	    } else {
		if is_set(perm, S_ISVTX) && show_special { 'T' } else { '-' }
	    })
}
