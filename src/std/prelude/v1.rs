//! The first version of the prelude of The Rust Standard Library.
//!
//! See the [module-level documentation](super) for more.


// Re-exported core operators
#[doc(no_inline)]
pub use crate::std::marker::{Send, Sized, Sync, Unpin};
#[doc(no_inline)]
pub use crate::std::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[doc(no_inline)]
pub use crate::std::mem::drop;

// Re-exported types and traits
#[doc(no_inline)]
pub use crate::std::convert::{AsMut, AsRef, From, Into};
#[doc(no_inline)]
pub use crate::std::iter::{DoubleEndedIterator, ExactSizeIterator};
#[doc(no_inline)]
pub use crate::std::iter::{Extend, IntoIterator, Iterator};
#[doc(no_inline)]
pub use crate::std::option::Option::{self, None, Some};
#[doc(no_inline)]
pub use crate::std::result::Result::{self, Err, Ok};

// Re-exported built-in macros
#[allow(deprecated)]
#[doc(no_inline)]
pub use core::prelude::v1::{
    assert, cfg, column, compile_error, concat, concat_idents, env, file, format_args,
    format_args_nl, include, include_bytes, include_str, line, log_syntax, module_path, option_env,
    stringify, trace_macros, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
};

#[doc(no_inline)]
pub use core::prelude::v1::concat_bytes;

// Do not `doc(inline)` these `doc(hidden)` items.
#[allow(deprecated)]
pub use core::prelude::v1::{RustcDecodable, RustcEncodable};

// Do not `doc(no_inline)` so that they become doc items on their own
// (no public module for them to be re-exported from).
pub use core::prelude::v1::{
    alloc_error_handler, derive, global_allocator, test, test_case,
};

pub use core::prelude::v1::derive_const;

// Do not `doc(no_inline)` either.
pub use core::prelude::v1::cfg_accessible;

// Do not `doc(no_inline)` either.
pub use core::prelude::v1::cfg_eval;

// Do not `doc(no_inline)` either.
pub use core::prelude::v1::type_ascribe;

// The file so far is equivalent to core/src/prelude/v1.rs. It is duplicated
// rather than glob imported because we want docs to show these re-exports as
// pointing to within `std`.
// Below are the items from the alloc crate.

#[doc(no_inline)]
pub use crate::std::borrow::ToOwned;
#[doc(no_inline)]
pub use crate::std::boxed::Box;
#[doc(no_inline)]
pub use crate::std::string::{String, ToString};
#[doc(no_inline)]
pub use crate::std::vec::Vec;
