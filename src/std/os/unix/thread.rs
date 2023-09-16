//! Unix-specific extensions to primitives in the [`std::thread`] module.
//!
//! [`std::thread`]: crate::std::thread

#[allow(deprecated)]
use crate::std::os::unix::raw::pthread_t;
use crate::std::sys_common::{AsInner, IntoInner};
use crate::std::thread::JoinHandle;

#[allow(deprecated)]
pub type RawPthread = pthread_t;

/// Unix-specific extensions to [`JoinHandle`].
pub trait JoinHandleExt {
    /// Extracts the raw pthread_t without taking ownership
    fn as_pthread_t(&self) -> RawPthread;

    /// Consumes the thread, returning the raw pthread_t
    ///
    /// This function **transfers ownership** of the underlying pthread_t to
    /// the caller. Callers are then the unique owners of the pthread_t and
    /// must either detach or join the pthread_t once it's no longer needed.
    fn into_pthread_t(self) -> RawPthread;
}

impl<T> JoinHandleExt for JoinHandle<T> {
    fn as_pthread_t(&self) -> RawPthread {
        self.as_inner().id() as RawPthread
    }

    fn into_pthread_t(self) -> RawPthread {
        self.into_inner().into_id() as RawPthread
    }
}
