#![cfg(any(
    target_os = "linux",
    target_os = "android",
    all(target_os = "emscripten", target_feature = "atomics"),
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "dragonfly",
    target_os = "fuchsia",
    target_os = "dragonos",
))]

use crate::std::sync::atomic::AtomicU32;
use crate::std::time::Duration;
use dlibc;

/// Wait for a futex_wake operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonos",
))]
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    use super::time::Timespec;
    use crate::std::ptr::null;
    use crate::std::sync::atomic::Ordering::Relaxed;

    // Calculate the timeout as an absolute timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout
        .and_then(|d| Timespec::now(dlibc::CLOCK_MONOTONIC).checked_add_duration(&d))
        .and_then(|t| t.to_timespec());

    loop {
        // No need to wait if the value already changed.
        if futex.load(Relaxed) != expected {
            return true;
        }

        let r = unsafe {
            cfg_if::cfg_if! {
                if #[cfg(target_os = "freebsd")] {
                    // FreeBSD doesn't have futex(), but it has
                    // _umtx_op(UMTX_OP_WAIT_UINT_PRIVATE), which is nearly
                    // identical. It supports absolute timeouts through a flag
                    // in the _umtx_time struct.
                    let umtx_timeout = timespec.map(|t| dlibc::_umtx_time {
                        _timeout: t,
                        _flags: dlibc::UMTX_ABSTIME,
                        _clockid: dlibc::CLOCK_MONOTONIC as u32,
                    });
                    let umtx_timeout_ptr = umtx_timeout.as_ref().map_or(null(), |t| t as *const _);
                    let umtx_timeout_size = umtx_timeout.as_ref().map_or(0, |t| crate::std::mem::size_of_val(t));
                    dlibc::_umtx_op(
                        futex as *const AtomicU32 as *mut _,
                        dlibc::UMTX_OP_WAIT_UINT_PRIVATE,
                        expected as dlibc::c_ulong,
                        crate::std::ptr::invalid_mut(umtx_timeout_size),
                        umtx_timeout_ptr as *mut _,
                    )
                } else if #[cfg(any(target_os = "linux", target_os = "android",target_os = "dragonos",))] {
                    // Use FUTEX_WAIT_BITSET rather than FUTEX_WAIT to be able to give an
                    // absolute time rather than a relative time.
                    dlibc::syscall(
                        dlibc::SYS_futex,
                        futex as *const AtomicU32,
                        dlibc::FUTEX_WAIT_BITSET | dlibc::FUTEX_PRIVATE_FLAG,
                        expected,
                        timespec.as_ref().map_or(null(), |t| t as *const dlibc::timespec),
                        null::<u32>(), // This argument is unused for FUTEX_WAIT_BITSET.
                        !0u32,         // A full bitmask, to make it behave like a regular FUTEX_WAIT.
                    )
                } else {
                    compile_error!("unknown target_os");
                }
            }
        };

        match (r < 0).then(super::os::errno) {
            Some(dlibc::ETIMEDOUT) => return false,
            Some(dlibc::EINTR) => continue,
            _ => return true,
        }
    }
}

/// Wake up one thread that's blocked on futex_wait on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
///
/// On some platforms, this always returns false.
#[cfg(any(target_os = "linux", target_os = "android", target_os = "dragonos",))]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    let ptr = futex as *const AtomicU32;
    let op = dlibc::FUTEX_WAKE | dlibc::FUTEX_PRIVATE_FLAG;
    unsafe { dlibc::syscall(dlibc::SYS_futex, ptr, op, 1) > 0 }
}

/// Wake up all threads that are waiting on futex_wait on this futex.
#[cfg(any(target_os = "linux", target_os = "android", target_os = "dragonos",))]
pub fn futex_wake_all(futex: &AtomicU32) {
    let ptr = futex as *const AtomicU32;
    let op = dlibc::FUTEX_WAKE | dlibc::FUTEX_PRIVATE_FLAG;
    unsafe {
        dlibc::syscall(dlibc::SYS_futex, ptr, op, i32::MAX);
    }
}

// FreeBSD doesn't tell us how many threads are woken up, so this always returns false.
#[cfg(target_os = "freebsd")]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    use crate::std::ptr::null_mut;
    unsafe {
        dlibc::_umtx_op(
            futex as *const AtomicU32 as *mut _,
            dlibc::UMTX_OP_WAKE_PRIVATE,
            1,
            null_mut(),
            null_mut(),
        )
    };
    false
}

#[cfg(target_os = "freebsd")]
pub fn futex_wake_all(futex: &AtomicU32) {
    use crate::std::ptr::null_mut;
    unsafe {
        dlibc::_umtx_op(
            futex as *const AtomicU32 as *mut _,
            dlibc::UMTX_OP_WAKE_PRIVATE,
            i32::MAX as dlibc::c_ulong,
            null_mut(),
            null_mut(),
        )
    };
}

#[cfg(target_os = "openbsd")]
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    use super::time::Timespec;
    use crate::std::ptr::{null, null_mut};

    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout
        .and_then(|d| Timespec::zero().checked_add_duration(&d))
        .and_then(|t| t.to_timespec());

    let r = unsafe {
        dlibc::futex(
            futex as *const AtomicU32 as *mut u32,
            dlibc::FUTEX_WAIT,
            expected as i32,
            timespec
                .as_ref()
                .map_or(null(), |t| t as *const dlibc::timespec),
            null_mut(),
        )
    };

    r == 0 || super::os::errno() != dlibc::ETIMEDOUT
}

#[cfg(target_os = "openbsd")]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    use crate::std::ptr::{null, null_mut};
    unsafe {
        dlibc::futex(
            futex as *const AtomicU32 as *mut u32,
            dlibc::FUTEX_WAKE,
            1,
            null(),
            null_mut(),
        ) > 0
    }
}

#[cfg(target_os = "openbsd")]
pub fn futex_wake_all(futex: &AtomicU32) {
    use crate::std::ptr::{null, null_mut};
    unsafe {
        dlibc::futex(
            futex as *const AtomicU32 as *mut u32,
            dlibc::FUTEX_WAKE,
            i32::MAX,
            null(),
            null_mut(),
        );
    }
}

#[cfg(target_os = "dragonfly")]
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    // A timeout of 0 means infinite.
    // We round smaller timeouts up to 1 millisecond.
    // Overflows are rounded up to an infinite timeout.
    let timeout_ms = timeout
        .and_then(|d| Some(i32::try_from(d.as_millis()).ok()?.max(1)))
        .unwrap_or(0);

    let r = unsafe {
        dlibc::umtx_sleep(
            futex as *const AtomicU32 as *const i32,
            expected as i32,
            timeout_ms,
        )
    };

    r == 0 || super::os::errno() != dlibc::ETIMEDOUT
}

// DragonflyBSD doesn't tell us how many threads are woken up, so this always returns false.
#[cfg(target_os = "dragonfly")]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    unsafe { dlibc::umtx_wakeup(futex as *const AtomicU32 as *const i32, 1) };
    false
}

#[cfg(target_os = "dragonfly")]
pub fn futex_wake_all(futex: &AtomicU32) {
    unsafe { dlibc::umtx_wakeup(futex as *const AtomicU32 as *const i32, i32::MAX) };
}

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_futex_wake(addr: *const AtomicU32, count: dlibc::c_int) -> dlibc::c_int;
    fn emscripten_futex_wait(
        addr: *const AtomicU32,
        val: dlibc::c_uint,
        max_wait_ms: dlibc::c_double,
    ) -> dlibc::c_int;
}

#[cfg(target_os = "emscripten")]
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    unsafe {
        emscripten_futex_wait(
            futex,
            expected,
            timeout.map_or(f64::INFINITY, |d| d.as_secs_f64() * 1000.0),
        ) != -dlibc::ETIMEDOUT
    }
}

#[cfg(target_os = "emscripten")]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    unsafe { emscripten_futex_wake(futex, 1) > 0 }
}

#[cfg(target_os = "emscripten")]
pub fn futex_wake_all(futex: &AtomicU32) {
    unsafe { emscripten_futex_wake(futex, i32::MAX) };
}

#[cfg(target_os = "fuchsia")]
pub mod zircon {
    pub type zx_futex_t = crate::std::sync::atomic::AtomicU32;
    pub type zx_handle_t = u32;
    pub type zx_status_t = i32;
    pub type zx_time_t = i64;

    pub const ZX_HANDLE_INVALID: zx_handle_t = 0;

    pub const ZX_TIME_INFINITE: zx_time_t = zx_time_t::MAX;

    pub const ZX_OK: zx_status_t = 0;
    pub const ZX_ERR_INVALID_ARGS: zx_status_t = -10;
    pub const ZX_ERR_BAD_HANDLE: zx_status_t = -11;
    pub const ZX_ERR_WRONG_TYPE: zx_status_t = -12;
    pub const ZX_ERR_BAD_STATE: zx_status_t = -20;
    pub const ZX_ERR_TIMED_OUT: zx_status_t = -21;

    extern "C" {
        pub fn zx_clock_get_monotonic() -> zx_time_t;
        pub fn zx_futex_wait(
            value_ptr: *const zx_futex_t,
            current_value: zx_futex_t,
            new_futex_owner: zx_handle_t,
            deadline: zx_time_t,
        ) -> zx_status_t;
        pub fn zx_futex_wake(value_ptr: *const zx_futex_t, wake_count: u32) -> zx_status_t;
        pub fn zx_futex_wake_single_owner(value_ptr: *const zx_futex_t) -> zx_status_t;
        pub fn zx_thread_self() -> zx_handle_t;
    }
}

#[cfg(target_os = "fuchsia")]
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    // Sleep forever if the timeout is longer than fits in a i64.
    let deadline = timeout
        .and_then(|d| {
            i64::try_from(d.as_nanos())
                .ok()?
                .checked_add(unsafe { zircon::zx_clock_get_monotonic() })
        })
        .unwrap_or(zircon::ZX_TIME_INFINITE);

    unsafe {
        zircon::zx_futex_wait(
            futex,
            AtomicU32::new(expected),
            zircon::ZX_HANDLE_INVALID,
            deadline,
        ) != zircon::ZX_ERR_TIMED_OUT
    }
}

// Fuchsia doesn't tell us how many threads are woken up, so this always returns false.
#[cfg(target_os = "fuchsia")]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    unsafe { zircon::zx_futex_wake(futex, 1) };
    false
}

#[cfg(target_os = "fuchsia")]
pub fn futex_wake_all(futex: &AtomicU32) {
    unsafe { zircon::zx_futex_wake(futex, u32::MAX) };
}
