use crate::unix::header::time::{sigevent};

pub struct aiocb {
    pub aio_fildes: ::c_int,
    pub aio_lio_opcode: ::c_int,
    pub aio_reqprio: ::c_int,
    pub aio_buf: *mut ::c_void,
    pub aio_nbytes: usize,
    pub aio_sigevent: sigevent,
}

// #[no_mangle]
pub extern "C" fn aio_read(_aiocbp: *mut aiocb) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn aio_write(_aiocbp: *mut aiocb) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn lio_listio(
    _mode: ::c_int,
    _list: *const *const aiocb,
    _nent: ::c_int,
    _sig: *mut sigevent,
) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn aio_error(_aiocbp: *const aiocb) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn aio_return(_aiocbp: *mut aiocb) -> usize {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn aio_cancel(_fildes: ::c_int, _aiocbp: *mut aiocb) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn aio_suspend(
    _list: *const *const aiocb,
    _nent: ::c_int,
    _timeout: *const ::timespec,
) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub extern "C" fn aio_fsync(_operation: ::c_int, _aiocbp: *mut aiocb) -> ::c_int {
    unimplemented!();
}
