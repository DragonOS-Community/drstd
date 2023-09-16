use crate::std::os::hermit::io::OwnedFd;
use crate::std::os::hermit::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::std::sys_common::{self, AsInner, FromInner, IntoInner};
use crate::std::{net, sys};

macro_rules! impl_as_raw_fd {
    ($($t:ident)*) => {$(
                impl AsRawFd for net::$t {
            #[inline]
            fn as_raw_fd(&self) -> RawFd {
                self.as_inner().socket().as_raw_fd()
            }
        }
    )*};
}
impl_as_raw_fd! { TcpStream TcpListener UdpSocket }

macro_rules! impl_from_raw_fd {
    ($($t:ident)*) => {$(
                impl FromRawFd for net::$t {
            #[inline]
            unsafe fn from_raw_fd(fd: RawFd) -> net::$t {
                unsafe {
                    let socket = sys::net::Socket::from_inner(FromInner::from_inner(OwnedFd::from_raw_fd(fd)));
                    net::$t::from_inner(sys_common::net::$t::from_inner(socket))
                }
            }
        }
    )*};
}
impl_from_raw_fd! { TcpStream TcpListener UdpSocket }

macro_rules! impl_into_raw_fd {
    ($($t:ident)*) => {$(
                impl IntoRawFd for net::$t {
            #[inline]
            fn into_raw_fd(self) -> RawFd {
                self.into_inner().into_socket().into_inner().into_inner().into_raw_fd()
            }
        }
    )*};
}
impl_into_raw_fd! { TcpStream TcpListener UdpSocket }
