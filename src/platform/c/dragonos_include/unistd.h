#ifndef _RELIBC_UNISTD_H
#define _RELIBC_UNISTD_H

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#define F_OK 0

#define R_OK 4

#define W_OK 2

#define X_OK 1

#define SEEK_SET 0

#define SEEK_CUR 1

#define SEEK_END 2

#define F_ULOCK 0

#define F_LOCK 1

#define F_TLOCK 2

#define F_TEST 3

#define STDIN_FILENO 0

#define STDOUT_FILENO 1

#define STDERR_FILENO 2

#define _PC_LINK_MAX 0

#define _PC_MAX_CANON 1

#define _PC_MAX_INPUT 2

#define _PC_NAME_MAX 3

#define _PC_PATH_MAX 4

#define _PC_PIPE_BUF 5

#define _PC_CHOWN_RESTRICTED 6

#define _PC_NO_TRUNC 7

#define _PC_VDISABLE 8

#define _PC_SYNC_IO 9

#define _PC_ASYNC_IO 10

#define _PC_PRIO_IO 11

#define _PC_SOCK_MAXBUF 12

#define _PC_FILESIZEBITS 13

#define _PC_REC_INCR_XFER_SIZE 14

#define _PC_REC_MAX_XFER_SIZE 15

#define _PC_REC_MIN_XFER_SIZE 16

#define _PC_REC_XFER_ALIGN 17

#define _PC_ALLOC_SIZE_MIN 18

#define _PC_SYMLINK_MAX 19

#define _PC_2_SYMLINKS 20

#define _SC_ARG_MAX 0

#define _SC_CHILD_MAX 1

#define _SC_CLK_TCK 2

#define _SC_NGROUPS_MAX 3

#define _SC_OPEN_MAX 4

#define _SC_STREAM_MAX 5

#define _SC_TZNAME_MAX 6

#define _SC_VERSION 29

#define _SC_PAGESIZE 30

#define _SC_PAGE_SIZE 30

#define _SC_RE_DUP_MAX 44

#define _SC_GETPW_R_SIZE_MAX 70

#define _SC_LOGIN_NAME_MAX 71

#define _SC_TTY_NAME_MAX 72

#define _SC_SYMLOOP_MAX 173

#define _SC_HOST_NAME_MAX 180

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern char *optarg;

extern int optind;

extern int opterr;

extern int optopt;

void _exit(int status);

int access(const char *path, int mode);

unsigned int alarm(unsigned int seconds);

int chdir(const char *path);

int chroot(const char *path);

int chown(const char *path, uid_t owner, gid_t group);

int close(int fildes);

int dup(int fildes);

int dup2(int fildes, int fildes2);

int execv(const char *path, char *const *argv);

int execve(const char *path, char *const *argv, char *const *envp);

int execvp(const char *file, char *const *argv);

int fchown(int fildes, uid_t owner, gid_t group);

int fchdir(int fildes);

pid_t fork(void);

int fsync(int fildes);

int ftruncate(int fildes, off_t length);

char *getcwd(char *buf, size_t size);

gid_t getegid(void);

uid_t geteuid(void);

gid_t getgid(void);

int gethostname(char *name, size_t len);

char *getlogin(void);

int getlogin_r(char *name, size_t namesize);

int getpagesize(void);

pid_t getpgid(pid_t pid);

pid_t getpgrp(void);

pid_t getpid(void);

pid_t getppid(void);

pid_t getsid(pid_t pid);

uid_t getuid(void);

char *getwd(char *path_name);

int isatty(int fd);

int lchown(const char *path, uid_t owner, gid_t group);

int link(const char *path1, const char *path2);

off_t lseek(int fildes, off_t offset, int whence);

int pipe(int *fildes);

int pipe2(int *fildes, int flags);

ssize_t pread(int fildes, void *buf, size_t nbyte, off_t offset);

int pthread_atfork(void (*prepare)(void), void (*parent)(void), void (*child)(void));

ssize_t pwrite(int fildes, const void *buf, size_t nbyte, off_t offset);

ssize_t read(int fildes, const void *buf, size_t nbyte);

ssize_t readlink(const char *path, char *buf, size_t bufsize);

int rmdir(const char *path);

int setgid(gid_t gid);

int setpgid(pid_t pid, pid_t pgid);

pid_t setpgrp(void);

int setregid(gid_t rgid, gid_t egid);

int setreuid(uid_t ruid, uid_t euid);

int setuid(uid_t uid);

unsigned int sleep(unsigned int seconds);

void swab(const void *src, void *dest, ssize_t nbytes);

int symlink(const char *path1, const char *path2);

pid_t tcgetpgrp(int fd);

int tcsetpgrp(int fd, pid_t pgrp);

int truncate(const char *path, off_t length);

char *ttyname(int fildes);

int ttyname_r(int fildes, char *name, size_t namesize);

useconds_t ualarm(useconds_t usecs, useconds_t interval);

int unlink(const char *path);

int usleep(useconds_t useconds);

ssize_t write(int fildes, const void *buf, size_t nbyte);

int brk(void *addr);

void *sbrk(intptr_t incr);

int getopt(int argc, char *const *argv, const char *optstring);

long fpathconf(int _fildes, int name);

long pathconf(const char *_path, int name);

long sysconf(int name);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _RELIBC_UNISTD_H */

#include <bits/fcntl.h>
#include <bits/unistd.h>
