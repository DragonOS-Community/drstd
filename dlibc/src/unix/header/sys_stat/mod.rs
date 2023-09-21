//! ::stat implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/sysstat.h.html

use crate::unix::{
    header::fcntl::{O_NOFOLLOW, O_PATH},
};



use crate::unix::platform;

pub const S_IFMT: ::c_int = 0o0_170_000;

pub const S_IFDIR: ::c_int = 0o040_000;
pub const S_IFCHR: ::c_int = 0o020_000;
pub const S_IFBLK: ::c_int = 0o060_000;
pub const S_IFREG: ::c_int = 0o100_000;
pub const S_IFIFO: ::c_int = 0o010_000;
pub const S_IFLNK: ::c_int = 0o120_000;
pub const S_IFSOCK: ::c_int = 0o140_000;

pub const S_IRWXU: ::c_int = 0o0_700;
pub const S_IRUSR: ::c_int = 0o0_400;
pub const S_IWUSR: ::c_int = 0o0_200;
pub const S_IXUSR: ::c_int = 0o0_100;

pub const S_IRWXG: ::c_int = 0o0_070;
pub const S_IRGRP: ::c_int = 0o0_040;
pub const S_IWGRP: ::c_int = 0o0_020;
pub const S_IXGRP: ::c_int = 0o0_010;

pub const S_IRWXO: ::c_int = 0o0_007;
pub const S_IROTH: ::c_int = 0o0_004;
pub const S_IWOTH: ::c_int = 0o0_002;
pub const S_IXOTH: ::c_int = 0o0_001;
pub const S_ISUID: ::c_int = 0o4_000;
pub const S_ISGID: ::c_int = 0o2_000;
pub const S_ISVTX: ::c_int = 0o1_000;

// #[no_mangle]
// pub unsafe extern "C" fn chmod(path: *const ::c_char, mode: ::mode_t) -> ::c_int {
//     platform::pal::chmod(path, mode)
// }

// #[no_mangle]
// pub extern "C" fn fchmod(fildes: ::c_int, mode: ::mode_t) -> ::c_int {
//     platform::pal::fchmod(fildes, mode)
// }

// #[no_mangle]
// pub extern "C" fn fstat(fildes: ::c_int, buf: *mut ::stat) -> ::c_int {
//     platform::pal::fstat(fildes, buf)
// }

#[no_mangle]
pub unsafe extern "C" fn __fxstat(_ver: ::c_int, fildes: ::c_int, buf: *mut ::stat) -> ::c_int {
    ::fstat(fildes, buf)
}

// #[no_mangle]
// pub extern "C" fn futimens(fd: ::c_int, times: *const timespec) -> ::c_int {
//     unsafe{platform::pal::futimens(fd, times)}
// }

#[no_mangle]
pub unsafe extern "C" fn lstat(path: *const ::c_char, buf: *mut ::stat) -> ::c_int {
    let fd = platform::pal::open(path, O_PATH | O_NOFOLLOW, 0);
    if fd < 0 {
        return -1;
    }

    let res = platform::pal::fstat(fd, buf);

    platform::pal::close(fd);

    res
}

// #[no_mangle]
// pub unsafe extern "C" fn mkdir(path: *const ::c_char, mode: ::mode_t) -> ::c_int {
//     platform::pal::mkdir(path, mode)
// }

// #[no_mangle]
// pub unsafe extern "C" fn mkfifo(path: *const ::c_char, mode: ::mode_t) -> ::c_int {
//     platform::pal::mkfifo(path, mode)
// }

// #[no_mangle]
// pub extern "C" fn mknod(path: *const ::c_char, mode: ::mode_t, dev: ::dev_t) -> ::c_int {
//     unimplemented!();
// }

#[no_mangle]
pub unsafe extern "C" fn stat(file: *const ::c_char, buf: *mut ::stat) -> ::c_int {
    let fd = platform::pal::open(file, O_PATH, 0);
    if fd < 0 {
        return -1;
    }

    let res = platform::pal::fstat(fd, buf);

    platform::pal::close(fd);

    res
}

// #[no_mangle]
// pub extern "C" fn umask(mask: ::mode_t) -> ::mode_t {
//     platform::pal::umask(mask)
// }
