
use crate::unix::platform::pal::{e};
use crate::unix::c_str::CStr;
use crate::unix::platform::pal::e;
use crate::unix::*;
use dsc::syscall;

pub extern "C" fn utimens(_path: &CStr, _times: *const timespec) -> ::c_int {
    // e(unsafe { syscall!(UTIMENSAT, AT_FDCWD, path.as_ptr(), times, 0) }) as ::c_int
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t) -> ::ssize_t {
    e(unsafe { syscall!(SYS_WRITE, fd, buf, count) }) as ::ssize_t
}

#[no_mangle]
pub extern "C" fn access(_path: *const ::c_char, _amode: ::c_int) -> ::c_int {
    unimplemented!()
}

pub extern "C" fn brk(addr: *mut ::c_void) -> *mut ::c_void {
    unsafe { syscall!(SYS_BRK, addr) as *mut ::c_void }
}

#[no_mangle]
pub extern "C" fn chdir(dir: *const ::c_char) -> ::c_int {
    e(unsafe { syscall!(SYS_CHDIR, dir) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn chmod(_path: *const c_char, _mode: mode_t) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn chown(_path: *const ::c_char, _uid: uid_t, _gid: gid_t) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn clock_gettime(_clk_id: ::clockid_t, _tp: *mut ::timespec) -> ::c_int {
    -ENOSYS
}

#[no_mangle]
pub extern "C" fn close(fd: ::c_int) -> ::c_int {
    e(unsafe { syscall!(SYS_CLOSE, fd) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn dup(fd: ::c_int) -> ::c_int {
    e(unsafe { syscall!(SYS_DUP, fd) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn dup2(src: ::c_int, dst: ::c_int) -> ::c_int {
    e(unsafe { syscall!(SYS_DUP2, src, dst) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn execve(
    prog: *const ::c_char,
    argv_: *const *const ::c_char,
    envp: *const *const ::c_char,
) -> ::c_int {
    e(unsafe { syscall!(SYS_EXECVE, prog, argv_, envp) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int {
    e(unsafe { syscall!(SYS_FSTAT, fildes, buf) }) as ::c_int
}

//#[no_mangle]
pub extern "C" fn exit(status: ::c_int) -> ! {
    unsafe {
        syscall!(SYS_EXIT, status);
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn fchdir(_dirfd: ::c_int) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fchmod(_fd: ::c_int, _mode: mode_t) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn fchown(_fd: ::c_int, _owner: ::uid_t, _group: ::gid_t) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn flock(_fd: ::c_int, _operation: ::c_int) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn fstatvfs(_fd: ::c_int, _buf: *mut statvfs) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fcntl(fd: ::c_int, cmd: ::c_int, arg: ::c_int) -> ::c_int {
    e(unsafe { syscall!(SYS_FCNTL, fd, cmd, arg) }) as ::c_int
}

//#[no_mangle]
pub extern "C" fn fork() -> ::pid_t {
    e(unsafe { syscall!(SYS_FORK) }) as ::pid_t
}

#[no_mangle]
pub extern "C" fn fpath(_filedes: ::c_int, _name: *const ::c_char) -> c_long {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn fsync(_fd: ::c_int) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn ftruncate(fd: ::c_int, length: off_t) -> ::c_int {
    e(unsafe { syscall!(SYS_FTRUNCATE, fd, length) }) as ::c_int
}

pub extern "C" fn futex(_addr: *mut ::c_int, _op: ::c_int, _val: ::c_int, _val2: usize) -> ::c_int {
    // unsafe { syscall!(FUTEX, addr, op, val, val2, 0, 0) as ::c_int }
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn futimens(_fd: ::c_int, _times: *const ::timespec) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn utimensat(
    _dirfd: ::c_int,
    _path: *const ::c_char,
    _times: *const ::timespec,
    _flag: ::c_int,
) -> ::c_int {
    unimplemented!()
}

//#[no_mangle]
pub extern "C" fn getcwd(buf: *mut ::c_char, size: ::size_t) -> *mut ::c_char {
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
pub extern "C" fn getdents(fd: ::c_int, dirents: *mut dirent, bytes: usize) -> ::c_int {
    unsafe { syscall!(SYS_GET_DENTS, fd, dirents, bytes) as ::c_int }
}

#[no_mangle]
pub extern "C" fn getegid() -> gid_t {
    0
}

#[no_mangle]
pub extern "C" fn geteuid() -> uid_t {
    0
}

#[no_mangle]
pub extern "C" fn getgid() -> gid_t {
    0
}

//#[no_mangle]
pub extern "C" fn getpagesize() -> usize {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn getpgid(_pid: ::pid_t) -> ::pid_t {
    0
}

#[no_mangle]
pub extern "C" fn getpid() -> ::pid_t {
    e(unsafe { syscall!(SYS_GETPID) }) as ::pid_t
}

#[no_mangle]
pub extern "C" fn getppid() -> ::pid_t {
    0
}

#[no_mangle]
pub extern "C" fn getrlimit(_resource: ::c_int, _rlim: *mut ::rlimit) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn getsid(_pid: ::pid_t) -> ::pid_t {
    0
}

#[no_mangle]
pub extern "C" fn gettid() -> ::pid_t {
    unimplemented!()
}

//#[no_mangle]
pub extern "C" fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int {
    e(unsafe { syscall!(SYS_GETTIMEOFDAY, tp, tz) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn getuid() -> uid_t {
    0
}

#[no_mangle]
pub extern "C" fn lchown(_path: *const ::c_char, _uid: uid_t, _gid: gid_t) -> ::c_int {
    0
}

#[no_mangle]
pub extern "C" fn link(_src: *const ::c_char, _dst: *const ::c_char) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t {
    e(unsafe { syscall!(SYS_LSEEK, fd, offset, whence) }) as off_t
}

#[no_mangle]
pub extern "C" fn mkdir(path: *const ::c_char, mode: mode_t) -> ::c_int {
    e(unsafe { syscall!(SYS_MKDIR, path, mode) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn mkfifo(_path: *const ::c_char, _mode: mode_t) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn mlock(_addr: *const ::c_void, _len: ::size_t) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn mlockall(_flags: ::c_int) -> ::c_int {
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
) -> *mut ::c_void {
    e(unsafe { syscall!(SYS_MMAP, addr, len, prot, flags, fd, offset) }) as *mut ::c_void
}

#[no_mangle]
pub extern "C" fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int {
    e(unsafe { syscall!(SYS_MPROTECT, addr, len, prot) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn msync(_addr: *mut ::c_void, _len: ::size_t, _flags: ::c_int) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn munlock(_addr: *const ::c_void, _len: ::size_t) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn munlockall() -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int {
    e(unsafe { syscall!(SYS_MUNMAP, addr, len) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> ::c_int {
    e(unsafe { syscall!(SYS_NANOSLEEP, rqtp, rmtp) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn open(path: *const ::c_char, oflag: ::c_int, mode: mode_t) -> ::c_int {
    e(unsafe { syscall!(SYS_OPEN, path, oflag, mode) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int {
    if flags == 0 {
        e(unsafe { syscall!(SYS_PIPE, fds) }) as ::c_int
    } else {
        unimplemented!()
    }
}

#[no_mangle]
pub unsafe extern "C" fn pte_clone(_stack: *mut usize) -> ::pid_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn read(fd: ::c_int, buf: *mut ::c_void, count: ::size_t) -> ::ssize_t {
    e(unsafe { syscall!(SYS_READ, fd, buf, count) }) as ::ssize_t
}

pub extern "C" fn readlink(_path: *const c_char, _buf: *mut c_char, _bufsz: ::size_t) -> ::ssize_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rename(_oldname: *const ::c_char, _newname: *const ::c_char) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn rmdir(path: *const ::c_char) -> ::c_int {
    e(unsafe { syscall!(SYS_UNLINK_AT, 0, path, AT_REMOVEDIR) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn sched_yield() -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn setpgid(_pid: ::pid_t, _pgid: ::pid_t) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn setregid(_rgid: gid_t, _egid: gid_t) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn setreuid(_ruid: uid_t, _euid: uid_t) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn symlink(_path1: *const ::c_char, _path2: *const ::c_char) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn umask(_mask: mode_t) -> mode_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn uname(_buf: *mut ::utsname) -> ::c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn unlink(c: *const ::c_char) -> ::c_int {
    e(unsafe { syscall!(SYS_UNLINK_AT, AT_FDCWD, c, 0) }) as ::c_int
}

#[no_mangle]
pub extern "C" fn waitpid(pid: ::pid_t, status: *mut ::c_int, options: ::c_int) -> ::pid_t {
    e(unsafe { syscall!(SYS_WAIT4, pid, status, options, 0) }) as ::pid_t
}

#[no_mangle]
pub extern "C" fn verify() -> bool {
    return true;
}
