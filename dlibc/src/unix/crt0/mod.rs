use core::arch::asm;
use core::arch::global_asm;

// #[cfg(target_arch = "x86_64")]
// global_asm!(
//     "
//     .globl _start
//     .type _start, @function
// _start:
//     mov rdi, rsp
//     and rsp, 0xFFFFFFFFFFFFFFF0

//     sub rsp, 8

//     mov DWORD PTR [rsp], 0x00001F80
//     ldmxcsr [rsp]
//     mov WORD PTR [rsp], 0x037F
//     fldcw [rsp]

//     add rsp, 8

//     call relibc_start
//     .size _start, . - _start
// "
// );

#[cfg(target_arch = "x86_64")]
#[naked]
#[no_mangle]
#[start]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        concat!(
            "
        mov rdi, rsp
        and rsp, 0xFFFFFFFFFFFFFFF0

        sub rsp, 8

        mov DWORD PTR [rsp], 0x00001F80
        ldmxcsr [rsp]
        mov WORD PTR [rsp], 0x037F
        fldcw [rsp]

        add rsp, 8

        call relibc_start
        .size _start, . - _start"
        ),
        options(noreturn)
    )
}

#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
    .section .init
    .global _init
    _init:
        push rbp
        mov rbp, rsp
        // Created a new stack frame and updated the stack pointer
        // Body will be filled in by gcc and ended by crtn.o
        // This happens after crti.o and gcc has inserted code
        // Pop the stack frame
        pop rbp
        ret

    .section .fini
    .global _fini
    _fini:
        push rbp
        mov rbp, rsp
        // Created a new stack frame and updated the stack pointer
        // Body will be filled in by gcc and ended by crtn.o
        // This happens after crti.o and gcc has inserted code
        // Pop the stack frame
        pop rbp
        ret
"#
);
