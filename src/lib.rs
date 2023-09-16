#![no_std]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(linkage)]
#![feature(c_variadic)]
#![feature(naked_functions)]
#![feature(start)]
#![feature(thread_local)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(lang_items)]
#![feature(std_internals)]
#![feature(extend_one)]
#![feature(exact_size_is_empty)]
#![feature(core_panic)]
#![feature(char_internals)]
#![feature(str_internals)]
#![feature(error_in_core)]
#![feature(error_generic_member_access)]
#![feature(saturating_int_impl)]
#![feature(edition_panic)]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![feature(concat_bytes)]
#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(format_args_nl)]
#![feature(concat_idents)]
#![feature(ip_in_core)]
#![feature(ip)]
#![feature(cfg_eval)]
#![feature(cfg_accessible)]
#![feature(derive_const)]
#![feature(strict_provenance)]
#![feature(slice_concat_trait)]
#![feature(deprecated_suggestion)]
#![feature(prelude_2024)]
#![feature(exclusive_wrapper)]
#![feature(never_type)]
#![feature(dropck_eyepatch)]
#![feature(must_not_suspend)]
#![feature(negative_impls)]
#![feature(let_chains)]
#![feature(rustc_attrs)]
#![feature(allow_internal_unstable)]
#![feature(type_alias_impl_trait)]
#![feature(specialization)]
#![feature(decl_macro)]
#![feature(doc_notable_trait)]
#![feature(doc_cfg)]
#![feature(ascii_char)]
#![feature(type_ascription)]
#![feature(if_let_guard)]
#![feature(prelude_import)]
#![feature(doc_masked)]
#![feature(rustdoc_internals)]
#![feature(async_iterator)]
#![feature(assert_matches)]
#![feature(const_format_args)]
#![feature(portable_simd)]
#![feature(slice_internals)]
#![feature(utf8_chunks)]
#![feature(stmt_expr_attributes)]
#![feature(cfg_target_thread_local)]
#![feature(hashmap_internals)]
#![feature(try_reserve_kind)]
#![feature(hasher_prefixfree_extras)]
#![feature(inline_const)]
#![feature(allow_internal_unsafe)]
#![feature(raw_os_nonzero)]
#![feature(try_blocks)]
#![feature(offset_of)]
#![feature(btree_extract_if)]
#![feature(panic_internals)]
#![feature(slice_ptr_get)]
#![feature(alloc_layout_extra)]
#![feature(pointer_byte_offsets)]
#![feature(new_uninit)]
#![feature(get_mut_unchecked)]
#![feature(error_iter)]
#![feature(maybe_uninit_slice)]
#![feature(ptr_as_uninit)]
#![feature(maybe_uninit_write_slice)]
#![feature(panic_info_message)]
#![feature(panic_can_unwind)]
#![feature(const_mut_refs)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate memoffset;

//TODO: fix this: adjust to dragonos sc
#[cfg(target_os = "dragonos")]
#[macro_use]
extern crate dsc;

pub mod std;
pub use self::std::*;

#[cfg(target_os = "dragonos")]
#[macro_use]
pub use dlibc::{eprint, eprintln, print, println};

// use core::panic::PanicInfo;

// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}
