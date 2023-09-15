//! Linux and Android-specific networking functionality.

#![doc(cfg(any(target_os = "linux", target_os = "android")))]

pub(crate) mod addr;

pub(crate) mod tcp;

#[cfg(test)]
mod tests;
