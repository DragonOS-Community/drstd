#[cfg(any(doc, target_os = "android", target_os = "linux"))]
use super::{recv_vectored_with_ancillary_from, send_vectored_with_ancillary_to, SocketAncillary};
use super::{sockaddr_un, SocketAddr};
use crate::std::fmt;
use crate::std::io::{self, IoSlice, IoSliceMut};
use crate::std::net::Shutdown;
use crate::std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "tvos",
    target_os = "macos",
    target_os = "watchos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use crate::std::os::unix::ucred;
use crate::std::path::Path;
use crate::std::sys::cvt;
use crate::std::sys::net::Socket;
use crate::std::sys_common::{AsInner, FromInner};
use crate::std::time::Duration;
use dlibc;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "tvos",
    target_os = "macos",
    target_os = "watchos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub use ucred::UCred;

/// A Unix stream socket.
///
/// # Examples
///
/// ```no_run
/// use std::os::unix::net::UnixStream;
/// use std::io::prelude::*;
///
/// fn main() -> std::io::Result<()> {
///     let mut stream = UnixStream::connect("/path/to/my/socket")?;
///     stream.write_all(b"hello world")?;
///     let mut response = String::new();
///     stream.read_to_string(&mut response)?;
///     println!("{response}");
///     Ok(())
/// }
/// ```
pub struct UnixStream(pub(super) Socket);

impl fmt::Debug for UnixStream {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("UnixStream");
        builder.field("fd", self.0.as_inner());
        if let Ok(addr) = self.local_addr() {
            builder.field("local", &addr);
        }
        if let Ok(addr) = self.peer_addr() {
            builder.field("peer", &addr);
        }
        builder.finish()
    }
}

impl UnixStream {
    /// Connects to the socket named by `path`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// let socket = match UnixStream::connect("/tmp/sock") {
    ///     Ok(sock) => sock,
    ///     Err(e) => {
    ///         println!("Couldn't connect: {e:?}");
    ///         return
    ///     }
    /// };
    /// ```
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<UnixStream> {
        unsafe {
            let inner = Socket::new_raw(dlibc::AF_UNIX, dlibc::SOCK_STREAM)?;
            let (addr, len) = sockaddr_un(path.as_ref())?;

            cvt(dlibc::connect(
                inner.as_raw_fd(),
                &addr as *const _ as *const _,
                len,
            ))?;
            Ok(UnixStream(inner))
        }
    }

    /// Connects to the socket specified by [`address`].
    ///
    /// [`address`]: crate::std::os::unix::net::SocketAddr
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::{UnixListener, UnixStream};
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let listener = UnixListener::bind("/path/to/the/socket")?;
    ///     let addr = listener.local_addr()?;
    ///
    ///     let sock = match UnixStream::connect_addr(&addr) {
    ///         Ok(sock) => sock,
    ///         Err(e) => {
    ///             println!("Couldn't connect: {e:?}");
    ///             return Err(e)
    ///         }
    ///     };
    ///     Ok(())
    /// }
    /// ````
    pub fn connect_addr(socket_addr: &SocketAddr) -> io::Result<UnixStream> {
        unsafe {
            let inner = Socket::new_raw(dlibc::AF_UNIX, dlibc::SOCK_STREAM)?;
            cvt(dlibc::connect(
                inner.as_raw_fd(),
                &socket_addr.addr as *const _ as *const _,
                socket_addr.len,
            ))?;
            Ok(UnixStream(inner))
        }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// Returns two `UnixStream`s which are connected to each other.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// let (sock1, sock2) = match UnixStream::pair() {
    ///     Ok((sock1, sock2)) => (sock1, sock2),
    ///     Err(e) => {
    ///         println!("Couldn't create a pair of sockets: {e:?}");
    ///         return
    ///     }
    /// };
    /// ```
    pub fn pair() -> io::Result<(UnixStream, UnixStream)> {
        let (i1, i2) = Socket::new_pair(dlibc::AF_UNIX, dlibc::SOCK_STREAM)?;
        Ok((UnixStream(i1), UnixStream(i2)))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned `UnixStream` is a reference to the same stream that this
    /// object references. Both handles will read and write the same stream of
    /// data, and options set on one stream will be propagated to the other
    /// stream.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let sock_copy = socket.try_clone().expect("Couldn't clone socket");
    ///     Ok(())
    /// }
    /// ```
    pub fn try_clone(&self) -> io::Result<UnixStream> {
        self.0.duplicate().map(UnixStream)
    }

    /// Returns the socket address of the local half of this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     Ok(())
    /// }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        SocketAddr::new(|addr, len| unsafe { dlibc::getsockname(self.as_raw_fd(), addr, len) })
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let addr = socket.peer_addr().expect("Couldn't get peer address");
    ///     Ok(())
    /// }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        SocketAddr::new(|addr, len| unsafe { dlibc::getpeername(self.as_raw_fd(), addr, len) })
    }

    /// Gets the peer credentials for this Unix domain socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(peer_credentials_unix_socket)]
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let peer_cred = socket.peer_cred().expect("Couldn't get peer credentials");
    ///     Ok(())
    /// }
    /// ```
    #[cfg(any(
        target_os = "android",
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "tvos",
        target_os = "macos",
        target_os = "watchos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub fn peer_cred(&self) -> io::Result<UCred> {
        ucred::peer_cred(self)
    }

    /// Sets the read timeout for the socket.
    ///
    /// If the provided value is [`None`], then [`read`] calls will block
    /// indefinitely. An [`Err`] is returned if the zero [`Duration`] is passed to this
    /// method.
    ///
    /// [`read`]: io::Read::read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    /// use std::time::Duration;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.set_read_timeout(Some(Duration::new(1, 0))).expect("Couldn't set read timeout");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// An [`Err`] is returned if the zero [`Duration`] is passed to this
    /// method:
    ///
    /// ```no_run
    /// use std::io;
    /// use std::os::unix::net::UnixStream;
    /// use std::time::Duration;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let result = socket.set_read_timeout(Some(Duration::new(0, 0)));
    ///     let err = result.unwrap_err();
    ///     assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    ///     Ok(())
    /// }
    /// ```
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.0.set_timeout(timeout, dlibc::SO_RCVTIMEO)
    }

    /// Sets the write timeout for the socket.
    ///
    /// If the provided value is [`None`], then [`write`] calls will block
    /// indefinitely. An [`Err`] is returned if the zero [`Duration`] is
    /// passed to this method.
    ///
    /// [`read`]: io::Read::read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    /// use std::time::Duration;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.set_write_timeout(Some(Duration::new(1, 0)))
    ///         .expect("Couldn't set write timeout");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// An [`Err`] is returned if the zero [`Duration`] is passed to this
    /// method:
    ///
    /// ```no_run
    /// use std::io;
    /// use std::net::UdpSocket;
    /// use std::time::Duration;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UdpSocket::bind("127.0.0.1:34254")?;
    ///     let result = socket.set_write_timeout(Some(Duration::new(0, 0)));
    ///     let err = result.unwrap_err();
    ///     assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    ///     Ok(())
    /// }
    /// ```
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.0.set_timeout(timeout, dlibc::SO_SNDTIMEO)
    }

    /// Returns the read timeout of this socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    /// use std::time::Duration;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.set_read_timeout(Some(Duration::new(1, 0))).expect("Couldn't set read timeout");
    ///     assert_eq!(socket.read_timeout()?, Some(Duration::new(1, 0)));
    ///     Ok(())
    /// }
    /// ```
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.timeout(dlibc::SO_RCVTIMEO)
    }

    /// Returns the write timeout of this socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    /// use std::time::Duration;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.set_write_timeout(Some(Duration::new(1, 0)))
    ///         .expect("Couldn't set write timeout");
    ///     assert_eq!(socket.write_timeout()?, Some(Duration::new(1, 0)));
    ///     Ok(())
    /// }
    /// ```
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0.timeout(dlibc::SO_SNDTIMEO)
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.set_nonblocking(true).expect("Couldn't set nonblocking");
    ///     Ok(())
    /// }
    /// ```
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.0.set_nonblocking(nonblocking)
    }

    /// Moves the socket to pass unix credentials as control message in [`SocketAncillary`].
    ///
    /// Set the socket option `SO_PASSCRED`.
    ///
    /// # Examples
    ///
    #[cfg_attr(
        any(
            target_os = "android",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "freebsd"
        ),
        doc = "```no_run"
    )]
    #[cfg_attr(
        not(any(
            target_os = "android",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "freebsd"
        )),
        doc = "```ignore"
    )]
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.set_passcred(true).expect("Couldn't set passcred");
    ///     Ok(())
    /// }
    /// ```
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
    pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
        self.0.set_passcred(passcred)
    }

    /// Get the current value of the socket for passing unix credentials in [`SocketAncillary`].
    /// This value can be change by [`set_passcred`].
    ///
    /// Get the socket option `SO_PASSCRED`.
    ///
    /// [`set_passcred`]: UnixStream::set_passcred
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
    pub fn passcred(&self) -> io::Result<bool> {
        self.0.passcred()
    }

    /// Set the id of the socket for network filtering purpose
    ///
    #[cfg_attr(
        any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"),
        doc = "```no_run"
    )]
    #[cfg_attr(
        not(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd")),
        doc = "```ignore"
    )]
    /// #![feature(unix_set_mark)]
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let sock = UnixStream::connect("/tmp/sock")?;
    ///     sock.set_mark(32)?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(any(doc, target_os = "linux", target_os = "freebsd", target_os = "openbsd",))]
    pub fn set_mark(&self, mark: u32) -> io::Result<()> {
        self.0.set_mark(mark)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     if let Ok(Some(err)) = socket.take_error() {
    ///         println!("Got error: {err:?}");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Platform specific
    /// On Redox this always returns `None`.
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0.take_error()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O calls on the
    /// specified portions to immediately return with an appropriate value
    /// (see the documentation of [`Shutdown`]).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixStream;
    /// use std::net::Shutdown;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     socket.shutdown(Shutdown::Both).expect("shutdown function failed");
    ///     Ok(())
    /// }
    /// ```
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.0.shutdown(how)
    }

    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue. On success,
    /// returns the number of bytes peeked.
    ///
    /// Successive calls return the same data. This is accomplished by passing
    /// `MSG_PEEK` as a flag to the underlying `recv` system call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(unix_socket_peek)]
    ///
    /// use std::os::unix::net::UnixStream;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let mut buf = [0; 10];
    ///     let len = socket.peek(&mut buf).expect("peek failed");
    ///     Ok(())
    /// }
    /// ```
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.peek(buf)
    }

    /// Receives data and ancillary data from socket.
    ///
    /// On success, returns the number of bytes read.
    ///
    /// # Examples
    ///
    #[cfg_attr(any(target_os = "android", target_os = "linux"), doc = "```no_run")]
    #[cfg_attr(
        not(any(target_os = "android", target_os = "linux")),
        doc = "```ignore"
    )]
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};
    /// use std::io::IoSliceMut;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let mut buf1 = [1; 8];
    ///     let mut buf2 = [2; 16];
    ///     let mut buf3 = [3; 8];
    ///     let mut bufs = &mut [
    ///         IoSliceMut::new(&mut buf1),
    ///         IoSliceMut::new(&mut buf2),
    ///         IoSliceMut::new(&mut buf3),
    ///     ][..];
    ///     let mut fds = [0; 8];
    ///     let mut ancillary_buffer = [0; 128];
    ///     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
    ///     let size = socket.recv_vectored_with_ancillary(bufs, &mut ancillary)?;
    ///     println!("received {size}");
    ///     for ancillary_result in ancillary.messages() {
    ///         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {
    ///             for fd in scm_rights {
    ///                 println!("receive file descriptor: {fd}");
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[cfg(any(doc, target_os = "android", target_os = "linux"))]
    pub fn recv_vectored_with_ancillary(
        &self,
        bufs: &mut [IoSliceMut<'_>],
        ancillary: &mut SocketAncillary<'_>,
    ) -> io::Result<usize> {
        let (count, _, _) = recv_vectored_with_ancillary_from(&self.0, bufs, ancillary)?;

        Ok(count)
    }

    /// Sends data and ancillary data on the socket.
    ///
    /// On success, returns the number of bytes written.
    ///
    /// # Examples
    ///
    #[cfg_attr(any(target_os = "android", target_os = "linux"), doc = "```no_run")]
    #[cfg_attr(
        not(any(target_os = "android", target_os = "linux")),
        doc = "```ignore"
    )]
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::{UnixStream, SocketAncillary};
    /// use std::io::IoSlice;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixStream::connect("/tmp/sock")?;
    ///     let buf1 = [1; 8];
    ///     let buf2 = [2; 16];
    ///     let buf3 = [3; 8];
    ///     let bufs = &[
    ///         IoSlice::new(&buf1),
    ///         IoSlice::new(&buf2),
    ///         IoSlice::new(&buf3),
    ///     ][..];
    ///     let fds = [0, 1, 2];
    ///     let mut ancillary_buffer = [0; 128];
    ///     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
    ///     ancillary.add_fds(&fds[..]);
    ///     socket.send_vectored_with_ancillary(bufs, &mut ancillary)
    ///         .expect("send_vectored_with_ancillary function failed");
    ///     Ok(())
    /// }
    /// ```
    #[cfg(any(doc, target_os = "android", target_os = "linux"))]
    pub fn send_vectored_with_ancillary(
        &self,
        bufs: &[IoSlice<'_>],
        ancillary: &mut SocketAncillary<'_>,
    ) -> io::Result<usize> {
        send_vectored_with_ancillary_to(&self.0, None, bufs, ancillary)
    }
}

impl io::Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::Read::read(&mut &*self, buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        io::Read::read_vectored(&mut &*self, bufs)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        io::Read::is_read_vectored(&&*self)
    }
}

impl<'a> io::Read for &'a UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }
}

impl io::Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Write::write(&mut &*self, buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        io::Write::write_vectored(&mut &*self, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        io::Write::is_write_vectored(&&*self)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::Write::flush(&mut &*self)
    }
}

impl<'a> io::Write for &'a UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl AsRawFd for UnixStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FromRawFd for UnixStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> UnixStream {
        UnixStream(Socket::from_inner(FromInner::from_inner(
            OwnedFd::from_raw_fd(fd),
        )))
    }
}

impl IntoRawFd for UnixStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl AsFd for UnixStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl From<UnixStream> for OwnedFd {
    #[inline]
    fn from(unix_stream: UnixStream) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(unix_stream.into_raw_fd()) }
    }
}

impl From<OwnedFd> for UnixStream {
    #[inline]
    fn from(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}
