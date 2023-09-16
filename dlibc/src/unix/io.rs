pub use core_io::*;

use crate::unix::platform;

pub fn last_os_error() -> Error {
    let errno = unsafe { platform::errno };
    Error::from_raw_os_error(errno)
}
