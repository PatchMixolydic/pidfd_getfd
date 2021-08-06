#![deny(unsafe_op_in_unsafe_fn)]

use libc::{syscall, SYS_pidfd_getfd};
use pidfd::PidFd;
use std::{
    fs::File,
    io,
    os::unix::prelude::{AsRawFd, FromRawFd, RawFd},
};

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

pub trait PidFdExt {
    fn get_fd(&self, target_fd: RawFd, flags: GetfdFlags) -> Result<File, io::Error>;
}

impl PidFdExt for PidFd {
    fn get_fd(&self, target_fd: RawFd, flags: GetfdFlags) -> Result<File, io::Error> {
        get_file_from_pidfd(self.as_raw_fd(), target_fd, flags)
    }
}

/// Takes the file description referred to by `target_fd` within `pidfd`
/// and creates a duplicate of it within this process.
///
/// This is a convenience wrapper. For the raw syscall, see [`pidfd_getfd`].
pub fn get_file_from_pidfd(
    pidfd: RawFd,
    target_fd: RawFd,
    flags: GetfdFlags,
) -> Result<File, io::Error> {
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
