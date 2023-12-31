use super::abi;
use crate::std::ptr::null;
use crate::std::sync::atomic::AtomicU32;
use crate::std::time::Duration;

pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    // Calculate the timeout as a relative timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout.and_then(|dur| {
        Some(abi::timespec {
            tv_sec: dur.as_secs().try_into().ok()?,
            tv_nsec: dur.subsec_nanos().into(),
        })
    });

    let r = unsafe {
        abi::futex_wait(
            futex.as_ptr(),
            expected,
            timespec
                .as_ref()
                .map_or(null(), |t| t as *const abi::timespec),
            abi::FUTEX_RELATIVE_TIMEOUT,
        )
    };

    r != -abi::errno::ETIMEDOUT
}

#[inline]
pub fn futex_wake(futex: &AtomicU32) -> bool {
    unsafe { abi::futex_wake(futex.as_ptr(), 1) > 0 }
}

#[inline]
pub fn futex_wake_all(futex: &AtomicU32) {
    unsafe {
        abi::futex_wake(futex.as_ptr(), i32::MAX);
    }
}
