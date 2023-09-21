//! Raw Unix-like file descriptors.

use crate::std::fs;
use crate::std::io;
#[cfg(target_os = "hermit")]
use crate::std::os::hermit::io::OwnedFd;
#[cfg(not(target_os = "hermit"))]
use crate::std::os::raw;
#[cfg(all(doc, not(target_arch = "wasm32")))]
use crate::std::os::unix::io::AsFd;

#[cfg(unix)]
use crate::std::os::unix::io::OwnedFd;
#[cfg(target_os = "wasi")]
use crate::std::os::wasi::io::OwnedFd;
use crate::std::sys_common::{AsInner, IntoInner};
use dlibc;
#[cfg(target_os = "hermit")]
use hermit_abi as libc;

/// Raw file descriptors.
#[rustc_allowed_through_unstable_modules]
#[cfg(not(target_os = "hermit"))]
pub type RawFd = raw::c_int;
#[rustc_allowed_through_unstable_modules]
#[cfg(target_os = "hermit")]
pub type RawFd = i32;

/// A trait to extract the raw file descriptor from an underlying object.
///
/// This is only available on unix and WASI platforms and must be imported in
/// order to call the method. Windows platforms have a corresponding
/// `AsRawHandle` and `AsRawSocket` set of traits.
#[rustc_allowed_through_unstable_modules]
pub trait AsRawFd {
    /// Extracts the raw file descriptor.
    ///
    /// This function is typically used to **borrow** an owned file descriptor.
    /// When used in this way, this method does **not** pass ownership of the
    /// raw file descriptor to the caller, and the file descriptor is only
    /// guaranteed to be valid while the original object has not yet been
    /// destroyed.
    ///
    /// However, borrowing is not strictly required. See [`AsFd::as_fd`]
    /// for an API which strictly borrows a file descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// use std::os::fd::{AsRawFd, RawFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// // Note that `raw_fd` is only valid as long as `f` exists.
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.as_raw_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_raw_fd(&self) -> RawFd;
}

/// A trait to express the ability to construct an object from a raw file
/// descriptor.
#[rustc_allowed_through_unstable_modules]
pub trait FromRawFd {
    /// Constructs a new instance of `Self` from the given raw file
    /// descriptor.
    ///
    /// This function is typically used to **consume ownership** of the
    /// specified file descriptor. When used in this way, the returned object
    /// will take responsibility for closing it when the object goes out of
    /// scope.
    ///
    /// However, consuming ownership is not strictly required. Use a
    /// [`From<OwnedFd>::from`] implementation for an API which strictly
    /// consumes ownership.
    ///
    /// # Safety
    ///
    /// The `fd` passed in must be a valid and open file descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.into_raw_fd();
    /// // SAFETY: no other functions should call `from_raw_fd`, so there
    /// // is only one owner for the file descriptor.
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let f = unsafe { File::from_raw_fd(raw_fd) };
    /// # Ok::<(), io::Error>(())
    /// ```
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

/// A trait to express the ability to consume an object and acquire ownership of
/// its raw file descriptor.
#[rustc_allowed_through_unstable_modules]
pub trait IntoRawFd {
    /// Consumes this object, returning the raw underlying file descriptor.
    ///
    /// This function is typically used to **transfer ownership** of the underlying
    /// file descriptor to the caller. When used in this way, callers are then the unique
    /// owners of the file descriptor and must close it once it's no longer needed.
    ///
    /// However, transferring ownership is not strictly required. Use a
    /// [`Into<OwnedFd>::into`] implementation for an API which strictly
    /// transfers ownership.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// use std::os::fd::{IntoRawFd, RawFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.into_raw_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn into_raw_fd(self) -> RawFd;
}

impl AsRawFd for RawFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        *self
    }
}
impl IntoRawFd for RawFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self
    }
}
impl FromRawFd for RawFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> RawFd {
        fd
    }
}

impl AsRawFd for fs::File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.as_inner().as_raw_fd()
    }
}
impl FromRawFd for fs::File {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> fs::File {
        unsafe { fs::File::from(OwnedFd::from_raw_fd(fd)) }
    }
}
impl IntoRawFd for fs::File {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.into_inner().into_inner().into_raw_fd()
    }
}

impl AsRawFd for io::Stdin {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        dlibc::STDIN_FILENO
    }
}

impl AsRawFd for io::Stdout {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        dlibc::STDOUT_FILENO
    }
}

impl AsRawFd for io::Stderr {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        dlibc::STDERR_FILENO
    }
}

impl<'a> AsRawFd for io::StdinLock<'a> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        dlibc::STDIN_FILENO
    }
}

impl<'a> AsRawFd for io::StdoutLock<'a> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        dlibc::STDOUT_FILENO
    }
}

impl<'a> AsRawFd for io::StderrLock<'a> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        dlibc::STDERR_FILENO
    }
}

/// This impl allows implementing traits that require `AsRawFd` on Arc.
/// ```
/// # #[cfg(any(unix, target_os = "wasi"))] mod group_cfg {
/// # #[cfg(target_os = "wasi")]
/// # use std::os::wasi::io::AsRawFd;
/// # #[cfg(unix)]
/// # use std::os::unix::io::AsRawFd;
/// use std::net::UdpSocket;
/// use std::sync::Arc;
/// trait MyTrait: AsRawFd {
/// }
/// impl MyTrait for Arc<UdpSocket> {}
/// impl MyTrait for Box<UdpSocket> {}
/// # }
/// ```
impl<T: AsRawFd> AsRawFd for crate::std::sync::Arc<T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}

impl<T: AsRawFd> AsRawFd for crate::std::rc::Rc<T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}

impl<T: AsRawFd> AsRawFd for Box<T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}
