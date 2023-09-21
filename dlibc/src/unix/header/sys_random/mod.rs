


pub const GRND_NONBLOCK: ::c_uint = 1;
pub const GRND_RANDOM: ::c_uint = 2;

// #[no_mangle]
// pub unsafe extern "C" fn getrandom(buf: *mut ::c_void, buflen: ::size_t, flags: ::c_uint) -> ::ssize_t {
//     platform::pal::getrandom(
//         buf,
//         flags as usize,
//         buflen as u32
//     )
// }
