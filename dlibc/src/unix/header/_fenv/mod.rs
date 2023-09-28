//! fenv.h implementation for Redox, following
//! http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/fenv.h.html

pub const FE_ALL_EXCEPT: ::c_int = 0;
pub const FE_TONEAREST: ::c_int = 0;

pub type fexcept_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fenv_t {
    pub cw: u64,
}

// #[no_mangle]
pub unsafe extern "C" fn feclearexcept(_excepts: ::c_int) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fegenenv(_envp: *mut fenv_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fegetexceptflag(_flagp: *mut fexcept_t, _excepts: ::c_int) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fegetround() -> ::c_int {
    FE_TONEAREST
}

// #[no_mangle]
pub unsafe extern "C" fn feholdexcept(_envp: *mut fenv_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn feraiseexcept(_except: ::c_int) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fesetenv(_envp: *const fenv_t) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fesetexceptflag(_flagp: *const fexcept_t, _excepts: ::c_int) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fesetround(_round: ::c_int) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn fetestexcept(_excepts: ::c_int) -> ::c_int {
    unimplemented!();
}

// #[no_mangle]
pub unsafe extern "C" fn feupdateenv(_envp: *const fenv_t) -> ::c_int {
    unimplemented!();
}
