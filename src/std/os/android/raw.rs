//! Android-specific raw type definitions

#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
            the standard library, the `libc` crate on \
            crates.io should be used instead for the correct \
            definitions"
)]
#![allow(deprecated)]

use crate::std::os::raw::c_long;

pub type pthread_t = c_long;

#[doc(inline)]
pub use self::arch::{blkcnt_t, blksize_t, dev_t, ino_t, mode_t, nlink_t, off_t, stat, time_t};

#[cfg(any(target_arch = "arm", target_arch = "x86"))]
mod arch {
    use crate::std::os::raw::{c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong};
    use crate::std::os::unix::raw::{gid_t, uid_t};

        pub type dev_t = u32;
        pub type mode_t = c_uint;

        pub type blkcnt_t = c_ulong;
        pub type blksize_t = c_ulong;
        pub type ino_t = c_ulong;
        pub type nlink_t = u32;
        pub type off_t = i32;
        pub type time_t = c_long;

    #[repr(C)]
    #[derive(Clone)]
        pub struct stat {
                pub st_dev: c_ulonglong,
                pub __pad0: [c_uchar; 4],
                pub __st_ino: u32,
                pub st_mode: c_uint,
                pub st_nlink: c_uint,
                pub st_uid: uid_t,
                pub st_gid: gid_t,
                pub st_rdev: c_ulonglong,
                pub __pad3: [c_uchar; 4],
                pub st_size: c_longlong,
                pub st_blksize: u32,
                pub st_blocks: c_ulonglong,

                pub st_atime: time_t,
                pub st_atime_nsec: c_long,
                pub st_mtime: time_t,
                pub st_mtime_nsec: c_long,
                pub st_ctime: time_t,
                pub st_ctime_nsec: c_long,

                pub st_ino: c_ulonglong,
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
mod arch {
    use crate::std::os::raw::{c_int, c_long, c_uint, c_ulong};
    use crate::std::os::unix::raw::{gid_t, uid_t};

        pub type dev_t = u64;
        pub type mode_t = c_uint;

        pub type blkcnt_t = c_ulong;
        pub type blksize_t = c_ulong;
        pub type ino_t = c_ulong;
        pub type nlink_t = u32;
        pub type off_t = i64;
        pub type time_t = c_long;

    #[repr(C)]
    #[derive(Clone)]
        pub struct stat {
                pub st_dev: dev_t,
                pub st_ino: ino_t,
                pub st_mode: mode_t,
                pub st_nlink: nlink_t,
                pub st_uid: uid_t,
                pub st_gid: gid_t,
                pub st_rdev: dev_t,
                pub __pad1: c_ulong,
                pub st_size: off_t,
                pub st_blksize: c_int,
                pub __pad2: c_int,
                pub st_blocks: c_long,

                pub st_atime: time_t,
                pub st_atime_nsec: c_long,
                pub st_mtime: time_t,
                pub st_mtime_nsec: c_long,
                pub st_ctime: time_t,
                pub st_ctime_nsec: c_long,

                pub __unused4: c_uint,
                pub __unused5: c_uint,
    }
}

#[cfg(target_arch = "x86_64")]
mod arch {
    use crate::std::os::raw::{c_long, c_uint, c_ulong};
    use crate::std::os::unix::raw::{gid_t, uid_t};

        pub type dev_t = u64;
        pub type mode_t = c_uint;

        pub type blkcnt_t = c_ulong;
        pub type blksize_t = c_ulong;
        pub type ino_t = c_ulong;
        pub type nlink_t = u32;
        pub type off_t = i64;
        pub type time_t = c_long;

    #[repr(C)]
    #[derive(Clone)]
        pub struct stat {
                pub st_dev: dev_t,
                pub st_ino: ino_t,
                pub st_nlink: c_ulong,
                pub st_mode: c_uint,
                pub st_uid: uid_t,
                pub st_gid: gid_t,
                pub __pad0: c_uint,
                pub st_rdev: dev_t,
                pub st_size: off_t,
                pub st_blksize: c_long,
                pub st_blocks: c_long,

                pub st_atime: time_t,
                pub st_atime_nsec: c_long,
                pub st_mtime: time_t,
                pub st_mtime_nsec: c_long,
                pub st_ctime: time_t,
                pub st_ctime_nsec: c_long,

                pub __pad3: [c_long; 3],
    }
}
