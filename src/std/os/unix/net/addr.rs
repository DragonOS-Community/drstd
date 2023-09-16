use crate::std::ffi::OsStr;
#[cfg(any(doc, target_os = "android", target_os = "linux"))]
use crate::std::os::net::linux_ext;
use crate::std::os::unix::ffi::OsStrExt;
use crate::std::path::Path;
use crate::std::sealed::Sealed;
use crate::std::sys::cvt;
use crate::std::{fmt, io, mem, ptr};
use dlibc;

// FIXME(#43348): Make libc adapt #[doc(cfg(...))] so we don't need these fake definitions here?
// #[cfg(not(unix))]
// #[allow(non_camel_case_types)]
// mod libc {
//     pub use dlibc::c_int;
//     pub type socklen_t = u32;
//     pub struct sockaddr;
//     #[derive(Clone)]
//     pub struct sockaddr_un;
// }

fn sun_path_offset(addr: &dlibc::sockaddr_un) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = (addr as *const dlibc::sockaddr_un).addr();
    let path = (&addr.sun_path as *const dlibc::c_char).addr();
    path - base
}

pub(super) fn sockaddr_un(path: &Path) -> io::Result<(dlibc::sockaddr_un, dlibc::socklen_t)> {
    // SAFETY: All zeros is a valid representation for `sockaddr_un`.
    let mut addr: dlibc::sockaddr_un = unsafe { mem::zeroed() };
    addr.sun_family = dlibc::AF_UNIX as dlibc::sa_family_t;

    let bytes = path.as_os_str().as_bytes();

    if bytes.contains(&0) {
        return Err(io::const_io_error!(
            io::ErrorKind::InvalidInput,
            "paths must not contain interior null bytes",
        ));
    }

    if bytes.len() >= addr.sun_path.len() {
        return Err(io::const_io_error!(
            io::ErrorKind::InvalidInput,
            "path must be shorter than SUN_LEN",
        ));
    }
    // SAFETY: `bytes` and `addr.sun_path` are not overlapping and
    // both point to valid memory.
    // NOTE: We zeroed the memory above, so the path is already null
    // terminated.
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            addr.sun_path.as_mut_ptr().cast(),
            bytes.len(),
        )
    };

    let mut len = sun_path_offset(&addr) + bytes.len();
    match bytes.get(0) {
        Some(&0) | None => {}
        Some(_) => len += 1,
    }
    Ok((addr, len as dlibc::socklen_t))
}

enum AddressKind<'a> {
    Unnamed,
    Pathname(&'a Path),
    Abstract(&'a [u8]),
}

/// An address associated with a Unix socket.
///
/// # Examples
///
/// ```
/// use std::os::unix::net::UnixListener;
///
/// let socket = match UnixListener::bind("/tmp/sock") {
///     Ok(sock) => sock,
///     Err(e) => {
///         println!("Couldn't bind: {e:?}");
///         return
///     }
/// };
/// let addr = socket.local_addr().expect("Couldn't get local address");
/// ```
#[derive(Clone)]
pub struct SocketAddr {
    pub(super) addr: dlibc::sockaddr_un,
    pub(super) len: dlibc::socklen_t,
}

impl SocketAddr {
    pub(super) fn new<F>(f: F) -> io::Result<SocketAddr>
    where
        F: FnOnce(*mut dlibc::sockaddr, *mut dlibc::socklen_t) -> dlibc::c_int,
    {
        unsafe {
            let mut addr: dlibc::sockaddr_un = mem::zeroed();
            let mut len = mem::size_of::<dlibc::sockaddr_un>() as dlibc::socklen_t;
            cvt(f(&mut addr as *mut _ as *mut _, &mut len))?;
            SocketAddr::from_parts(addr, len)
        }
    }

    pub(super) fn from_parts(
        addr: dlibc::sockaddr_un,
        mut len: dlibc::socklen_t,
    ) -> io::Result<SocketAddr> {
        if len == 0 {
            // When there is a datagram from unnamed unix socket
            // linux returns zero bytes of address
            len = sun_path_offset(&addr) as dlibc::socklen_t; // i.e., zero-length address
        } else if addr.sun_family != dlibc::AF_UNIX as dlibc::sa_family_t {
            return Err(io::const_io_error!(
                io::ErrorKind::InvalidInput,
                "file descriptor did not correspond to a Unix socket",
            ));
        }

        Ok(SocketAddr { addr, len })
    }

    /// Constructs a `SockAddr` with the family `AF_UNIX` and the provided path.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is longer than `SUN_LEN` or if it contains
    /// NULL bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::os::unix::net::SocketAddr;
    /// use std::path::Path;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let address = SocketAddr::from_pathname("/path/to/socket")?;
    /// assert_eq!(address.as_pathname(), Some(Path::new("/path/to/socket")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a `SocketAddr` with a NULL byte results in an error.
    ///
    /// ```
    /// use std::os::unix::net::SocketAddr;
    ///
    /// assert!(SocketAddr::from_pathname("/path/with/\0/bytes").is_err());
    /// ```
    pub fn from_pathname<P>(path: P) -> io::Result<SocketAddr>
    where
        P: AsRef<Path>,
    {
        sockaddr_un(path.as_ref()).map(|(addr, len)| SocketAddr { addr, len })
    }

    /// Returns `true` if the address is unnamed.
    ///
    /// # Examples
    ///
    /// A named address:
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixListener;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixListener::bind("/tmp/sock")?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.is_unnamed(), false);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// An unnamed address:
    ///
    /// ```
    /// use std::os::unix::net::UnixDatagram;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixDatagram::unbound()?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.is_unnamed(), true);
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn is_unnamed(&self) -> bool {
        matches!(self.address(), AddressKind::Unnamed)
    }

    /// Returns the contents of this address if it is a `pathname` address.
    ///
    /// # Examples
    ///
    /// With a pathname:
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixListener;
    /// use std::path::Path;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixListener::bind("/tmp/sock")?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.as_pathname(), Some(Path::new("/tmp/sock")));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Without a pathname:
    ///
    /// ```
    /// use std::os::unix::net::UnixDatagram;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixDatagram::unbound()?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.as_pathname(), None);
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn as_pathname(&self) -> Option<&Path> {
        if let AddressKind::Pathname(path) = self.address() {
            Some(path)
        } else {
            None
        }
    }

    fn address(&self) -> AddressKind<'_> {
        let len = self.len as usize - sun_path_offset(&self.addr);
        let path = unsafe { mem::transmute::<&[dlibc::c_char], &[u8]>(&self.addr.sun_path) };

        // macOS seems to return a len of 16 and a zeroed sun_path for unnamed addresses
        if len == 0
            || (cfg!(not(any(target_os = "linux", target_os = "android")))
                && self.addr.sun_path[0] == 0)
        {
            AddressKind::Unnamed
        } else if self.addr.sun_path[0] == 0 {
            AddressKind::Abstract(&path[1..len])
        } else {
            AddressKind::Pathname(OsStr::from_bytes(&path[..len - 1]).as_ref())
        }
    }
}

impl Sealed for SocketAddr {}

#[doc(cfg(any(target_os = "android", target_os = "linux")))]
#[cfg(any(doc, target_os = "android", target_os = "linux"))]
impl linux_ext::addr::SocketAddrExt for SocketAddr {
    fn as_abstract_name(&self) -> Option<&[u8]> {
        if let AddressKind::Abstract(name) = self.address() {
            Some(name)
        } else {
            None
        }
    }

    fn from_abstract_name<N>(name: N) -> crate::std::io::Result<Self>
    where
        N: AsRef<[u8]>,
    {
        let name = name.as_ref();
        unsafe {
            let mut addr: dlibc::sockaddr_un = mem::zeroed();
            addr.sun_family = dlibc::AF_UNIX as dlibc::sa_family_t;

            if name.len() + 1 > addr.sun_path.len() {
                return Err(io::const_io_error!(
                    io::ErrorKind::InvalidInput,
                    "abstract socket name must be shorter than SUN_LEN",
                ));
            }

            crate::std::ptr::copy_nonoverlapping(
                name.as_ptr(),
                addr.sun_path.as_mut_ptr().add(1) as *mut u8,
                name.len(),
            );
            let len = (sun_path_offset(&addr) + 1 + name.len()) as dlibc::socklen_t;
            SocketAddr::from_parts(addr, len)
        }
    }
}

impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.address() {
            AddressKind::Unnamed => write!(fmt, "(unnamed)"),
            AddressKind::Abstract(name) => write!(fmt, "\"{}\" (abstract)", name.escape_ascii()),
            AddressKind::Pathname(path) => write!(fmt, "{path:?} (pathname)"),
        }
    }
}
