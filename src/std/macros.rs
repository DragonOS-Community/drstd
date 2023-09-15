//! Standard library macros
//!
//! This module contains a set of macros which are exported from the standard
//! library. Each macro is available for use when linking against the standard
//! library.
// ignore-tidy-dbg

//#[doc = include_str!("../../core/src/macros/panic.md")]
#[macro_export]
#[rustc_builtin_macro(std_panic)]
#[allow_internal_unstable(edition_panic)]
#[cfg_attr(not(test), rustc_diagnostic_item = "std_panic_macro")]
macro_rules! panic {
    // Expands to either `$crate::std::panic::panic_2015` or `$crate::std::panic::panic_2021`
    // depending on the edition of the caller.
    ($($arg:tt)*) => {
        /* compiler built-in */
    };
}

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use [`io::stdout().flush()`][flush] to ensure the output is emitted
/// immediately.
///
/// The `print!` macro will lock the standard output on each call. If you call
/// `print!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
/// ```
/// use std::io::{stdout, Write};
///
/// let mut lock = stdout().lock();
/// write!(lock, "hello world").unwrap();
/// ```
///
/// Use `print!` only for the primary output of your program. Use
/// [`eprint!`] instead to print error and progress messages.
///
/// See [the formatting documentation in `std::fmt`](../std/fmt/index.html)
/// for details of the macro argument syntax.
///
/// [flush]: crate::std::io::Write::flush
/// [`println!`]: crate::std::println
/// [`eprint!`]: crate::std::eprint
/// [lock]: crate::std::io::Stdout
///
/// # Panics
///
/// Panics if writing to `io::stdout()` fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// use std::io::{self, Write};
///
/// print!("this ");
/// print!("will ");
/// print!("be ");
/// print!("on ");
/// print!("the ");
/// print!("same ");
/// print!("line ");
///
/// io::stdout().flush().unwrap();
///
/// print!("this string has a newline, why not choose println! instead?\n");
///
/// io::stdout().flush().unwrap();
/// ```
#[macro_export]
#[cfg_attr(not(test), rustc_diagnostic_item = "print_macro")]
#[allow_internal_unstable(print_internals)]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::std::io::_print($crate::std::format_args!($($arg)*));
    }};
}

/// Prints to the standard output, with a newline.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`)).
///
/// This macro uses the same syntax as [`format!`], but writes to the standard output instead.
/// See [`std::fmt`] for more information.
///
/// The `println!` macro will lock the standard output on each call. If you call
/// `println!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
/// ```
/// use std::io::{stdout, Write};
///
/// let mut lock = stdout().lock();
/// writeln!(lock, "hello world").unwrap();
/// ```
///
/// Use `println!` only for the primary output of your program. Use
/// [`eprintln!`] instead to print error and progress messages.
///
/// See [the formatting documentation in `std::fmt`](../std/fmt/index.html)
/// for details of the macro argument syntax.
///
/// [`std::fmt`]: crate::std::fmt
/// [`eprintln!`]: crate::std::eprintln
/// [lock]: crate::std::io::Stdout
///
/// # Panics
///
/// Panics if writing to [`io::stdout`] fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// [`io::stdout`]: crate::std::io::stdout
///
/// # Examples
///
/// ```
/// println!(); // prints just a newline
/// println!("hello there!");
/// println!("format {} arguments", "some");
/// let local_variable = "some";
/// println!("format {local_variable} arguments");
/// ```
#[macro_export]
#[cfg_attr(not(test), rustc_diagnostic_item = "println_macro")]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! println {
    () => {
        $crate::std::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::std::io::_print($crate::std::format_args_nl!($($arg)*));
    }};
}

/// Prints to the standard error.
///
/// Equivalent to the [`print!`] macro, except that output goes to
/// [`io::stderr`] instead of [`io::stdout`]. See [`print!`] for
/// example usage.
///
/// Use `eprint!` only for error and progress messages. Use `print!`
/// instead for the primary output of your program.
///
/// [`io::stderr`]: crate::std::io::stderr
/// [`io::stdout`]: crate::std::io::stdout
///
/// See [the formatting documentation in `std::fmt`](../std/fmt/index.html)
/// for details of the macro argument syntax.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// Writing to non-blocking stderr can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// eprint!("Error: Could not complete task");
/// ```
#[macro_export]
#[cfg_attr(not(test), rustc_diagnostic_item = "eprint_macro")]
#[allow_internal_unstable(print_internals)]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::std::io::_eprint($crate::std::format_args!($($arg)*));
    }};
}

/// Prints to the standard error, with a newline.
///
/// Equivalent to the [`println!`] macro, except that output goes to
/// [`io::stderr`] instead of [`io::stdout`]. See [`println!`] for
/// example usage.
///
/// Use `eprintln!` only for error and progress messages. Use `println!`
/// instead for the primary output of your program.
///
/// See [the formatting documentation in `std::fmt`](../std/fmt/index.html)
/// for details of the macro argument syntax.
///
/// [`io::stderr`]: crate::std::io::stderr
/// [`io::stdout`]: crate::std::io::stdout
/// [`println!`]: crate::std::println
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// Writing to non-blocking stderr can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// eprintln!("Error: Could not complete task");
/// ```
#[macro_export]
#[cfg_attr(not(test), rustc_diagnostic_item = "eprintln_macro")]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! eprintln {
    () => {
        $crate::std::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::std::io::_eprint($crate::std::format_args_nl!($($arg)*));
    }};
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
///
/// An example:
///
/// ```rust
/// let a = 2;
/// let b = dbg!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
///
/// The macro works by using the `Debug` implementation of the type of
/// the given expression to print the value to [stderr] along with the
/// source location of the macro invocation as well as the source code
/// of the expression.
///
/// Invoking the macro on an expression moves and takes ownership of it
/// before returning the evaluated expression unchanged. If the type
/// of the expression does not implement `Copy` and you don't want
/// to give up ownership, you can instead borrow with `dbg!(&expr)`
/// for some expression `expr`.
///
/// The `dbg!` macro works exactly the same in release builds.
/// This is useful when debugging issues that only occur in release
/// builds or when debugging in release mode is significantly faster.
///
/// Note that the macro is intended as a debugging tool and therefore you
/// should avoid having uses of it in version control for long periods
/// (other than in tests and similar).
/// Debug output from production code is better done with other facilities
/// such as the [`debug!`] macro from the [`log`] crate.
///
/// # Stability
///
/// The exact output printed by this macro should not be relied upon
/// and is subject to future changes.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// # Further examples
///
/// With a method call:
///
/// ```rust
/// fn foo(n: usize) {
///     if let Some(_) = dbg!(n.checked_sub(4)) {
///         // ...
///     }
/// }
///
/// foo(3)
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:4] n.checked_sub(4) = None
/// ```
///
/// Naive factorial implementation:
///
/// ```rust
/// fn factorial(n: u32) -> u32 {
///     if dbg!(n <= 1) {
///         dbg!(1)
///     } else {
///         dbg!(n * factorial(n - 1))
///     }
/// }
///
/// dbg!(factorial(4));
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = true
/// [src/main.rs:4] 1 = 1
/// [src/main.rs:5] n * factorial(n - 1) = 2
/// [src/main.rs:5] n * factorial(n - 1) = 6
/// [src/main.rs:5] n * factorial(n - 1) = 24
/// [src/main.rs:11] factorial(4) = 24
/// ```
///
/// The `dbg!(..)` macro moves the input:
///
/// ```compile_fail
/// /// A wrapper around `usize` which importantly is not Copyable.
/// #[derive(Debug)]
/// struct NoCopy(usize);
///
/// let a = NoCopy(42);
/// let _ = dbg!(a); // <-- `a` is moved here.
/// let _ = dbg!(a); // <-- `a` is moved again; error!
/// ```
///
/// You can also use `dbg!()` without a value to just print the
/// file and line whenever it's reached.
///
/// Finally, if you want to `dbg!(..)` multiple values, it will treat them as
/// a tuple (and return it, too):
///
/// ```
/// assert_eq!(dbg!(1usize, 2u32), (1, 2));
/// ```
///
/// However, a single argument with a trailing comma will still not be treated
/// as a tuple, following the convention of ignoring trailing commas in macro
/// invocations. You can use a 1-tuple directly if you need one:
///
/// ```
/// assert_eq!(1, dbg!(1u32,)); // trailing comma ignored
/// assert_eq!((1,), dbg!((1u32,))); // 1-tuple
/// ```
///
/// [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
/// [`debug!`]: https://docs.rs/log/*/log/macro.debug.html
/// [`log`]: https://crates.io/crates/log
#[macro_export]
#[cfg_attr(not(test), rustc_diagnostic_item = "dbg_macro")]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::std::eprintln!("[{}:{}]", $crate::std::file!(), $crate::std::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::std::eprintln!("[{}:{}] {} = {:#?}",
                    $crate::std::file!(), $crate::std::line!(), $crate::std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::std::dbg!($val)),+,)
    };
}

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!((*a - *b).abs() < 1.0e-6, "{} is not approximately equal to {}", *a, *b);
    }};
}

#[macro_export]
#[cfg(not(feature = "trace"))]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

#[macro_export]
#[cfg(feature = "trace")]
macro_rules! trace {
    ($($arg:tt)*) => ({
        use $crate::{Pal, Sys};
        eprintln!($($arg)*);
    });
}

#[macro_export]
#[cfg(not(feature = "trace"))]
macro_rules! trace_expr {
    ($expr:expr, $($arg:tt)*) => {
        $expr
    };
}

#[macro_export]
#[cfg(feature = "trace")]
macro_rules! trace_expr {
    ($expr:expr, $($arg:tt)*) => ({
        use $crate::header::errno::STR_ERROR;
        use $crate::platform;

        trace!("{}", format_args!($($arg)*));

        #[allow(unused_unsafe)]
        let trace_old_errno = unsafe { dlibc::errno };
        #[allow(unused_unsafe)]
        unsafe { dlibc::errno = 0; }

        let ret = $expr;

        #[allow(unused_unsafe)]
        let trace_errno = unsafe { dlibc::errno } as isize;
        if trace_errno == 0 {
            #[allow(unused_unsafe)]
            unsafe { dlibc::errno = trace_old_errno; }
        }

        let trace_strerror = if trace_errno >= 0 && trace_errno < STR_ERROR.len() as isize {
            STR_ERROR[trace_errno as usize]
        } else {
            "Unknown error"
        };

        trace!("{} = {} ({}, {})", format_args!($($arg)*), ret, trace_errno, trace_strerror);

        ret
    });
}

#[macro_export]
macro_rules! strto_impl {
    (
        $rettype:ty, $signed:expr, $maxval:expr, $minval:expr, $s:ident, $endptr:ident, $base:ident
    ) => {{
        // ensure these are constants
        const CHECK_SIGN: bool = $signed;
        const MAX_VAL: $rettype = $maxval;
        const MIN_VAL: $rettype = $minval;

        let set_endptr = |idx: isize| {
            if !$endptr.is_null() {
                // This is stupid, but apparently strto* functions want
                // const input but mut output, yet the man page says
                // "stores the address of the first invalid character in *endptr"
                // so obviously it doesn't want us to clone it.
                *$endptr = $s.offset(idx) as *mut _;
            }
        };

        let invalid_input = || {
            dlibc::errno = EINVAL;
            set_endptr(0);
        };

        // only valid bases are 2 through 36
        if $base != 0 && ($base < 2 || $base > 36) {
            invalid_input();
            return 0;
        }

        let mut idx = 0;

        // skip any whitespace at the beginning of the string
        while ctype::isspace(*$s.offset(idx) as c_int) != 0 {
            idx += 1;
        }

        // check for +/-
        let positive = match is_positive(*$s.offset(idx)) {
            Some((pos, i)) => {
                idx += i;
                pos
            }
            None => {
                invalid_input();
                return 0;
            }
        };

        // convert the string to a number
        let num_str = $s.offset(idx);
        let res = match $base {
            0 => detect_base(num_str)
                .and_then(|($base, i)| convert_integer(num_str.offset(i), $base)),
            8 => convert_octal(num_str),
            16 => convert_hex(num_str),
            _ => convert_integer(num_str, $base),
        };

        // check for error parsing octal/hex prefix
        // also check to ensure a number was indeed parsed
        let (num, i, overflow) = match res {
            Some(res) => res,
            None => {
                invalid_input();
                return 0;
            }
        };
        idx += i;

        let overflow = if CHECK_SIGN {
            overflow || (num as c_long).is_negative()
        } else {
            overflow
        };
        // account for the sign
        let num = num as $rettype;
        let num = if overflow {
            dlibc::errno = ERANGE;
            if CHECK_SIGN {
                if positive {
                    MAX_VAL
                } else {
                    MIN_VAL
                }
            } else {
                MAX_VAL
            }
        } else {
            if positive {
                num
            } else {
                // not using -num to keep the compiler happy
                num.overflowing_neg().0
            }
        };

        set_endptr(idx);

        num
    }};
}
#[macro_export]
macro_rules! strto_float_impl {
    ($type:ident, $s:expr, $endptr:expr) => {{
        let mut s = $s;
        let endptr = $endptr;

        // TODO: Handle named floats: NaN, Inf...

        while ctype::isspace(*s as c_int) != 0 {
            s = s.offset(1);
        }

        let mut result: $type = 0.0;
        let mut radix = 10;

        let result_sign = match *s as u8 {
            b'-' => {
                s = s.offset(1);
                -1.0
            }
            b'+' => {
                s = s.offset(1);
                1.0
            }
            _ => 1.0,
        };

        if *s as u8 == b'0' && *s.offset(1) as u8 == b'x' {
            s = s.offset(2);
            radix = 16;
        }

        while let Some(digit) = (*s as u8 as char).to_digit(radix) {
            result *= radix as $type;
            result += digit as $type;
            s = s.offset(1);
        }

        if *s as u8 == b'.' {
            s = s.offset(1);

            let mut i = 1.0;
            while let Some(digit) = (*s as u8 as char).to_digit(radix) {
                i *= radix as $type;
                result += digit as $type / i;
                s = s.offset(1);
            }
        }

        let s_before_exponent = s;

        let exponent = match (*s as u8, radix) {
            (b'e' | b'E', 10) | (b'p' | b'P', 16) => {
                s = s.offset(1);

                let is_exponent_positive = match *s as u8 {
                    b'-' => {
                        s = s.offset(1);
                        false
                    }
                    b'+' => {
                        s = s.offset(1);
                        true
                    }
                    _ => true,
                };

                // Exponent digits are always in base 10.
                if (*s as u8 as char).is_digit(10) {
                    let mut exponent_value = 0;

                    while let Some(digit) = (*s as u8 as char).to_digit(10) {
                        exponent_value *= 10;
                        exponent_value += digit;
                        s = s.offset(1);
                    }

                    let exponent_base = match radix {
                        10 => 10u128,
                        16 => 2u128,
                        _ => unreachable!(),
                    };

                    if is_exponent_positive {
                        Some(exponent_base.pow(exponent_value) as $type)
                    } else {
                        Some(1.0 / (exponent_base.pow(exponent_value) as $type))
                    }
                } else {
                    // Exponent had no valid digits after 'e'/'p' and '+'/'-', rollback
                    s = s_before_exponent;
                    None
                }
            }
            _ => None,
        };

        if !endptr.is_null() {
            // This is stupid, but apparently strto* functions want
            // const input but mut output, yet the man page says
            // "stores the address of the first invalid character in *endptr"
            // so obviously it doesn't want us to clone it.
            *endptr = s as *mut _;
        }

        if let Some(exponent) = exponent {
            result_sign * result * exponent
        } else {
            result_sign * result
        }
    }};
}
#[macro_export]
macro_rules! c_str {
    ($lit:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            $crate::c_str::CStr::from_bytes_with_nul_unchecked(concat!($lit, "\0").as_bytes())
        }
    };
}
