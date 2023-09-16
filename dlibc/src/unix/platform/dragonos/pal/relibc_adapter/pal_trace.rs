use crate::unix::*;
use dsc::syscall;
use crate::unix::platform::pal::{e,errno};

#[no_mangle]
pub extern "C" fn ptrace(request: ::c_int) -> ::c_long{
	unimplemented!()
}