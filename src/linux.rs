use libc::{syscall, SYS_pidfd_getfd};
use pidfd::PidFd;
use std::{
    fs::File,
    // `io::Result` is renamed to provide clearer hints with rust-analyzer
    io::{self, Result as IoResult},
    os::unix::prelude::{AsRawFd, FromRawFd, RawFd},
};

#[cfg(feature = "nightly")]
use std::os::linux::process::PidFd as StdPidFd;

/// Various flags used to configure calls to [`get_file_from_pidfd`].
///
/// Currently, there are no flags.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct GetfdFlags;

impl GetfdFlags {
    pub const fn empty() -> Self {
        Self
    }

    pub const fn bits(&self) -> u32 {
        0
    }
}

/// An extension trait to provide a convenient interface to [`get_file_from_pidfd`].
pub trait PidFdExt {
    fn get_file(&self, target_fd: RawFd, flags: GetfdFlags) -> IoResult<File>;
}

impl PidFdExt for PidFd {
    fn get_file(&self, target_fd: RawFd, flags: GetfdFlags) -> IoResult<File> {
        get_file_from_pidfd(self.as_raw_fd(), target_fd, flags)
    }
}

#[cfg(feature = "nightly")]
impl PidFdExt for StdPidFd {
    fn get_file(&self, target_fd: RawFd, flags: GetfdFlags) -> IoResult<File> {
        get_file_from_pidfd(self.as_raw_fd(), target_fd, flags)
    }
}

/// Takes the file description referred to by `target_fd` within `pidfd`
/// and creates a duplicate of it within this process.
///
/// This is a convenience wrapper. For the raw syscall, see [`pidfd_getfd`].
///
/// For more information, including the meaning of any returned [`io::Error`]s,
/// see the man page for [`pidfd_getfd(2)`].
///
/// [`pidfd_getfd(2)`]: https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html
pub fn get_file_from_pidfd(pidfd: RawFd, target_fd: RawFd, flags: GetfdFlags) -> IoResult<File> {
    // SAFETY: `flags` being 0 seems to be the only safety invariant for now.
    // Invalid fds return errors.
    let res = unsafe { pidfd_getfd(pidfd, target_fd, flags.bits()) };

    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        // SAFETY: `pidfd_getfd` returns a valid file descriptor
        // on success.
        Ok(unsafe { File::from_raw_fd(res) })
    }
}

/// Takes the file description referred to by `targetfd` within `pidfd`
/// and creates a duplicate of it within this process.
///
/// This is the raw syscall. For a somewhat more convenient wrapper, see
/// [`get_file_from_pidfd`].
///
/// If this syscall is successful, it returns the new file descriptor.
/// Otherwise, it returns -1 and sets `errno`. For more information, see
/// the man page for [`pidfd_getfd(2)`].
///
/// ## Safety
/// The caller is responsible for upholding any requirements detailed in the
/// aforementioned man page. At the time of writing (2021-08-05, kernel version
/// 5.10.28), `flags` must be set to 0.
///
/// [`pidfd_getfd(2)`]: https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html
pub unsafe fn pidfd_getfd(
    pidfd: libc::c_int,
    targetfd: libc::c_int,
    flags: libc::c_uint,
) -> libc::c_int {
    // SAFETY: The caller is responsible for upholding the invariants of this
    // syscall.
    unsafe { syscall(SYS_pidfd_getfd, pidfd, targetfd, flags) as libc::c_int }
}
