use crate::unix::platform::pal::{e,errno};
use mem;
use crate::unix::*;
use dsc::syscall;

use stack_t;

#[no_mangle]
pub extern "C" fn getitimer(which: ::c_int, out: *mut ::itimerval) -> ::c_int {
	// e(unsafe { syscall!(GETITIMER, which, out) }) as ::c_int
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn kill(pid: ::pid_t, sig: ::c_int) -> ::c_int {
	e(unsafe { syscall!(SYS_KILL, pid, sig) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn killpg(pgrp: ::pid_t, sig: ::c_int) -> ::c_int {
	e(unsafe { syscall!(SYS_KILL, -(pgrp as isize) as ::pid_t, sig) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn raise(sig: ::c_int) -> ::c_int {
	let tid = e(unsafe { syscall!(SYS_GETPID) }) as ::pid_t;
	if tid == !0 {
		-1
	} else {
		// e(unsafe { syscall!(TKILL, tid, sig) }) as ::c_int
		self::kill(tid, sig)
	}
}

#[no_mangle]
pub extern "C" fn setitimer(which: ::c_int, new: *const ::itimerval, old: *mut ::itimerval) -> ::c_int {
	// e(unsafe { syscall!(SETITIMER, which, new, old) }) as ::c_int
	unimplemented!()
}

use crate::unix::header::signal;
#[no_mangle]
pub extern "C" fn sigaction(
	signum: ::c_int,
	act: *const sigaction,
	oldact: *mut sigaction)
	-> ::c_int {
	e(unsafe {
		syscall!(
			SYS_SIGACTION,
			signum,
			act,
			oldact,
			mem::size_of::<sigset_t>()
		)
	}) as ::c_int
}

#[no_mangle]
pub extern "C" fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> ::c_int {
	// e(unsafe { syscall!(SIGALTSTACK, ss, old_ss) }) as ::c_int
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn sigprocmask(how: ::c_int, set: *const sigset_t, oset: *mut sigset_t) -> ::c_int {
	// e(unsafe { syscall!(RT_SIGPROCMASK, how, set, oset, mem::size_of::<sigset_t>()) }) as ::c_int
	unimplemented!()
}