//! ::statvfs implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/sysstatvfs.h.html

use crate::unix::{
    header::fcntl::O_PATH,
    platform,
};

//pub const ST_RDONLY
//pub const ST_NOSUID

// #[no_mangle]
// pub extern "C" fn fstatvfs(fildes: ::c_int, buf: *mut ::statvfs) -> ::c_int {
//     platform::pal::fstatvfs(fildes, buf)
// }

#[no_mangle]
pub unsafe extern "C" fn statvfs(file: *const ::c_char, buf: *mut ::statvfs) -> ::c_int {
    let fd = platform::pal::open(file as *const ::c_char, O_PATH, 0);
    if fd < 0 {
        return -1;
    }

    let res = platform::pal::fstatvfs(fd, buf);

    platform::pal::close(fd);

    res
}
