//! WASI-specific networking functionality

use crate::std::io;
use crate::std::net;
use crate::std::sys_common::AsInner;

/// WASI-specific extensions to [`std::net::TcpListener`].
///
/// [`std::net::TcpListener`]: crate::std::net::TcpListener
pub trait TcpListenerExt {
    /// Accept a socket.
    ///
    /// This corresponds to the `sock_accept` syscall.
    fn sock_accept(&self, flags: u16) -> io::Result<u32>;
}

impl TcpListenerExt for net::TcpListener {
    fn sock_accept(&self, flags: u16) -> io::Result<u32> {
        self.as_inner().as_inner().as_inner().sock_accept(flags)
    }
}
