
pub mod relibc_adapter;
pub use self::relibc_adapter::*;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut errno: ::c_int = 0;

#[no_mangle]
pub extern "C" fn e(sys: usize) -> usize {
    if (sys as isize) < 0 && (sys as isize) >= -256 {
        unsafe {
            errno = -(sys as isize) as ::c_int;
        }
        !0
    } else {
        sys
    }
}

#[no_mangle]
pub extern "C" fn getrandom(buf: &mut [u8], flags: ::c_uint) -> ::ssize_t{
    unimplemented!()
}

// #[no_mangle]
// pub extern "C" fn pthread_key_delete(key: pthread_key_t) -> ::c_int{
//     0
// }
// #[no_mangle]
// pub extern "C" fn syscall(num: ::c_long) -> ::c_long{
//     0
// }
// #[no_mangle]
// pub extern "C" fn isalnum(c: ::c_int) -> ::c_int{
//  ::c_int::from(isdigit(c) != 0 || isalpha(c) != 0)
// }
// #[no_mangle]
// pub extern "C" fn isalpha(c: ::c_int) -> ::c_int {
//     ::c_int::from(islower(c) != 0 || isupper(c) != 0)
// }

// #[no_mangle]
// pub extern "C" fn isascii(c: ::c_int) -> ::c_int {
//     ::c_int::from((c & !0x7f) == 0)
// }

// #[no_mangle]
// pub extern "C" fn isblank(c: ::c_int) -> ::c_int {
//     ::c_int::from(c == ::c_int::from(b' ') || c == ::c_int::from(b'\t'))
// }

// #[no_mangle]
// pub extern "C" fn iscntrl(c: ::c_int) -> ::c_int {
//     ::c_int::from((c >= 0x00 && c <= 0x1f) || c == 0x7f)
// }

// #[no_mangle]
// pub extern "C" fn isdigit(c: ::c_int) -> ::c_int {
//     ::c_int::from(c >= ::c_int::from(b'0') && c <= ::c_int::from(b'9'))
// }

// #[no_mangle]
// pub extern "C" fn isgraph(c: ::c_int) -> ::c_int {
//     ::c_int::from(c >= 0x21 && c <= 0x7e)
// }

// #[no_mangle]
// pub extern "C" fn islower(c: ::c_int) -> ::c_int {
//     ::c_int::from(c >= ::c_int::from(b'a') && c <= ::c_int::from(b'z'))
// }

// #[no_mangle]
// pub extern "C" fn isprint(c: ::c_int) -> ::c_int {
//     ::c_int::from(c >= 0x20 && c < 0x7f)
// }

// #[no_mangle]
// pub extern "C" fn ispunct(c: ::c_int) -> ::c_int {
//     ::c_int::from(
//         (c >= ::c_int::from(b'!') && c <= ::c_int::from(b'/'))
//             || (c >= ::c_int::from(b':') && c <= ::c_int::from(b'@'))
//             || (c >= ::c_int::from(b'[') && c <= ::c_int::from(b'`'))
//             || (c >= ::c_int::from(b'{') && c <= ::c_int::from(b'~')),
//     )
// }

// #[no_mangle]
// pub extern "C" fn isspace(c: ::c_int) -> ::c_int {
//     ::c_int::from(
//         c == ::c_int::from(b' ')
//             || c == ::c_int::from(b'\t')
//             || c == ::c_int::from(b'\n')
//             || c == ::c_int::from(b'\r')
//             || c == 0x0b
//             || c == 0x0c,
//     )
// }

// #[no_mangle]
// pub extern "C" fn isupper(c: ::c_int) -> ::c_int {
//     ::c_int::from(c >= ::c_int::from(b'A') && c <= ::c_int::from(b'Z'))
// }

// #[no_mangle]
// pub extern "C" fn isxdigit(c: ::c_int) -> ::c_int {
//     ::c_int::from(isdigit(c) != 0 || (c | 32 >= ::c_int::from(b'a') && c | 32 <= ::c_int::from(b'f')))
// }

// #[no_mangle]
// /// The comment in musl:
// /// "nonsense function that should NEVER be used!"
// pub extern "C" fn toascii(c: ::c_int) -> ::c_int {
//     c & 0x7f
// }

// #[no_mangle]
// pub extern "C" fn tolower(c: ::c_int) -> ::c_int {
//     if isupper(c) != 0 {
//         c | 0x20
//     } else {
//         c
//     }
// }

// #[no_mangle]
// pub extern "C" fn toupper(c: ::c_int) -> ::c_int {
//     if islower(c) != 0 {
//         c & !0x20
//     } else {
//         c
//     }
// }
// #[no_mangle]
// pub extern "C" fn qsort(
//     base: *mut c_void,
//     num: ::size_t,
//     size: ::size_t,
//     compar: ::Option<unsafe extern "C" fn(*const c_void, *const c_void) -> ::c_int>,
// ){
// 	unimplemented!()
// }
// unsafe extern "C" fn void_cmp(a: *const c_void, b: *const c_void) -> ::c_int {
//     *(a as *const i32) - *(b as *const i32) as ::c_int
// }
// #[no_mangle]
// pub unsafe extern "C" fn bsearch(
//     key: *const c_void,
//     base: *const c_void,
//     num: ::size_t,
//     size: ::size_t,
//     compar: ::Option<unsafe extern "C" fn(*const c_void, *const c_void) -> ::c_int>,
// ) -> *mut c_void{
// 	let mut start = base;
//     let mut len = num;
//     let cmp_fn = compar.unwrap_or(void_cmp);
//     while len > 0 {
//         let med = (start as ::size_t + (len >> 1) * size) as *const c_void;
//         let diff = cmp_fn(key, med);
//         if diff == 0 {
//             return med as *mut c_void;
//         } else if diff > 0 {
//             start = (med as usize + size) as *const c_void;
//             len -= 1;
//         }
//         len >>= 1;
//     }
//     ptr::null_mut()
// }

// pub unsafe fn parse_mode_flags(mode_str: *const ::c_char) -> i32 {
//     let mut flags = if !strchr(mode_str, b'+' as i32).is_null() {
//         O_RDWR
//     } else if (*mode_str) == b'r' as i8 {
//         O_RDONLY
//     } else {
//         O_WRONLY
//     };
//     if !strchr(mode_str, b'x' as i32).is_null() {
//         flags |= O_EXCL;
//     }
//     if !strchr(mode_str, b'e' as i32).is_null() {
//         flags |= O_CLOEXEC;
//     }
//     if (*mode_str) != b'r' as i8 {
//         flags |= O_CREAT;
//     }
//     if (*mode_str) == b'w' as i8 {
//         flags |= O_TRUNC;
//     } else if (*mode_str) == b'a' as i8 {
//         flags |= O_APPEND;
//     }

//     flags
// }
// #[no_mangle]
// pub extern "C" fn fopen(filename: *const ::c_char, mode: *const ::c_char) -> *mut FILE{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "freopen$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn freopen(filename: *const ::c_char, mode: *const ::c_char, file: *mut FILE) -> *mut FILE{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn fflush(file: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fclose(file: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn remove(filename: *const ::c_char) -> ::c_int{
//     let r = unlink(filename);
//     if r == -EISDIR {
//         rmdir(filename)
//     } else {
//         r
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn tmpfile() -> *mut FILE{
// 	let mut file_name = *b"/tmp/tmpfileXXXXXX\0";
//     let file_name = file_name.as_mut_ptr() as *mut ::c_char;
//     let fd = mkstemp(file_name);

//     if fd < 0 {
//         return ptr::null_mut();
//     }

//     let fp = fdopen(fd, "w+".as_ptr() as *const i8);
//     {
//         unlink(file_name);
//     }

//     if fp.is_null() {
//         close(fd);
//     }

//     fp
// }
// #[no_mangle]
// pub extern "C" fn setvbuf(stream: *mut FILE, buffer: *mut ::c_char, mode: ::c_int, size: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setbuf(stream: *mut FILE, buf: *mut ::c_char){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getchar() -> ::c_int{
// 	fgetc(&mut *stdin)
// }
// #[no_mangle]
// pub extern "C" fn putchar(c: ::c_int) -> ::c_int{
// 	fputc(c, &mut *stdout)
// }
// #[no_mangle]
// pub extern "C" fn fgetc(stream: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fgets(buf: *mut ::c_char, n: ::c_int, stream: *mut FILE) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fputc(c: ::c_int, stream: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "fputs$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn fputs(s: *const ::c_char, stream: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn puts(s: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ungetc(c: ::c_int, stream: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fread(ptr: *mut c_void, size: ::size_t, nobj: ::size_t, stream: *mut FILE) -> ::size_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "fwrite$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn fwrite(ptr: *const c_void, size: ::size_t, nobj: ::size_t, stream: *mut FILE) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fseek(stream: *mut FILE, offset: c_long, whence: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ftell(stream: *mut FILE) -> c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn rewind(stream: *mut FILE){
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__fgetpos50")]
// #[no_mangle]
// pub extern "C" fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__fsetpos50")]
// #[no_mangle]
// pub extern "C" fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn feof(stream: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ferror(stream: *mut FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn clearerr(stream: *mut FILE){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn perror(s: *const ::c_char){
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn atof(s: *const ::c_char) -> c_double{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn atoi(s: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn atol(s: *const ::c_char) -> c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn atoll(s: *const ::c_char) -> c_longlong{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "strtod$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn strtod(s: *const ::c_char, endp: *mut *mut ::c_char) -> c_double{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtof(s: *const ::c_char, endp: *mut *mut ::c_char) -> c_float{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtol(s: *const ::c_char, endp: *mut *mut ::c_char, base: ::c_int) -> c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtoll(s: *const ::c_char, endp: *mut *mut ::c_char, base: ::c_int) -> c_longlong{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtoul(s: *const ::c_char, endp: *mut *mut ::c_char, base: ::c_int) -> c_ulong{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtoull(s: *const ::c_char, endp: *mut *mut ::c_char, base: ::c_int) -> c_ulonglong{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn calloc(nobj: ::size_t, size: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn malloc(size: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn realloc(p: *mut c_void, size: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn free(p: *mut c_void){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn abort() -> !{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn _exit(status: ::c_int) -> !{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "system$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn system(s: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getenv(s: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn strcpy(dst: *mut ::c_char, src: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strncpy(dst: *mut ::c_char, src: *const ::c_char, n: ::size_t) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn stpcpy(dst: *mut ::c_char, src: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strcat(s: *mut ::c_char, ct: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strncat(s: *mut ::c_char, ct: *const ::c_char, n: ::size_t) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strcmp(cs: *const ::c_char, ct: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strncmp(cs: *const ::c_char, ct: *const ::c_char, n: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strcoll(cs: *const ::c_char, ct: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strchr(cs: *const ::c_char, c: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strrchr(cs: *const ::c_char, c: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strspn(cs: *const ::c_char, ct: *const ::c_char) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strcspn(cs: *const ::c_char, ct: *const ::c_char) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strdup(cs: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strndup(cs: *const ::c_char, n: ::size_t) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strpbrk(cs: *const ::c_char, ct: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strstr(cs: *const ::c_char, ct: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strcasecmp(s1: *const ::c_char, s2: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strncasecmp(s1: *const ::c_char, s2: *const ::c_char, n: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub unsafe extern "C" fn strlen(cs: *const ::c_char) -> ::size_t{
// 	strnlen(cs,usize::MAX)
// }
// #[no_mangle]
// pub unsafe extern "C" fn strnlen(cs: *const ::c_char, maxlen: ::size_t) -> ::size_t{
// 	let mut i = 0;
//     while i < maxlen {
//         if *cs.add(i) == 0 {
//             break;
//         }
//         i += 1;
//     }
//     i as ::size_t
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "strerror$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn strerror(n: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtok(s: *mut ::c_char, t: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strtok_r(s: *mut ::c_char, t: *const ::c_char, p: *mut *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strxfrm(s: *mut ::c_char, ct: *const ::c_char, n: ::size_t) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strsignal(sig: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn wcslen(buf: *const wchar_t) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn wcstombs(dest: *mut ::c_char, src: *const wchar_t, n: ::size_t) -> ::size_t{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn memchr(cx: *const c_void, c: ::c_int, n: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn wmemchr(cx: *const wchar_t, c: wchar_t, n: ::size_t) -> *mut wchar_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memcmp(cx: *const c_void, ct: *const c_void, n: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memset(dest: *mut c_void, c: ::c_int, n: ::size_t) -> *mut c_void{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn fprintf(stream: *mut ::FILE, format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn printf(format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn snprintf(s: *mut ::c_char, n: ::size_t, format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sprintf(s: *mut ::c_char, format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "linux", not(target_env = "uclibc")),
//     link_name = "__isoc99_fscanf"
// )]
// #[no_mangle]
// pub extern "C" fn fscanf(stream: *mut ::FILE, format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "linux", not(target_env = "uclibc")),
//     link_name = "__isoc99_scanf"
// )]
// #[no_mangle]
// pub extern "C" fn scanf(format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "linux", not(target_env = "uclibc")),
//     link_name = "__isoc99_sscanf"
// )]
// #[no_mangle]
// pub extern "C" fn sscanf(s: *const ::c_char, format: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getchar_unlocked() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn putchar_unlocked(c: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "connect$UNIX2003"
// )]
// #[cfg_attr(target_os = "illumos", link_name = "__xnet_connect")]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_connect")]
// #[no_mangle]
// pub extern "C" fn connect(socket: ::c_int, address: *const sockaddr, len: socklen_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "listen$UNIX2003"
// )]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_listen")]
// #[no_mangle]
// pub extern "C" fn listen(socket: ::c_int, backlog: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]

// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "getpeername$UNIX2003"
// )]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_getpeername")]
// #[no_mangle]
// pub extern "C" fn getpeername(
//     socket: ::c_int,
//     address: *mut sockaddr,
//     address_len: *mut socklen_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "getsockname$UNIX2003"
// )]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_getsockname")]
// #[no_mangle]
// pub extern "C" fn getsockname(
//     socket: ::c_int,
//     address: *mut sockaddr,
//     address_len: *mut socklen_t,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "socketpair$UNIX2003"
// )]
// #[cfg_attr(target_os = "illumos", link_name = "__xnet_socketpair")]

// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]



// #[cfg_attr(
//     all(target_os = "macos", not(target_arch = "aarch64")),
//     link_name = "stat$INODE64"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__stat50")]
// #[cfg_attr(
//     all(target_os = "freebsd", any(freebsd11, freebsd10)),
//     link_name = "stat@FBSD_1.0"
// )]
// #[no_mangle]
// pub extern "C" fn stat(path: *const ::c_char, buf: *mut stat) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pclose(stream: *mut ::FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "fdopen$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn fdopen(fd: ::c_int, mode: *const ::c_char) -> *mut ::FILE{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fileno(stream: *mut ::FILE) -> ::c_int{
// 	unimplemented!()
// }


// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "creat$UNIX2003"
// )]

// #[no_mangle]
// pub extern "C" fn creat(path: *const ::c_char, mode: mode_t) -> ::c_int{
// 	unimplemented!()
// }



// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86_64"),
//     link_name = "opendir$INODE64"
// )]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "opendir$INODE64$UNIX2003"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__opendir30")]
// #[no_mangle]
// pub extern "C" fn opendir(dirname: *const ::c_char) -> *mut ::DIR{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", not(target_arch = "aarch64")),
//     link_name = "readdir$INODE64"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__readdir30")]
// #[cfg_attr(
//     all(target_os = "freebsd", any(freebsd11, freebsd10)),
//     link_name = "readdir@FBSD_1.0"
// )]
// #[no_mangle]
// pub extern "C" fn readdir(dirp: *mut ::DIR) -> *mut ::dirent{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "closedir$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn closedir(dirp: *mut ::DIR) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86_64"),
//     link_name = "rewinddir$INODE64"
// )]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "rewinddir$INODE64$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn rewinddir(dirp: *mut ::DIR){
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn fchmodat(
//     dirfd: ::c_int,
//     pathname: *const ::c_char,
//     mode: ::mode_t,
//     flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn fchownat(
//     dirfd: ::c_int,
//     pathname: *const ::c_char,
//     owner: ::uid_t,
//     group: ::gid_t,
//     flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", not(target_arch = "aarch64")),
//     link_name = "fstatat$INODE64"
// )]
// #[cfg_attr(
//     all(target_os = "freebsd", any(freebsd11, freebsd10)),
//     link_name = "fstatat@FBSD_1.1"
// )]
// #[no_mangle]
// pub extern "C" fn fstatat(
//     dirfd: ::c_int,
//     pathname: *const ::c_char,
//     buf: *mut stat,
//     flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn linkat(
//     olddirfd: ::c_int,
//     oldpath: *const ::c_char,
//     newdirfd: ::c_int,
//     newpath: *const ::c_char,
//     flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn renameat(
//     olddirfd: ::c_int,
//     oldpath: *const ::c_char,
//     newdirfd: ::c_int,
//     newpath: *const ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn symlinkat(
//     target: *const ::c_char,
//     newdirfd: ::c_int,
//     linkpath: *const ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn alarm(seconds: ::c_uint) -> ::c_uint{
// 	unimplemented!()
// }






// #[no_mangle]
// pub extern "C" fn execl(path: *const ::c_char, arg0: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn execle(path: *const ::c_char, arg0: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn execlp(file: *const ::c_char, arg0: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn execv(prog: *const ::c_char, argv_: *const *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn execvp(c: *const ::c_char, argv_: *const *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn fpathconf(filedes: ::c_int, name: ::c_int) -> c_long{
// 	unimplemented!()
// }




// #[no_mangle]
// pub extern "C" fn getgroups(ngroups_max: ::c_int, groups: *mut gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "illumos", link_name = "getloginx")]
// #[no_mangle]
// pub extern "C" fn getlogin() -> *mut ::c_char{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "getopt$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn getopt(argc: ::c_int, argv_: *const *mut ::c_char, optstr: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn getpgrp() -> ::pid_t{
// 	unimplemented!()
// }



// #[no_mangle]
// pub extern "C" fn isatty(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn pathconf(path: *const ::c_char, name: ::c_int) -> c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pipe(fds: *mut ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_memalign(memptr: *mut *mut ::c_void, align: ::size_t, size: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "read$UNIX2003"
// )]


// #[no_mangle]
// pub extern "C" fn seteuid(uid: uid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setegid(gid: gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setgid(gid: gid_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn setsid() -> ::pid_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setuid(uid: uid_t) -> ::c_int{
// 	unimplemented!()
// }


// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "sleep$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn sleep(secs: ::c_uint) -> ::c_uint{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn tcgetpgrp(fd: ::c_int) -> ::pid_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcsetpgrp(fd: ::c_int, pgrp: ::pid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ttyname(fd: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "ttyname_r$UNIX2003"
// )]
// #[cfg_attr(target_os = "illumos", link_name = "__posix_ttyname_r")]
// #[no_mangle]
// pub extern "C" fn ttyname_r(fd: ::c_int, buf: *mut ::c_char, buflen: ::size_t) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "wait$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn wait(status: *mut ::c_int) -> ::pid_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "waitpid$UNIX2003"
// )]

// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "write$UNIX2003"
// )]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pread$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pwrite$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t{
// 	unimplemented!()
// }


// #[cfg_attr(target_os = "netbsd", link_name = "__utime50")]
// #[no_mangle]
// pub extern "C" fn utime(file: *const ::c_char, buf: *const utimbuf) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "kill$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn kill(pid: ::pid_t, sig: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "killpg$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn killpg(pgrp: ::pid_t, sig: ::c_int) -> ::c_int{
// 	unimplemented!()
// }








// #[no_mangle]
// pub extern "C" fn if_nametoindex(ifname: *const ::c_char) -> ::c_uint{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn if_indextoname(ifindex: ::c_uint, ifname: *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", not(target_arch = "aarch64")),
//     link_name = "lstat$INODE64"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__lstat50")]
// #[cfg_attr(
//     all(target_os = "freebsd", any(freebsd11, freebsd10)),
//     link_name = "lstat@FBSD_1.0"
// )]
// #[no_mangle]
// pub extern "C" fn lstat(path: *const ::c_char, buf: *mut stat) -> ::c_int{
// 	unimplemented!()
// }



// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "setenv$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn setenv(name: *const ::c_char, val: *const ::c_char, overwrite: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "unsetenv$UNIX2003"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__unsetenv13")]
// #[no_mangle]
// pub extern "C" fn unsetenv(name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }



// #[no_mangle]
// pub extern "C" fn truncate(path: *const ::c_char, length: off_t) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn signal(signum: ::c_int, handler: sighandler_t) -> sighandler_t{
// 	unimplemented!()
// }

// #[cfg_attr(target_os = "netbsd", link_name = "__getrusage50")]
// #[no_mangle]
// pub extern "C" fn getrusage(resource: ::c_int, usage: *mut rusage) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(
//     any(
//         target_os = "macos",
//         target_os = "ios",
//         target_os = "tvos",
//         target_os = "watchos"
//     ),
//     link_name = "realpath$DARWIN_EXTSN"
// )]
// #[no_mangle]
// pub extern "C" fn realpath(pathname: *const ::c_char, resolved: *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }



// #[cfg_attr(target_os = "netbsd", link_name = "__times13")]
// #[no_mangle]
// pub extern "C" fn times(buf: *mut ::tms) -> ::clock_t{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_self() -> ::pthread_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_join$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_join(native: ::pthread_t, value: *mut *mut ::c_void) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_exit(value: *mut ::c_void) -> !{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_attr_init(attr: *mut ::pthread_attr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_attr_destroy(attr: *mut ::pthread_attr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_attr_setstacksize(attr: *mut ::pthread_attr_t, stack_size: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_attr_setdetachstate(attr: *mut ::pthread_attr_t, state: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_detach(thread: ::pthread_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__libc_thr_yield")]

// #[no_mangle]
// pub extern "C" fn pthread_key_create(
//     key: *mut pthread_key_t,
//     dtor: ::Option<unsafe extern "C" fn(*mut ::c_void)>,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_getspecific(key: pthread_key_t) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_setspecific(key: pthread_key_t, value: *const ::c_void) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutex_init(
//     lock: *mut pthread_mutex_t,
//     attr: *const pthread_mutexattr_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutex_trylock(lock: *mut pthread_mutex_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_mutexattr_destroy$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, _type: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_cond_init$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t)
//     -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_cond_wait$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_cond_wait(cond: *mut pthread_cond_t, lock: *mut pthread_mutex_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_cond_timedwait$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_cond_timedwait(
//     cond: *mut pthread_cond_t,
//     lock: *mut pthread_mutex_t,
//     abstime: *const ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_cond_signal(cond: *mut pthread_cond_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_init$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_init(
//     lock: *mut pthread_rwlock_t,
//     attr: *const pthread_rwlockattr_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_destroy$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_rdlock$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_tryrdlock$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_tryrdlock(lock: *mut pthread_rwlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_wrlock$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_trywrlock$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_trywrlock(lock: *mut pthread_rwlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "pthread_rwlock_unlock$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(target_os = "illumos", link_name = "__xnet_getsockopt")]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_getsockopt")]
// #[no_mangle]
// pub extern "C" fn getsockopt(
//     sockfd: ::c_int,
//     level: ::c_int,
//     optname: ::c_int,
//     optval: *mut ::c_void,
//     optlen: *mut ::socklen_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn raise(signum: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(target_os = "netbsd", link_name = "__utimes50")]
// #[no_mangle]
// pub extern "C" fn utimes(filename: *const ::c_char, times: *const ::timeval) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn dlopen(filename: *const ::c_char, flag: ::c_int) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn dlerror() -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn dlsym(handle: *mut ::c_void, symbol: *const ::c_char) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn dlclose(handle: *mut ::c_void) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]
// #[cfg_attr(target_os = "illumos", link_name = "__xnet_getaddrinfo")]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_getaddrinfo")]
// #[no_mangle]
// pub extern "C" fn getaddrinfo(
//     node: *const ::c_char,
//     service: *const ::c_char,
//     hints: *const addrinfo,
//     res: *mut *mut addrinfo,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg(not(all(
//     libc_cfg_target_vendor,
//     target_arch = "powerpc",
//     target_vendor = "nintendo"
// )))]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_freeaddrinfo")]
// #[no_mangle]
// pub extern "C" fn freeaddrinfo(res: *mut addrinfo){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn hstrerror(errcode: ::c_int) -> *const ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn gai_strerror(errcode: ::c_int) -> *const ::c_char{
// 	unimplemented!()
// }
// #[cfg_attr(
//     any(
//         all(
//             target_os = "linux",
//             not(any(target_env = "musl", target_env = "ohos"))
//         ),
//         target_os = "freebsd",
//         target_os = "dragonfly",
//         target_os = "haiku"
//     ),
//     link_name = "__res_init"
// )]
// #[cfg_attr(
//     any(
//         target_os = "macos",
//         target_os = "ios",
//         target_os = "tvos",
//         target_os = "watchos"
//     ),
//     link_name = "res_9_init"
// )]
// #[no_mangle]
// pub extern "C" fn res_init() -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(target_os = "netbsd", link_name = "__gmtime_r50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__localtime_r50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "mktime$UNIX2003"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__mktime50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn mktime(tm: *mut tm) -> time_t{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__time50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn time(time: *mut time_t) -> time_t{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__gmtime50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn gmtime(time_p: *const time_t) -> *mut tm{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__locatime50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn localtime(time_p: *const time_t) -> *mut tm{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__difftime50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn difftime(time1: time_t, time0: time_t) -> ::c_double{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__timegm50")]
// #[cfg_attr(any(target_env = "musl", target_env = "ohos"), allow(deprecated))]
// // FIXME: for `time_t`
// #[no_mangle]
// pub extern "C" fn timegm(tm: *mut ::tm) -> time_t{
// 	unimplemented!()
// }

// #[cfg_attr(target_os = "netbsd", link_name = "__mknod50")]
// #[cfg_attr(
//     all(target_os = "freebsd", any(freebsd11, freebsd10)),
//     link_name = "mknod@FBSD_1.0"
// )]
// #[no_mangle]
// pub extern "C" fn mknod(pathname: *const ::c_char, mode: ::mode_t, dev: ::dev_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn gethostname(name: *mut ::c_char, len: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn endservent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getservbyname(name: *const ::c_char, proto: *const ::c_char) -> *mut servent{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getservbyport(port: ::c_int, proto: *const ::c_char) -> *mut servent{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getservent() -> *mut servent{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setservent(stayopen: ::c_int){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getprotobyname(name: *const ::c_char) -> *mut protoent{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getprotobynumber(proto: ::c_int) -> *mut protoent{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn chroot(name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "usleep$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn usleep(secs: ::c_uint) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "send$UNIX2003"
// )]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_send")]
// #[no_mangle]
// pub extern "C" fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t, flags: ::c_int) -> ::ssize_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "recv$UNIX2003"
// )]
// #[cfg_attr(target_os = "espidf", link_name = "lwip_recv")]
// #[no_mangle]
// pub extern "C" fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::ssize_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "putenv$UNIX2003"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__putenv50")]
// #[no_mangle]
// pub extern "C" fn putenv(string: *mut ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "poll$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86_64"),
//     link_name = "select$1050"
// )]
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "select$UNIX2003"
// )]
// #[cfg_attr(target_os = "netbsd", link_name = "__select50")]
// #[no_mangle]
// pub extern "C" fn select(
//     nfds: ::c_int,
//     readfds: *mut fd_set,
//     writefds: *mut fd_set,
//     errorfds: *mut fd_set,
//     timeout: *mut timeval,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__setlocale50")]
// #[no_mangle]
// pub extern "C" fn setlocale(category: ::c_int, locale: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn localeconv() -> *mut lconv{
// 	unimplemented!()
// }

// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "sem_wait$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn sem_wait(sem: *mut sem_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_trywait(sem: *mut sem_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_post(sem: *mut sem_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn statvfs(path: *const ::c_char, buf: *mut statvfs) -> ::c_int{
// 	unimplemented!()
// }


// #[cfg_attr(target_os = "netbsd", link_name = "__sigemptyset14")]
// #[no_mangle]
// pub extern "C" fn sigemptyset(set: *mut sigset_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__sigaddset14")]
// #[no_mangle]
// pub extern "C" fn sigaddset(set: *mut sigset_t, signum: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__sigfillset14")]
// #[no_mangle]
// pub extern "C" fn sigfillset(set: *mut sigset_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__sigdelset14")]
// #[no_mangle]
// pub extern "C" fn sigdelset(set: *mut sigset_t, signum: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__sigismember14")]
// #[no_mangle]
// pub extern "C" fn sigismember(set: *const sigset_t, signum: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[cfg_attr(target_os = "netbsd", link_name = "__sigprocmask14")]
// #[no_mangle]
// pub extern "C" fn sigprocmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "netbsd", link_name = "__sigpending14")]
// #[no_mangle]
// pub extern "C" fn sigpending(set: *mut sigset_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn sysconf(name: ::c_int) -> ::c_long{
// 	unimplemented!()
// }



// #[no_mangle]
// pub extern "C" fn fseeko(stream: *mut ::FILE, offset: ::off_t, whence: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ftello(stream: *mut ::FILE) -> ::off_t{
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "tcdrain$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn tcdrain(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn cfgetispeed(termios: *const ::termios) -> ::speed_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn cfgetospeed(termios: *const ::termios) -> ::speed_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn cfsetispeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn cfsetospeed(termios: *mut ::termios, speed: ::speed_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcgetattr(fd: ::c_int, termios: *mut ::termios) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcsetattr(fd: ::c_int, optional_actions: ::c_int, termios: *const ::termios) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcflow(fd: ::c_int, action: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcflush(fd: ::c_int, action: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcgetsid(fd: ::c_int) -> ::pid_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tcsendbreak(fd: ::c_int, duration: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// fn get_nstime() -> u64 {
//     let mut ts = MaybeUninit::uninit();
//     clock_gettime(CLOCK_MONOTONIC, ts.as_mut_ptr());
//     unsafe { ts.assume_init() }.tv_nsec as u64
// }
// fn inner_mktemp<T, F>(name: *mut ::c_char, suffix_len: ::c_int, mut attempt: F) -> Option<T>
// where
//     F: FnMut() -> Option<T>,
// {
//     let len = unsafe { strlen(name) as ::c_int };

//     if len < 6 || suffix_len > len - 6 {
//         unsafe { errno = EINVAL };
//         return None;
//     }

//     for i in (len - suffix_len - 6)..(len - suffix_len) {
//         if unsafe { *name.offset(i as isize) } != b'X' as ::c_char {
//             unsafe { errno = EINVAL };
//             return None;
//         }
//     }

//     let mut rng = JitterRng::new_with_timer(get_nstime);
//     let _ = rng.test_timer();

//     for _ in 0..100 {
//         let char_iter = iter::repeat(())
//             .map(|()| rng.sample(Alphanumeric))
//             .take(6)
//             .enumerate();
//         unsafe {
//             for (i, c) in char_iter {
//                 *name.offset((len as isize) - (suffix_len as isize) - (i as isize) - 1) =
//                     c as ::c_char
//             }
//         }

//         if let result @ Some(_) = attempt() {
//             return result;
//         }
//     }

//     unsafe { errno = EEXIST }

//     None
// }
// #[no_mangle]
// pub extern "C" fn mkostemps(name: *mut ::c_char, suffix_len: ::c_int, mut flags: ::c_int) -> ::c_int {
//     flags &= !O_ACCMODE;
//     flags |= O_RDWR | O_CREAT | O_EXCL;

//     inner_mktemp(name, suffix_len, || {
//         let fd = open(name, flags, 0o600);

//         if fd >= 0 {
//             Some(fd)
//         } else {
//             None
//         }
//     })
//     .unwrap_or(-1)
// }
// #[no_mangle]
// pub extern "C" fn mkstemp(template: *mut ::c_char) -> ::c_int{
// 	mkostemps(template, 0, 0)
// }
// #[no_mangle]
// pub extern "C" fn mkdtemp(template: *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn tmpnam(ptr: *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn openlog(ident: *const ::c_char, logopt: ::c_int, facility: ::c_int){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn closelog(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setlogmask(maskpri: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[cfg_attr(target_os = "macos", link_name = "syslog$DARWIN_EXTSN")]
// #[no_mangle]
// pub extern "C" fn syslog(priority: ::c_int, message: *const ::c_char){
// 	unimplemented!()
// }
// #[cfg_attr(
//     all(target_os = "macos", target_arch = "x86"),
//     link_name = "nice$UNIX2003"
// )]
// #[no_mangle]
// pub extern "C" fn nice(incr: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn grantpt(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_openpt(flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ptsname(fd: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn unlockpt(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn strcasestr(cs: *const ::c_char, ct: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getline(lineptr: *mut *mut ::c_char, n: *mut ::size_t, stream: *mut FILE) -> ssize_t{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn lockf(fd: ::c_int, cmd: ::c_int, len: ::off_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn stpncpy(dst: *mut ::c_char, src: *const ::c_char, n: ::size_t) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn dladdr(addr: *const ::c_void, info: *mut Dl_info) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn open_wmemstream(
//     ptr: *mut *mut wchar_t,
//     sizeloc: *mut ::size_t,
// ) -> *mut FILE{
// 	unimplemented!()
// }

// #[doc(hidden)]
// #[no_mangle]
// pub extern "C" fn __libc_current_sigrtmax() -> ::c_int{
// 	unimplemented!()
// }
// #[doc(hidden)]
// #[no_mangle]
// pub extern "C" fn __libc_current_sigrtmin() -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn sem_destroy(sem: *mut sem_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_init(sem: *mut sem_t, pshared: ::c_int, value: ::c_uint) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fdatasync(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mincore(addr: *mut ::c_void, len: ::size_t, vec: *mut ::c_uchar) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn clock_getres(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn clock_settime(clk_id: ::clockid_t, tp: *const ::timespec) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn clock_getcpuclockid(pid: ::pid_t, clk_id: *mut ::clockid_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn dirfd(dirp: *mut ::DIR) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_getattr_np(native: ::pthread_t, attr: *mut ::pthread_attr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_attr_getstack(
// 	attr: *const ::pthread_attr_t,
// 	stackaddr: *mut *mut ::c_void,
// 	stacksize: *mut ::size_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setgroups(ngroups: ::size_t, ptr: *const ::gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int{
// 	if flags == 0 {
// 		e(unsafe { syscall!(SYS_PIPE, fds) }) as ::c_int
// 	} else {
// 		unimplemented!()
// 	}
// }
// #[no_mangle]
// pub extern "C" fn statfs(path: *const ::c_char, buf: *mut statfs) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fstatfs(fd: ::c_int, buf: *mut statfs) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memrchr(cx: *const ::c_void, c: ::c_int, n: ::size_t) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t, advise: ::c_int) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn duplocale(base: ::locale_t) -> ::locale_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn freelocale(loc: ::locale_t){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn newlocale(mask: ::c_int, locale: *const ::c_char, base: ::locale_t) -> ::locale_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn uselocale(loc: ::locale_t) -> ::locale_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mknodat(
// 	dirfd: ::c_int,
// 	pathname: *const ::c_char,
// 	mode: ::mode_t,
// 	dev: dev_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_condattr_getclock(
// 	attr: *const pthread_condattr_t,
// 	clock_id: *mut clockid_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_condattr_setclock(
// 	attr: *mut pthread_condattr_t,
// 	clock_id: ::clockid_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_setpshared(
// 	attr: *mut pthread_mutexattr_t,
// 	pshared: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_rwlockattr_getpshared(
// 	attr: *const pthread_rwlockattr_t,
// 	val: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, val: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ptsname_r(fd: ::c_int, buf: *mut ::c_char, buflen: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn clearenv() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn waitid(idtype: idtype_t, id: id_t, infop: *mut ::siginfo_t, options: ::c_int)
// 	-> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getresuid(ruid: *mut ::uid_t, euid: *mut ::uid_t, suid: *mut ::uid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getresgid(rgid: *mut ::gid_t, egid: *mut ::gid_t, sgid: *mut ::gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn acct(filename: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn sbrk(increment: ::intptr_t) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[deprecated(
// 	since = "0.2.66",
// 	note = "causes memory corruption, see rust-lang/libc#1596"
// )]
// #[no_mangle]
// pub extern "C" fn vfork() -> ::pid_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setresgid(rgid: ::gid_t, egid: ::gid_t, sgid: ::gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setresuid(ruid: ::uid_t, euid: ::uid_t, suid: ::uid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn wait4(
// 	pid: ::pid_t,
// 	status: *mut ::c_int,
// 	options: ::c_int,
// 	rusage: *mut ::rusage,
// ) -> ::pid_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn login_tty(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn execvpe(
// 	file: *const ::c_char,
// 	argv_: *const *const ::c_char,
// 	envp: *const *const ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fexecve(
// 	fd: ::c_int,
// 	argv_: *const *const ::c_char,
// 	envp: *const *const ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn freeifaddrs(ifa: *mut ::ifaddrs){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn readv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn sendmsg(fd: ::c_int, msg: *const ::msghdr, flags: ::c_int) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn recvmsg(fd: ::c_int, msg: *mut ::msghdr, flags: ::c_int) -> ::ssize_t{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn strchrnul(s: *const ::c_char, c: ::c_int) -> *mut ::c_char{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn pause() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mkdirat(dirfd: ::c_int, pathname: *const ::c_char,
// 				mode: ::mode_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn openat(dirfd: ::c_int, pathname: *const ::c_char,
// 				flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fdopendir(fd: ::c_int) -> *mut ::DIR{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn readdir_r(dirp: *mut ::DIR, entry: *mut ::dirent,
// 					result: *mut *mut ::dirent) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sendmmsg(
// 	sockfd: ::c_int,
// 	msgvec: *mut ::mmsghdr,
// 	vlen: ::c_uint,
// 	flags: ::c_uint,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn recvmmsg(
// 	sockfd: ::c_int,
// 	msgvec: *mut ::mmsghdr,
// 	vlen: ::c_uint,
// 	flags: ::c_uint,
// 	timeout: *mut ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn setrlimit(resource: ::c_int, rlim: *const ::rlimit) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn prlimit(
// 	pid: ::pid_t,
// 	resource: ::c_int,
// 	new_limit: *const ::rlimit,
// 	old_limit: *mut ::rlimit,
// ) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn getpriority(which: ::c_int, who: ::id_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setpriority(which: ::c_int, who: ::id_t, prio: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// // Musl targets need the `mask` argument of `fanotify_mark` be specified
// // `::c_ulonglong` instead of `u64` or there will be a type mismatch between
// // `long long unsigned int` and the expected `uint64_t`.
// #[no_mangle]
// pub extern "C" fn fanotify_mark(
// 	fd: ::c_int,
// 	flags: ::c_uint,
// 	mask: ::c_ulonglong,
// 	dirfd: ::c_int,
// 	path: *const ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getauxval(type_: ::c_ulong) -> ::c_ulong{
// 	unimplemented!()
// }

// // Added in `musl` 1.1.20
// #[no_mangle]
// pub extern "C" fn explicit_bzero(s: *mut ::c_void, len: ::size_t){
// 	unimplemented!()
// }
// // Added in `musl` 1.2.2
// #[no_mangle]
// pub extern "C" fn reallocarray(ptr: *mut ::c_void, nmemb: ::size_t, size: ::size_t) -> *mut ::c_void{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn adjtimex(buf: *mut ::timex) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn clock_adjtime(clk_id: ::clockid_t, buf: *mut ::timex) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn ctermid(s: *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn memfd_create(name: *const ::c_char, flags: ::c_uint) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mlock2(addr: *const ::c_void, len: ::size_t, flags: ::c_uint) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn malloc_usable_size(ptr: *mut ::c_void) -> ::size_t{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn euidaccess(pathname: *const ::c_char, mode: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn eaccess(pathname: *const ::c_char, mode: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn asctime_r(tm: *const ::tm, buf: *mut ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn strftime(
// 	s: *mut ::c_char,
// 	max: ::size_t,
// 	format: *const ::c_char,
// 	tm: *const ::tm,
// ) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strptime(s: *const ::c_char, format: *const ::c_char, tm: *mut ::tm) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub unsafe extern "C" fn dirname(path: *mut ::c_char) -> *mut ::c_char{
// 	if path.is_null() || strlen(path) == 0 {
//         return ".\0".as_ptr() as *mut ::c_char;
//     }
//     let mut end = strlen(path) as isize - 1;
//     while end > 0 && *path.offset(end) == b'/' as ::c_char {
//         end -= 1;
//     }
//     while end >= 0 && *path.offset(end) != b'/' as ::c_char {
//         end -= 1;
//     }
//     while end > 0 && *path.offset(end) == b'/' as ::c_char {
//         end -= 1;
//     }
//     if end == -1 {
//         return ".\0".as_ptr() as *mut ::c_char;
//     }
//     *path.offset(end + 1) = 0;
//     path
// }
// #[no_mangle]
// pub unsafe extern "C" fn basename(path: *mut ::c_char) -> *mut ::c_char{
// 	if path.is_null() || strlen(path) == 0 {
//         return ".\0".as_ptr() as *mut ::c_char;
//     }
//     let mut end = strlen(path) as isize - 1;
//     while end >= 0 && *path.offset(end) == b'/' as ::c_char {
//         end -= 1;
//     }
//     if end == -1 {
//         return "/\0".as_ptr() as *mut ::c_char;
//     }
//     let mut begin = end;
//     while begin >= 0 && *path.offset(begin) != b'/' as ::c_char {
//         begin -= 1;
//     }
//     *path.offset(end + 1) = 0;
//     path.offset(begin + 1) as *mut ::c_char
// }
// #[no_mangle]
// pub extern "C" fn aio_read(aiocbp: *mut aiocb) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn aio_write(aiocbp: *mut aiocb) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn aio_fsync(op: ::c_int, aiocbp: *mut aiocb) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn aio_error(aiocbp: *const aiocb) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn aio_return(aiocbp: *mut aiocb) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn aio_suspend(
// 	aiocb_list: *const *const aiocb,
// 	nitems: ::c_int,
// 	timeout: *const ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn aio_cancel(fd: ::c_int, aiocbp: *mut aiocb) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn lio_listio(
// 	mode: ::c_int,
// 	aiocb_list: *const *mut aiocb,
// 	nitems: ::c_int,
// 	sevp: *mut ::sigevent,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pwritev(
// 	fd: ::c_int,
// 	iov: *const ::iovec,
// 	iovcnt: ::c_int,
// 	offset: ::off_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn preadv(
// 	fd: ::c_int,
// 	iov: *const ::iovec,
// 	iovcnt: ::c_int,
// 	offset: ::off_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getnameinfo(
// 	sa: *const ::sockaddr,
// 	salen: ::socklen_t,
// 	host: *mut ::c_char,
// 	hostlen: ::socklen_t,
// 	serv: *mut ::c_char,
// 	sevlen: ::socklen_t,
// 	flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getloadavg(
// 	loadavg: *mut ::c_double,
// 	nelem: ::c_int
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn process_vm_readv(
// 	pid: ::pid_t,
// 	local_iov: *const ::iovec,
// 	liovcnt: ::c_ulong,
// 	remote_iov: *const ::iovec,
// 	riovcnt: ::c_ulong,
// 	flags: ::c_ulong,
// ) -> isize{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn process_vm_writev(
// 	pid: ::pid_t,
// 	local_iov: *const ::iovec,
// 	liovcnt: ::c_ulong,
// 	remote_iov: *const ::iovec,
// 	riovcnt: ::c_ulong,
// 	flags: ::c_ulong,
// ) -> isize{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn futimes(
// 	fd: ::c_int,
// 	times: *const ::timeval
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getspnam_r(
// 	name: *const ::c_char,
// 	spbuf: *mut spwd,
// 	buf: *mut ::c_char,
// 	buflen: ::size_t,
// 	spbufp: *mut *mut spwd,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn shm_open(name: *const ::c_char, oflag: ::c_int, mode: mode_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn shm_unlink(name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn mq_open(name: *const ::c_char, oflag: ::c_int) -> ::mqd_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_close(mqd: ::mqd_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_unlink(name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_receive(
// 	mqd: ::mqd_t,
// 	msg_ptr: *mut ::c_char,
// 	msg_len: ::size_t,
// 	msg_prio: *mut ::c_uint,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_timedreceive(
// 	mqd: ::mqd_t,
// 	msg_ptr: *mut ::c_char,
// 	msg_len: ::size_t,
// 	msg_prio: *mut ::c_uint,
// 	abs_timeout: *const ::timespec,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_send(
// 	mqd: ::mqd_t,
// 	msg_ptr: *const ::c_char,
// 	msg_len: ::size_t,
// 	msg_prio: ::c_uint,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_timedsend(
// 	mqd: ::mqd_t,
// 	msg_ptr: *const ::c_char,
// 	msg_len: ::size_t,
// 	msg_prio: ::c_uint,
// 	abs_timeout: *const ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_getattr(mqd: ::mqd_t, attr: *mut ::mq_attr) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mq_setattr(
// 	mqd: ::mqd_t,
// 	newattr: *const ::mq_attr,
// 	oldattr: *mut ::mq_attr
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_mutex_consistent(mutex: *mut pthread_mutex_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_cancel(thread: ::pthread_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_getrobust(
// 	attr: *const pthread_mutexattr_t,
// 	robustness: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_setrobust(
// 	attr: *mut pthread_mutexattr_t,
// 	robustness: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn strerror_r(errnum: ::c_int, buf: *mut ::c_char, buflen: ::size_t) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn abs(i: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn labs(i: ::c_long) -> ::c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn rand() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn srand(seed: ::c_uint){
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn drand48() -> ::c_double{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn erand48(xseed: *mut ::c_ushort) -> ::c_double{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn lrand48() -> ::c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn nrand48(xseed: *mut ::c_ushort) -> ::c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mrand48() -> ::c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn jrand48(xseed: *mut ::c_ushort) -> ::c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn srand48(seed: ::c_long){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn seed48(xseed: *mut ::c_ushort) -> *mut ::c_ushort{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn lcong48(p: *mut ::c_ushort){
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn lutimes(file: *const ::c_char, times: *const ::timeval) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn setpwent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn endpwent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getpwent() -> *mut passwd{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setgrent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn endgrent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getgrent() -> *mut ::group{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setspent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn endspent(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getspent() -> *mut spwd{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn getspnam(name: *const ::c_char) -> *mut spwd{
// 	unimplemented!()
// }

// // System V IPC
// #[no_mangle]
// pub extern "C" fn shmget(key: ::key_t, size: ::size_t, shmflg: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn shmat(shmid: ::c_int, shmaddr: *const ::c_void, shmflg: ::c_int) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn shmdt(shmaddr: *const ::c_void) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn shmctl(shmid: ::c_int, cmd: ::c_int, buf: *mut ::shmid_ds) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ftok(pathname: *const ::c_char, proj_id: ::c_int) -> ::key_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn semget(key: ::key_t, nsems: ::c_int, semflag: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn semop(semid: ::c_int, sops: *mut ::sembuf, nsops: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn semctl(semid: ::c_int, semnum: ::c_int, cmd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn msgctl(msqid: ::c_int, cmd: ::c_int, buf: *mut msqid_ds) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn msgget(key: ::key_t, msgflg: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn msgrcv(
// 	msqid: ::c_int,
// 	msgp: *mut ::c_void,
// 	msgsz: ::size_t,
// 	msgtyp: ::c_long,
// 	msgflg: ::c_int,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn msgsnd(
// 	msqid: ::c_int,
// 	msgp: *const ::c_void,
// 	msgsz: ::size_t,
// 	msgflg: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn __errno_location() -> *mut ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn fallocate(fd: ::c_int, mode: ::c_int, offset: ::off_t, len: ::off_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_fallocate(fd: ::c_int, offset: ::off_t, len: ::off_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn readahead(fd: ::c_int, offset: ::off64_t, count: ::size_t) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getxattr(
// 	path: *const ::c_char,
// 	name: *const ::c_char,
// 	value: *mut ::c_void,
// 	size: ::size_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn lgetxattr(
// 	path: *const ::c_char,
// 	name: *const ::c_char,
// 	value: *mut ::c_void,
// 	size: ::size_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fgetxattr(
// 	filedes: ::c_int,
// 	name: *const ::c_char,
// 	value: *mut ::c_void,
// 	size: ::size_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setxattr(
// 	path: *const ::c_char,
// 	name: *const ::c_char,
// 	value: *const ::c_void,
// 	size: ::size_t,
// 	flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn lsetxattr(
// 	path: *const ::c_char,
// 	name: *const ::c_char,
// 	value: *const ::c_void,
// 	size: ::size_t,
// 	flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fsetxattr(
// 	filedes: ::c_int,
// 	name: *const ::c_char,
// 	value: *const ::c_void,
// 	size: ::size_t,
// 	flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn listxattr(path: *const ::c_char, list: *mut ::c_char, size: ::size_t) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn llistxattr(path: *const ::c_char, list: *mut ::c_char, size: ::size_t) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn flistxattr(filedes: ::c_int, list: *mut ::c_char, size: ::size_t) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn removexattr(path: *const ::c_char, name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn lremovexattr(path: *const ::c_char, name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fremovexattr(filedes: ::c_int, name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn signalfd(fd: ::c_int, mask: *const ::sigset_t, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timerfd_create(clockid: ::clockid_t, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timerfd_gettime(fd: ::c_int, curr_value: *mut itimerspec) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timerfd_settime(
// 	fd: ::c_int,
// 	flags: ::c_int,
// 	new_value: *const itimerspec,
// 	old_value: *mut itimerspec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn quotactl(
// 	cmd: ::c_int,
// 	special: *const ::c_char,
// 	id: ::c_int,
// 	data: *mut ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn dup3(oldfd: ::c_int, newfd: ::c_int, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mkostemps(template: *mut ::c_char, suffixlen: ::c_int, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sigtimedwait(
// 	set: *const sigset_t,
// 	info: *mut siginfo_t,
// 	timeout: *const ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn nl_langinfo_l(item: ::nl_item, locale: ::locale_t) -> *mut ::c_char{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn accept4(
// 	fd: ::c_int,
// 	addr: *mut ::sockaddr,
// 	len: *mut ::socklen_t,
// 	flg: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_getaffinity_np(
// 	thread: ::pthread_t,
// 	cpusetsize: ::size_t,
// 	cpuset: *mut ::cpu_set_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_setaffinity_np(
// 	thread: ::pthread_t,
// 	cpusetsize: ::size_t,
// 	cpuset: *const ::cpu_set_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_setschedprio(native: ::pthread_t, priority: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn reboot(how_to: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setfsgid(gid: ::gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setfsuid(uid: ::uid_t) -> ::c_int{
// 	unimplemented!()
// }

// // Not available now on Android
// #[no_mangle]
// pub extern "C" fn mkfifoat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn if_nameindex() -> *mut if_nameindex{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn if_freenameindex(ptr: *mut if_nameindex){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sync_file_range(
// 	fd: ::c_int,
// 	offset: ::off64_t,
// 	nbytes: ::off64_t,
// 	flags: ::c_uint,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mremap(
// 	addr: *mut ::c_void,
// 	len: ::size_t,
// 	new_len: ::size_t,
// 	flags: ::c_int
// ) -> *mut ::c_void{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn glob(
// 	pattern: *const ::c_char,
// 	flags: ::c_int,
// 	errfunc: ::Option<extern "C" fn(epath: *const ::c_char, errno: ::c_int) -> ::c_int>,
// 	pglob: *mut ::glob_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn globfree(pglob: *mut ::glob_t){
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn seekdir(dirp: *mut ::DIR, loc: ::c_long){
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn telldir(dirp: *mut ::DIR) -> ::c_long{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int{
// 	unimplemented!()
// }


// #[no_mangle]
// pub extern "C" fn remap_file_pages(
// 	addr: *mut ::c_void,
// 	size: ::size_t,
// 	prot: ::c_int,
// 	pgoff: ::size_t,
// 	flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn mkstemps(template: *mut ::c_char, suffixlen: ::c_int) -> ::c_int{
// 	mkostemps(template, suffixlen, 0)
// }

// #[no_mangle]
// pub extern "C" fn nl_langinfo(item: ::nl_item) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn getdomainname(name: *mut ::c_char, len: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setdomainname(name: *const ::c_char, len: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn vhangup() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sync(){
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn syncfs(fd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_getaffinity(pid: ::pid_t, cpusetsize: ::size_t, cpuset: *mut cpu_set_t)
// 	-> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_setaffinity(
// 	pid: ::pid_t,
// 	cpusetsize: ::size_t,
// 	cpuset: *const cpu_set_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn epoll_create(size: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn epoll_wait(
// 	epfd: ::c_int,
// 	events: *mut ::epoll_event,
// 	maxevents: ::c_int,
// 	timeout: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_getschedparam(
// 	native: ::pthread_t,
// 	policy: *mut ::c_int,
// 	param: *mut ::sched_param,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn unshare(flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn umount(target: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_get_priority_max(policy: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tee(fd_in: ::c_int, fd_out: ::c_int, len: ::size_t, flags: ::c_uint) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn splice(
// 	fd_in: ::c_int,
// 	off_in: *mut ::loff_t,
// 	fd_out: ::c_int,
// 	off_out: *mut ::loff_t,
// 	len: ::size_t,
// 	flags: ::c_uint,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_timedwait(sem: *mut sem_t, abstime: *const ::timespec) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_getvalue(sem: *mut sem_t, sval: *mut ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_setparam(pid: ::pid_t, param: *const ::sched_param) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn setns(fd: ::c_int, nstype: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn swapoff(path: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn vmsplice(
// 	fd: ::c_int,
// 	iov: *const ::iovec,
// 	nr_segs: ::size_t,
// 	flags: ::c_uint,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn mount(
// 	src: *const ::c_char,
// 	target: *const ::c_char,
// 	fstype: *const ::c_char,
// 	flags: ::c_ulong,
// 	data: *const ::c_void,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn personality(persona: ::c_ulong) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn prctl(option: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_getparam(pid: ::pid_t, param: *mut ::sched_param) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ppoll(
// 	fds: *mut ::pollfd,
// 	nfds: nfds_t,
// 	timeout: *const ::timespec,
// 	sigmask: *const sigset_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_getprotocol(
// 	attr: *const pthread_mutexattr_t,
// 	protocol: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_setprotocol(
// 	attr: *mut pthread_mutexattr_t,
// 	protocol: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_mutex_timedlock(
// 	lock: *mut pthread_mutex_t,
// 	abstime: *const ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrierattr_init(attr: *mut ::pthread_barrierattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrierattr_destroy(attr: *mut ::pthread_barrierattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrierattr_getpshared(
// 	attr: *const ::pthread_barrierattr_t,
// 	shared: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrierattr_setpshared(
// 	attr: *mut ::pthread_barrierattr_t,
// 	shared: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrier_init(
// 	barrier: *mut pthread_barrier_t,
// 	attr: *const ::pthread_barrierattr_t,
// 	count: ::c_uint,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_spin_init(lock: *mut ::pthread_spinlock_t, pshared: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_spin_destroy(lock: *mut ::pthread_spinlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_spin_lock(lock: *mut ::pthread_spinlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_spin_trylock(lock: *mut ::pthread_spinlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_spin_unlock(lock: *mut ::pthread_spinlock_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn clone(
// 	cb: extern "C" fn(*mut ::c_void) -> ::c_int,
// 	child_stack: *mut ::c_void,
// 	flags: ::c_int,
// 	arg: *mut ::c_void
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_getscheduler(pid: ::pid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn clock_nanosleep(
// 	clk_id: ::clockid_t,
// 	flags: ::c_int,
// 	rqtp: *const ::timespec,
// 	rmtp: *mut ::timespec,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_attr_getguardsize(
// 	attr: *const ::pthread_attr_t,
// 	guardsize: *mut ::size_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_get_priority_min(policy: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_condattr_getpshared(
// 	attr: *const pthread_condattr_t,
// 	pshared: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sysinfo(info: *mut ::sysinfo) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn umount2(target: *const ::c_char, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_setschedparam(
// 	native: ::pthread_t,
// 	policy: ::c_int,
// 	param: *const ::sched_param,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn swapon(path: *const ::c_char, swapflags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_setscheduler(
// 	pid: ::pid_t,
// 	policy: ::c_int,
// 	param: *const ::sched_param,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sendfile(
// 	out_fd: ::c_int,
// 	in_fd: ::c_int,
// 	offset: *mut off_t,
// 	count: ::size_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sigsuspend(mask: *const ::sigset_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getgrgid_r(
// 	gid: ::gid_t,
// 	grp: *mut ::group,
// 	buf: *mut ::c_char,
// 	buflen: ::size_t,
// 	result: *mut *mut ::group,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_close(sem: *mut sem_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getdtablesize() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getgrnam_r(
// 	name: *const ::c_char,
// 	grp: *mut ::group,
// 	buf: *mut ::c_char,
// 	buflen: ::size_t,
// 	result: *mut *mut ::group,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn initgroups(user: *const ::c_char, group: ::gid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_sigmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_open(name: *const ::c_char, oflag: ::c_int) -> *mut sem_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getgrnam(name: *const ::c_char) -> *mut ::group{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_kill(thread: ::pthread_t, sig: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sem_unlink(name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getpwnam_r(
// 	name: *const ::c_char,
// 	pwd: *mut passwd,
// 	buf: *mut ::c_char,
// 	buflen: ::size_t,
// 	result: *mut *mut passwd,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getpwuid_r(
// 	uid: ::uid_t,
// 	pwd: *mut passwd,
// 	buf: *mut ::c_char,
// 	buflen: ::size_t,
// 	result: *mut *mut passwd,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sigwait(set: *const sigset_t, sig: *mut ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_atfork(
// 	prepare: ::Option<unsafe extern "C" fn()>,
// 	parent: ::Option<unsafe extern "C" fn()>,
// 	child: ::Option<unsafe extern "C" fn()>,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getgrgid(gid: ::gid_t) -> *mut ::group{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getgrouplist(
// 	user: *const ::c_char,
// 	group: ::gid_t,
// 	groups: *mut ::gid_t,
// 	ngroups: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_mutexattr_getpshared(
// 	attr: *const pthread_mutexattr_t,
// 	pshared: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn popen(command: *const ::c_char, mode: *const ::c_char) -> *mut ::FILE{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn faccessat(
// 	dirfd: ::c_int,
// 	pathname: *const ::c_char,
// 	mode: ::c_int,
// 	flags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_create(
// 	native: *mut ::pthread_t,
// 	attr: *const ::pthread_attr_t,
// 	f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
// 	value: *mut ::c_void,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn dl_iterate_phdr(
// 	callback: ::Option<
// 		unsafe extern "C" fn(
// 			info: *mut ::dl_phdr_info,
// 			size: ::size_t,
// 			data: *mut ::c_void,
// 		) -> ::c_int,
// 	>,
// 	data: *mut ::c_void,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn setmntent(filename: *const ::c_char, ty: *const ::c_char) -> *mut ::FILE{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getmntent(stream: *mut ::FILE) -> *mut ::mntent{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn addmntent(stream: *mut ::FILE, mnt: *const ::mntent) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn endmntent(streamp: *mut ::FILE) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn hasmntopt(mnt: *const ::mntent, opt: *const ::c_char) -> *mut ::c_char{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn posix_spawn(
// 	pid: *mut ::pid_t,
// 	path: *const ::c_char,
// 	file_actions: *const ::posix_spawn_file_actions_t,
// 	attrp: *const ::posix_spawnattr_t,
// 	argv_: *const *mut ::c_char,
// 	envp: *const *mut ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnp(
// 	pid: *mut ::pid_t,
// 	file: *const ::c_char,
// 	file_actions: *const ::posix_spawn_file_actions_t,
// 	attrp: *const ::posix_spawnattr_t,
// 	argv_: *const *mut ::c_char,
// 	envp: *const *mut ::c_char,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_getsigdefault(
// 	attr: *const posix_spawnattr_t,
// 	default: *mut ::sigset_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_setsigdefault(
// 	attr: *mut posix_spawnattr_t,
// 	default: *const ::sigset_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_getsigmask(
// 	attr: *const posix_spawnattr_t,
// 	default: *mut ::sigset_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_setsigmask(
// 	attr: *mut posix_spawnattr_t,
// 	default: *const ::sigset_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_getflags(
// 	attr: *const posix_spawnattr_t,
// 	flags: *mut ::c_short,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: ::c_short) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_getpgroup(
// 	attr: *const posix_spawnattr_t,
// 	flags: *mut ::pid_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: ::pid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_getschedpolicy(
// 	attr: *const posix_spawnattr_t,
// 	flags: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_getschedparam(
// 	attr: *const posix_spawnattr_t,
// 	param: *mut ::sched_param,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawnattr_setschedparam(
// 	attr: *mut posix_spawnattr_t,
// 	param: *const ::sched_param,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawn_file_actions_addopen(
// 	actions: *mut posix_spawn_file_actions_t,
// 	fd: ::c_int,
// 	path: *const ::c_char,
// 	oflag: ::c_int,
// 	mode: ::mode_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawn_file_actions_addclose(
// 	actions: *mut posix_spawn_file_actions_t,
// 	fd: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_spawn_file_actions_adddup2(
// 	actions: *mut posix_spawn_file_actions_t,
// 	fd: ::c_int,
// 	newfd: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fread_unlocked(
// 	ptr: *mut ::c_void,
// 	size: ::size_t,
// 	nobj: ::size_t,
// 	stream: *mut ::FILE,
// ) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn inotify_rm_watch(fd: ::c_int, wd: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn inotify_init() -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn inotify_init1(flags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn inotify_add_watch(fd: ::c_int, path: *const ::c_char, mask: u32) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fanotify_init(flags: ::c_uint, event_f_flags: ::c_uint) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn regcomp(preg: *mut ::regex_t, pattern: *const ::c_char, cflags: ::c_int) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn regexec(
// 	preg: *const ::regex_t,
// 	input: *const ::c_char,
// 	nmatch: ::size_t,
// 	pmatch: *mut regmatch_t,
// 	eflags: ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn regerror(
// 	errcode: ::c_int,
// 	preg: *const ::regex_t,
// 	errbuf: *mut ::c_char,
// 	errbuf_size: ::size_t,
// ) -> ::size_t{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn regfree(preg: *mut ::regex_t){
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn iconv_open(tocode: *const ::c_char, fromcode: *const ::c_char) -> iconv_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn iconv(
// 	cd: iconv_t,
// 	inbuf: *mut *mut ::c_char,
// 	inbytesleft: *mut ::size_t,
// 	outbuf: *mut *mut ::c_char,
// 	outbytesleft: *mut ::size_t,
// ) -> ::size_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn iconv_close(cd: iconv_t) -> ::c_int{
// 	unimplemented!()
// }



// #[no_mangle]
// pub extern "C" fn timer_create(
// 	clockid: ::clockid_t,
// 	sevp: *mut ::sigevent,
// 	timerid: *mut ::timer_t,
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timer_delete(timerid: ::timer_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timer_getoverrun(timerid: ::timer_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timer_gettime(timerid: ::timer_t, curr_value: *mut ::itimerspec) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn timer_settime(
// 	timerid: ::timer_t,
// 	flags: ::c_int,
// 	new_value: *const ::itimerspec,
// 	old_value: *mut ::itimerspec,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn gethostid() -> ::c_long{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_getcpuclockid(thread: ::pthread_t, clk_id: *mut ::clockid_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn memmem(
// 	haystack: *const ::c_void,
// 	haystacklen: ::size_t,
// 	needle: *const ::c_void,
// 	needlelen: ::size_t,
// ) -> *mut ::c_void{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sched_getcpu() -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn pthread_getname_np(thread: ::pthread_t, name: *mut ::c_char, len: ::size_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn pthread_setname_np(thread: ::pthread_t, name: *const ::c_char) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn getopt_long(
// 	argc: ::c_int,
// 	argv_: *const *mut ::c_char,
// 	optstring: *const ::c_char,
// 	longopts: *const option,
// 	longindex: *mut ::c_int,
// ) -> ::c_int{
// 	unimplemented!()
// }

// #[no_mangle]
// pub extern "C" fn copy_file_range(
// 	fd_in: ::c_int,
// 	off_in: *mut ::off64_t,
// 	fd_out: ::c_int,
// 	off_out: *mut ::off64_t,
// 	len: ::size_t,
// 	flags: ::c_uint,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fallocate64(
// 	fd: ::c_int,
// 	mode: ::c_int,
// 	offset: ::off64_t,
// 	len: ::off64_t
// ) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fgetpos64(stream: *mut ::FILE, ptr: *mut fpos64_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fopen64(filename: *const ::c_char, mode: *const ::c_char) -> *mut ::FILE{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn freopen64(
// 	filename: *const ::c_char,
// 	mode: *const ::c_char,
// 	file: *mut ::FILE,
// ) -> *mut ::FILE{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fseeko64(stream: *mut ::FILE, offset: ::off64_t, whence: ::c_int) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn fsetpos64(stream: *mut ::FILE, ptr: *const fpos64_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn ftello64(stream: *mut ::FILE) -> ::off64_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn posix_fallocate64(fd: ::c_int, offset: ::off64_t, len: ::off64_t) -> ::c_int{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn sendfile64(
// 	out_fd: ::c_int,
// 	in_fd: ::c_int,
// 	offset: *mut off64_t,
// 	count: ::size_t,
// ) -> ::ssize_t{
// 	unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn tmpfile64() -> *mut ::FILE{
// 	unimplemented!()
// }
// pub extern "C" fn readlinkat(dirfd: ::c_int,
//     pathname: *const ::c_char,
//     buf: *mut ::c_char,
//     bufsiz: ::size_t) -> ::ssize_t{
//         unimplemented!()
// }
// pub extern "C" fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE{
//     unimplemented!()
// }
// pub extern "C" fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE{
//     unimplemented!()
// }
// pub extern "C" fn atexit(cb: extern "C" fn()) -> ::c_int{
//     unimplemented!()
// }





