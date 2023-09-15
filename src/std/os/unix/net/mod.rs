//! Unix-specific networking functionality.

#![allow(irrefutable_let_patterns)]

mod addr;
#[doc(cfg(any(target_os = "android", target_os = "linux")))]
#[cfg(any(doc, target_os = "android", target_os = "linux"))]
mod ancillary;
mod datagram;
mod listener;
mod stream;
#[cfg(all(test, not(target_os = "emscripten")))]
mod tests;

pub use self::addr::*;
#[cfg(any(doc, target_os = "android", target_os = "linux"))]
pub use self::ancillary::*;
pub use self::datagram::*;
pub use self::listener::*;
pub use self::stream::*;
