


#[no_mangle]
pub extern "C" fn epoll_create1(_flags: ::c_int) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn epoll_ctl(_epfd: ::c_int, _op: ::c_int, _fd: ::c_int, _event: *mut ::epoll_event)
	-> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn epoll_pwait(
	_epfd: ::c_int,
	_events: *mut ::epoll_event,
	_maxevents: ::c_int,
	_timeout: ::c_int,
	_sigmask: *const ::sigset_t,
) -> ::c_int{
	unimplemented!()
}