use dsc::syscall;

use super::{pal::Pal, types::{c_void, c_int}};

pub struct Sys;

impl Sys {
    pub unsafe fn ioctl() {
        unimplemented!();
    }
}

pub fn e(sys: usize) -> usize {
    if (sys as isize) < 0 && (sys as isize) >= -256 {
        unsafe {
            errno = -(sys as isize) as c_int;
        }
        !0
    } else {
        sys
    }
}

impl Pal for Sys {
    fn access(path: &CStr, mode: c_int) -> c_int {
        unimplemented!()
    }

    fn brk(addr: *mut c_void) -> *mut c_void {
        unsafe { syscall!(SYS_BRK, addr) as *mut c_void }
    }

    fn chdir(path: &CStr) -> c_int {
        e(unsafe { syscall!(SYS_CHDIR, path.as_ptr()) }) as c_int
    }

    fn chmod(path: &CStr, mode: mode_t) -> c_int {
        return 0;
    }

    fn chown(path: &CStr, owner: uid_t, group: gid_t) -> c_int {
        return 0;
    }

    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int {
        return -ENOSYS;
    }

    fn close(fildes: c_int) -> c_int {
        e(unsafe { syscall!(SYS_CLOSE, fildes) }) as c_int
    }

    fn dup(fildes: c_int) -> c_int {
        e(unsafe { syscall!(SYS_DUP, fildes) }) as c_int
    }

    fn dup2(fildes: c_int, fildes2: c_int) -> c_int {
        e(unsafe { syscall!(SYS_DUP2, fildes, fildes2) }) as c_int
    }

    unsafe fn execve(path: &CStr, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int {
        e(syscall!(SYS_EXECVE, path.as_ptr(), argv, envp)) as c_int
    }

    fn exit(status: c_int) -> ! {
        unsafe {
            syscall!(SYS_EXIT, status);
        }
        loop {}
    }

    fn fchdir(fildes: c_int) -> c_int {
        unimplemented!()
    }

    fn fchmod(fildes: c_int, mode: mode_t) -> c_int {
        return 0;
    }

    fn fchown(fildes: c_int, owner: uid_t, group: gid_t) -> c_int {
        return 0;
    }

    fn flock(fd: c_int, operation: c_int) -> c_int {
        return 0;
    }

    fn fstat(fildes: c_int, buf: *mut stat) -> c_int {
        e(unsafe { syscall!(SYS_FSTAT, fildes, buf) }) as c_int
    }

    fn fstatvfs(fildes: c_int, buf: *mut statvfs) -> c_int {
        unimplemented!()
    }

    fn fcntl(fildes: c_int, cmd: c_int, arg: c_int) -> c_int {
        let rc = e(unsafe { syscall!(SYS_FCNTL, fildes, cmd, arg) }) as c_int;
        return rc;
    }

    fn fork() -> pid_t {
        e(unsafe { syscall!(SYS_FORK) }) as pid_t
    }

    fn fpath(fildes: c_int, out: &mut [u8]) -> ssize_t {
        unimplemented!()
    }

    fn fsync(fildes: c_int) -> c_int {
        return 0;
    }

    fn ftruncate(fildes: c_int, length: off_t) -> c_int {
        e(unsafe { syscall!(SYS_FTRUNCATE, fildes, length) }) as c_int
    }

    fn futex(addr: *mut c_int, op: c_int, val: c_int, val2: usize) -> c_int {
        unimplemented!()
    }

    fn futimens(fd: c_int, times: *const timespec) -> c_int {
        unimplemented!()
    }

    fn utimens(path: &CStr, times: *const timespec) -> c_int {
        unimplemented!()
    }

    fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
        if size > 2 {
            unsafe {
                *buf = b'/' as c_char;
                *buf.add(1) = b'\0' as c_char;
            }
        }

        return buf;
    }

    fn getdents(fd: c_int, dirents: *mut dirent, bytes: usize) -> c_int {
        unsafe { syscall!(SYS_GET_DENTS, fd, dirents, bytes) as c_int }
    }

    fn getegid() -> gid_t {
        return 0;
    }

    fn geteuid() -> uid_t {
        return 0;
    }

    fn getgid() -> gid_t {
        return 0;
    }

    fn getpagesize() -> usize {
        return 4096;
    }

    fn getpgid(pid: pid_t) -> pid_t {
        return 0;
    }

    fn getpid() -> pid_t {
        e(unsafe { syscall!(SYS_GETPID) }) as pid_t
    }

    fn getppid() -> pid_t {
        return 0;
    }

    fn getrandom(buf: &mut [u8], flags: c_uint) -> ssize_t {
        unimplemented!()
    }

    unsafe fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int {
        unimplemented!()
    }

    fn getsid(pid: pid_t) -> pid_t {
        return 0;
    }

    fn gettid() -> pid_t {
        return Self::getpid();
    }

    fn gettimeofday(tp: *mut timeval, tzp: *mut timezone) -> c_int {
        e(unsafe { syscall!(SYS_GETTIMEOFDAY, tp, tzp) }) as c_int
    }

    fn getuid() -> uid_t {
        return 0;
    }

    fn lchown(path: &CStr, owner: uid_t, group: gid_t) -> c_int {
        return 0;
    }

    fn link(path1: &CStr, path2: &CStr) -> c_int {
        unimplemented!()
    }

    fn lseek(fildes: c_int, offset: off_t, whence: c_int) -> off_t {
        e(unsafe { syscall!(SYS_LSEEK, fildes, offset, whence) }) as off_t
    }

    fn mkdir(path: &CStr, mode: mode_t) -> c_int {
        e(unsafe { syscall!(SYS_MKDIR, path.as_ptr(), mode) }) as c_int
    }

    fn mkfifo(path: &CStr, mode: mode_t) -> c_int {
        unimplemented!()
    }

    unsafe fn mlock(addr: *const c_void, len: usize) -> c_int {
        unimplemented!()
    }

    fn mlockall(flags: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fildes: c_int,
        off: off_t,
    ) -> *mut c_void {
        e(syscall!(SYS_MMAP, addr, len, prot, flags, fildes, off)) as *mut c_void
    }

    unsafe fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int {
        e(syscall!(SYS_MPROTECT, addr, len, prot)) as c_int
    }

    unsafe fn msync(addr: *mut c_void, len: usize, flags: c_int) -> c_int {
        unimplemented!()
    }

    unsafe fn munlock(addr: *const c_void, len: usize) -> c_int {
        unimplemented!()
    }

    fn munlockall() -> c_int {
        unimplemented!()
    }

    unsafe fn munmap(addr: *mut c_void, len: usize) -> c_int {
        e(syscall!(SYS_MUNMAP, addr, len)) as c_int
    }

    fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int {
        e(unsafe { syscall!(SYS_NANOSLEEP, rqtp, rmtp) }) as c_int
    }

    fn open(path: &CStr, oflag: c_int, mode: mode_t) -> c_int {
        e(unsafe { syscall!(SYS_OPEN, path.as_ptr(), oflag, mode) }) as c_int
    }

    fn pipe2(fildes: &mut [c_int], flags: c_int) -> c_int {
        if flags == 0 {
            e(unsafe { syscall!(SYS_PIPE, fildes.as_mut_ptr()) }) as c_int
        } else {
            unimplemented!()
        }
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn pte_clone(stack: *mut usize) -> pid_t {
        unimplemented!()
    }

    fn read(fildes: c_int, buf: &mut [u8]) -> ssize_t {
        e(unsafe { syscall!(SYS_READ, fildes, buf.as_mut_ptr(), buf.len()) }) as ssize_t
    }

    fn readlink(pathname: &CStr, out: &mut [u8]) -> ssize_t {
        unimplemented!()
    }

    fn rename(old: &CStr, new: &CStr) -> c_int {
        unimplemented!()
    }

    fn rmdir(path: &CStr) -> c_int {
        e(unsafe { syscall!(SYS_UNLINK_AT, 0, path.as_ptr(), AT_REMOVEDIR) }) as c_int
    }

    fn sched_yield() -> c_int {
        unimplemented!()
    }

    fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
        unimplemented!()
    }

    fn setregid(rgid: gid_t, egid: gid_t) -> c_int {
        unimplemented!()
    }

    fn setreuid(ruid: uid_t, euid: uid_t) -> c_int {
        unimplemented!()
    }

    fn symlink(path1: &CStr, path2: &CStr) -> c_int {
        unimplemented!()
    }

    fn umask(mask: mode_t) -> mode_t {
        unimplemented!()
    }

    fn uname(utsname: *mut utsname) -> c_int {
        unimplemented!()
    }

    fn unlink(path: &CStr) -> c_int {
        e(unsafe { syscall!(SYS_UNLINK_AT, AT_FDCWD, path.as_ptr(), 0) }) as c_int
    }

    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t {
        e(unsafe { syscall!(SYS_WAIT4, pid, stat_loc, options, 0) }) as pid_t
    }

    fn write(fildes: c_int, buf: &[u8]) -> ssize_t {
        e(unsafe { syscall!(SYS_WRITE, fildes, buf.as_ptr(), buf.len()) }) as ssize_t
    }

    fn verify() -> bool {
        return true;
    }
}