//! sys/auxv.h implementation

#[no_mangle]
pub extern "C" fn getauxval(_t: ::c_ulong) -> ::c_ulong {
    0
}
