#ifndef _RELIBC_STDIO_H
#define _RELIBC_STDIO_H

#include "stdarg.h"
#include "stddef.h"
#include "stdint.h"
#include "sys/types.h"

#define EOF -1

#define BUFSIZ 1024

#define UNGET 8

#define FILENAME_MAX 4096

#define F_PERM 1

#define F_NORD 4

#define F_NOWR 8

#define F_EOF 16

#define F_ERR 32

#define F_SVB 64

#define F_APP 128

#define F_BADJ 256

#define SEEK_SET 0

#define SEEK_CUR 1

#define SEEK_END 2

#define _IOFBF 0

#define _IOLBF 1

#define _IONBF 2

#define L_tmpnam 7

#define TMP_MAX 2147483647

/**
 * This struct gets exposed to the C API.
 */
typedef struct FILE FILE;

typedef off_t fpos_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern FILE *stdin;

extern FILE *stdout;

extern FILE *stderr;

/**
 * Clears EOF and ERR indicators on a stream
 */
void clearerr(FILE *stream);

/**
 * Close a file
 * This function does not guarentee that the file buffer will be flushed or that the file
 * descriptor will be closed, so if it is important that the file be written to, use `fflush()`
 * prior to using this function.
 */
int fclose(FILE *stream);

/**
 * Open a file from a file descriptor
 */
FILE *fdopen(int fildes, const char *mode);

/**
 * Check for EOF
 */
int feof(FILE *stream);

/**
 * Check for ERR
 */
int ferror(FILE *stream);

/**
 * Flush output to stream, or sync read position
 * Ensure the file is unlocked before calling this function, as it will attempt to lock the file
 * itself.
 */
int fflush(FILE *stream);

/**
 * Get a single char from a stream
 */
int fgetc(FILE *stream);

/**
 * Get the position of the stream and store it in pos
 */
int fgetpos(FILE *stream, fpos_t *pos);

/**
 * Get a string from the stream
 */
char *fgets(char *original, int max, FILE *stream);

/**
 * Get the underlying file descriptor
 */
int fileno(FILE *stream);

/**
 * Lock the file
 * Do not call any functions other than those with the `_unlocked` postfix while the file is
 * locked
 */
void flockfile(FILE *file);

/**
 * Open the file in mode `mode`
 */
FILE *fopen(const char *filename, const char *mode);

/**
 * Clear the buffers of a stream
 * Ensure the file is unlocked before calling this function, as it will attempt to lock the file
 * itself.
 */
void __fpurge(FILE *stream);

/**
 * Insert a character into the stream
 */
int fputc(int c, FILE *stream);

/**
 * Insert a string into a stream
 */
int fputs(const char *s, FILE *stream);

/**
 * Read `nitems` of size `size` into `ptr` from `stream`
 */
size_t fread(void *ptr, size_t size, size_t nitems, FILE *stream);

FILE *freopen(const char *filename, const char *mode, FILE *stream);

/**
 * Seek to an offset `offset` from `whence`
 */
int fseek(FILE *stream, long offset, int whence);

/**
 * Seek to an offset `offset` from `whence`
 */
int fseeko(FILE *stream, off_t off, int whence);

/**
 * Seek to a position `pos` in the file from the beginning of the file
 */
int fsetpos(FILE *stream, const fpos_t *pos);

/**
 * Get the current position of the cursor in the file
 */
long ftell(FILE *stream);

/**
 * Get the current position of the cursor in the file
 */
off_t ftello(FILE *stream);

/**
 * Try to lock the file. Returns 0 for success, 1 for failure
 */
int ftrylockfile(FILE *file);

/**
 * Unlock the file
 */
void funlockfile(FILE *file);

/**
 * Write `nitems` of size `size` from `ptr` to `stream`
 */
size_t fwrite(const void *ptr, size_t size, size_t nitems, FILE *stream);

/**
 * Get a single char from a stream
 */
int getc(FILE *stream);

/**
 * Get a single char from `stdin`
 */
int getchar(void);

/**
 * Get a char from a stream without locking the stream
 */
int getc_unlocked(FILE *stream);

/**
 * Get a char from `stdin` without locking `stdin`
 */
int getchar_unlocked(void);

/**
 * Get a string from `stdin`
 */
char *gets(char *s);

/**
 * Get an integer from `stream`
 */
int getw(FILE *stream);

int pclose(FILE *stream);

void perror(const char *s);

FILE *popen(const char *command, const char *mode);

/**
 * Put a character `c` into `stream`
 */
int putc(int c, FILE *stream);

/**
 * Put a character `c` into `stdout`
 */
int putchar(int c);

/**
 * Put a character `c` into `stream` without locking `stream`
 */
int putc_unlocked(int c, FILE *stream);

/**
 * Put a character `c` into `stdout` without locking `stdout`
 */
int putchar_unlocked(int c);

/**
 * Put a string `s` into `stdout`
 */
int puts(const char *s);

/**
 * Put an integer `w` into `stream`
 */
int putw(int w, FILE *stream);

/**
 * Delete file or directory `path`
 */
int remove(const char *path);

int rename(const char *oldpath, const char *newpath);

/**
 * Rewind `stream` back to the beginning of it
 */
void rewind(FILE *stream);

/**
 * Reset `stream` to use buffer `buf`. Buffer must be `BUFSIZ` in length
 */
void setbuf(FILE *stream, char *buf);

/**
 * Reset `stream` to use buffer `buf` of size `size`
 * If this isn't the meaning of unsafe, idk what is
 */
int setvbuf(FILE *stream, char *buf, int mode, size_t size);

char *tempnam(const char *dir, const char *pfx);

FILE *tmpfile(void);

char *tmpnam(char *s);

/**
 * Push character `c` back onto `stream` so it'll be read next
 */
int ungetc(int c, FILE *stream);

int vfprintf(FILE *file, const char *format, va_list ap);

int vprintf(const char *format, va_list ap);

int vasprintf(char **strp, const char *format, va_list ap);

int vsnprintf(char *s, size_t n, const char *format, va_list ap);

int vsprintf(char *s, const char *format, va_list ap);

int vfscanf(FILE *file, const char *format, va_list ap);

int vscanf(const char *format, va_list ap);

int vsscanf(const char *s, const char *format, va_list ap);

ssize_t __getline(char **lineptr, size_t *n, FILE *stream);

ssize_t __getdelim(char **lineptr, size_t *n, int delim, FILE *stream);

size_t __fpending(FILE *stream);

int __freadable(FILE *stream);

int __fwritable(FILE *stream);

int __freading(FILE *stream);

int __fwriting(FILE *stream);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _RELIBC_STDIO_H */

#include "bits/stdio.h"
