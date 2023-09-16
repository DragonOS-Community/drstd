

use crate::unix::platform::pal::{e,errno};
use crate::unix::c_str::CStr;
use crate::unix::*;
use dsc::syscall;

pub extern "C" fn utimens(path: &CStr, times: *const timespec) -> ::c_int {
	// e(unsafe { syscall!(UTIMENSAT, AT_FDCWD, path.as_ptr(), times, 0) }) as ::c_int
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t) -> ::ssize_t{
	e(unsafe { syscall!(SYS_WRITE, fd, buf, count) }) as ::ssize_t
}

#[no_mangle]
pub extern "C" fn access(path: *const ::c_char, amode: ::c_int) -> ::c_int{
	unimplemented!()
}

pub extern "C" fn brk(addr: *mut ::c_void) -> *mut ::c_void{
	unsafe { syscall!(SYS_BRK, addr) as *mut ::c_void }
}

#[no_mangle]
pub extern "C" fn chdir(dir: *const ::c_char) -> ::c_int{
	e(unsafe { syscall!(SYS_CHDIR, dir) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn chmod(path: *const c_char, mode: mode_t) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn chown(path: *const ::c_char, uid: uid_t, gid: gid_t) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int{
	-ENOSYS
}

#[no_mangle]
pub extern "C" fn close(fd: ::c_int) -> ::c_int{
	e(unsafe { syscall!(SYS_CLOSE, fd) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn dup(fd: ::c_int) -> ::c_int{
	e(unsafe { syscall!(SYS_DUP, fd) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn dup2(src: ::c_int, dst: ::c_int) -> ::c_int{
	e(unsafe { syscall!(SYS_DUP2, src, dst) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn execve(
    prog: *const ::c_char,
    argv_: *const *const ::c_char,
    envp: *const *const ::c_char,
) -> ::c_int{
	e(unsafe{syscall!(SYS_EXECVE, prog, argv_, envp)}) as ::c_int
}

#[no_mangle]
pub extern "C" fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int{
	e(unsafe { syscall!(SYS_FSTAT, fildes, buf) }) as ::c_int
}

//#[no_mangle]
pub extern "C" fn exit(status: ::c_int) -> !{
	unsafe {
		syscall!(SYS_EXIT, status);
	}
	loop {}
}

#[no_mangle]
pub extern "C" fn fchdir(dirfd: ::c_int) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn fchmod(fd: ::c_int, mode: mode_t) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn fchown(fd: ::c_int, owner: ::uid_t, group: ::gid_t) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn flock(fd: ::c_int, operation: ::c_int) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn fstatvfs(fd: ::c_int, buf: *mut statvfs) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn fcntl(fd: ::c_int, cmd: ::c_int,arg: ::c_int) -> ::c_int{
	e(unsafe { syscall!(SYS_FCNTL, fd, cmd, arg) }) as ::c_int
}

//#[no_mangle]
pub extern "C" fn fork() -> ::pid_t{
	e(unsafe { syscall!(SYS_FORK) }) as ::pid_t
}

#[no_mangle]
pub extern "C" fn fpath(filedes: ::c_int, name:*const ::c_char) -> c_long{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn fsync(fd: ::c_int) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn ftruncate(fd: ::c_int, length: off_t) -> ::c_int{
	e(unsafe { syscall!(SYS_FTRUNCATE, fd, length) }) as ::c_int
}

pub extern "C" fn futex(addr: *mut ::c_int, op: ::c_int, val: ::c_int, val2: usize) -> ::c_int {
	// unsafe { syscall!(FUTEX, addr, op, val, val2, 0, 0) as ::c_int }
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn utimensat(
	dirfd: ::c_int,
	path: *const ::c_char,
	times: *const ::timespec,
	flag: ::c_int,
) -> ::c_int{
	unimplemented!()
}

//#[no_mangle]
pub extern "C" fn getcwd(buf: *mut ::c_char, size: ::size_t) -> *mut ::c_char{
	// if e(unsafe { syscall!(GETCWD, buf, size) }) == !0 {
	//     ptr::null_mut()
	// } else {
	//     buf
	// }
	// 临时实现，设置所有的cwd为根目录
	if size > 2 {
		unsafe {
			*buf = b'/' as ::c_char;
			*buf.add(1) = b'\0' as ::c_char;
		}
	}

	buf
}

#[no_mangle]
pub extern "C" fn getdents(
    fd: ::c_int, 
    dirents: *mut dirent, 
    bytes: usize) -> ::c_int{
        unimplemented!()
}

#[no_mangle]
pub extern "C" fn getegid() -> gid_t{
	0
}

#[no_mangle]
pub extern "C" fn geteuid() -> uid_t{
	0
}

#[no_mangle]
pub extern "C" fn getgid() -> gid_t{
	0
}

//#[no_mangle]
pub extern "C" fn getpagesize() -> usize{
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn getpgid(pid: ::pid_t) -> ::pid_t{
	0
}

#[no_mangle]
pub extern "C" fn getpid() -> ::pid_t{
	e(unsafe { syscall!(SYS_GETPID) }) as ::pid_t
}

#[no_mangle]
pub extern "C" fn getppid() -> ::pid_t{
	0
}

#[no_mangle]
pub extern "C" fn getrlimit(resource: ::c_int, rlim: *mut ::rlimit) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn getsid(pid: ::pid_t) -> ::pid_t{
	0
}

#[no_mangle]
pub extern "C" fn gettid() -> ::pid_t{
	unimplemented!()
}

//#[no_mangle]
pub extern "C" fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int{
	e(unsafe { syscall!(SYS_GETTIMEOFDAY, tp, tz) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn getuid() -> uid_t{
	0
}

#[no_mangle]
pub extern "C" fn lchown(path: *const ::c_char, uid: uid_t, gid: gid_t) -> ::c_int{
	0
}

#[no_mangle]
pub extern "C" fn link(src: *const ::c_char, dst: *const ::c_char) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t{
	e(unsafe { syscall!(SYS_LSEEK, fd, offset, whence) }) as off_t
}

#[no_mangle]
pub extern "C" fn mkdir(path: *const ::c_char, mode: mode_t) -> ::c_int{
	e(unsafe { syscall!(SYS_MKDIR, path, mode) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn mkfifo(path: *const ::c_char, mode: mode_t) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn mlock(addr: *const ::c_void, len: ::size_t) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn mlockall(flags: ::c_int) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn mmap(
    addr: *mut ::c_void,
    len: ::size_t,
    prot: ::c_int,
    flags: ::c_int,
    fd: ::c_int,
    offset: off_t,
) -> *mut ::c_void{
	e(unsafe{syscall!(SYS_MMAP, addr, len, prot, flags, fd, offset)}) as *mut ::c_void
}

#[no_mangle]
pub extern "C" fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int{
	e(unsafe{syscall!(SYS_MPROTECT, addr, len, prot)}) as ::c_int
}

#[no_mangle]
pub extern "C" fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn munlock(addr: *const ::c_void, len: ::size_t) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn munlockall() -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int{
	e(unsafe{syscall!(SYS_MUNMAP, addr, len)}) as ::c_int
}

#[no_mangle]
pub extern "C" fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> ::c_int{
	e(unsafe { syscall!(SYS_NANOSLEEP, rqtp, rmtp) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn open(path: *const ::c_char, oflag: ::c_int, mode: mode_t) -> ::c_int{
	e(unsafe { syscall!(SYS_OPEN, path, oflag, mode) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int{
	if flags == 0 {
		e(unsafe { syscall!(SYS_PIPE, fds) }) as ::c_int
	} else {
		unimplemented!()
	}
}

#[no_mangle]
pub unsafe extern "C" fn pte_clone(stack: *mut usize) -> ::pid_t{
    unimplemented!()    
}

#[no_mangle]
pub extern "C" fn read(fd: ::c_int, buf: *mut ::c_void, count: ::size_t) -> ::ssize_t{
	e(unsafe { syscall!(SYS_READ, fd, buf, count) }) as ::ssize_t
}

pub extern "C" fn readlink(path: *const c_char, buf: *mut c_char, bufsz: ::size_t) -> ::ssize_t{
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rename(oldname: *const ::c_char, newname: *const ::c_char) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn rmdir(path: *const ::c_char) -> ::c_int{
	e(unsafe { syscall!(SYS_UNLINK_AT, 0, path, AT_REMOVEDIR) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn sched_yield() -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn setpgid(pid: ::pid_t, pgid: ::pid_t) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn setregid(rgid: gid_t, egid: gid_t) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn setreuid(ruid: uid_t, euid: uid_t) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn symlink(path1: *const ::c_char, path2: *const ::c_char) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn umask(mask: mode_t) -> mode_t{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn uname(buf: *mut ::utsname) -> ::c_int{
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn unlink(c: *const ::c_char) -> ::c_int{
	e(unsafe { syscall!(SYS_UNLINK_AT, AT_FDCWD, c, 0) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn waitpid(pid: ::pid_t, status: *mut ::c_int, options: ::c_int) -> ::pid_t{
	e(unsafe { syscall!(SYS_WAIT4, pid, status, options, 0) }) as ::pid_t
}

#[no_mangle]
pub extern "C" fn verify() -> bool{
    return true;
}