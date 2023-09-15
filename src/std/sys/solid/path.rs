use crate::std::ffi::OsStr;
use crate::std::io;
use crate::std::path::{Path, PathBuf, Prefix};
use crate::std::sys::unsupported;

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'\\'
}

#[inline]
pub fn is_verbatim_sep(b: u8) -> bool {
    b == b'\\'
}

pub fn parse_prefix(_: &OsStr) -> Option<Prefix<'_>> {
    None
}

pub const MAIN_SEP_STR: &str = "\\";
pub const MAIN_SEP: char = '\\';

pub(crate) fn absolute(_path: &Path) -> io::Result<PathBuf> {
    unsupported()
}
