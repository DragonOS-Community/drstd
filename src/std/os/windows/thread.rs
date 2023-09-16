//! Windows-specific extensions to primitives in the [`std::thread`] module.
//!
//! [`std::thread`]: crate::std::thread

use crate::std::os::windows::io::{AsRawHandle, IntoRawHandle, RawHandle};
use crate::std::sys_common::{AsInner, IntoInner};
use crate::std::thread;

impl<T> AsRawHandle for thread::JoinHandle<T> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.as_inner().handle().as_raw_handle() as *mut _
    }
}

impl<T> IntoRawHandle for thread::JoinHandle<T> {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.into_inner().into_handle().into_raw_handle() as *mut _
    }
}
