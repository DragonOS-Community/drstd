//! Platform-specific extensions to `std` for Unix platforms.
//!
//! Provides access to platform-level information on Unix platforms, and
//! exposes Unix-specific functions that would otherwise be inappropriate as
//! part of the core `std` library.
//!
//! It exposes more ways to deal with platform-specific strings ([`OsStr`],
//! [`OsString`]), allows to set permissions more granularly, extract low-level
//! file descriptors from files and sockets, and has platform-specific helpers
//! for spawning processes.
//!
//! # Examples
//!
//! ```no_run
//! use std::fs::File;
//! use std::os::unix::prelude::*;
//!
//! fn main() -> std::io::Result<()> {
//!     let f = File::create("foo.txt")?;
//!     let fd = f.as_raw_fd();
//!
//!     // use fd with native unix bindings
//!
//!     Ok(())
//! }
//! ```
//!
//! [`OsStr`]: crate::std::ffi::OsStr
//! [`OsString`]: crate::std::ffi::OsString

#![doc(cfg(unix))]

// Use linux as the default platform when documenting on other platforms like Windows
#[cfg(doc)]
use crate::std::os::linux as platform;

#[cfg(not(doc))]
mod platform {
    #[cfg(target_os = "android")]
    pub use crate::std::os::android::*;
    #[cfg(target_os = "dragonfly")]
    pub use crate::std::os::dragonfly::*;
    #[cfg(target_os = "emscripten")]
    pub use crate::std::os::emscripten::*;
    #[cfg(target_os = "espidf")]
    pub use crate::std::os::espidf::*;
    #[cfg(target_os = "freebsd")]
    pub use crate::std::os::freebsd::*;
    #[cfg(target_os = "fuchsia")]
    pub use crate::std::os::fuchsia::*;
    #[cfg(target_os = "haiku")]
    pub use crate::std::os::haiku::*;
    #[cfg(target_os = "horizon")]
    pub use crate::std::os::horizon::*;
    #[cfg(target_os = "illumos")]
    pub use crate::std::os::illumos::*;
    #[cfg(target_os = "ios")]
    pub use crate::std::os::ios::*;
    #[cfg(target_os = "l4re")]
    pub use crate::std::os::l4re::*;
    #[cfg(target_os = "linux")]
    pub use crate::std::os::linux::*;
    #[cfg(target_os = "macos")]
    pub use crate::std::os::macos::*;
    #[cfg(target_os = "netbsd")]
    pub use crate::std::os::netbsd::*;
    #[cfg(target_os = "nto")]
    pub use crate::std::os::nto::*;
    #[cfg(target_os = "openbsd")]
    pub use crate::std::os::openbsd::*;
    #[cfg(target_os = "redox")]
    pub use crate::std::os::redox::*;
    #[cfg(target_os = "solaris")]
    pub use crate::std::os::solaris::*;
    #[cfg(target_os = "tvos")]
    pub use crate::std::os::tvos::*;
    #[cfg(target_os = "vita")]
    pub use crate::std::os::vita::*;
    #[cfg(target_os = "vxworks")]
    pub use crate::std::os::vxworks::*;
    #[cfg(target_os = "watchos")]
    pub use crate::std::os::watchos::*;
    #[cfg(target_os = "dragonos")]
    pub use crate::std::os::linux::*;
}

pub mod ffi;
pub mod fs;
pub mod io;
pub mod net;
pub mod process;
pub mod raw;
pub mod thread;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "nto",
    target_os = "dragonos"
))]
pub mod ucred;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
pub mod prelude {
    #[doc(no_inline)]
        pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)]
        pub use super::fs::DirEntryExt;
    #[doc(no_inline)]
        pub use super::fs::FileExt;
    #[doc(no_inline)]
        pub use super::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
    #[doc(no_inline)]
        pub use super::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
    #[doc(no_inline)]
        pub use super::process::{CommandExt, ExitStatusExt};
    #[doc(no_inline)]
        pub use super::thread::JoinHandleExt;
}
