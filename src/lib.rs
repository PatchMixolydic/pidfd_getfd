#![cfg_attr(feature = "nightly", feature(linux_pidfd))]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(not(target_os = "linux"))]
compile_error!("pidfds are currently only supported on Linux");

#[cfg(any(test, doctest))]
mod tests {
    /// Used to allow for `no_run` tests.
    ///
    /// Using `get_file_from_pidfd`:
    /// ```no_run (0 is an invalid pidfd)
    /// use pidfd_getfd::{get_file_from_pidfd, GetFdFlags};
    /// use std::{
    ///     io::{self, Read},
    ///     os::unix::prelude::RawFd,
    /// };
    ///
    /// fn main() -> io::Result<()> {
    ///     let pidfd: RawFd = 0;
    ///     let target_fd: RawFd = 0;
    ///     let mut file = get_file_from_pidfd(pidfd, target_fd, GetFdFlags::empty())?;
    ///     let mut buf = Vec::new();
    ///     file.read_to_end(&mut buf)?;
    ///     println!("{:#?}", buf);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Using `pidfd`:
    /// ```no_run (/usr/bin/foo likely does not exist and might close fd 1 regardless)
    /// use pidfd::PidFd;
    /// use pidfd_getfd::{GetFdFlags, PidFdExt};
    /// use std::{io, process::Command};
    ///
    /// fn main() -> io::Result<()> {
    ///     let child = Command::new("/usr/bin/foo").spawn().expect("failed to run `foo`");
    ///     let pidfd = PidFd::from_std_checked(&child)?;
    ///     let file_from_child = pidfd.get_file(1, GetFdFlags::empty())?;
    ///     Ok(())
    /// }
    /// ```
    struct _DocTests;

    #[cfg(feature = "nightly")]
    /// Used to allow for `no_run` tests.
    ///
    /// Using `std`'s `PidFd`:
    /// ```no_run (/usr/bin/foo likely does not exist and might close fd 1 regardless)
    /// #![feature(linux_pidfd)]
    ///
    /// use pidfd_getfd::{GetFdFlags, PidFdExt};
    /// use std::{
    ///     io,
    ///     os::linux::process::{ChildExt, CommandExt},
    ///     process::Command,
    /// };
    ///
    /// fn main() -> io::Result<()> {
    ///     let child = Command::new("/usr/bin/foo")
    ///         .create_pidfd(true)
    ///         .spawn()
    ///         .expect("failed to run `foo`");
    ///
    ///     let file_from_child = child.pidfd()?.get_file(1, GetFdFlags::empty())?;
    ///     Ok(())
    /// }
    /// ```
    struct _NightlyDocTests;
}
