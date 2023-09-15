// FIXME: This is currently disabled on *BSD.

use super::{sockaddr_un, SocketAddr};
use crate::std::io::{self, IoSlice, IoSliceMut};
use crate::std::marker::PhantomData;
use crate::std::mem::{size_of, zeroed};
use crate::std::os::unix::io::RawFd;
use crate::std::path::Path;
use crate::std::ptr::{eq, read_unaligned};
use crate::std::slice::from_raw_parts;
use crate::std::sys::net::Socket;
use dlibc;

// FIXME(#43348): Make libc adapt #[doc(cfg(...))] so we don't need these fake definitions here?
#[cfg(all(
    doc,
    not(target_os = "linux"),
    not(target_os = "android"),
    not(target_os = "netbsd"),
    not(target_os = "freebsd")
))]
#[allow(non_camel_case_types)]
mod libc {
    pub use dlibc::c_int;
    pub struct ucred;
    pub struct cmsghdr;
    pub struct sockcred2;
    pub type pid_t = i32;
    pub type gid_t = u32;
    pub type uid_t = u32;
}

pub(super) fn recv_vectored_with_ancillary_from(
    socket: &Socket,
    bufs: &mut [IoSliceMut<'_>],
    ancillary: &mut SocketAncillary<'_>,
) -> io::Result<(usize, bool, io::Result<SocketAddr>)> {
    unsafe {
        let mut msg_name: dlibc::sockaddr_un = zeroed();
        let mut msg: dlibc::msghdr = zeroed();
        msg.msg_name = &mut msg_name as *mut _ as *mut _;
        msg.msg_namelen = size_of::<dlibc::sockaddr_un>() as dlibc::socklen_t;
        msg.msg_iov = bufs.as_mut_ptr().cast();
        msg.msg_iovlen = bufs.len() as _;
        msg.msg_controllen = ancillary.buffer.len() as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = ancillary.buffer.as_mut_ptr().cast();
        }

        let count = socket.recv_msg(&mut msg)?;

        ancillary.length = msg.msg_controllen as usize;
        ancillary.truncated = msg.msg_flags & dlibc::MSG_CTRUNC == dlibc::MSG_CTRUNC;

        let truncated = msg.msg_flags & dlibc::MSG_TRUNC == dlibc::MSG_TRUNC;
        let addr = SocketAddr::from_parts(msg_name, msg.msg_namelen);

        Ok((count, truncated, addr))
    }
}

pub(super) fn send_vectored_with_ancillary_to(
    socket: &Socket,
    path: Option<&Path>,
    bufs: &[IoSlice<'_>],
    ancillary: &mut SocketAncillary<'_>,
) -> io::Result<usize> {
    unsafe {
        let (mut msg_name, msg_namelen) =
            if let Some(path) = path { sockaddr_un(path)? } else { (zeroed(), 0) };

        let mut msg: dlibc::msghdr = zeroed();
        msg.msg_name = &mut msg_name as *mut _ as *mut _;
        msg.msg_namelen = msg_namelen;
        msg.msg_iov = bufs.as_ptr() as *mut _;
        msg.msg_iovlen = bufs.len() as _;
        msg.msg_controllen = ancillary.length as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = ancillary.buffer.as_mut_ptr().cast();
        }

        ancillary.truncated = false;

        socket.send_msg(&mut msg)
    }
}

fn add_to_ancillary_data<T>(
    buffer: &mut [u8],
    length: &mut usize,
    source: &[T],
    cmsg_level: dlibc::c_int,
    cmsg_type: dlibc::c_int,
) -> bool {
    #[cfg(not(target_os = "freebsd"))]
    let cmsg_size = source.len().checked_mul(size_of::<T>());
    #[cfg(target_os = "freebsd")]
    let cmsg_size = Some(unsafe { dlibc::SOCKCRED2SIZE(1) });

    let source_len = if let Some(source_len) = cmsg_size {
        if let Ok(source_len) = u32::try_from(source_len) {
            source_len
        } else {
            return false;
        }
    } else {
        return false;
    };

    unsafe {
        let additional_space = dlibc::CMSG_SPACE(source_len) as usize;

        let new_length = if let Some(new_length) = additional_space.checked_add(*length) {
            new_length
        } else {
            return false;
        };

        if new_length > buffer.len() {
            return false;
        }

        buffer[*length..new_length].fill(0);

        *length = new_length;

        let mut msg: dlibc::msghdr = zeroed();
        msg.msg_control = buffer.as_mut_ptr().cast();
        msg.msg_controllen = *length as _;

        let mut cmsg = dlibc::CMSG_FIRSTHDR(&msg);
        let mut previous_cmsg = cmsg;
        while !cmsg.is_null() {
            previous_cmsg = cmsg;
            cmsg = dlibc::CMSG_NXTHDR(&msg, cmsg);

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if eq(cmsg, previous_cmsg) {
                break;
            }
        }

        if previous_cmsg.is_null() {
            return false;
        }

        (*previous_cmsg).cmsg_level = cmsg_level;
        (*previous_cmsg).cmsg_type = cmsg_type;
        (*previous_cmsg).cmsg_len = dlibc::CMSG_LEN(source_len) as _;

        let data = dlibc::CMSG_DATA(previous_cmsg).cast();

        dlibc::memcpy(data, source.as_ptr().cast(), source_len as usize);
    }
    true
}

struct AncillaryDataIter<'a, T> {
    data: &'a [u8],
    phantom: PhantomData<T>,
}

impl<'a, T> AncillaryDataIter<'a, T> {
    /// Create `AncillaryDataIter` struct to iterate through the data unit in the control message.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message.
    unsafe fn new(data: &'a [u8]) -> AncillaryDataIter<'a, T> {
        AncillaryDataIter { data, phantom: PhantomData }
    }
}

impl<'a, T> Iterator for AncillaryDataIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if size_of::<T>() <= self.data.len() {
            unsafe {
                let unit = read_unaligned(self.data.as_ptr().cast());
                self.data = &self.data[size_of::<T>()..];
                Some(unit)
            }
        } else {
            None
        }
    }
}

#[cfg(all(
    doc,
    not(target_os = "android"),
    not(target_os = "linux"),
    not(target_os = "netbsd"),
    not(target_os = "freebsd")
))]
#[derive(Clone)]
pub struct SocketCred(());

/// Unix credential.
#[cfg(any(target_os = "android", target_os = "linux",))]
#[derive(Clone)]
pub struct SocketCred(dlibc::ucred);

#[cfg(target_os = "netbsd")]
#[derive(Clone)]
pub struct SocketCred(dlibc::sockcred);

#[cfg(target_os = "freebsd")]
#[derive(Clone)]
pub struct SocketCred(dlibc::sockcred2);

#[doc(cfg(any(target_os = "android", target_os = "linux")))]
#[cfg(any(target_os = "android", target_os = "linux"))]
impl SocketCred {
    /// Create a Unix credential struct.
    ///
    /// PID, UID and GID is set to 0.
        #[must_use]
    pub fn new() -> SocketCred {
        SocketCred(dlibc::ucred { pid: 0, uid: 0, gid: 0 })
    }

    /// Set the PID.
        pub fn set_pid(&mut self, pid: dlibc::pid_t) {
        self.0.pid = pid;
    }

    /// Get the current PID.
    #[must_use]
        pub fn get_pid(&self) -> dlibc::pid_t {
        self.0.pid
    }

    /// Set the UID.
        pub fn set_uid(&mut self, uid: dlibc::uid_t) {
        self.0.uid = uid;
    }

    /// Get the current UID.
    #[must_use]
        pub fn get_uid(&self) -> dlibc::uid_t {
        self.0.uid
    }

    /// Set the GID.
        pub fn set_gid(&mut self, gid: dlibc::gid_t) {
        self.0.gid = gid;
    }

    /// Get the current GID.
    #[must_use]
        pub fn get_gid(&self) -> dlibc::gid_t {
        self.0.gid
    }
}

#[cfg(target_os = "freebsd")]
impl SocketCred {
    /// Create a Unix credential struct.
    ///
    /// PID, UID and GID is set to 0.
        #[must_use]
    pub fn new() -> SocketCred {
        SocketCred(dlibc::sockcred2 {
            sc_version: 0,
            sc_pid: 0,
            sc_uid: 0,
            sc_euid: 0,
            sc_gid: 0,
            sc_egid: 0,
            sc_ngroups: 0,
            sc_groups: [0; 1],
        })
    }

    /// Set the PID.
        pub fn set_pid(&mut self, pid: dlibc::pid_t) {
        self.0.sc_pid = pid;
    }

    /// Get the current PID.
    #[must_use]
        pub fn get_pid(&self) -> dlibc::pid_t {
        self.0.sc_pid
    }

    /// Set the UID.
        pub fn set_uid(&mut self, uid: dlibc::uid_t) {
        self.0.sc_euid = uid;
    }

    /// Get the current UID.
    #[must_use]
        pub fn get_uid(&self) -> dlibc::uid_t {
        self.0.sc_euid
    }

    /// Set the GID.
        pub fn set_gid(&mut self, gid: dlibc::gid_t) {
        self.0.sc_egid = gid;
    }

    /// Get the current GID.
    #[must_use]
        pub fn get_gid(&self) -> dlibc::gid_t {
        self.0.sc_egid
    }
}

#[cfg(target_os = "netbsd")]
impl SocketCred {
    /// Create a Unix credential struct.
    ///
    /// PID, UID and GID is set to 0.
        pub fn new() -> SocketCred {
        SocketCred(dlibc::sockcred {
            sc_pid: 0,
            sc_uid: 0,
            sc_euid: 0,
            sc_gid: 0,
            sc_egid: 0,
            sc_ngroups: 0,
            sc_groups: [0u32; 1],
        })
    }

    /// Set the PID.
        pub fn set_pid(&mut self, pid: dlibc::pid_t) {
        self.0.sc_pid = pid;
    }

    /// Get the current PID.
    #[must_use]
        pub fn get_pid(&self) -> dlibc::pid_t {
        self.0.sc_pid
    }

    /// Set the UID.
        pub fn set_uid(&mut self, uid: dlibc::uid_t) {
        self.0.sc_uid = uid;
    }

    /// Get the current UID.
    #[must_use]
        pub fn get_uid(&self) -> dlibc::uid_t {
        self.0.sc_uid
    }

    /// Set the GID.
        pub fn set_gid(&mut self, gid: dlibc::gid_t) {
        self.0.sc_gid = gid;
    }

    /// Get the current GID.
    #[must_use]
        pub fn get_gid(&self) -> dlibc::gid_t {
        self.0.sc_gid
    }
}

/// This control message contains file descriptors.
///
/// The level is equal to `SOL_SOCKET` and the type is equal to `SCM_RIGHTS`.
pub struct ScmRights<'a>(AncillaryDataIter<'a, RawFd>);

impl<'a> Iterator for ScmRights<'a> {
    type Item = RawFd;

    fn next(&mut self) -> Option<RawFd> {
        self.0.next()
    }
}

#[cfg(all(
    doc,
    not(target_os = "android"),
    not(target_os = "linux"),
    not(target_os = "netbsd"),
    not(target_os = "freebsd")
))]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, ()>);

/// This control message contains unix credentials.
///
/// The level is equal to `SOL_SOCKET` and the type is equal to `SCM_CREDENTIALS` or `SCM_CREDS`.
#[cfg(any(target_os = "android", target_os = "linux",))]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, dlibc::ucred>);

#[cfg(target_os = "freebsd")]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, dlibc::sockcred2>);

#[cfg(target_os = "netbsd")]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, dlibc::sockcred>);

#[cfg(any(
    doc,
    target_os = "android",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "freebsd"
))]
impl<'a> Iterator for ScmCredentials<'a> {
    type Item = SocketCred;

    fn next(&mut self) -> Option<SocketCred> {
        Some(SocketCred(self.0.next()?))
    }
}

/// The error type which is returned from parsing the type a control message.
#[non_exhaustive]
#[derive(Debug)]
pub enum AncillaryError {
    Unknown { cmsg_level: i32, cmsg_type: i32 },
}

/// This enum represent one control message of variable type.
pub enum AncillaryData<'a> {
    ScmRights(ScmRights<'a>),
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
    ScmCredentials(ScmCredentials<'a>),
}

impl<'a> AncillaryData<'a> {
    /// Create an `AncillaryData::ScmRights` variant.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message and the control message must be type of
    /// `SOL_SOCKET` and level of `SCM_RIGHTS`.
    unsafe fn as_rights(data: &'a [u8]) -> Self {
        let ancillary_data_iter = AncillaryDataIter::new(data);
        let scm_rights = ScmRights(ancillary_data_iter);
        AncillaryData::ScmRights(scm_rights)
    }

    /// Create an `AncillaryData::ScmCredentials` variant.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message and the control message must be type of
    /// `SOL_SOCKET` and level of `SCM_CREDENTIALS` or `SCM_CREDS`.
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
    unsafe fn as_credentials(data: &'a [u8]) -> Self {
        let ancillary_data_iter = AncillaryDataIter::new(data);
        let scm_credentials = ScmCredentials(ancillary_data_iter);
        AncillaryData::ScmCredentials(scm_credentials)
    }

    fn try_from_cmsghdr(cmsg: &'a dlibc::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = dlibc::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = dlibc::CMSG_DATA(cmsg).cast();
            let data = from_raw_parts(data, data_len);

            match (*cmsg).cmsg_level {
                dlibc::SOL_SOCKET => match (*cmsg).cmsg_type {
                    dlibc::SCM_RIGHTS => Ok(AncillaryData::as_rights(data)),
                    #[cfg(any(target_os = "android", target_os = "linux",))]
                    dlibc::SCM_CREDENTIALS => Ok(AncillaryData::as_credentials(data)),
                    #[cfg(target_os = "freebsd")]
                    dlibc::SCM_CREDS2 => Ok(AncillaryData::as_credentials(data)),
                    #[cfg(target_os = "netbsd")]
                    dlibc::SCM_CREDS => Ok(AncillaryData::as_credentials(data)),
                    cmsg_type => {
                        Err(AncillaryError::Unknown { cmsg_level: dlibc::SOL_SOCKET, cmsg_type })
                    }
                },
                cmsg_level => {
                    Err(AncillaryError::Unknown { cmsg_level, cmsg_type: (*cmsg).cmsg_type })
                }
            }
        }
    }
}

/// This struct is used to iterate through the control messages.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Messages<'a> {
    buffer: &'a [u8],
    current: Option<&'a dlibc::cmsghdr>,
}

impl<'a> Iterator for Messages<'a> {
    type Item = Result<AncillaryData<'a>, AncillaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut msg: dlibc::msghdr = zeroed();
            msg.msg_control = self.buffer.as_ptr() as *mut _;
            msg.msg_controllen = self.buffer.len() as _;

            let cmsg = if let Some(current) = self.current {
                dlibc::CMSG_NXTHDR(&msg, current)
            } else {
                dlibc::CMSG_FIRSTHDR(&msg)
            };

            let cmsg = cmsg.as_ref()?;

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if let Some(current) = self.current {
                if eq(current, cmsg) {
                    return None;
                }
            }

            self.current = Some(cmsg);
            let ancillary_result = AncillaryData::try_from_cmsghdr(cmsg);
            Some(ancillary_result)
        }
    }
}

/// A Unix socket Ancillary data struct.
///
/// # Example
/// ```no_run
/// #![feature(unix_socket_ancillary_data)]
/// use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};
/// use std::io::IoSliceMut;
///
/// fn main() -> std::io::Result<()> {
///     let sock = UnixStream::connect("/tmp/sock")?;
///
///     let mut fds = [0; 8];
///     let mut ancillary_buffer = [0; 128];
///     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
///
///     let mut buf = [1; 8];
///     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];
///     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;
///
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
#[derive(Debug)]
pub struct SocketAncillary<'a> {
    buffer: &'a mut [u8],
    length: usize,
    truncated: bool,
}

impl<'a> SocketAncillary<'a> {
    /// Create an ancillary data with the given buffer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #![allow(unused_mut)]
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::SocketAncillary;
    /// let mut ancillary_buffer = [0; 128];
    /// let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
    /// ```
        pub fn new(buffer: &'a mut [u8]) -> Self {
        SocketAncillary { buffer, length: 0, truncated: false }
    }

    /// Returns the capacity of the buffer.
    #[must_use]
        pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the ancillary data is empty.
    #[must_use]
        pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of used bytes.
    #[must_use]
        pub fn len(&self) -> usize {
        self.length
    }

    /// Returns the iterator of the control messages.
        pub fn messages(&self) -> Messages<'_> {
        Messages { buffer: &self.buffer[..self.length], current: None }
    }

    /// Is `true` if during a recv operation the ancillary was truncated.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::{UnixStream, SocketAncillary};
    /// use std::io::IoSliceMut;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let sock = UnixStream::connect("/tmp/sock")?;
    ///
    ///     let mut ancillary_buffer = [0; 128];
    ///     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
    ///
    ///     let mut buf = [1; 8];
    ///     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];
    ///     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;
    ///
    ///     println!("Is truncated: {}", ancillary.truncated());
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
        pub fn truncated(&self) -> bool {
        self.truncated
    }

    /// Add file descriptors to the ancillary data.
    ///
    /// The function returns `true` if there was enough space in the buffer.
    /// If there was not enough space then no file descriptors was appended.
    /// Technically, that means this operation adds a control message with the level `SOL_SOCKET`
    /// and type `SCM_RIGHTS`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::{UnixStream, SocketAncillary};
    /// use std::os::unix::io::AsRawFd;
    /// use std::io::IoSlice;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let sock = UnixStream::connect("/tmp/sock")?;
    ///
    ///     let mut ancillary_buffer = [0; 128];
    ///     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
    ///     ancillary.add_fds(&[sock.as_raw_fd()][..]);
    ///
    ///     let buf = [1; 8];
    ///     let mut bufs = &mut [IoSlice::new(&buf[..])][..];
    ///     sock.send_vectored_with_ancillary(bufs, &mut ancillary)?;
    ///     Ok(())
    /// }
    /// ```
        pub fn add_fds(&mut self, fds: &[RawFd]) -> bool {
        self.truncated = false;
        add_to_ancillary_data(
            &mut self.buffer,
            &mut self.length,
            fds,
            dlibc::SOL_SOCKET,
            dlibc::SCM_RIGHTS,
        )
    }

    /// Add credentials to the ancillary data.
    ///
    /// The function returns `true` if there is enough space in the buffer.
    /// If there is not enough space then no credentials will be appended.
    /// Technically, that means this operation adds a control message with the level `SOL_SOCKET`
    /// and type `SCM_CREDENTIALS`, `SCM_CREDS`, or `SCM_CREDS2`.
    ///
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
        pub fn add_creds(&mut self, creds: &[SocketCred]) -> bool {
        self.truncated = false;
        add_to_ancillary_data(
            &mut self.buffer,
            &mut self.length,
            creds,
            dlibc::SOL_SOCKET,
            #[cfg(not(any(target_os = "netbsd", target_os = "freebsd")))]
            dlibc::SCM_CREDENTIALS,
            #[cfg(target_os = "freebsd")]
            dlibc::SCM_CREDS2,
            #[cfg(target_os = "netbsd")]
            dlibc::SCM_CREDS,
        )
    }

    /// Clears the ancillary data, removing all values.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};
    /// use std::io::IoSliceMut;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let sock = UnixStream::connect("/tmp/sock")?;
    ///
    ///     let mut fds1 = [0; 8];
    ///     let mut fds2 = [0; 8];
    ///     let mut ancillary_buffer = [0; 128];
    ///     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);
    ///
    ///     let mut buf = [1; 8];
    ///     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];
    ///
    ///     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;
    ///     for ancillary_result in ancillary.messages() {
    ///         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {
    ///             for fd in scm_rights {
    ///                 println!("receive file descriptor: {fd}");
    ///             }
    ///         }
    ///     }
    ///
    ///     ancillary.clear();
    ///
    ///     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;
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
        pub fn clear(&mut self) {
        self.length = 0;
        self.truncated = false;
    }
}
