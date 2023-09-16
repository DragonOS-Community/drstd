use crate::std::ffi::{OsStr, OsString};
use crate::std::mem;
use crate::std::sealed::Sealed;
use crate::std::sys::os_str::Buf;
use crate::std::sys_common::{AsInner, FromInner, IntoInner};

// Note: this file is currently reused in other `std::os::{platform}::ffi` modules to reduce duplication.
// Keep this in mind when applying changes to this file that only apply to `unix`.

/// Platform-specific extensions to [`OsString`].
///
/// This trait is sealed: it cannot be implemented outside the standard library.
/// This is so that future additional methods are not breaking changes.
pub trait OsStringExt: Sealed {
    /// Creates an [`OsString`] from a byte vector.
    ///
    /// See the module documentation for an example.
    fn from_vec(vec: Vec<u8>) -> Self;

    /// Yields the underlying byte vector of this [`OsString`].
    ///
    /// See the module documentation for an example.
    fn into_vec(self) -> Vec<u8>;
}

impl OsStringExt for OsString {
    #[inline]
    fn from_vec(vec: Vec<u8>) -> OsString {
        FromInner::from_inner(Buf { inner: vec })
    }
    #[inline]
    fn into_vec(self) -> Vec<u8> {
        self.into_inner().inner
    }
}

/// Platform-specific extensions to [`OsStr`].
///
/// This trait is sealed: it cannot be implemented outside the standard library.
/// This is so that future additional methods are not breaking changes.
pub trait OsStrExt: Sealed {
    /// Creates an [`OsStr`] from a byte slice.
    ///
    /// See the module documentation for an example.
    fn from_bytes(slice: &[u8]) -> &Self;

    /// Gets the underlying byte view of the [`OsStr`] slice.
    ///
    /// See the module documentation for an example.
    fn as_bytes(&self) -> &[u8];
}

impl OsStrExt for OsStr {
    #[inline]
    fn from_bytes(slice: &[u8]) -> &OsStr {
        unsafe { mem::transmute(slice) }
    }
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        &self.as_inner().inner
    }
}
