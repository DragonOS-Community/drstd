//! sys/utsname implementation, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/sysutsname.h.html

use crate::unix::platform;

pub const UTSLENGTH: usize = 65;

// #[no_mangle]
// pub unsafe extern "C" fn uname(uts: *mut ::utsname) -> ::c_int {
//     platform::pal::uname(uts)
// }
