use crate::std::cell::UnsafeCell;
use crate::std::mem::{forget, MaybeUninit};
use crate::std::sys::cvt_nz;
use crate::std::sys_common::lazy_box::{LazyBox, LazyInit};
use dlibc;

struct AllocatedMutex(UnsafeCell<dlibc::pthread_mutex_t>);

pub struct Mutex {
    inner: LazyBox<AllocatedMutex>,
}

#[inline]
pub unsafe fn raw(m: &Mutex) -> *mut dlibc::pthread_mutex_t {
    m.inner.0.get()
}

unsafe impl Send for AllocatedMutex {}
unsafe impl Sync for AllocatedMutex {}

impl LazyInit for AllocatedMutex {
    fn init() -> Box<Self> {
        let mutex = Box::new(AllocatedMutex(UnsafeCell::new(
            dlibc::PTHREAD_MUTEX_INITIALIZER,
        )));

        // Issue #33770
        //
        // A pthread mutex initialized with PTHREAD_MUTEX_INITIALIZER will have
        // a type of PTHREAD_MUTEX_DEFAULT, which has undefined behavior if you
        // try to re-lock it from the same thread when you already hold a lock
        // (https://pubs.opengroup.org/onlinepubs/9699919799/functions/pthread_mutex_init.html).
        // This is the case even if PTHREAD_MUTEX_DEFAULT == PTHREAD_MUTEX_NORMAL
        // (https://github.com/rust-lang/rust/issues/33770#issuecomment-220847521) -- in that
        // case, `pthread_mutexattr_settype(PTHREAD_MUTEX_DEFAULT)` will of course be the same
        // as setting it to `PTHREAD_MUTEX_NORMAL`, but not setting any mode will result in
        // a Mutex where re-locking is UB.
        //
        // In practice, glibc takes advantage of this undefined behavior to
        // implement hardware lock elision, which uses hardware transactional
        // memory to avoid acquiring the lock. While a transaction is in
        // progress, the lock appears to be unlocked. This isn't a problem for
        // other threads since the transactional memory will abort if a conflict
        // is detected, however no abort is generated when re-locking from the
        // same thread.
        //
        // Since locking the same mutex twice will result in two aliasing &mut
        // references, we instead create the mutex with type
        // PTHREAD_MUTEX_NORMAL which is guaranteed to deadlock if we try to
        // re-lock it from the same thread, thus avoiding undefined behavior.
        unsafe {
            let mut attr = MaybeUninit::<dlibc::pthread_mutexattr_t>::uninit();
            cvt_nz(dlibc::pthread_mutexattr_init(attr.as_mut_ptr())).unwrap();
            let attr = PthreadMutexAttr(&mut attr);
            cvt_nz(dlibc::pthread_mutexattr_settype(
                attr.0.as_mut_ptr(),
                dlibc::PTHREAD_MUTEX_NORMAL,
            ))
            .unwrap();
            cvt_nz(dlibc::pthread_mutex_init(mutex.0.get(), attr.0.as_ptr())).unwrap();
        }

        mutex
    }

    fn destroy(mutex: Box<Self>) {
        // We're not allowed to pthread_mutex_destroy a locked mutex,
        // so check first if it's unlocked.
        if unsafe { dlibc::pthread_mutex_trylock(mutex.0.get()) == 0 } {
            unsafe { dlibc::pthread_mutex_unlock(mutex.0.get()) };
            drop(mutex);
        } else {
            // The mutex is locked. This happens if a MutexGuard is leaked.
            // In this case, we just leak the Mutex too.
            forget(mutex);
        }
    }

    fn cancel_init(_: Box<Self>) {
        // In this case, we can just drop it without any checks,
        // since it cannot have been locked yet.
    }
}

impl Drop for AllocatedMutex {
    #[inline]
    fn drop(&mut self) {
        let r = unsafe { dlibc::pthread_mutex_destroy(self.0.get()) };
        if cfg!(target_os = "dragonfly") {
            // On DragonFly pthread_mutex_destroy() returns EINVAL if called on a
            // mutex that was just initialized with dlibc::PTHREAD_MUTEX_INITIALIZER.
            // Once it is used (locked/unlocked) or pthread_mutex_init() is called,
            // this behaviour no longer occurs.
            debug_assert!(r == 0 || r == dlibc::EINVAL);
        } else {
            debug_assert_eq!(r, 0);
        }
    }
}

impl Mutex {
    #[inline]
    pub const fn new() -> Mutex {
        Mutex {
            inner: LazyBox::new(),
        }
    }

    #[inline]
    pub unsafe fn lock(&self) {
        let r = dlibc::pthread_mutex_lock(raw(self));
        debug_assert_eq!(r, 0);
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        let r = dlibc::pthread_mutex_unlock(raw(self));
        debug_assert_eq!(r, 0);
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        dlibc::pthread_mutex_trylock(raw(self)) == 0
    }
}

pub(super) struct PthreadMutexAttr<'a>(pub &'a mut MaybeUninit<dlibc::pthread_mutexattr_t>);

impl Drop for PthreadMutexAttr<'_> {
    fn drop(&mut self) {
        unsafe {
            let result = dlibc::pthread_mutexattr_destroy(self.0.as_mut_ptr());
            debug_assert_eq!(result, 0);
        }
    }
}
