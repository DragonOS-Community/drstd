use crate::std::any::TypeId;
use dlibc;
macro_rules! ok {
    ($($t:ident)*) => {$(
        assert!(TypeId::of::<dlibc::$t>() == TypeId::of::<raw::$t>(),
                "{} is wrong", stringify!($t));
    )*}
}

#[test]
fn same() {
    use crate::std::os::raw;
    ok!(c_char c_schar c_uchar c_short c_ushort c_int c_uint c_long c_ulong
        c_longlong c_ulonglong c_float c_double);
}
