
pub mod ffi;
pub mod io;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
pub mod prelude {
    #[doc(no_inline)]
        pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)]
        pub use super::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
}
