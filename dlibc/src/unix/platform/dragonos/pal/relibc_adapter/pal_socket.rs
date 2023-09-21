use crate::unix::*;
use dsc::syscall;
use crate::unix::platform::pal::{e};
use sockaddr;
#[no_mangle]
pub unsafe extern "C" fn accept(socket: ::c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> ::c_int{
	e(syscall!(
        SYS_ACCEPT,
        socket as usize,
        address as usize,
        address_len as usize
    )) as ::c_int
}

#[no_mangle]
pub unsafe extern "C" fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int{
	e(syscall!(SYS_BIND, socket, address, address_len)) as ::c_int
}

#[no_mangle]
pub unsafe extern "C" fn connect(socket: ::c_int, address: *const sockaddr, address_len: socklen_t) -> ::c_int {
    e(syscall!(SYS_CONNECT, socket, address, address_len)) as ::c_int
}

#[no_mangle]
pub unsafe extern "C" fn getpeername(
    socket: ::c_int,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> ::c_int {
    e(syscall!(SYS_GETPEERNAME, socket, address, address_len)) as ::c_int
}

#[no_mangle]
pub unsafe extern "C" fn getsockname(
    socket: ::c_int,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> ::c_int {
    e(syscall!(SYS_GETSOCKNAME, socket, address, address_len)) as ::c_int
}

#[no_mangle]
pub extern "C" fn getsockopt(
    socket: ::c_int,
    level: ::c_int,
    option_name: ::c_int,
    option_value: *mut c_void,
    option_len: *mut socklen_t,
) -> ::c_int {
    e(unsafe {
        syscall!(
            SYS_GETSOCKOPT,
            socket,
            level,
            option_name,
            option_value,
            option_len
        )
    }) as ::c_int
}

#[no_mangle]
pub extern "C" fn listen(socket: ::c_int, backlog: ::c_int) -> ::c_int {
    e(unsafe { syscall!(SYS_LISTEN, socket, backlog) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn recvfrom(
	socket: ::c_int,
	buf: *mut ::c_void,
	len: ::size_t,
	flags: ::c_int,
	addr: *mut ::sockaddr,
	addrlen: *mut ::socklen_t,
) -> ::ssize_t{
	e(unsafe{syscall!(
        SYS_RECVFROM,
        socket,
        buf,
        len,
        flags,
        addr,
        addrlen
    )}) as ssize_t
}

#[no_mangle]
pub extern "C" fn sendto(
    socket: ::c_int,
    buf: *const ::c_void,
    len: ::size_t,
    flags: ::c_int,
    addr: *const sockaddr,
    addrlen: socklen_t,
) -> ::ssize_t{
	e(unsafe{syscall!(
        SYS_SENDTO, socket, buf, len, flags, addr, addrlen
    )}) as ssize_t
}

#[no_mangle]
pub extern "C" fn setsockopt(
    socket: ::c_int,
    level: ::c_int,
    name: ::c_int,
    value: *const ::c_void,
    option_len: socklen_t,
) -> ::c_int{
	e(unsafe {
        syscall!(
            SYS_SETSOCKOPT,
            socket,
            level,
            name,
            value,
            option_len
        )
    }) as ::c_int
}

#[no_mangle]
pub extern "C" fn shutdown(socket: ::c_int, how: ::c_int) -> ::c_int{
	e(unsafe { syscall!(SYS_SHUTDOWN, socket, how) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn socket(domain: ::c_int, ty: ::c_int, protocol: ::c_int) -> ::c_int{
	e(unsafe{syscall!(SYS_SOCKET, domain, ty, protocol)}) as ::c_int
}

#[no_mangle]
pub extern "C" fn socketpair(
    _domain: ::c_int,
    _type_: ::c_int,
    _protocol: ::c_int,
    _socket_vector: *mut ::c_int,
) -> ::c_int{
	unimplemented!()
}