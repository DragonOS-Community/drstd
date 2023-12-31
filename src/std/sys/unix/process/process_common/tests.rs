use super::*;

use crate::std::ffi::OsStr;
use crate::std::mem;
use crate::std::ptr;
use crate::std::sys::{cvt, cvt_nz};
use dlibc;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("received error for `{}`: {}", stringify!($e), e),
        }
    };
}

#[test]
#[cfg_attr(
    any(
        // See #14232 for more information, but it appears that signal delivery to a
        // newly spawned process may just be raced in the macOS, so to prevent this
        // test from being flaky we ignore it on macOS.
        target_os = "macos",
        // When run under our current QEMU emulation test suite this test fails,
        // although the reason isn't very clear as to why. For now this test is
        // ignored there.
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv64",
    ),
    ignore
)]
fn test_process_mask() {
    // Test to make sure that a signal mask *does* get inherited.
    fn test_inner(mut cmd: Command) {
        unsafe {
            let mut set = mem::MaybeUninit::<dlibc::sigset_t>::uninit();
            let mut old_set = mem::MaybeUninit::<dlibc::sigset_t>::uninit();
            t!(cvt(sigemptyset(set.as_mut_ptr())));
            t!(cvt(sigaddset(set.as_mut_ptr(), dlibc::SIGINT)));
            t!(cvt_nz(dlibc::pthread_sigmask(
                dlibc::SIG_SETMASK,
                set.as_ptr(),
                old_set.as_mut_ptr()
            )));

            cmd.stdin(Stdio::MakePipe);
            cmd.stdout(Stdio::MakePipe);

            let (mut cat, mut pipes) = t!(cmd.spawn(Stdio::Null, true));
            let stdin_write = pipes.stdin.take().unwrap();
            let stdout_read = pipes.stdout.take().unwrap();

            t!(cvt_nz(dlibc::pthread_sigmask(
                dlibc::SIG_SETMASK,
                old_set.as_ptr(),
                ptr::null_mut()
            )));

            t!(cvt(dlibc::kill(cat.id() as dlibc::pid_t, dlibc::SIGINT)));
            // We need to wait until SIGINT is definitely delivered. The
            // easiest way is to write something to cat, and try to read it
            // back: if SIGINT is unmasked, it'll get delivered when cat is
            // next scheduled.
            let _ = stdin_write.write(b"Hello");
            drop(stdin_write);

            // Exactly 5 bytes should be read.
            let mut buf = [0; 5];
            let ret = t!(stdout_read.read(&mut buf));
            assert_eq!(ret, 5);
            assert_eq!(&buf, b"Hello");

            t!(cat.wait());
        }
    }

    // A plain `Command::new` uses the posix_spawn path on many platforms.
    let cmd = Command::new(OsStr::new("cat"));
    test_inner(cmd);

    // Specifying `pre_exec` forces the fork/exec path.
    let mut cmd = Command::new(OsStr::new("cat"));
    unsafe { cmd.pre_exec(Box::new(|| Ok(()))) };
    test_inner(cmd);
}

#[test]
#[cfg_attr(
    any(
        // See test_process_mask
        target_os = "macos",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv64",
    ),
    ignore
)]
fn test_process_group_posix_spawn() {
    unsafe {
        // Spawn a cat subprocess that's just going to hang since there is no I/O.
        let mut cmd = Command::new(OsStr::new("cat"));
        cmd.pgroup(0);
        cmd.stdin(Stdio::MakePipe);
        cmd.stdout(Stdio::MakePipe);
        let (mut cat, _pipes) = t!(cmd.spawn(Stdio::Null, true));

        // Check that we can kill its process group, which means there *is* one.
        t!(cvt(dlibc::kill(-(cat.id() as dlibc::pid_t), dlibc::SIGINT)));

        t!(cat.wait());
    }
}

#[test]
#[cfg_attr(
    any(
        // See test_process_mask
        target_os = "macos",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv64",
    ),
    ignore
)]
fn test_process_group_no_posix_spawn() {
    unsafe {
        // Same as above, create hang-y cat. This time, force using the non-posix_spawnp path.
        let mut cmd = Command::new(OsStr::new("cat"));
        cmd.pgroup(0);
        cmd.pre_exec(Box::new(|| Ok(()))); // pre_exec forces fork + exec
        cmd.stdin(Stdio::MakePipe);
        cmd.stdout(Stdio::MakePipe);
        let (mut cat, _pipes) = t!(cmd.spawn(Stdio::Null, true));

        // Check that we can kill its process group, which means there *is* one.
        t!(cvt(dlibc::kill(-(cat.id() as dlibc::pid_t), dlibc::SIGINT)));

        t!(cat.wait());
    }
}

#[test]
fn test_program_kind() {
    let vectors = &[
        ("foo", ProgramKind::PathLookup),
        ("foo.out", ProgramKind::PathLookup),
        ("./foo", ProgramKind::Relative),
        ("../foo", ProgramKind::Relative),
        ("dir/foo", ProgramKind::Relative),
        // Note that paths on Unix can't contain / in them, so this is actually the directory "fo\\"
        // followed by the file "o".
        ("fo\\/o", ProgramKind::Relative),
        ("/foo", ProgramKind::Absolute),
        ("/dir/../foo", ProgramKind::Absolute),
    ];

    for (program, expected_kind) in vectors {
        assert_eq!(
            ProgramKind::new(program.as_ref()),
            *expected_kind,
            "actual != expected program kind for input {program}",
        );
    }
}
