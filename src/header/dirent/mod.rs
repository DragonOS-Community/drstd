use crate::platform::types::{ino_t, off_t, c_ushort, c_uchar,c_char};

#[repr(C)]
#[derive(Clone)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: off_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}