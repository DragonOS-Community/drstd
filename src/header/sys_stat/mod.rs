use crate::platform::types::*;

use super::time::timespec;

#[repr(C)]
#[derive(Default)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_nlink: nlink_t,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,

    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,

    // Compared to glibc, our struct is for some reason 24 bytes too small.
    // Accessing atime works, so clearly the struct isn't incorrect...
    // This works.
    pub _pad: [c_char; 24],
}