use crate::std::fs::Metadata;
use crate::std::sys_common::AsInner;
use dlibc;

#[allow(deprecated)]
use crate::std::os::haiku::raw;

/// OS-specific extensions to [`fs::Metadata`].
///
/// [`fs::Metadata`]: crate::std::fs::Metadata
pub trait MetadataExt {
    /// Gain a reference to the underlying `stat` structure which contains
    /// the raw information returned by the OS.
    ///
    /// The contents of the returned `stat` are **not** consistent across
    /// Unix platforms. The `os::unix::fs::MetadataExt` trait contains the
    /// cross-Unix abstractions contained within the raw stat.
    #[deprecated(
        since = "1.8.0",
        note = "deprecated in favor of the accessor \
                methods of this trait"
    )]
    #[allow(deprecated)]
    fn as_raw_stat(&self) -> &raw::stat;

    fn st_dev(&self) -> u64;
    fn st_ino(&self) -> u64;
    fn st_mode(&self) -> u32;
    fn st_nlink(&self) -> u64;
    fn st_uid(&self) -> u32;
    fn st_gid(&self) -> u32;
    fn st_rdev(&self) -> u64;
    fn st_size(&self) -> u64;
    fn st_atime(&self) -> i64;
    fn st_atime_nsec(&self) -> i64;
    fn st_mtime(&self) -> i64;
    fn st_mtime_nsec(&self) -> i64;
    fn st_ctime(&self) -> i64;
    fn st_ctime_nsec(&self) -> i64;
    fn st_crtime(&self) -> i64;
    fn st_crtime_nsec(&self) -> i64;
    fn st_blksize(&self) -> u64;
    fn st_blocks(&self) -> u64;
}

impl MetadataExt for Metadata {
    #[allow(deprecated)]
    fn as_raw_stat(&self) -> &raw::stat {
        unsafe { &*(self.as_inner().as_inner() as *const dlibc::stat as *const raw::stat) }
    }
    fn st_dev(&self) -> u64 {
        self.as_inner().as_inner().st_dev as u64
    }
    fn st_ino(&self) -> u64 {
        self.as_inner().as_inner().st_ino as u64
    }
    fn st_mode(&self) -> u32 {
        self.as_inner().as_inner().st_mode as u32
    }
    fn st_nlink(&self) -> u64 {
        self.as_inner().as_inner().st_nlink as u64
    }
    fn st_uid(&self) -> u32 {
        self.as_inner().as_inner().st_uid as u32
    }
    fn st_gid(&self) -> u32 {
        self.as_inner().as_inner().st_gid as u32
    }
    fn st_rdev(&self) -> u64 {
        self.as_inner().as_inner().st_rdev as u64
    }
    fn st_size(&self) -> u64 {
        self.as_inner().as_inner().st_size as u64
    }
    fn st_atime(&self) -> i64 {
        self.as_inner().as_inner().st_atime as i64
    }
    fn st_atime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_atime_nsec as i64
    }
    fn st_mtime(&self) -> i64 {
        self.as_inner().as_inner().st_mtime as i64
    }
    fn st_mtime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_mtime_nsec as i64
    }
    fn st_ctime(&self) -> i64 {
        self.as_inner().as_inner().st_ctime as i64
    }
    fn st_ctime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_ctime_nsec as i64
    }
    fn st_crtime(&self) -> i64 {
        self.as_inner().as_inner().st_crtime as i64
    }
    fn st_crtime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_crtime_nsec as i64
    }
    fn st_blksize(&self) -> u64 {
        self.as_inner().as_inner().st_blksize as u64
    }
    fn st_blocks(&self) -> u64 {
        self.as_inner().as_inner().st_blocks as u64
    }
}
