//! Linux and Android-specific extensions to socket addresses.

use crate::std::os::unix::net::SocketAddr;
use crate::std::sealed::Sealed;

/// Platform-specific extensions to [`SocketAddr`].
pub trait SocketAddrExt: Sealed {
    /// Creates a Unix socket address in the abstract namespace.
    ///
    /// The abstract namespace is a Linux-specific extension that allows Unix
    /// sockets to be bound without creating an entry in the filesystem.
    /// Abstract sockets are unaffected by filesystem layout or permissions,
    /// and no cleanup is necessary when the socket is closed.
    ///
    /// An abstract socket address name may contain any bytes, including zero.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is longer than `SUN_LEN - 1`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::{UnixListener, SocketAddr};
    /// use std::os::linux::net::SocketAddrExt;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let addr = SocketAddr::from_abstract_name(b"hidden")?;
    ///     let listener = match UnixListener::bind_addr(&addr) {
    ///         Ok(sock) => sock,
    ///         Err(err) => {
    ///             println!("Couldn't bind: {err:?}");
    ///             return Err(err);
    ///         }
    ///     };
    ///     Ok(())
    /// }
    /// ```
    fn from_abstract_name<N>(name: N) -> crate::std::io::Result<SocketAddr>
    where
        N: AsRef<[u8]>;

    /// Returns the contents of this address if it is in the abstract namespace.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::{UnixListener, SocketAddr};
    /// use std::os::linux::net::SocketAddrExt;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let name = b"hidden";
    ///     let name_addr = SocketAddr::from_abstract_name(name)?;
    ///     let socket = UnixListener::bind_addr(&name_addr)?;
    ///     let local_addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(local_addr.as_abstract_name(), Some(&name[..]));
    ///     Ok(())
    /// }
    /// ```
    fn as_abstract_name(&self) -> Option<&[u8]>;
}
