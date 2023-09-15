
#[allow(unused_extern_crates)]
pub extern crate hermit_abi as abi;

pub mod ffi;
pub mod io;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
pub mod prelude {
    #[doc(no_inline)]
        pub use super::ffi::{OsStrExt, OsStringExt};
}
