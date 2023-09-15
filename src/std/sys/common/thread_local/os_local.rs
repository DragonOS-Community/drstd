use super::lazy::LazyKeyInner;
use crate::std::cell::Cell;
use crate::std::sys_common::thread_local_key::StaticKey as OsStaticKey;
use crate::std::{fmt, marker, panic, ptr};

#[doc(hidden)]
#[allow_internal_unstable(thread_local_internals)]
#[allow_internal_unsafe]
#[rustc_macro_transparency = "semitransparent"]
pub macro thread_local_inner {
    // used to generate the `LocalKey` value for const-initialized thread locals
    (@key $t:ty, const $init:expr) => {{
        #[inline]
        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn __getit(
            _init: $crate::std::option::Option<&mut $crate::std::option::Option<$t>>,
        ) -> $crate::std::option::Option<&'static $t> {
            const INIT_EXPR: $t = $init;

            // On platforms without `#[thread_local]` we fall back to the
            // same implementation as below for os thread locals.
            #[inline]
            const fn __init() -> $t { INIT_EXPR }
            static __KEY: $crate::std::thread::local_impl::Key<$t> =
                $crate::std::thread::local_impl::Key::new();
            unsafe {
                __KEY.get(move || {
                    if let $crate::std::option::Option::Some(init) = _init {
                        if let $crate::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if $crate::std::cfg!(debug_assertions) {
                            $crate::std::unreachable!("missing initial value");
                        }
                    }
                    __init()
                })
            }
        }

        unsafe {
            $crate::std::thread::LocalKey::new(__getit)
        }
    }},

    // used to generate the `LocalKey` value for `thread_local!`
    (@key $t:ty, $init:expr) => {
        {
            #[inline]
            fn __init() -> $t { $init }

            // `#[inline] does not work on windows-gnu due to linking errors around dllimports.
            // See https://github.com/rust-lang/rust/issues/109797.
            #[cfg_attr(not(windows), inline)]
            unsafe fn __getit(
                init: $crate::std::option::Option<&mut $crate::std::option::Option<$t>>,
            ) -> $crate::std::option::Option<&'static $t> {
                static __KEY: $crate::std::thread::local_impl::Key<$t> =
                    $crate::std::thread::local_impl::Key::new();

                unsafe {
                    __KEY.get(move || {
                        if let $crate::std::option::Option::Some(init) = init {
                            if let $crate::std::option::Option::Some(value) = init.take() {
                                return value;
                            } else if $crate::std::cfg!(debug_assertions) {
                                $crate::std::unreachable!("missing default value");
                            }
                        }
                        __init()
                    })
                }
            }

            unsafe {
                $crate::std::thread::LocalKey::new(__getit)
            }
        }
    },
    ($(#[$attr:meta])* $vis:vis $name:ident, $t:ty, $($init:tt)*) => {
        $(#[$attr])* $vis const $name: $crate::std::thread::LocalKey<$t> =
            $crate::std::thread::local_impl::thread_local_inner!(@key $t, $($init)*);
    },
}

/// Use a regular global static to store this key; the state provided will then be
/// thread-local.
pub struct Key<T> {
    // OS-TLS key that we'll use to key off.
    os: OsStaticKey,
    marker: marker::PhantomData<Cell<T>>,
}

impl<T> fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Key").finish_non_exhaustive()
    }
}

unsafe impl<T> Sync for Key<T> {}

struct Value<T: 'static> {
    inner: LazyKeyInner<T>,
    key: &'static Key<T>,
}

impl<T: 'static> Key<T> {
        pub const fn new() -> Key<T> {
        Key { os: OsStaticKey::new(Some(destroy_value::<T>)), marker: marker::PhantomData }
    }

    /// It is a requirement for the caller to ensure that no mutable
    /// reference is active when this method is called.
    pub unsafe fn get(&'static self, init: impl FnOnce() -> T) -> Option<&'static T> {
        // SAFETY: See the documentation for this method.
        let ptr = unsafe { self.os.get() as *mut Value<T> };
        if ptr.addr() > 1 {
            // SAFETY: the check ensured the pointer is safe (its destructor
            // is not running) + it is coming from a trusted source (self).
            if let Some(ref value) = unsafe { (*ptr).inner.get() } {
                return Some(value);
            }
        }
        // SAFETY: At this point we are sure we have no value and so
        // initializing (or trying to) is safe.
        unsafe { self.try_initialize(init) }
    }

    // `try_initialize` is only called once per os thread local variable,
    // except in corner cases where thread_local dtors reference other
    // thread_local's, or it is being recursively initialized.
    unsafe fn try_initialize(&'static self, init: impl FnOnce() -> T) -> Option<&'static T> {
        // SAFETY: No mutable references are ever handed out meaning getting
        // the value is ok.
        let ptr = unsafe { self.os.get() as *mut Value<T> };
        if ptr.addr() == 1 {
            // destructor is running
            return None;
        }

        let ptr = if ptr.is_null() {
            // If the lookup returned null, we haven't initialized our own
            // local copy, so do that now.
            let ptr = Box::into_raw(Box::new(Value { inner: LazyKeyInner::new(), key: self }));
            // SAFETY: At this point we are sure there is no value inside
            // ptr so setting it will not affect anyone else.
            unsafe {
                self.os.set(ptr as *mut u8);
            }
            ptr
        } else {
            // recursive initialization
            ptr
        };

        // SAFETY: ptr has been ensured as non-NUL just above an so can be
        // dereferenced safely.
        unsafe { Some((*ptr).inner.initialize(init)) }
    }
}

unsafe extern "C" fn destroy_value<T: 'static>(ptr: *mut u8) {
    // SAFETY:
    //
    // The OS TLS ensures that this key contains a null value when this
    // destructor starts to run. We set it back to a sentinel value of 1 to
    // ensure that any future calls to `get` for this thread will return
    // `None`.
    //
    // Note that to prevent an infinite loop we reset it back to null right
    // before we return from the destructor ourselves.
    //
    // Wrap the call in a catch to ensure unwinding is caught in the event
    // a panic takes place in a destructor.
    if let Err(_) = panic::catch_unwind(|| unsafe {
        let ptr = Box::from_raw(ptr as *mut Value<T>);
        let key = ptr.key;
        key.os.set(ptr::invalid_mut(1));
        drop(ptr);
        key.os.set(ptr::null_mut());
    }) {
        rtabort!("thread local panicked on drop");
    }
}
